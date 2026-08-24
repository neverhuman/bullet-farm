use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

fn command(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bullet-family"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run bullet-family")
}

fn unsupported_lock_fixture() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "bullet-check-cli-unsupported-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("remove stale fixture");
    }
    let hub = root.join("bullet-farm");
    fs::create_dir_all(hub.join("scripts")).expect("fixture directories");
    fs::write(
        hub.join("Cargo.toml"),
        "[package]\nname='bullet-family'\nversion='0.0.0'\n",
    )
    .expect("Cargo fixture");
    fs::write(hub.join("scripts/setup.sh"), "#!/bin/sh\nexit 1\n").expect("setup fixture");
    fs::write(
        hub.join("repos.manifest.toml"),
        concat!(
            "schema_version = \"1.2.0\"\n",
            "family = \"bullet-farm\"\n",
            "umbrella_repo = \"bullet-farm\"\n",
            "required_repos = [\"bullet-farm\", \"bullet-kernel\", \"bullet-git\", \"bullet-portal\"]\n",
        ),
    )
    .expect("manifest fixture");
    fs::write(
        hub.join("family.lock"),
        "schema_version = \"unsupported\"\n",
    )
    .expect("lock fixture");
    root
}

#[test]
fn blocked_report_is_printed_before_nonzero_exit() {
    let output = command(&["check", "fast"]);
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("check FAST: BLOCKED\n"));
    assert!(stdout.contains("repair:"));

    let explicit = command(&[
        "--root",
        env!("CARGO_MANIFEST_DIR"),
        "check",
        "fast",
        "--json",
    ]);
    assert_eq!(explicit.status.code(), Some(3));
    assert!(explicit.stderr.is_empty());
}

#[test]
fn json_is_stable_sorted_and_every_selected_release_gate_is_blocked() {
    let first = command(&["check", "release", "--json"]);
    let second = command(&["check", "release", "--json"]);
    assert_eq!(first.status.code(), Some(3));
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty());
    let report: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["command"], "check");
    assert_eq!(report["tier"], "RELEASE");
    assert_eq!(report["status"], "BLOCKED");
    let gates = report["gates"].as_array().unwrap();
    assert_eq!(gates.len(), 26);
    assert!(gates.iter().all(|gate| {
        gate["status"] == "BLOCKED"
            && gate["repair"]
                .as_str()
                .is_some_and(|repair| !repair.is_empty())
    }));
    assert!(
        gates
            .windows(2)
            .all(|pair| pair[0]["id"].as_str() < pair[1]["id"].as_str())
    );
}

#[test]
fn required_is_blocked_and_arguments_are_strict() {
    assert_eq!(
        command(&["check", "required", "--json"]).status.code(),
        Some(3)
    );
    for args in [
        vec!["check"],
        vec!["check", "other"],
        vec!["check", "fast", "--yaml"],
        vec!["check", "fast", "--json", "--json"],
        vec!["check", "--json", "fast"],
    ] {
        let output = command(&args);
        assert_eq!(output.status.code(), Some(2), "args={args:?}");
        assert!(output.stdout.is_empty(), "args={args:?}");
        assert!(String::from_utf8(output.stderr).unwrap().contains("USAGE"));
    }
}

#[test]
fn legacy_success_and_corrupt_schema_exit_codes_remain_available() {
    let success = command(&["hub", "check"]);
    assert_eq!(success.status.code(), Some(0));
    assert_eq!(success.stdout, b"hub-check: ok\n");

    let fixture = unsupported_lock_fixture();
    let unsupported = command(&[
        "--root",
        fixture.to_str().expect("UTF-8 fixture"),
        "checkout",
        "verify",
    ]);
    assert_eq!(unsupported.status.code(), Some(4));
    assert!(unsupported.stdout.is_empty());
    assert!(
        String::from_utf8(unsupported.stderr)
            .unwrap()
            .contains("UNSUPPORTED_SCHEMA")
    );
    fs::remove_dir_all(fixture).expect("remove fixture");
}
