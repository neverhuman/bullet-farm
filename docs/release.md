# Bullet Farm release contract

Status: **BLOCKED — no V1 release candidate is authorized**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-25

Applies to: the four-repository Bullet Farm family

This document is the short release index. It does not replace generated wire
contracts, the family lock, policy registry, test maps, or signed receipts.
Historical design material under `docs/spec/` has no release authority.
Unreleased product changes are recorded in the [changelog](../CHANGELOG.md).

## Evidence classes

| Class | Proves | Does not prove |
| --- | --- | --- |
| `COMPONENT_PROOF` | One crate, service, or portal surface passed its mapped tests | Cross-process transaction safety |
| `SYNTHETIC_PROOF` | Deterministic simulator behavior | A provider, forge, or production mutation |
| `TRANSACTION_PROOF` | One exact offline five-plane transaction with independent receipts | External provider or forge conformance |
| `LIVE_PROOF` | An admitted provider or effect adapter passed the same exact-subject transaction | A release on every platform |
| `RELEASE_PROOF` | Packages, installer, recovery, security, signatures, provenance, and required live profiles passed from tagged bytes | Future versions or untested environments |

An exit code, model statement, process shutdown, HTTP success, branch push, or
pull request is never release evidence by itself. `UNKNOWN`, timeout, zero
tests, unsupported, skipped, flaky, or infrastructure error never equals
`VERIFIED`.

## Deployment stages

| Stage | Required baseline | Additional requirement |
| --- | --- | --- |
| Self-hosted single host | Exact offline transaction and one separately admitted low-cost provider canary | Local Jeryu protected integration plus the Wave 5 install, recovery, security, and release gates |
| GitHub adapter | Self-hosted production baseline | Separately certified exact-subject GitHub effect, check, integration, read-back, and reconciliation receipts |
| Distributed team mode | Self-hosted production baseline | PostgreSQL and workload-mTLS conformance plus the Wave 10 partition, failover, freeze, and restore gates |

Multi-tenant SaaS is outside this roadmap. A provider/model/adapter/profile is
eligible only under its own exact, unexpired certification; one provider's
receipt never certifies another provider or profile.

## Current hard blockers

| Gate | Status | Evidence needed to clear it |
| --- | --- | --- |
| Hub-only installation | `BLOCKED` | The checked-in alpha.4 lock is schema 2 and intentionally rejected. Publish a real schema-3 lock with authenticated Jeryu URL/slug and signed exact subjects, then reproduce the already-tested two-run clean-install invariant using a signed prebuilt installer from tagged release bytes in a fresh home |
| Production Kernel transaction | `BLOCKED` | Atomic lease/command/event/outbox, snapshots, authenticated ingress, typed operation decisions, and an authenticated exact-ID offline worker/reconciler are committed. Receipt-bound WAL-consistent backup and a fail-closed quarantined restore with an explicit restore epoch are also committed. Signed provider/effect dispatch and execution, independent verification/effects integration, normalized full truth, CAS/retention/GC, production restore admission/publication, and cross-plane crash receipts remain |
| Production BulletGit transaction | `BLOCKED` | Durable CAS/journal, generation-atomic apply, and preservation-bound cleanup are committed; online authority, complete Candidate/Integration manifests, shared-wire consumption, and reviewed tagged `jeryu-gitd` remain |
| Offline five-plane proof | `BLOCKED` | One signed `TRANSACTION_PROOF` covering authority, runner death/salvage, independent verification, ambiguous-effect reconciliation, protected integration, preservation, and truthful portal projection |
| Jeryu live effect | `BLOCKED` | Operator-restored authentication and read-back/reconciliation receipt; the running forge must not be modified to work around missing capability |
| GitHub live effect | `BLOCKED` | Configured GitHub App test repository and exact-subject integration/reconciliation receipt |
| Provider conformance | `BLOCKED` | Bounded fail-closed offline protocol subsets are committed for Claude stream JSON, Codex App Server JSONL, Cursor ACP, and Antigravity structured output. They do not spawn an admitted binary, use provider credentials, prove native typed extensions or event schemas, or produce live conformance. Signed executable/profile admission, isolated credentials and egress, supervision/deadlines, strict duplicate-key decoding, and live receipts for all four providers remain |
| Security quality | `BLOCKED` | The latest Hub Jankurai report is 60 (raw 61), with 7 caps and 34 findings, including 23 `high`. No finding has tool severity `hard` (some checks carry a `hard` tag), but the caps, high findings, and score still block release; release needs at least 90, zero caps/hard findings, and all required scans |
| Release supply chain | `BLOCKED` | A read-only Linux verifier checks an exact non-circular signed five-target manifest and every declared byte subject. It is not a package builder, archive extractor, or prebuilt installer. Reproducible archives, semantic SBOM/provenance validation, package signatures from protected release keys, installer smoke, and tagged release receipts remain |
| Platform containment | `BLOCKED` | Linux production containment plus fail-closed proof on every other packaged platform until an equivalent native backend passes |

Missing credentials produce a neutral, unregistered live lane only when that
lane is not required for the requested profile. Missing required tools,
adapters, receipts, or signatures fail the release.

## Current component receipt snapshot

These reviewed commits are component evidence, not release or live evidence:

| Subject | Committed receipt | Remaining authority boundary |
| --- | --- | --- |
| Kernel command worker | Kernel `77a0ecd` | Authenticated offline exact-ID execution/reconciliation only; no provider or effect dispatch |
| Kernel backup/restore | Kernel `798f0c8` | Exact receipt and quarantined offline restore only; restored state is not admitted for production use |
| Codex offline protocol | Kernel `ca376e4` | Bounded App Server transcript subset; public runtime remains blocked |
| Claude offline protocol | Kernel `c34d578` | Bounded stream-JSON transcript subset; public runtime remains blocked |
| Cursor offline protocol | Kernel `ea89929` | Bounded ACP transcript subset; native typed-extension and live conformance remain unproved |
| Antigravity offline protocol | Kernel `5badc85` | Bounded structured-output subset; native stream schema and live conformance remain unproved |
| Setup transaction | Hub `5148a52` | Source/component fixture proof; no authenticated public schema-3 lock or prebuilt installer |
| Bundle verifier | Hub `352f963` | Read-only exact-byte verification; no package production, extraction, installation, or signing authority |

## Local pre-release gates

Run from the public hub in the canonical ordinary-clone family:

```bash
just fast
just contract
just check-family
just family-contract
just security
just audit
bullet-family check release --json
```

These commands prove repository and family prerequisites only. They do not
authorize a release until the transaction, live, recovery, packaging, and
signing receipts above exist. `check release` is a read-only, fail-closed
inventory of those blockers; it executes no release mutation while the
mechanisms are absent. There is intentionally no green no-op nightly.

The release build must compile at MSRV Rust 1.95 and pinned Rust 1.97.1, use
`cargo --locked` and `npm ci`, verify generated output in a temporary directory,
and start from clean signed tags matching `family.lock`.

## Installer acceptance

The release installer starts from a hub-only clone and must:

1. verify the hub tag and lock before creating member directories;
2. use Jeryu source metadata from the lock, never a sibling-path guess;
3. create ordinary clones, never Git worktrees, at exact locked commits;
4. reject dirty, symlinked, non-empty, or conflicting destinations before mutation;
5. verify signed tags, commit/tree identities, lockfiles, and generated digests;
6. use locked/offline dependency modes when requested;
7. on the supported Linux path, bound every child process by a deadline and per-stream output cap,
   terminating its full process group when either bound is crossed;
8. be idempotent; and
9. leave exact clean member OIDs and zero tracked changes after two runs in a fresh home.

`scripts/setup.sh` is a source-development bootstrap convenience: it must launch Cargo before the
Rust admission boundary exists, so running it is not authenticated installer or release evidence.
Release installation requires a signed prebuilt `bullet-family` binary whose release manifest and
checksums have been verified. Before any mutation, that binary must bind the canonical absolute Cargo,
Node, and npm CLI subjects it admits.

The read-only Linux verifier is available as:

```bash
bullet-family release verify \
  --bundle /absolute/path/to/bundle \
  --allowed-signers /absolute/path/to/allowed_signers
```

It binds the manifest, schema-3 lock, five byte-sorted target entries, archive/SBOM/provenance bytes,
detached signatures, and exact Ed25519 signer status. It does not produce packages, interpret SBOM or
provenance semantics, extract archives, run an installer, provision signing trust, or claim safety from
concurrent replacement of intermediate bundle directories.

No package builder or archive extractor is implemented, and no signed prebuilt `bullet-family` installer
has been published. Passing the verifier against a preassembled test fixture is not package-production or
installer evidence.

The Rust setup/checkout mechanism and its signed local four-repository fixture implement these
rules, including two idempotent exact installs. Fallible dependency, generated-contract, and exact-
family checks now complete before Linux no-replace member publication, and the outer manifest is the
final durable marker; injected transaction boundaries recover to prior or complete next state. All
production Git/tool/doctor/coordination children use bounded capture, and local Rust fusion replays
byte-identically without tracked drift. Setup rejects unsupported platforms before mutation. The
checked-in alpha.4 lock remains schema 2, so the public command still fails before mutation with
schema-3 regeneration guidance. Release evidence remains blocked until authenticated Jeryu subjects
and a signed prebuilt binary exist and the invariant is replayed from those exact release bytes. No public
schema-3 family lock or live provider, Jeryu, or GitHub receipt exists today.

## Package matrix

Release archives are required for Linux x86_64/aarch64, macOS x86_64/arm64,
and Windows x64. The built portal is embedded in the Rust distribution. Linux
is the initial production runner. Other packages must refuse real mutation
unless their native containment backend has equivalent release evidence.

Every archive is bound to the same hub tag and family lock and carries an SBOM,
checksum, signature, and provenance statement. The final manifest binds the
hub tag without embedding its own digest.

## Tagging rule

Do not create or advertise a V1 release tag while any required row above is
`BLOCKED`, `UNKNOWN`, or supported only by component/synthetic evidence. When a
gate changes, update this index in the same reviewed transaction that adds its
independently verifiable receipt; prose alone cannot change status.
