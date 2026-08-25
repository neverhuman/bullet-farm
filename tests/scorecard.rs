//! The scorecard instrument scores floors until evidence is admitted.

use std::path::PathBuf;

use bullet_family::scorecard::{evaluate, render_markdown};
use serde_json::Value;

fn hub() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn add_unknown_field(value: &mut Value) {
    value["unexpected"] = Value::Bool(true);
}

fn duplicate_dimension(value: &mut Value) {
    value["dimensions"][1]["id"] = Value::from(1);
}

fn set_bad_weight(value: &mut Value) {
    value["dimensions"][0]["weight"] = Value::from(13);
}

fn redistribute_weights(value: &mut Value) {
    value["dimensions"][0]["weight"] = Value::from(13);
    value["dimensions"][1]["weight"] = Value::from(12);
}

fn change_blend(value: &mut Value) {
    value["blend"]["architecture"] = Value::from(0.3);
    value["blend"]["implemented"] = Value::from(0.5);
}

fn change_baseline(value: &mut Value) {
    value["architecture_implemented_floor"] = Value::from(100.0);
}

fn change_dimension_floor(value: &mut Value) {
    value["dimensions"][0]["implemented_floor"] = Value::from(65);
}

fn change_dimension_name(value: &mut Value) {
    value["dimensions"][0]["name"] = Value::from("Authority-ish");
}

fn change_rubric(value: &mut Value) {
    value["rubric"] = Value::from("d2-v2");
}

fn duplicate_row(value: &mut Value) {
    value["rows"][1]["id"] = value["rows"][0]["id"].clone();
}

fn set_unknown_kind(value: &mut Value) {
    value["rows"][0]["kind"] = Value::from("file");
}

fn change_row_claim(value: &mut Value) {
    value["rows"][0]["claim"] = Value::from("Everything is done");
}

fn change_row_mapping(value: &mut Value) {
    value["rows"][0]["dimension"] = Value::from(2);
}

fn set_path_shaped_evidence(value: &mut Value) {
    value["rows"][0]["evidence"] = serde_json::json!({
        "source": "ci-observation",
        "subject_id": "/etc/passwd"
    });
}

#[test]
fn rubric_loads_and_keeps_implemented_floors() {
    let report = evaluate(&hub()).expect("scorecard");
    assert_eq!(report.rubric, "d2-v1");
    assert_eq!(report.dimensions.len(), 12);
    assert!(
        (report.implemented - 39.7).abs() < 0.15,
        "implemented={}",
        report.implemented
    );
    assert!(
        (report.blended - 43.3).abs() < 0.15,
        "blended={}",
        report.blended
    );
    let txn = report
        .rows
        .iter()
        .find(|row| row.id == "g2.transaction-proof")
        .expect("g2");
    assert!(!txn.admitted);
    assert!(!report.authoritative);
    assert!(report.rows.iter().all(|row| !row.admitted));
    let evolution = report
        .rows
        .iter()
        .find(|row| row.id == "d7.evolution-off")
        .expect("evolution row");
    assert_eq!(evolution.refusal_reason, "NO_EVIDENCE_REFERENCE");

    let original: Value =
        serde_json::from_slice(&std::fs::read(hub().join("policy/scorecard-v1.json")).unwrap())
            .unwrap();
    for (name, mutate) in [
        ("unknown field", add_unknown_field as fn(&mut Value)),
        ("duplicate dimension", duplicate_dimension),
        ("bad weight", set_bad_weight),
        ("redistributed weights", redistribute_weights),
        ("changed blend", change_blend),
        ("changed baseline", change_baseline),
        ("changed dimension floor", change_dimension_floor),
        ("changed dimension name", change_dimension_name),
        ("changed rubric", change_rubric),
        ("duplicate row", duplicate_row),
        ("unknown kind", set_unknown_kind),
        ("changed row claim", change_row_claim),
        ("changed row mapping", change_row_mapping),
        ("path-shaped evidence", set_path_shaped_evidence),
    ] {
        let mut hostile = original.clone();
        mutate(&mut hostile);
        let directory = tempfile::tempdir().unwrap();
        std::fs::create_dir(directory.path().join("policy")).unwrap();
        std::fs::write(
            directory.path().join("policy/scorecard-v1.json"),
            serde_json::to_vec(&hostile).unwrap(),
        )
        .unwrap();
        assert!(evaluate(directory.path()).is_err(), "{name} was accepted");
    }
}

#[test]
fn markdown_names_the_instrument() {
    let report = evaluate(&hub()).expect("scorecard");
    let page = render_markdown(&report);
    assert!(page.contains("not release authority"));
    assert!(page.contains("frozen baseline estimate"));
    assert!(
        page.contains("SEMANTIC_VERIFIER_UNAVAILABLE") || page.contains("NO_EVIDENCE_REFERENCE")
    );
    assert!(page.contains("g2.transaction-proof"));
    assert!(!page.contains("| yes |"));
    assert_eq!(
        page,
        include_str!("../docs/assurance/scorecard.generated.md"),
        "tracked generated scorecard must equal the renderer byte-for-byte"
    );
}
