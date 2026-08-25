const DEMO_LAUNCHER: &str = include_str!("../scripts/demo.sh");

#[test]
fn demo_launcher_uses_a_fresh_default_and_preserves_explicit_data() {
    assert!(
        DEMO_LAUNCHER.contains("if [[ -n \"${BULLET_DATA_DIR:-}\" ]]"),
        "an explicit demo data directory must remain supported"
    );
    assert!(
        DEMO_LAUNCHER.contains("DATA=\"$BULLET_DATA_DIR\""),
        "the explicit demo data directory must be used exactly"
    );
    assert!(
        DEMO_LAUNCHER.contains("DATA=\"$(mktemp -d \"$DEMO_ROOT/run.XXXXXXXX\")\""),
        "the default demo must allocate a fresh bounded run directory"
    );
    assert!(
        !DEMO_LAUNCHER.contains("DATA=\"${BULLET_DATA_DIR:-$KERNEL/target/demo}\""),
        "the launcher must not reuse one schema-sensitive default database"
    );
    assert!(
        !DEMO_LAUNCHER.contains("rm -"),
        "the launcher must preserve prior demo runs instead of deleting them"
    );
}

#[test]
fn demo_launcher_labels_synthetic_evidence_before_running_the_simulator() {
    let evidence_label = DEMO_LAUNCHER
        .find("echo \"evidence_class: SYNTHETIC_PROOF\"")
        .expect("demo must print its synthetic evidence class");
    let release_label = DEMO_LAUNCHER
        .find("echo \"release_gate_eligible: false\"")
        .expect("demo must print that it cannot clear a release gate");
    let kernel_run = DEMO_LAUNCHER
        .find("cargo run -q -p bullet -- demo")
        .expect("demo must still run the Kernel simulator");

    assert!(evidence_label < kernel_run);
    assert!(release_label < kernel_run);
}
