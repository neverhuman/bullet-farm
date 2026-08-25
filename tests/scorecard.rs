//! The scorecard instrument scores floors until evidence is admitted.

use std::path::PathBuf;

use bullet_family::scorecard::{evaluate, render_markdown};

fn hub() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
}

#[test]
fn markdown_names_the_instrument() {
    let report = evaluate(&hub()).expect("scorecard");
    let page = render_markdown(&report);
    assert!(page.contains("not release authority"));
    assert!(page.contains("g2.transaction-proof"));
}
