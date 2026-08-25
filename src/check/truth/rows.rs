//! One falsifiable claim per registered release gate. Nightshift style: each
//! sentence names what remains unproved, never work remaining.
//!
//! Evidence class comes from the gate catalog, never from this table. A claim
//! sentence must never read as closed while its gate is unreceipted; the unit
//! tests below enforce vocabulary and one-to-one coverage of the catalog.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Owner {
    /// Closable offline from the checked-out family with admitted local tools.
    Local,
    /// Needs an operator credential, signer, service, or platform.
    External,
}

impl Owner {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Local => "LOCAL (closable offline)",
            Self::External => "EXTERNAL (needs operator credential, signer, service, or platform)",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ClaimRow {
    pub id: &'static str,
    pub claim: &'static str,
    pub why: &'static str,
    pub acceptance: &'static str,
    pub owner: Owner,
}

/// Words that would make an unreceipted claim read as closed.
pub(super) const CLOSED_WORDS: &[&str] = &["verified", "proven", "done", "complete"];

const fn row(
    id: &'static str,
    claim: &'static str,
    why: &'static str,
    acceptance: &'static str,
    owner: Owner,
) -> ClaimRow {
    ClaimRow {
        id,
        claim,
        why,
        acceptance,
        owner,
    }
}

const fn provider(id: &'static str, claim: &'static str) -> ClaimRow {
    row(
        id,
        claim,
        "A bounded offline parser proves nothing about the live adapter, its credentials, quota, isolation, or patch output.",
        "Register the exact adapter/version/profile receipt covering isolation, failure, quota, and patch conformance from one admitted live run with operator credentials.",
        Owner::External,
    )
}

pub(super) const ROWS: &[ClaimRow] = &[
    row(
        "release.backup-restore",
        "Nobody has restored a tagged backup and read the restored state back as an exact-subject release receipt.",
        "A backup whose restore has never been observed protects nothing; the pointed-at bytes, not the database, are the recovery subject.",
        "Run the tagged backup/restore suite from release bytes and register a signed receipt binding backup digest, restore epoch, and the exact four-repository subjects.",
        Owner::Local,
    ),
    row(
        "release.checksums",
        "No checksum set exists for any release archive, because no release archive exists.",
        "Without checksums an installer cannot tell substituted bytes from released bytes.",
        "Generate and re-read checksums for every exact archive in the five-target matrix and bind them in the signed release manifest.",
        Owner::Local,
    ),
    row(
        "release.fault-suite",
        "Nobody has crashed the tagged family at every SQLite/WAL/CAS/journal/generation boundary and read back either the prior or the whole next state.",
        "Crash safety that was only reasoned about is not crash safety; one unobserved boundary can lose or duplicate an effect.",
        "Run the tagged crash-boundary and recovery fault suite from release bytes and register its exact signed receipt.",
        Owner::Local,
    ),
    row(
        "release.forge.github-app",
        "No GitHub App has integrated an exact Candidate into a protected test repository and read the result back.",
        "GitHub is an effect adapter; until a lost response reconciles to UNKNOWN and read-back adopts the original OID, every GitHub effect is unproved.",
        "Configure the GitHub App test repository with branch protection and register exact-subject dispatch, check, integration, read-back, and reconciliation receipts.",
        Owner::External,
    ),
    row(
        "release.forge.jeryu",
        "No protected Jeryu integration has run with restored operator authentication and produced a read-back receipt.",
        "Jeryu is the source forge; an integration that never read its own result back cannot claim protected-ref safety.",
        "With operator-restored authentication run read-only probes, then one exact protected integration with UNKNOWN/read-back reconciliation, and register the receipt; never modify the running forge.",
        Owner::External,
    ),
    row(
        "release.installable-lock",
        "The checked-in family.lock is still schema 2, which every installer path rejects by design.",
        "Installation from the hub cannot start until a lock carries authenticated Jeryu sources and signed exact subjects.",
        "Publish immutable signed member tags, generate the schema-3 lock with Jeryu URL/slug, exact tree OIDs, lockfiles, and artifact digests, and commit it under a signed hub tag.",
        Owner::External,
    ),
    row(
        "release.installer-twice",
        "Nobody has run the signed prebuilt installer twice in a fresh HOME from tagged hub-only bytes.",
        "A source bootstrap that launches Cargo before admission is not an installer receipt; idempotence and exact clean member OIDs must be observed, not assumed.",
        "From a signed prebuilt bullet-family binary and tagged bytes, run setup twice in a fresh HOME and register exact clean member OIDs, zero tracked changes, and no worktrees.",
        Owner::External,
    ),
    row(
        "release.jankurai-90",
        "No pinned Jankurai report reads 90 or better with zero caps and zero hard findings for the exact release subjects.",
        "Hard findings and caps are release blockers; a machine-local or skip-green audit lane cannot substitute for the pinned receipt.",
        "Run pinned Jankurai 1.6.11 with zero skips against the exact tagged subjects and register the score/cap/hard-finding receipt at 90 or better with zero caps and zero hard findings.",
        Owner::Local,
    ),
    row(
        "release.manifest-non-circular",
        "No signed final release manifest binds the hub tag without embedding its own digest.",
        "A manifest that includes its own digest cannot be checked; a manifest without a signature cannot be trusted.",
        "Generate the final manifest binding hub tag, schema-3 lock, five byte-sorted target entries, and archive/SBOM/provenance digests, then sign it with the protected release key.",
        Owner::External,
    ),
    row(
        "release.package-matrix",
        "None of the five release archives (Linux x86_64/aarch64, macOS x86_64/arm64, Windows x64) has been built with the Portal embedded.",
        "A release is bytes on every declared platform; the Vite preview lane and a separately built farmd are not package evidence.",
        "Build and smoke all five archives from the exact tagged family with the Portal embedded in the Rust distribution and register each archive digest.",
        Owner::External,
    ),
    row(
        "release.platform-containment",
        "Only Linux containment has been observed live; no packaged non-Linux platform has been seen refusing real mutation.",
        "A platform without an equivalent containment backend must fail closed, and that refusal is itself a receipt to produce.",
        "Register the Linux production containment receipt plus a fail-closed mutation-refusal receipt on every other packaged platform.",
        Owner::External,
    ),
    row(
        "release.provenance",
        "No signed build provenance statement exists for any archive.",
        "Without provenance a consumer cannot connect archive bytes to the hub tag, lock, and toolchains that produced them.",
        "Produce provenance bound to the exact hub tag, lock, toolchains, and archive digests, sign it, and re-read it with the release verifier.",
        Owner::External,
    ),
    provider(
        "release.provider.antigravity",
        "No admitted Antigravity binary has run the exact-subject conformance transaction under provider-only egress; the offline structured-output subset is the only Antigravity evidence.",
    ),
    provider(
        "release.provider.claude",
        "No admitted Claude binary has run the exact-subject conformance transaction under provider-only egress; the offline stream-JSON subset is the only Claude evidence.",
    ),
    provider(
        "release.provider.codex",
        "No admitted Codex binary has run the exact-subject conformance transaction under provider-only egress; the offline App Server JSONL subset is the only Codex evidence.",
    ),
    provider(
        "release.provider.cursor",
        "No admitted Cursor binary has run the exact-subject conformance transaction under provider-only egress; the offline ACP subset is the only Cursor evidence.",
    ),
    row(
        "release.receipt-contracts",
        "The receipt verifier has only ever checked fixtures; no external allowed-signers policy or real signed release receipt has been provisioned.",
        "A verifier without a provisioned signer policy and a real receipt is a component, not release evidence.",
        "Provision the external signer policy, trusted-time observation, kind-specific semantic verifier, and exact tagged receipts, then register their verification.",
        Owner::External,
    ),
    row(
        "release.rust-msrv-1-95",
        "Nobody has built and tested the exact tagged family with Rust 1.95 and registered the receipt.",
        "MSRV is a release promise; a local toolchain match today is not a tagged-bytes build receipt.",
        "Build and test the exact tagged family with the admitted Rust 1.95 toolchain under cargo --locked and register the receipt.",
        Owner::Local,
    ),
    row(
        "release.rust-pinned-1-97-1",
        "Nobody has built and tested the exact tagged family with pinned Rust 1.97.1 and registered the receipt.",
        "The pinned toolchain is the second required build; one toolchain receipt never covers the other.",
        "Build and test the exact tagged family with the admitted pinned Rust 1.97.1 toolchain under cargo --locked and register the receipt.",
        Owner::Local,
    ),
    row(
        "release.sbom",
        "No software bill of materials exists for any release archive.",
        "Consumers cannot audit or respond to a vulnerability in bytes whose contents were never enumerated.",
        "Generate and validate an SBOM for every exact release archive and bind each digest in the signed manifest.",
        Owner::Local,
    ),
    row(
        "release.scan.dependency",
        "No pinned dependency scan receipt exists for the exact release lockfiles.",
        "A scan run on a working tree at some earlier commit says nothing about the tagged lockfiles.",
        "Run the admitted cargo-deny 0.19.8 against the exact tagged lockfiles and register its receipt.",
        Owner::Local,
    ),
    row(
        "release.scan.license",
        "No pinned license policy scan receipt exists for the exact release artifacts.",
        "License violations discovered after tagging invalidate the release bytes.",
        "Run the admitted license scanner against the exact archives and SBOMs and register its receipt.",
        Owner::Local,
    ),
    row(
        "release.scan.secret",
        "No pinned secret scan receipt exists for the exact tagged trees.",
        "A canary or credential in tagged bytes is unrecoverable once published.",
        "Run the admitted gitleaks 8.21.2 against every exact tagged tree and register its receipt.",
        Owner::Local,
    ),
    row(
        "release.scan.workflow",
        "No pinned workflow policy scan receipt exists for the exact workflow bytes.",
        "Hosted workflows are a supply-chain surface; unpinned or over-permissioned steps break provenance.",
        "Run the admitted zizmor 1.25.2 against the exact workflow bytes and register its receipt.",
        Owner::Local,
    ),
    row(
        "release.signatures",
        "Nothing in the package matrix carries a signature from a protected release key.",
        "An unsigned archive, checksum set, SBOM, or manifest can be replaced by anyone who can write to the download path.",
        "Sign every archive, checksum set, SBOM, provenance statement, and the final manifest with admitted release keys and re-read them with the release verifier.",
        Owner::External,
    ),
    row(
        "release.transaction-demo",
        "Nobody has produced a signed five-plane TRANSACTION_PROOF from `just demo`; the demo still ends in synthetic success.",
        "The offline transaction is the baseline every live and release gate builds on; synthetic success renamed is still synthetic.",
        "Run the non-synthetic tagged demo through real child boundaries and the protected local forge simulator, ending in one signed TRANSACTION_PROOF covering authority, runner death/salvage, independent verification, ambiguous-effect reconciliation, protected integration, preservation, and truthful projection.",
        Owner::Local,
    ),
];

pub(super) fn find(id: &str) -> Option<&'static ClaimRow> {
    ROWS.iter().find(|row| row.id == id)
}

/// True when `text` contains one of [`CLOSED_WORDS`] as a whole word, ignoring case.
pub(super) fn contains_closed_word(text: &str) -> bool {
    text.split(|byte: char| !byte.is_ascii_alphanumeric())
        .any(|token| CLOSED_WORDS.contains(&token.to_ascii_lowercase().as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::prerequisites;

    #[test]
    fn rows_cover_the_release_catalog_exactly_once() {
        let report = prerequisites::report_release().unwrap();
        let catalog = report
            .gates()
            .iter()
            .map(|gate| gate.id())
            .collect::<Vec<_>>();
        let rows = ROWS.iter().map(|row| row.id).collect::<Vec<_>>();
        for id in &catalog {
            assert!(rows.contains(id), "catalog gate without a claim row: {id}");
        }
        for id in &rows {
            assert!(
                catalog.contains(id),
                "claim row without a catalog gate: {id}"
            );
        }
        assert_eq!(rows.len(), catalog.len());
        assert!(
            rows.windows(2).all(|pair| pair[0] < pair[1]),
            "rows must stay sorted"
        );
    }

    #[test]
    fn unreceipted_claims_never_read_as_closed() {
        for row in ROWS {
            for text in [row.claim, row.why, row.acceptance] {
                assert!(!text.trim().is_empty(), "{} has an empty field", row.id);
                assert!(
                    text.ends_with('.'),
                    "{} field is not a sentence: {text}",
                    row.id
                );
                assert!(
                    !contains_closed_word(text),
                    "{} reads as closed: {text}",
                    row.id
                );
            }
        }
        assert!(contains_closed_word("this is Done."));
        assert!(contains_closed_word("prior-or-complete-next"));
        assert!(!contains_closed_word("completeness verification verifier"));
    }
}
