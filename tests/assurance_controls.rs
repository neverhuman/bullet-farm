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
