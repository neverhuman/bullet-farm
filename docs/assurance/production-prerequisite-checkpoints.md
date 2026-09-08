# Local production prerequisite checkpoints

Status: **retained component observations; no release authority**

Owner: Bullet Farm maintainers

Last reconciled: 2026-09-08

The [active plan](full-product-dogfood-plan.md) and [gap register](product-gaps.md)
remain the current program. These dated observations retain their original
subjects and scope; a later commit needs its own proof. Older or failed
artifacts receive no credit merely because they are retained.

## Product-plan observations

The September 8 component checkpoint passed the Portal `family` lane's three
real-farmd browser tests on these exact source subjects:

| Member | Commit | Tree |
| --- | --- | --- |
| Portal | `b66a2053d25b5d306ee4c4b345bc6aa8ed34347c` | `3132496669137555cadf28c208a9658cb86b914c` |
| Kernel | `b179eafc2f86af5251d8b79a17106fe2b67e9ba7` | `99b6b38d0d45845dc098c31f70228e76f06033dd` |
| BulletGit | `48755d95cf8469d48e1a022f2f7223c07393d6a3` | `7f3e36d8b1f838774f84fe0a15ca02df0a855a48` |

From the Portal checkout, `bash scripts/ci-local.sh family` invokes
`ops/ci/real-farmd.sh` and `e2e/real-farmd.spec.ts`. The executed path connects
authenticated public commands, the UDS worker, product Runner/BulletGit,
immutable retained-artifact validation, durable `UNKNOWN`, and worker restart
read-back. Its provider is synthetic, its outer receipt is `UNSIGNED_FIXTURE`,
and its evidence is `COMPONENT_PROOF`; independent, transaction, and release
eligibility remain false. The browser uses Vite preview and real sibling farmd;
this observation supplies no packaged-origin or installation certification.

The accepted baseline repairs validate UTC timestamps in producer `+00:00` and fixture `Z` forms, checkpoint and close a quiescent WAL
ledger before immutable reads, await owned Gitd termination, and clean up CI workers. Kernel `Cargo.toml`/`Cargo.lock` now pin rusqlite
0.39.0 with bundled SQLite 3.51.3 and retained checked unsigned SQL conversions. The SQLite C source matches the vendor's fixed release
hash. The relevant Kernel consumers are `apps/bullet-runner/src/bin/bullet-command-worker/receipt/preservation.rs`,
`crates/adapters/src/sqlite/open.rs::close_quiescent`, and `crates/runner/src/gitd/session.rs`; Portal owns the CI fixture lifecycle in
`ops/ci/real-farmd.sh`. The bundled C SHA3-256 is `32d5424f97e0a7fc5ed2f6335afbb58be4e0298bd7117a34e39d345ff13d859e`, matching the [SQLite
3.51.3 release](https://sqlite.org/releaselog/3_51_3.html). Focused Runner library, SQLite adapter, and worker-receipt checks passed 103,
36, and 24 tests respectively; the Rust 1.95 workspace/all-targets build also passed. These observations do not constitute a release MSRV
receipt.

Hub `d3c4db3600e6f1d93800d258e3ac7c1a2b4db4f3` passed `required`: 770 Rust
tests, two formal models, and documentation/media checks. Its repaired
`scripts/demo.sh` captures and revalidates five Cargo executable subjects;
both existing launcher tests, including 35 fake-Cargo cases, passed.

The September 8 actual `just demo` and separate component-receipt verification
then passed on these newer Hub and Kernel subjects, with Portal and BulletGit
unchanged from the initial checkpoint above:

| Member | Commit | Tree |
| --- | --- | --- |
| Hub | `cdcbfd50e643635483acb427cf376f131684740d` | `b8a07dce4e284d1730f8a8e783bbf5444f6f2383` |
| Kernel | `071173a94e2d50d404aa07801a07064bd8a1e375` | `247dcc67595af8aa5f73420b7a78399494fed6ff` |

The demo disabled Portal startup and used synthetic execution and ephemeral
fixture authority. Receipt SHA256
`b379e69f9c82ad3ef1b13c16acd54fc52616e88da69048ef748ce0e0205ca2c1`
verified as `COMPONENT_PROOF`, with `EPHEMERAL_SELF_SIGNED` component signing
and `UNSIGNED_DIAGNOSTIC` verification trust. Transaction-gate and release-profile
eligibility remain false; this adds no native-provider, installation, or
independent transaction certification.

The following full-family attempt on those same four subjects completed
BulletGit `required`, Kernel `required` (1,067 selected standalone tests and
34 contract tests), and all nine Kernel family tests. Portal passed 131 Vitest
and 14 standalone Playwright tests, then its security lane refused two matches
of a public fixture key identifier in a retained receipt. The attempt failed;
its logs and original receipt remain preserved. Portal
`3cc19fd388dfbedfca689f1703f9dc719ea9ac23` (tree
`42a023a75de4955a1305f4e96057ddc2b85fad71`) now extends the default scanner
policy with one exact field-and-public-label match. Seven pinned canaries,
eight refusal mutations, and focused security and CI-policy checks passed.
The original failed-run receipt remains preserved unchanged; later runs produce
their own receipts. No artifact directory or whole field was excluded.

Two complete repaired full-family runs passed on September 8 at 04:07 and
04:18 UTC on Hub `cdcbfd50`, Kernel `071173a9`, BulletGit `48755d95`, and Portal `3cc19fd3`.
Each retained all eleven canonical report entries: 1,699 selected cases and
two formal models. Their canonical results match exactly; the observation digest is
`blake3:7a3d352c9fc6ef8fc579fe25728befe78ab83d98afd15c36860f7b1d2675bcbc`.
Each is an unsigned `DIAGNOSTIC_ONLY` observation with `release_authority=false`.
Real run metadata remains separately retained. These dated observations precede
this documentation commit; later source subjects require their own proof. Public App
identity, PR publication, the complete root workflow inventory, portable audit,
and branch protection remain blocked. WP-02 retains `COMPONENT` evidence in
the typed inventory; all G1–G18 remain `DESIGNED`, all release profiles remain
`BLOCKED`, and Operating HOLD remains effective. WP-22 retains its separate
publication component evidence.

The next admission prerequisite closes the incomplete single-location V1
initialization route before record-body reads or sidecar publication. Pristine
bootstrap now inventories both family metadata locations and binds permitted
interrupted stages and generation directories to the durable initialization
intent under the final lock. Retained incidents, foreign generations, ignored
stages, symlinks, and hardlinks cannot be treated as a pristine retry.
`src/coord/store/ledger/admission.rs` owns this guard; the existing initialization
journal still handles exact retries and process death. The final focused ledger
suite passed 51 tests, public coordinator lifecycle/CLI passed 11, and the
repository's two-pass Clippy policy passed. This is a `COMPONENT` prerequisite:
complete two-location inventory/replay/disposition/review records, Genesis-bound
admission references, and the operator checkpoint remain absent. Filesystem
absence does not prove that incident history never existed, and cooperative
same-UID checks do not supply production custody separation. All G1–G18 evidence
classes and all profile results in the typed inventory remain unchanged.

## Gap-register context

September 7 publication component (W0/WP-22; G1/G8/G12): the Hub now owns exact
source capture, deterministic aggregate templates, durable publication requests,
whole-history object scanning, atomic immutable-source/review refs, portable
split reconstruction, and an unsigned exact-event CI observation. Focused local
proof is component evidence only. Public App identity, PR publication, complete
hosted member/family/scheduled inventories, portable audit, protected integration,
and independent release campaigns remain required. No G-gap or product profile
is closed by the narrow publication bootstrap; see the
[runbook](../runbooks/publication.md).

September 8 connected component checkpoint (WP-02; G2/G3/G4/G13/G14): all
three Portal real-farmd family tests passed on exact Portal, Kernel, and
BulletGit subjects recorded in the [active plan](full-product-dogfood-plan.md).
The public command traversed the UDS worker and product Runner/BulletGit,
retained-artifact admission, durable `UNKNOWN`, and restart read-back. Strict UTC
timestamp validation, quiescent WAL shutdown, owned Gitd kill/wait, CI cleanup,
and the pinned SQLite 3.51.3 fix repaired observed baseline blockers. The
provider remains synthetic and the receipt `UNSIGNED_FIXTURE` /
`COMPONENT_PROOF`, with every higher eligibility flag false. This source
browser campaign uses Vite preview; packaged-origin and installation evidence
remain separate. WP-02 records `COMPONENT`, while all G1–G18 remain `DESIGNED`,
all profiles remain `BLOCKED`, and coordinator Operating HOLD remains effective.
A subsequent actual Hub demo and separate component-receipt verification passed
on Hub `cdcbfd50` and Kernel `071173a9`, with Portal `b66a2053` and BulletGit
`48755d95`; exact commits, trees, and receipt digest are in the active plan.
That demo used synthetic execution and ephemeral fixture authority, with
transaction-gate and release-profile eligibility false. The following family
attempt completed Kernel `required` (1,067 selected standalone and 34 contract
tests) and all nine Kernel family tests, then failed Portal security on a
public fixture key identifier. The original failed-run receipt remains preserved unchanged. Repaired
Portal `3cc19fd3` passed its seven pinned scanner canaries, eight refusal
mutations, and focused security and CI-policy checks. Two repaired full-family
runs then passed at 04:07 and 04:18 UTC with all eleven canonical report entries:
1,699 selected cases and two formal models per run. Their canonical results match
exactly; actual run metadata remains separately retained. These unsigned
`DIAGNOSTIC_ONLY` observations have `release_authority=false` and bind the dated
subjects in the active plan, preceding this documentation commit. No successful public CI,
protected integration, native-provider, installation, or transaction
certification is claimed.

September 8 coordinator prerequisite: the single-location V1 initialization
consumer now refuses before sidecar publication. The pristine initialization
path checks both metadata locations and exact interrupted-journal subjects;
retained incident material, foreign generations, and ignored or aliased stages
refuse. Focused ledger (51 tests), public lifecycle/CLI (11 tests), and mapped
Clippy checks passed. This closes an omitted-input bypass component; it does not
supply the complete two-location admission packet or lift Operating HOLD.
The [active plan](full-product-dogfood-plan.md) retains supervised upgrades before
execution migrations, the real Runner/API/Candidate path, all three initial
subscription providers and twelve tasks, publication/CI, and every later profile.
The typed inventory was revalidated together with this entry: all G1–G18 remain
`DESIGNED`, and all 18 product plus two diagnostic profiles remain `BLOCKED`.

## Shared prerequisite observations

September 8 schema prerequisite: authentic schema-22 migration prefixes now return `UPGRADE_REQUIRED` before writable source SQLite startup.
A private preflight preserves source WAL and rollback journals, rejects external super-journal recovery, and bounds snapshot input and
recovered size to 1 GiB. Focused adapter library (40), cross-process startup (5), and lease transaction (13) tests and all-target Clippy
passed after independent hostile review. The supervised upgrade command, maintenance custody, prefix-aware verified backup, high-water
enforcement, and rollback remain open; this startup preflight supplies none of their authority.

A subsequent serving-custody component retains a shared lock on the admitted main descriptor before preflight and throughout the ledger
lifetime. It supports the ext filesystem family and refuses unsupported filesystems or contention. Tests prove independent SQLite close,
duplicate descriptors, process death, and a forced creation-to-lock race; busy refusal preserves the exact inode. The focused custody
identity, 40 adapter, five cross-process, and 13 lease tests and Clippy passed with independent review. Online backups now use the same
shared custody and read-only SQLite, retaining the source guard through publication and bounded receipt read-back. Exclusive-owner and
missing-source refusal tests preserve the complete source/destination inventory. Prefix-aware upgrade backup, exclusive maintenance, and
generation replacement custody remain open.

Typed schema inspection now retains the verified catalog prefix, its digest, normalized authority and restore state. Serving and current
backup still require the current schema; backup receipts take their schema and restore subject from the inspected copy. Schema-22 integrity
requires the complete result `["ok"]`. The extended schema identity, 40 adapter tests, 18 integration tests and Clippy passed with
independent review. Authority is a sampled row: coherent upgrade admission still requires transaction and maintenance custody. This
component adds neither prefix-backup permission nor an upgrade or high-water authority.

Online backup now explicitly closes its admitted source on every producer result. A real SQLite close failure retains the connection and
custody until process exit and refuses new admissions before filesystem effects. Confirmed close followed by postflight or cleanup failure
reports the failure without poisoning later admissions. The extended child-process identity, 40 adapter tests, 18 integration tests and
Clippy passed. Existing in-flight operations and unrelated serving shutdown paths are outside this repair; prefix backup and exclusive
upgrade custody remain open.

A two-location preservation component now binds the complete outer and Hub inventories to one purpose-specific record. `bullet-family
preservation-bind` accepts supplied canonical mode-0600 observations in private mode-0700 parents and publishes a sealed mode-0400 record
outside the family. It never observes or moves the incident directories. Successful creation or exact-existing adoption requires retained
file and parent synchronization plus identity and byte read-back; persistent post-link sync failure refuses, retains the inode, and permits
exact retry after recovery. Five pair, ten publisher, eight sealed reader, and two CLI fixture tests passed. This is component evidence;
paired replay and historical dispositions, independent review and operator admission, locked Genesis references, and the complete
preservation fault journal remain open. Operating HOLD is unchanged.

A subsequent `preservation-bind replay` component consumes supplied sealed pair/request records and both complete retained ledger copies. It
binds exact copy bytes, replay projections, every historical claim, and ordered dispositions to one sealed `REPLAY_FACTS_ONLY` record, with
read-back and exact retry after synchronization failure. Recorded receipts must exist in the replayed history; other claims remain retained
for recovery. Missing, corrupt or interrupted histories, omitted claims, invented receipts, and unsafe or conflicting paths refuse. Six new
replay identities and 25 related tests passed, followed by combined Clippy and canonical inventory checks. The producer does not observe
incident directories or admit Genesis: complete incident-range handling, independent review, durable locked admission references and the
operator checkpoint remain required. Operating HOLD is unchanged.

A CI expectation component now derives 55 invocations from the reviewed 53 nested job definitions and exact eight-workflow inventory. The
canonical plan binds aggregate and member commits/trees, manifest and workflow digests, matrix values, dependencies, and runner selection.
`bullet-publish ci-plan` supports an admitted aggregate checkout or an existing publication store/request, without creating another checkout
or Git objects. Independent review against all eight source workflows and 29 publication tests passed. Its output explicitly contains no
execution evidence. The 27 required invocations and 28 scheduled invocations remain distinct; root workflow generation/activation, hosted
tool and worker admission, job observation envelopes, the stable final check, and additional family/MSRV and assurance campaigns remain
open.

A bootstrap CI diagnostic adapter now binds the selected immutable plan row, aggregate and member subjects, actual GitHub
event/workflow/run/attempt, and bounded member observation bytes. `ci-job-context` and `ci-job-observe` support only
`bullet-farm:REQUIRED:source_scan` under the existing bootstrap root job. The observer executes the exact source-bound semantic validator
and retains its output digests. The bootstrap job does not execute the nested source scan; both outputs state `execution_evidence=false`,
with no run or release verdict. Twenty-nine publication tests and both Clippy passes succeeded. This sampled validation does not attest
hosted provenance or isolate mutable same-UID tools. Generated root job execution, complete execution observations and the stable required
convergence check remain open. The measured Rust inventory is now 789 identities, partitioned into 570 Hub and 219 wire tests, with
precisely the six new replay identities added and no prior identity removed. The typed inventory remains unchanged: all 18 gaps are
`DESIGNED`, and all 18 product plus two diagnostic profiles remain `BLOCKED`.

The complete Kernel `required` check subsequently passed on commit
`3299443d4f15bb27e84aa8f4f70512ca800f4dd5`, tree
`1e260d91f13f462354bd5dcb6679240ad65f21c5`: 1,067 selected standalone tests,
34 contract tests, and the mapped lint, security and documentation checks.
Log SHA256: `da64c0cce7709258ecb861a9585f1ab3274cabaea2c874cd9790ebf5653fad28`.
Only that run's log and newly generated fast/contract reports receive current
credit; inherited coverage, family, faults and older observation files do not.
