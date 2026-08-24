# Bullet Farm release contract

Status: **BLOCKED — no V1 release candidate is authorized**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-24  
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
| Hub-only installation | `BLOCKED` | Publish a real schema-3 lock with authenticated Jeryu URL/slug and signed exact subjects, then reproduce the already-tested two-run clean-install invariant from tagged release bytes in a fresh home |
| Production Kernel transaction | `BLOCKED` | Durable normalized migrations, atomic command/event/outbox state, monotonic leases/fences, backup/restore, and crash-boundary receipts |
| Production BulletGit transaction | `BLOCKED` | Online authority check, durable journal/CAS, generation-atomic apply, exact Candidate/proof manifests, preservation-bound cleanup, and reviewed tagged `jeryu-gitd` capability |
| Offline five-plane proof | `BLOCKED` | One signed `TRANSACTION_PROOF` covering authority, runner death/salvage, independent verification, ambiguous-effect reconciliation, protected integration, preservation, and truthful portal projection |
| Jeryu live effect | `BLOCKED` | Operator-restored authentication and read-back/reconciliation receipt; the running forge must not be modified to work around missing capability |
| GitHub live effect | `BLOCKED` | Configured GitHub App test repository and exact-subject integration/reconciliation receipt |
| Provider conformance | `BLOCKED` | Isolation, canary-secret, malformed/crash/cancel/timeout, quota, and exact-patch receipts for every provider/model/adapter/profile selected for activation |
| Security quality | `BLOCKED` | Jankurai at least 90 with zero caps/hard findings, plus pinned secret, dependency, license, workflow, and generated-drift gates |
| Release supply chain | `BLOCKED` | Reproducible archives, SBOM, checksums, signatures, provenance, installer smoke, and final non-circular signed release manifest |
| Platform containment | `BLOCKED` | Linux production containment plus fail-closed proof on every other packaged platform until an equivalent native backend passes |

Missing credentials produce a neutral, unregistered live lane only when that
lane is not required for the requested profile. Missing required tools,
adapters, receipts, or signatures fail the release.

## Local pre-release gates

Run from the public hub in the canonical ordinary-clone family:

```bash
just fast
just contract
just check-family
just family-contract
just security
just audit
```

These commands prove repository and family prerequisites only. They do not
authorize a release until the transaction, live, recovery, packaging, and
signing receipts above exist. There is intentionally no green no-op nightly
and no `release` command while those mechanisms are absent.

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

The Rust setup/checkout mechanism and its signed local four-repository fixture implement these
rules, including two idempotent core installs with exact clean ordinary clones. All production Git,
tool, doctor, and coordination children use bounded capture; Unix timeout and output-flood tests
prove prompt process-group termination, including a spawned grandchild. Setup rejects unsupported
platforms before any clone or dependency child starts. The checked-in alpha.4 lock remains schema
2, so the public command must still fail before mutation with explicit schema-3 regeneration
guidance. Installer release evidence is incomplete until authenticated Jeryu subjects are published
and the same invariant is reproduced from the signed release bytes.

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
