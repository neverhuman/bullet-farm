//! Static negative inventory. No file or environment input can clear these gates.

use super::model::{CheckModelError, CheckReport, CheckTier, GateClass, GateResult};

pub(super) fn report(tier: CheckTier) -> Result<CheckReport, CheckModelError> {
    let gates = match tier {
        CheckTier::Fast => vec![blocked(
            "catalog.fast",
            GateClass::Component,
            "the Rust fast gate catalog and bounded executor have not landed",
            "implement the admitted Rust fast catalog, affected-path routing, and generated-drift gates",
        )?],
        CheckTier::Required => vec![blocked(
            "catalog.required",
            GateClass::Component,
            "the Rust required gate catalog and proof parsers have not landed",
            "implement the no-skip Rust required catalog, zero-test admission, and exact family subject checks",
        )?],
        CheckTier::Release => release_gates()?,
    };
    CheckReport::new(tier, gates)
}

fn release_gates() -> Result<Vec<GateResult>, CheckModelError> {
    [
        (
            "release.backup-restore",
            GateClass::Release,
            "no exact-subject backup and restore receipt is registered",
            "run the tagged backup/restore suite and register its signed release receipt",
        ),
        (
            "release.checksums",
            GateClass::Release,
            "release archive checksums are absent",
            "generate and verify checksums for every exact release archive",
        ),
        (
            "release.fault-suite",
            GateClass::Release,
            "the crash-boundary and recovery fault receipt is absent",
            "run the tagged fault suite and register its exact signed receipt",
        ),
        (
            "release.forge.github-app",
            GateClass::Live,
            "no protected GitHub App integration and reconciliation receipt is registered",
            "configure the test repository and register exact dispatch, read-back, reconciliation, and integration evidence",
        ),
        (
            "release.forge.jeryu",
            GateClass::Live,
            "no protected Jeryu integration and reconciliation receipt is registered",
            "restore operator authentication and register exact protected integration and reconciliation evidence",
        ),
        (
            "release.installable-lock",
            GateClass::Release,
            "the checked-in family lock is not an installable signed schema-3 release lock",
            "publish immutable signed member tags and generate the authenticated schema-3 family lock",
        ),
        (
            "release.installer-twice",
            GateClass::Release,
            "no two-run fresh-HOME installer receipt from tagged hub-only bytes is registered",
            "verify the signed prebuilt installer twice with exact clean member OIDs and no worktrees",
        ),
        (
            "release.jankurai-90",
            GateClass::Release,
            "Jankurai release evidence at score 90 with zero hard findings and caps is absent",
            "resolve all release findings and register the pinned Jankurai >=90 zero-hard/zero-cap receipt",
        ),
        (
            "release.manifest-non-circular",
            GateClass::Release,
            "the final non-circular signed release manifest is absent",
            "generate and sign a manifest that binds the hub tag without embedding its own digest",
        ),
        (
            "release.package-matrix",
            GateClass::Release,
            "the five required platform archives with the embedded Portal are absent",
            "build and smoke Linux x86_64/aarch64, macOS x86_64/arm64, and Windows x64 archives from the exact tagged family with the Portal embedded",
        ),
        (
            "release.platform-containment",
            GateClass::Release,
            "platform containment or fail-closed refusal receipts are absent",
            "prove Linux production containment and mutation refusal on every unsupported packaged platform",
        ),
        (
            "release.provenance",
            GateClass::Release,
            "signed build provenance for the package matrix is absent",
            "produce and verify provenance bound to the exact hub tag, lock, toolchains, and archives",
        ),
        (
            "release.provider.antigravity",
            GateClass::Live,
            "Antigravity release conformance evidence is absent",
            "register the exact adapter/version/profile isolation, failure, quota, and patch conformance receipt",
        ),
        (
            "release.provider.claude",
            GateClass::Live,
            "Claude release conformance evidence is absent",
            "register the exact adapter/version/profile isolation, failure, quota, and patch conformance receipt",
        ),
        (
            "release.provider.codex",
            GateClass::Live,
            "Codex release conformance evidence is absent",
            "register the exact adapter/version/profile isolation, failure, quota, and patch conformance receipt",
        ),
        (
            "release.provider.cursor",
            GateClass::Live,
            "Cursor release conformance evidence is absent",
            "register the exact adapter/version/profile isolation, failure, quota, and patch conformance receipt",
        ),
        (
            "release.receipt-contracts",
            GateClass::Release,
            "frozen signed release receipt schemas and verification trust roots do not exist",
            "freeze generated release receipt and manifest contracts with pinned verification roots before accepting evidence",
        ),
        (
            "release.rust-msrv-1-95",
            GateClass::Release,
            "an exact release build receipt for Rust 1.95 is absent",
            "build and test the exact tagged family with the admitted Rust 1.95 MSRV toolchain",
        ),
        (
            "release.rust-pinned-1-97-1",
            GateClass::Release,
            "an exact release build receipt for pinned Rust 1.97.1 is absent",
            "build and test the exact tagged family with the admitted pinned Rust 1.97.1 toolchain",
        ),
        (
            "release.scan.dependency",
            GateClass::Release,
            "the pinned dependency scan receipt is absent",
            "run the admitted dependency scanner against exact lockfiles and register its receipt",
        ),
        (
            "release.scan.license",
            GateClass::Release,
            "the pinned license policy scan receipt is absent",
            "run the admitted license scanner against exact artifacts and register its receipt",
        ),
        (
            "release.scan.secret",
            GateClass::Release,
            "the pinned secret scan receipt is absent",
            "run the admitted secret scanner against exact tagged trees and register its receipt",
        ),
        (
            "release.scan.workflow",
            GateClass::Release,
            "the pinned workflow policy scan receipt is absent",
            "run the admitted workflow scanner against exact workflow bytes and register its receipt",
        ),
        (
            "release.sbom",
            GateClass::Release,
            "software bills of materials for the package matrix are absent",
            "generate and validate an SBOM for every exact release archive",
        ),
        (
            "release.signatures",
            GateClass::Release,
            "verified signatures for the package matrix are absent",
            "sign every archive, checksum set, SBOM, provenance statement, and final manifest with admitted release keys",
        ),
        (
            "release.transaction-demo",
            GateClass::Transaction,
            "the exact offline five-plane transaction demo receipt is absent",
            "run the non-synthetic tagged transaction demo and register its independent exact-subject receipt",
        ),
    ]
    .into_iter()
    .map(|(id, class, detail, repair)| blocked(id, class, detail, repair))
    .collect()
}

fn blocked(
    id: &str,
    class: GateClass,
    detail: &str,
    repair: &str,
) -> Result<GateResult, CheckModelError> {
    GateResult::blocked(id, class, detail, repair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::model::GateStatus;

    #[test]
    fn every_foundation_profile_is_explicitly_blocked_with_repair() {
        for tier in [CheckTier::Fast, CheckTier::Required, CheckTier::Release] {
            let report = report(tier).unwrap();
            assert_eq!(report.status(), GateStatus::Blocked);
            assert_eq!(report.exit_code(), 3);
            assert!(report.gates().iter().all(|gate| {
                gate.status() == GateStatus::Blocked
                    && gate.repair().is_some_and(|repair| !repair.is_empty())
            }));
        }
    }

    #[test]
    fn release_inventory_is_complete_and_cannot_be_cleared_by_input() {
        let report = report(CheckTier::Release).unwrap();
        let ids = report
            .gates()
            .iter()
            .map(GateResult::id)
            .collect::<Vec<_>>();
        for required in [
            "release.receipt-contracts",
            "release.rust-msrv-1-95",
            "release.rust-pinned-1-97-1",
            "release.installable-lock",
            "release.transaction-demo",
            "release.jankurai-90",
            "release.package-matrix",
            "release.installer-twice",
            "release.sbom",
            "release.checksums",
            "release.signatures",
            "release.provenance",
            "release.manifest-non-circular",
            "release.backup-restore",
            "release.fault-suite",
            "release.forge.jeryu",
            "release.forge.github-app",
            "release.provider.claude",
            "release.provider.codex",
            "release.provider.cursor",
            "release.provider.antigravity",
            "release.platform-containment",
            "release.scan.dependency",
            "release.scan.license",
            "release.scan.secret",
            "release.scan.workflow",
        ] {
            assert!(ids.contains(&required), "missing {required}");
        }
        assert_eq!(ids.len(), 26);
    }
}
