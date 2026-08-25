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
    let spaced = temp.path().join("member root");
    let injected = format!("; /usr/bin/touch {}", marker.display());
    let mut path = OsString::from(bin.as_os_str());
    path.push(":");
    path.push(env::var_os("PATH").expect("PATH is set"));

    let status = Command::new("just")
        .args([
            "setup",
            "--root",
            spaced.to_str().expect("UTF-8 fixture path"),
            &injected,
        ])
        .current_dir(root())
        .env("PATH", path)
        .env("BULLET_TEST_CAPTURE", &capture)
        .env("BULLET_SETUP_CARGO_BIN", "/bin/true")
        .env("BULLET_SETUP_NODE_BIN", "/bin/true")
        .env("BULLET_SETUP_NPM_CLI", "/bin/true")
        .status()
        .expect("run setup recipe");

    assert!(status.success());
    assert!(
        !marker.exists(),
        "recipe executed an interpolated shell command"
    );
    let captured = fs::read(&capture).expect("captured Cargo argv");
    let arguments = captured
        .split(|byte| *byte == 0)
        .filter(|argument| !argument.is_empty())
        .map(|argument| String::from_utf8(argument.to_vec()).expect("UTF-8 argument"))
        .collect::<Vec<_>>();
    let expected = ["--root".to_owned(), spaced.display().to_string(), injected];
    assert!(
        arguments.ends_with(&expected),
        "setup argv did not preserve its literal tail: {arguments:?}"
    );
}
