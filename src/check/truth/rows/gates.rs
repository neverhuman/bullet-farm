//! The 26 gate claim rows. Sorted by gate id; the parent module's tests enforce
//! exact catalog coverage, gap coverage, vocabulary, and command existence.

use super::{ClaimRow, Owner, command, none};

const PROVIDER_WHY: &str = "The bounded offline parser and the zero-spawn neutral refusal prove nothing about the live adapter, its credentials, quota, isolation, or patch output; only a sealed PONG receipt under an operator-ratified policy is LIVE_PROOF for this adapter.";
const PROVIDER_ACCEPTANCE: &str = "After the operator act in `docs/runbooks/live-conformance.md` (authority keygen, a ratified v1alpha2 policy with generation 2 and an active provider-runner key, the `AGENT_CHAT.md` ratification line), run the real-mode lane and register the sealed exact adapter/version/profile receipt covering isolation, failure, quota, and patch conformance.";
const PROVIDER_OWNER: Owner = Owner::External(
    "operator ratifies policy generation 2 with a provider-runner key and supplies the real profile and credentials; the Kernel loader already accepts v1alpha2 (`0d848f6`), so ratification is the only policy blocker and no agent flips `live_admission_enabled`",
);
const PROVIDER_NOTE: &str = "real mode refuses to start without an absolute operator-ratified policy path; under the checked-in v1alpha1 policy the lane exits 78 (neutral refusal) and spawns nothing";
const RELEASE_VERIFY: &str =
    "bullet-family release verify --bundle /abs/bundle --allowed-signers /abs/allowed_signers";

const fn provider(
    id: &'static str,
    claim: &'static str,
    exists: &'static str,
    lane: &'static str,
) -> ClaimRow {
    ClaimRow {
        id,
        gap_ids: &["G5"],
        claim,
        why: PROVIDER_WHY,
        acceptance: PROVIDER_ACCEPTANCE,
        exists,
        owner: PROVIDER_OWNER,
        next: command(lane, PROVIDER_NOTE),
    }
}

pub(crate) const ROWS: &[ClaimRow] = &[
    ClaimRow {
        id: "release.backup-restore",
        gap_ids: &["G3", "G9"],
        claim: "Nobody has restored a tagged backup and read the restored state back as an exact-subject release receipt.",
        why: "A backup whose restore has never been observed protects nothing; the pointed-at bytes, not the database, are the recovery subject.",
        acceptance: "Run the tagged backup/restore suite from release bytes and register a signed receipt binding backup digest, restore epoch, and the exact four-repository subjects.",
        exists: "Kernel `798f0c8` `farm backup|restore` with an exact receipt and quarantined offline restore (COMPONENT_PROOF); restored state is not admitted for production use",
        owner: Owner::LocalThenExternal {
            offline: "the backup/restore suite and its receipt shape are Kernel code (V1-S2)",
            external: "release bytes from signed tagged subjects (G9) and production restore admission (G3)",
        },
        next: command(
            "cd ../bullet-kernel && cargo run --locked -p bullet -- farm backup --database /abs/ledger.sqlite --output /abs/snapshot.sqlite --receipt /abs/backup-receipt.json",
            "component backup with an exact receipt; `farm restore --backup … --receipt … --destination …` restores into quarantine only, and neither run is a tagged-bytes release receipt",
        ),
    },
    ClaimRow {
        id: "release.checksums",
        gap_ids: &["G9"],
        claim: "No checksum set exists for any release archive, because no release archive exists.",
        why: "Without checksums an installer cannot tell substituted bytes from released bytes.",
        acceptance: "Generate and re-read checksums for every exact archive in the five-target matrix and bind them in the signed release manifest.",
        exists: "hub `release verify` re-reads checksums only inside an already signed bundle (`352f963`, `ba09056`); no archive or package builder",
        owner: Owner::LocalThenExternal {
            offline: "a package builder and checksum generation over its archives are hub engineering (V1-S7)",
            external: "the five archives on macOS/Windows build platforms and the signed manifest that binds the checksums",
        },
        next: none(
            "no package builder exists; `bullet-family release verify` only re-reads checksums inside an assembled signed bundle",
        ),
    },
    ClaimRow {
        id: "release.fault-suite",
        gap_ids: &["G2", "G3"],
        claim: "Nobody has crashed the tagged family at every SQLite/WAL/CAS/journal/generation boundary and read back either the prior or the whole next state.",
        why: "Crash safety that was only reasoned about is not crash safety; one unobserved boundary can lose or duplicate an effect.",
        acceptance: "Run the tagged crash-boundary and recovery fault suite from release bytes and register its exact signed receipt.",
        exists: "Kernel atomic lease/command/outbox, injected setup transaction boundaries (hub `94b6549`), and BulletGit prior-or-whole-next recovery (`274fd6d`, `f551736`) as COMPONENT_PROOF",
        owner: Owner::LocalThenExternal {
            offline: "the crash-boundary and recovery fault suite is Kernel/BulletGit engineering (V1-S2/S3)",
            external: "the tagged release bytes the suite must run from",
        },
        next: none(
            "`just model-check` covers only the two bounded formal protocols (lease/fence/reclaim, command/effect ambiguity), not the crash-boundary suite",
        ),
    },
    ClaimRow {
        id: "release.forge.github-app",
        gap_ids: &["G7"],
        claim: "No GitHub App has integrated an exact Candidate into a protected test repository and read the result back.",
        why: "GitHub is an effect adapter; until a lost response reconciles to UNKNOWN and read-back adopts the original OID, every GitHub effect is unproved.",
        acceptance: "Configure the GitHub App test repository with branch protection and register exact-subject dispatch, check, integration, read-back, and reconciliation receipts.",
        exists: "a specified, uncertified effect adapter (ADR 0002, ADR 0008); no App, test repository, or credential",
        owner: Owner::External(
            "operator configures a GitHub App and a branch-protected test repository with separate delivery and attestation credentials",
        ),
        next: none("no typed GitHub App command exists"),
    },
    ClaimRow {
        id: "release.forge.jeryu",
        gap_ids: &["G6"],
        claim: "No protected Jeryu integration has run with restored operator authentication and produced a read-back receipt.",
        why: "Jeryu is the source forge; an integration that never read its own result back cannot claim protected-ref safety.",
        acceptance: "With operator-restored authentication run read-only probes, then one exact protected integration with UNKNOWN/read-back reconciliation, and register the receipt; never modify the running forge.",
        exists: "a local bare-forge component and a typed-quarantine Jeryu adapter; no authenticated read-back",
        owner: Owner::External(
            "operator restores scoped authentication on the unmodified local forge and names a test repository for read-back",
        ),
        next: none(
            "the read-only probes that come first have no typed command yet; never modify the running forge",
        ),
    },
    ClaimRow {
        id: "release.installable-lock",
        gap_ids: &["G1"],
        claim: "The checked-in family.lock is still schema 2, which every installer path rejects by design.",
        why: "Installation from the hub cannot start until a lock carries authenticated Jeryu sources and signed exact subjects.",
        acceptance: "Publish immutable signed member tags, generate the schema-3 lock with Jeryu URL/slug, exact tree OIDs, lockfiles, and artifact digests, and commit it under a signed hub tag.",
        exists: "schema-3 generation and strict verification code (`bullet-family lock generate|verify`), descriptor-relative setup, and the honest schema-2 refusal (`doctor` reports `UNSUPPORTED_SCHEMA`)",
        owner: Owner::External(
            "operator publishes signed immutable member tags with an authenticated Jeryu URL/slug; the schema-3 lock can only be generated from those",
        ),
        next: command(
            "bullet-family lock generate --tag <signed-hub-tag>",
            "generates the schema-3 lock only from signed tagged members with authenticated sources; until they exist `bullet-family doctor --json` reports the schema-2 refusal, and that refusal is correct",
        ),
    },
    ClaimRow {
        id: "release.installer-twice",
        gap_ids: &["G1"],
        claim: "Nobody has run the signed prebuilt installer twice in a fresh HOME from tagged hub-only bytes.",
        why: "A source bootstrap that launches Cargo before admission is not an installer receipt; idempotence and exact clean member OIDs must be observed, not assumed.",
        acceptance: "From a signed prebuilt bullet-family binary and tagged bytes, run setup twice in a fresh HOME and register exact clean member OIDs, zero tracked changes, and no worktrees.",
        exists: "two-run component fixture with local source transport and sealed Linux Cargo/Node/Bash/npm subjects (hub `94b6549`, `7efe2f3`); `scripts/setup.sh` stays a pre-admission source wrapper",
        owner: Owner::External(
            "a signed prebuilt `bullet-family` from tagged bytes and a fresh-HOME replay through production Jeryu transport",
        ),
        next: none(
            "`scripts/setup.sh` launches Cargo before admission; running it twice is not the installer receipt",
        ),
    },
    ClaimRow {
        id: "release.jankurai-90",
        gap_ids: &["G8"],
        claim: "No pinned Jankurai report reads 90 or better with zero caps and zero hard findings for the exact release subjects.",
        why: "Hard findings and caps are release blockers; a machine-local or skip-green audit lane cannot substitute for the pinned receipt.",
        acceptance: "Run pinned Jankurai 1.6.11 with zero skips against the exact tagged subjects and register the score/cap/hard-finding receipt at 90 or better with zero caps and zero hard findings.",
        exists: "pinned local jankurai 1.6.11 lane that fails closed below its floor and ratchets upward only (`just audit`); no hosted checksum-pinned artifact",
        owner: Owner::LocalThenExternal {
            offline: "reduce hard findings and caps to zero and raise the score to 90 with the pinned local binary",
            external: "a checksum-pinned hosted CI Jankurai artifact and the exact tagged subjects",
        },
        next: command(
            "just audit",
            "pinned local lane with an upward-only floor; a local binary is not the CI artifact and this checkout is not the tagged subject",
        ),
    },
    ClaimRow {
        id: "release.manifest-non-circular",
        gap_ids: &["G9"],
        claim: "No signed final release manifest binds the hub tag without embedding its own digest.",
        why: "A manifest that includes its own digest cannot be checked; a manifest without a signature cannot be trusted.",
        acceptance: "Generate the final manifest binding hub tag, schema-3 lock, five byte-sorted target entries, and archive/SBOM/provenance digests, then sign it with the protected release key.",
        exists: "hub `release verify` checks an exact non-circular signed five-target manifest against a preassembled fixture (`352f963`); no builder or signer",
        owner: Owner::External("the protected release signing key and its signer identity"),
        next: command(
            RELEASE_VERIFY,
            "read-only verifier of an already assembled bundle; it re-reads a manifest and cannot produce or sign one",
        ),
    },
    ClaimRow {
        id: "release.package-matrix",
        gap_ids: &["G9"],
        claim: "None of the five release archives (Linux x86_64/aarch64, macOS x86_64/arm64, Windows x64) has been built with the Portal embedded.",
        why: "A release is bytes on every declared platform; the Vite preview lane and a separately built farmd are not package evidence.",
        acceptance: "Build and smoke all five archives from the exact tagged family with the Portal embedded in the Rust distribution and register each archive digest.",
        exists: "Portal `8272844` deterministic clean-commit bundle manifest and a separately built farmd; no archive, embedding, or package builder",
        owner: Owner::External(
            "macOS x86_64/arm64 and Windows x64 build platforms plus the embedded Portal build; Linux alone is not the matrix",
        ),
        next: none("no package builder or Rust embedding of the Portal bundle exists"),
    },
    ClaimRow {
        id: "release.platform-containment",
        gap_ids: &["G10"],
        claim: "Only Linux containment has been observed live; no packaged non-Linux platform has been seen refusing real mutation.",
        why: "A platform without an equivalent containment backend must fail closed, and that refusal is itself a receipt to produce.",
        acceptance: "Register the Linux production containment receipt plus a fail-closed mutation-refusal receipt on every other packaged platform.",
        exists: "Linux user+net namespace, slirp4netns, nftables, and CONNECT-proxy isolation proofs (Kernel `d388733`, `ops/ci/egress.sh`); no non-Linux backend or refusal receipt",
        owner: Owner::External(
            "native macOS/Windows containment backends or their fail-closed refusal receipts on packaged bytes",
        ),
        next: command(
            "cd ../bullet-kernel && bash ops/ci/egress.sh",
            "Linux half only: 3/3 isolation proofs on a host with unprivileged namespaces (exit 78 neutral otherwise); no non-Linux refusal receipt exists",
        ),
    },
    ClaimRow {
        id: "release.provenance",
        gap_ids: &["G9"],
        claim: "No signed build provenance statement exists for any archive.",
        why: "Without provenance a consumer cannot connect archive bytes to the hub tag, lock, and toolchains that produced them.",
        acceptance: "Produce provenance bound to the exact hub tag, lock, toolchains, and archive digests, sign it, and re-read it with the release verifier.",
        exists: "hub `release verify` re-reads provenance bytes inside a signed bundle; no provenance producer",
        owner: Owner::External(
            "signed provenance needs the protected release key and archive digests from the package matrix",
        ),
        next: command(
            RELEASE_VERIFY,
            "re-reads provenance bytes inside an assembled signed bundle only",
        ),
    },
    provider(
        "release.provider.antigravity",
        "No admitted Antigravity binary has produced a sealed PONG receipt through the live-conformance path; the path exists (Kernel `ba485d5`, `b4735da`, `0d848f6`; `docs/runbooks/live-conformance.md`) but every run so far refused at POLICY_LIVE_ADMISSION_DISABLED under the checked-in v1alpha1 policy, so the offline structured-output subset and the zero-spawn neutral refusal are the only Antigravity evidence.",
        "Kernel `5badc85` bounded structured-output subset; the common policy→key→lease→admission→grant→egress→read-only-turn→canary→receipt path with the v1alpha2 loader mirror (`0d848f6`); every run refuses before spawn under the checked-in policy",
        "cd ../bullet-kernel && BULLET_LIVE_REAL=1 BULLET_POLICY_PATH=/abs/bullet-data/policy/policy.json BULLET_LIVE_PROVIDERS=agy bash ops/ci/nightly.sh",
    ),
    provider(
        "release.provider.claude",
        "No admitted Claude binary has produced a sealed PONG receipt through the live-conformance path; the path exists (Kernel `ba485d5`, `b4735da`, `0d848f6`; `docs/runbooks/live-conformance.md`) but every run so far refused at POLICY_LIVE_ADMISSION_DISABLED under the checked-in v1alpha1 policy, so the offline stream-JSON subset, the deep fake-process proof, and the zero-spawn neutral refusal are the only Claude evidence.",
        "Kernel `c34d578` bounded stream-JSON subset and the only deep positive fake-process proof; the common policy→key→lease→admission→grant→egress→read-only-turn→canary→receipt path with the v1alpha2 loader mirror (`0d848f6`); every run refuses before spawn under the checked-in policy",
        "cd ../bullet-kernel && BULLET_LIVE_REAL=1 BULLET_POLICY_PATH=/abs/bullet-data/policy/policy.json BULLET_LIVE_PROVIDERS=claude bash ops/ci/nightly.sh",
    ),
    provider(
        "release.provider.codex",
        "No admitted Codex binary has produced a sealed PONG receipt through the live-conformance path; the path exists (Kernel `ba485d5`, `b4735da`, `0d848f6`; `docs/runbooks/live-conformance.md`) but every run so far refused at POLICY_LIVE_ADMISSION_DISABLED under the checked-in v1alpha1 policy, so the offline App Server JSONL subset and the zero-spawn neutral refusal are the only Codex evidence.",
        "Kernel `ca376e4` bounded App Server JSONL subset; the common policy→key→lease→admission→grant→egress→read-only-turn→canary→receipt path with the v1alpha2 loader mirror (`0d848f6`); every run refuses before spawn under the checked-in policy",
        "cd ../bullet-kernel && BULLET_LIVE_REAL=1 BULLET_POLICY_PATH=/abs/bullet-data/policy/policy.json BULLET_LIVE_PROVIDERS=codex bash ops/ci/nightly.sh",
    ),
    provider(
        "release.provider.cursor",
        "No admitted Cursor binary has produced a sealed PONG receipt through the live-conformance path; the path exists (Kernel `ba485d5`, `b4735da`, `0d848f6`; `docs/runbooks/live-conformance.md`) but every run so far refused at POLICY_LIVE_ADMISSION_DISABLED under the checked-in v1alpha1 policy, so the offline ACP subset and the zero-spawn neutral refusal are the only Cursor evidence.",
        "Kernel `ea89929` bounded ACP subset; the common policy→key→lease→admission→grant→egress→read-only-turn→canary→receipt path with the v1alpha2 loader mirror (`0d848f6`); every run refuses before spawn under the checked-in policy",
        "cd ../bullet-kernel && BULLET_LIVE_REAL=1 BULLET_POLICY_PATH=/abs/bullet-data/policy/policy.json BULLET_LIVE_PROVIDERS=cursor bash ops/ci/nightly.sh",
    ),
    ClaimRow {
        id: "release.receipt-contracts",
        gap_ids: &["G9"],
        claim: "The receipt verifier has only ever checked fixtures; no external allowed-signers policy or real signed release receipt has been provisioned.",
        why: "A verifier without a provisioned signer policy and a real receipt is a component, not release evidence.",
        acceptance: "Provision the external signer policy, trusted-time observation, kind-specific semantic verifier, and exact tagged receipts, then register their verification.",
        exists: "strict canonical receipt/policy verifier with exact OpenSSH signer/namespace/interval checks (hub `143f8b9`) plus the MSRV receipt-admission path (`d762f86`); only fixtures have ever been checked",
        owner: Owner::External(
            "an independently provisioned allowed-signers policy, a trusted-time observation, and a real signed release receipt from tagged bytes",
        ),
        next: command(
            "bullet-family release receipt-verify --receipt /abs/receipt --signature /abs/signature --policy /abs/policy",
            "strict verifier that has only ever seen fixtures; a fixture pass is component evidence",
        ),
    },
    ClaimRow {
        id: "release.rust-msrv-1-95",
        gap_ids: &["G9"],
        claim: "No admitted Rust 1.95 build-and-test receipt has been supplied through the root-owned `/etc/bullet-farm/release-msrv-1-95-admission.toml` descriptor; the admission path exists (hub `d762f86`) and nothing has been admitted through it.",
        why: "MSRV is a release promise; a local toolchain match today is not a tagged-bytes build receipt, and a self-selected signer or a generic result digest cannot clear the gate.",
        acceptance: "Provision the root-owned descriptor, policy, and evidence directory with distinct source-tag, build-attestor, and trusted-time Ed25519 roots, then supply the receipt binding the schema-3 lock, clean signed subjects, dependency-lock digests, exact Rust 1.95 rustc/cargo bytes, and zero-skip build/test observations for all three Rust workspaces.",
        exists: "the semantic receipt-admission path for exactly this gate (`d762f86`; absent → BLOCKED, rejected → FAIL, admitted → PASS) and local lanes that already build under rustc 1.95.0 (`rust-toolchain.toml`, `scripts/ci-doctor.sh`)",
        owner: Owner::LocalThenExternal {
            offline: "the admission path and the 1.95 lane are hub code; every local lane already runs under rustc 1.95.0",
            external: "a root-owned descriptor with three distinct signer roots, an independently signed time observation, and the schema-3 lock the receipt must bind",
        },
        next: command(
            "bullet-family check release --json",
            "reads only the fixed `/etc/bullet-farm/release-msrv-1-95-admission.toml` descriptor (root-owned; environment variables and repository files cannot redirect it); this one gate becomes PASS only from an admitted receipt",
        ),
    },
    ClaimRow {
        id: "release.rust-pinned-1-97-1",
        gap_ids: &["G9"],
        claim: "Nobody has built and tested the exact tagged family with pinned Rust 1.97.1 and registered the receipt.",
        why: "The pinned toolchain is the second required build; one toolchain receipt never covers the other.",
        acceptance: "Build and test the exact tagged family with the admitted pinned Rust 1.97.1 toolchain under cargo --locked and register the receipt.",
        exists: "nothing beyond the MSRV admission pattern; `scripts/ci-doctor.sh` accepts rustc 1.95.0 only and no 1.97.1 lane or admission path exists",
        owner: Owner::LocalThenExternal {
            offline: "a pinned 1.97.1 lane and its admission path are ordinary hub engineering",
            external: "the exact tagged family bytes and the same signer/time roots as the MSRV receipt",
        },
        next: none(
            "no 1.97.1 lane exists; `scripts/ci-doctor.sh` refuses any rustc other than 1.95.0",
        ),
    },
    ClaimRow {
        id: "release.sbom",
        gap_ids: &["G9"],
        claim: "No software bill of materials exists for any release archive.",
        why: "Consumers cannot audit or respond to a vulnerability in bytes whose contents were never enumerated.",
        acceptance: "Generate and validate an SBOM for every exact release archive and bind each digest in the signed manifest.",
        exists: "hub `release verify` re-reads SBOM bytes inside a signed bundle; no archive, SBOM generator, or semantic SBOM validation",
        owner: Owner::LocalThenExternal {
            offline: "SBOM generation and validation over built archives",
            external: "the five archives of the package matrix and the signed manifest that binds each SBOM digest",
        },
        next: none("no package builder or admitted SBOM generator exists"),
    },
    ClaimRow {
        id: "release.scan.dependency",
        gap_ids: &["G8", "G9"],
        claim: "No pinned dependency scan receipt exists for the exact release lockfiles.",
        why: "A scan run on a working tree at some earlier commit says nothing about the tagged lockfiles.",
        acceptance: "Run the admitted cargo-deny 0.19.8 against the exact tagged lockfiles and register its receipt.",
        exists: "`cargo deny check bans` under the pinned cargo-deny 0.19.8 in `just security` on this checkout; advisories and tagged lockfiles are not covered",
        owner: Owner::LocalThenExternal {
            offline: "the pinned cargo-deny lane over this checkout, extended to advisories",
            external: "the exact tagged lockfiles of all three Rust workspaces",
        },
        next: command(
            "just security",
            "runs gitleaks, `cargo deny check bans`, and zizmor on this checkout only; not the tagged lockfiles",
        ),
    },
    ClaimRow {
        id: "release.scan.license",
        gap_ids: &["G8", "G9"],
        claim: "No pinned license policy scan receipt exists for the exact release artifacts.",
        why: "License violations discovered after tagging invalidate the release bytes.",
        acceptance: "Run the admitted license scanner against the exact archives and SBOMs and register its receipt.",
        exists: "no license scan is admitted; `just security` runs `cargo deny check bans` only",
        owner: Owner::LocalThenExternal {
            offline: "a pinned license policy scan is local engineering",
            external: "the exact tagged archives and SBOMs it must run against",
        },
        next: none(
            "`just security` runs `cargo deny check bans` only; no license scanner is admitted yet",
        ),
    },
    ClaimRow {
        id: "release.scan.secret",
        gap_ids: &["G8", "G9"],
        claim: "No pinned secret scan receipt exists for the exact tagged trees.",
        why: "A canary or credential in tagged bytes is unrecoverable once published.",
        acceptance: "Run the admitted gitleaks 8.21.2 against every exact tagged tree and register its receipt.",
        exists: "`gitleaks detect --source . --no-git --redact` under the pinned gitleaks 8.21.2 in `just security` on this working tree",
        owner: Owner::LocalThenExternal {
            offline: "the pinned gitleaks lane over this working tree",
            external: "every exact tagged tree of the four repositories",
        },
        next: command(
            "just security",
            "gitleaks over this working tree only (`--no-git`); the tagged trees of all four repositories remain",
        ),
    },
    ClaimRow {
        id: "release.scan.workflow",
        gap_ids: &["G8", "G9"],
        claim: "No pinned workflow policy scan receipt exists for the exact workflow bytes.",
        why: "Hosted workflows are a supply-chain surface; unpinned or over-permissioned steps break provenance.",
        acceptance: "Run the admitted zizmor 1.25.2 against the exact workflow bytes and register its receipt.",
        exists: "`zizmor .` under the pinned zizmor 1.25.2 in `just security` on this checkout",
        owner: Owner::LocalThenExternal {
            offline: "the pinned zizmor lane over this checkout",
            external: "the exact tagged workflow bytes",
        },
        next: command(
            "just security",
            "zizmor over this checkout's workflows only; not the tagged workflow bytes",
        ),
    },
    ClaimRow {
        id: "release.signatures",
        gap_ids: &["G9"],
        claim: "Nothing in the package matrix carries a signature from a protected release key.",
        why: "An unsigned archive, checksum set, SBOM, or manifest can be replaced by anyone who can write to the download path.",
        acceptance: "Sign every archive, checksum set, SBOM, provenance statement, and the final manifest with admitted release keys and re-read them with the release verifier.",
        exists: "hub `release verify` checks detached signatures and exact Ed25519 signer status inside an assembled bundle; nothing signs",
        owner: Owner::External("protected release keys and a signer identity"),
        next: command(
            RELEASE_VERIFY,
            "verifies detached signatures inside an assembled bundle; it signs nothing",
        ),
    },
    ClaimRow {
        id: "release.transaction-demo",
        gap_ids: &["G2"],
        claim: "Nobody has produced a signed five-plane TRANSACTION_PROOF from `just demo`; the demo still ends in synthetic success.",
        why: "The offline transaction is the baseline every live and release gate builds on; synthetic success renamed is still synthetic.",
        acceptance: "Run the non-synthetic tagged demo through real child boundaries and the protected local forge simulator, ending in one signed TRANSACTION_PROOF covering authority, runner death/salvage, independent verification, ambiguous-effect reconciliation, protected integration, preservation, and truthful projection.",
        exists: "`just demo` runs the Kernel simulator demo and prints synthetic receipts; atomic lease/command/outbox, fail-closed `bullet-gitd`, the fixture E2 verifier, and Portal PENDING→UNKNOWN are COMPONENT_PROOF",
        owner: Owner::Local(
            "Kernel + BulletGit + Portal engineering (V1-S4); the TRANSACTION_PROOF is credential-free and offline",
        ),
        next: command(
            "just demo",
            "today ends in synthetic success from the simulator; the receipt must replace that success, not rename it",
        ),
    },
];
