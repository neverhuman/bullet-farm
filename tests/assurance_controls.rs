use std::{collections::BTreeMap, fs, path::Path, process::Command};

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn parse_toml(path: &str) -> TomlValue {
    toml::from_str(&read(path)).unwrap_or_else(|error| panic!("parse {path}: {error}"))
}

#[test]
fn control_manifests_route_to_real_local_proof() {
    let lanes = parse_toml("agent/proof-lanes.toml");
    let actual = lanes["lane"]
        .as_array()
        .expect("lane array")
        .iter()
        .map(|lane| {
            (
                lane["name"].as_str().expect("lane name"),
                lane["command"].as_str().expect("lane command"),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        ("audit", "just audit"),
        ("contract", "just contract"),
        ("contract-drift", "just contract-check"),
        ("family", "just family"),
        ("fast", "just fast"),
        ("required", "just check"),
        ("security", "just security"),
    ]);
    assert_eq!(actual, expected);

    let security = parse_toml("agent/security-policy.toml");
    let required = security["required_tools"]
        .as_array()
        .expect("required tools");
    assert_eq!(required.len(), 3);
    assert!(
        required
            .iter()
            .any(|tool| tool.as_str() == Some("gitleaks"))
    );
    assert!(
        required
            .iter()
            .any(|tool| tool.as_str() == Some("cargo-deny"))
    );
    assert!(required.iter().any(|tool| tool.as_str() == Some("zizmor")));

    let boundaries = parse_toml("agent/boundaries.toml");
    assert_eq!(
        boundaries["stack"]["id"].as_str(),
        Some("rust-ts-vite-react-sqlite")
    );
    assert_eq!(
        boundaries["rust"]["domain_paths"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(
        boundaries["db"]["root_paths"].as_array().map(Vec::len),
        Some(0)
    );

    let audit = parse_toml("agent/audit-policy.toml");
    assert_eq!(audit["minimum_score"].as_integer(), Some(58));
    assert_eq!(
        audit["fail_on"]
            .as_array()
            .expect("audit fail_on")
            .iter()
            .filter_map(TomlValue::as_str)
            .collect::<Vec<_>>(),
        ["critical"]
    );
    assert_eq!(
        audit["scan"]["excluded_paths"].as_array().map(Vec::len),
        Some(0),
        "audit policy must not hide source paths to raise the score"
    );

    let adoption = parse_toml("agent/tool-adoption.toml");
    let tools = adoption["tools"].as_array().expect("adopted tools");
    assert_eq!(tools.len(), 1, "unproved tool adoption was declared");
    assert_eq!(tools[0]["id"].as_str(), Some("audit-ci"));
    assert_eq!(tools[0]["mode"].as_str(), Some("auto"));

    let audit_lane = read("ops/ci/audit.sh");
    assert!(audit_lane.contains("--policy \"$AUDIT_POLICY\""));
    assert!(!audit_lane.contains("--fail-under"));
    assert!(!audit_lane.contains("--fail-on"));
}

#[test]
fn exception_repairs_and_documented_budgets_are_complete_and_policy_bound() {
    let exceptions = parse_toml("agent/exceptions.toml");
    let surface = exceptions["exception_surface"]
        .as_table()
        .expect("exception surface");
    assert_eq!(surface["owner"].as_str(), Some("ops"));
    assert_eq!(surface["docs_url"].as_str(), Some("docs/errors.md"));
    let required = surface["required_fields"]
        .as_array()
        .expect("required exception fields")
        .iter()
        .map(|field| field.as_str().expect("field name"))
        .collect::<Vec<_>>();
    assert_eq!(
        required,
        [
            "purpose",
            "reason",
            "common_fixes",
            "docs_url",
            "repair_hint"
        ]
    );

    let kinds = exceptions["error_kind"].as_array().expect("error kinds");
    assert!(kinds.len() >= 8, "repair taxonomy is too narrow");
    for kind in kinds {
        for field in &required {
            let value = &kind[*field];
            let populated = value.as_str().is_some_and(|text| !text.trim().is_empty())
                || value.as_array().is_some_and(|items| !items.is_empty());
            assert!(populated, "error kind missing {field}: {kind:?}");
        }
    }

    let errors = read("docs/errors.md");
    for heading in [
        "## Invalid input",
        "## Conflict or changed subject",
        "## Unsupported or corrupt state",
        "## Dependency unavailable",
        "## Verification failed",
        "## Outcome unknown",
        "## Receipt missing",
    ] {
        assert!(
            errors.contains(heading),
            "missing repair heading: {heading}"
        );
    }

    let policy: JsonValue =
        serde_json::from_str(&read("policy/v1alpha1/policy.json")).expect("policy JSON");
    let budget = &policy["budget_policy"];
    let testing = read("docs/testing.md");
    for (field, label) in [
        ("maximum_lease_ttl_seconds", "15-second maximum lease TTL"),
        ("maximum_attempt_seconds", "1,800-second maximum Attempt"),
        ("maximum_changed_paths", "128 changed paths"),
    ] {
        assert!(budget[field].as_u64().is_some(), "missing policy {field}");
        assert!(
            testing.contains(label),
            "testing guide does not explain current policy field {field}"
        );
    }
    assert_eq!(budget["unknown_quota_is_headroom"].as_bool(), Some(false));
    assert!(testing.contains("`unknown_quota_is_headroom` is `false`"));
    assert!(read("docs/exceptions/README.md").contains("expiry date"));

    let docs_index = read("docs/README.md");
    for entrypoint in ["architecture.md", "boundaries.md", "errors.md"] {
        assert!(
            docs_index.contains(entrypoint),
            "documentation entrypoint is not indexed: {entrypoint}"
        );
    }
    assert!(read("docs/architecture.md").contains("## Current proof boundary"));
    assert!(read("docs/boundaries.md").contains("## Credential custody"));
}

#[test]
fn every_generated_zone_has_source_and_executable_regeneration_route() {
    let manifest = parse_toml("agent/generated-zones.toml");
    let zones = manifest["zone"].as_array().expect("zone array");
    let mut paths = Vec::new();
    for zone in zones {
        for field in ["path", "source", "command"] {
            assert!(
                zone[field]
                    .as_str()
                    .is_some_and(|value| !value.trim().is_empty()),
                "generated zone missing {field}: {zone:?}"
            );
        }
        assert_eq!(zone["read_only"].as_bool(), Some(true));
        let command = zone["command"].as_str().expect("command");
        assert!(
            command.starts_with("just ")
                || command.starts_with("bash ")
                || command.starts_with("cargo run --locked "),
            "unsupported regeneration route: {command}"
        );
        paths.push(zone["path"].as_str().expect("path"));
    }
    for expected in [
        "family.lock",
        ".fusion/",
        "contracts/v1alpha1/schema-bundle.json",
        "contracts/v1alpha1/bundle-manifest.json",
        "contracts/generated/",
        "policy/v1alpha1/policy.json",
        "fixtures/hostile/cases/",
        "fixtures/hostile/fixture-manifest.json",
        "fixtures/canonical/canonical-golden.json",
        "fixtures/canonical/authority-golden.json",
        "docs/assurance/invariant-crosswalk.generated.md",
        "docs/assurance/release-truth.generated.md",
        "formal/traces/",
    ] {
        assert!(
            paths.contains(&expected),
            "generated zone is unrouted: {expected}"
        );
    }
}

#[test]
fn ignored_and_top_level_surfaces_have_owner_and_test_routes() {
    let owners: JsonValue = serde_json::from_str(&read("agent/owner-map.json")).expect("owner map");
    let tests: JsonValue = serde_json::from_str(&read("agent/test-map.json")).expect("test map");
    for path in [
        ".fusion/",
        ".gitignore",
        ".jankurai/",
        "Justfile",
        "LICENSE",
        "SPLIT.md",
        "rust-toolchain.toml",
    ] {
        assert!(
            owners["owners"].get(path).is_some(),
            "owner route missing: {path}"
        );
        assert!(
            tests["tests"].get(path).is_some(),
            "test route missing: {path}"
        );
    }
}

#[test]
fn hosted_workflow_is_pinned_and_delegates_to_local_entrypoints() {
    let workflow = read(".github/workflows/ci.yml");
    for command in [
        "run: bash ops/ci/fast.sh",
        "run: bash ops/ci/required.sh",
        "run: bash ops/ci/contract.sh",
        "run: bash ops/ci/security.sh",
    ] {
        assert!(
            workflow.contains(command),
            "missing workflow delegation: {command}"
        );
    }
    assert!(!workflow.contains("run: bash scripts/ci-local.sh"));
    for line in workflow
        .lines()
        .filter(|line| line.trim_start().starts_with("- uses:"))
    {
        let reference = line.split_once('@').expect("action pin").1;
        let revision = reference
            .split_whitespace()
            .next()
            .expect("action revision");
        assert_eq!(revision.len(), 40, "action is not pinned: {line}");
        assert!(
            revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "invalid action pin: {line}"
        );
    }
    assert!(read("rust-toolchain.toml").contains("channel = \"1.95.0\""));
    assert!(workflow.contains("zizmor@1.25.2"));
}

#[test]
fn doctor_and_pre_push_gate_are_executable_controls() {
    let doctor = Command::new("bash")
        .args(["scripts/ci-doctor.sh", "fast"])
        .current_dir(root())
        .env_remove("RUSTUP_TOOLCHAIN")
        .status()
        .expect("run CI doctor");
    assert!(doctor.success());
    assert!(read("ops/git-hooks/pre-push").contains("ops/ci/quality-gates.sh"));
    assert!(read("ops/ci/quality-gates.sh").contains("exec bash ops/ci/fast.sh"));
    assert!(read("tools/security-lane.sh").contains("ops/ci/security.sh"));
}

#[test]
fn public_lock_recipe_uses_verify_vocabulary() {
    let justfile = read("Justfile");
    assert!(justfile.contains("lock-verify tag:"));
    assert!(justfile.contains("-- lock verify --tag \"$1\""));
    assert!(!justfile.contains("--tag {{tag}}"));
    assert!(!justfile.contains("lock-check"));
    assert!(!justfile.contains("-- lock check"));
}

#[cfg(unix)]
#[test]
fn parameterized_recipes_preserve_literal_arguments() {
    use std::{env, ffi::OsString, os::unix::fs::PermissionsExt};

    let temp = tempfile::tempdir().expect("parameterized recipe fixture");
    let bin = temp.path().join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    let cargo = bin.join("cargo");
    fs::write(
        &cargo,
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\0' \"$@\" >\"${BULLET_TEST_CAPTURE:?}\"\n",
    )
    .expect("write fake cargo");
    let mut permissions = fs::metadata(&cargo)
        .expect("fake cargo metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&cargo, permissions).expect("make fake cargo executable");

    let capture = temp.path().join("argv");
    let marker = temp.path().join("injected");
    let injected = format!("invalid; /usr/bin/touch {}", marker.display());
    let mut path = OsString::from(bin.as_os_str());
    path.push(":");
    path.push(env::var_os("PATH").expect("PATH is set"));

    let invoke = |recipe: &str, arguments: &[&str]| {
        let status = Command::new("just")
            .arg(recipe)
            .args(arguments)
            .current_dir(root())
            .env("PATH", &path)
            .env("BULLET_TEST_CAPTURE", &capture)
            .status()
            .unwrap_or_else(|error| panic!("run {recipe} recipe: {error}"));
        assert!(status.success(), "{recipe} recipe failed");
        assert!(!marker.exists(), "{recipe} executed caller text as shell");
        fs::read(&capture)
            .expect("captured Cargo argv")
            .split(|byte| *byte == 0)
            .filter(|argument| !argument.is_empty())
            .map(|argument| String::from_utf8(argument.to_vec()).expect("UTF-8 argument"))
            .collect::<Vec<_>>()
    };

    let coord = invoke("coord", &["--agent", &injected]);
    assert!(
        coord.ends_with(&["coord".to_owned(), "--agent".to_owned(), injected.clone()]),
        "coord did not preserve its literal argv tail: {coord:?}"
    );

    for (recipe, operation) in [("lock-generate", "generate"), ("lock-verify", "verify")] {
        let arguments = invoke(recipe, &[&injected]);
        let expected = [
            "lock".to_owned(),
            operation.to_owned(),
            "--tag".to_owned(),
            injected.clone(),
        ];
        assert!(
            arguments.ends_with(&expected),
            "{recipe} did not preserve its literal argv tail: {arguments:?}"
        );
    }

    let doctor = Command::new("just")
        .args(["ci-doctor", &injected])
        .current_dir(root())
        .env("PATH", path)
        .status()
        .expect("run ci-doctor recipe");
    assert!(!doctor.success(), "invalid doctor lane was accepted");
    assert!(!marker.exists(), "ci-doctor executed caller text as shell");
}

#[cfg(unix)]
#[test]
fn setup_recipe_preserves_literal_arguments() {
    use std::{env, ffi::OsString, os::unix::fs::PermissionsExt};

    let temp = tempfile::tempdir().expect("setup recipe fixture");
    let bin = temp.path().join("bin");
    fs::create_dir(&bin).expect("create fake bin");
    let capture = temp.path().join("argv");
    let marker = temp.path().join("ambient-cargo-executed");
    let cargo = bin.join("cargo");
    fs::write(
        &cargo,
        format!("#!/bin/bash\nprintf executed > '{}'\n", marker.display()),
    )
    .expect("write fake cargo");
    let mut permissions = fs::metadata(&cargo)
        .expect("fake cargo metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&cargo, permissions).expect("make fake cargo executable");

    let setup = temp.path().join("bullet-family");
    fs::write(
        &setup,
        format!(
            "#!/bin/bash\nset -euo pipefail\nprintf '%s\\0' \"$@\" > '{}'\n",
            capture.display()
        ),
    )
    .expect("write fake setup binary");
    let mut permissions = fs::metadata(&setup)
        .expect("fake setup metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&setup, permissions).expect("make fake setup executable");

    let injection_marker = temp.path().join("injected");
    let spaced = temp.path().join("member root");
    let injected = format!("; /usr/bin/touch {}", injection_marker.display());
    let mut path = OsString::from(bin.as_os_str());
    path.push(":");
    path.push(env::var_os("PATH").expect("PATH is set"));

    let refused = Command::new("just")
        .arg("setup")
        .current_dir(root())
        .env("PATH", &path)
        .env_remove("BULLET_SETUP_ADMITTED_BIN")
        .env("BULLET_SETUP_CARGO_BIN", "/bin/true")
        .env("BULLET_SETUP_NODE_BIN", "/bin/true")
        .env("BULLET_SETUP_NPM_CLI", "/bin/true")
        .output()
        .expect("run setup without admitted bootstrap");
    assert!(!refused.status.success());
    assert!(
        String::from_utf8_lossy(&refused.stderr)
            .contains("operator-pre-admitted bootstrap unavailable"),
        "unexpected refusal: {}",
        String::from_utf8_lossy(&refused.stderr)
    );
    assert!(!marker.exists(), "missing bootstrap executed ambient Cargo");

    let in_family = Command::new("just")
        .arg("setup")
        .current_dir(root())
        .env("PATH", &path)
        .env(
            "BULLET_SETUP_ADMITTED_BIN",
            env!("CARGO_BIN_EXE_bullet-family"),
        )
        .env("BULLET_SETUP_CARGO_BIN", "/bin/true")
        .env("BULLET_SETUP_NODE_BIN", "/bin/true")
        .env("BULLET_SETUP_NPM_CLI", "/bin/true")
        .output()
        .expect("run setup with in-family bootstrap");
    assert!(!in_family.status.success());
    assert!(
        String::from_utf8_lossy(&in_family.stderr).contains("outside the source family"),
        "unexpected refusal: {}",
        String::from_utf8_lossy(&in_family.stderr)
    );
    assert!(
        !marker.exists(),
        "in-family bootstrap executed ambient Cargo"
    );

    let status = Command::new("just")
        .args([
            "setup",
            "--root",
            spaced.to_str().expect("UTF-8 fixture path"),
            &injected,
        ])
        .current_dir(root())
        .env("PATH", path)
        .env("BULLET_SETUP_ADMITTED_BIN", &setup)
        .env("BULLET_SETUP_CARGO_BIN", "/bin/true")
        .env("BULLET_SETUP_NODE_BIN", "/bin/true")
        .env("BULLET_SETUP_NPM_CLI", "/bin/true")
        .status()
        .expect("run setup recipe");

    assert!(status.success());
    assert!(
        !injection_marker.exists(),
        "recipe executed an interpolated shell command"
    );
    assert!(
        !marker.exists(),
        "external bootstrap executed ambient Cargo"
    );
    let captured = fs::read(&capture).expect("captured setup argv");
    let arguments = captured
        .split(|byte| *byte == 0)
        .filter(|argument| !argument.is_empty())
        .map(|argument| String::from_utf8(argument.to_vec()).expect("UTF-8 argument"))
        .collect::<Vec<_>>();
    assert_eq!(
        arguments,
        [
            "setup".to_owned(),
            "--root".to_owned(),
            root().parent().unwrap().display().to_string(),
            "--source".to_owned(),
            "jeryu".to_owned(),
            "--cargo-bin".to_owned(),
            "/bin/true".to_owned(),
            "--node-bin".to_owned(),
            "/bin/true".to_owned(),
            "--npm-cli".to_owned(),
            "/bin/true".to_owned(),
            "--root".to_owned(),
            spaced.display().to_string(),
            injected,
        ]
    );
}
