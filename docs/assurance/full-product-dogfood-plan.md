# Full-product dogfood bridge

Status: **ACTIVE implementation plan; all release profiles remain `BLOCKED`**  
Owner: Bullet Farm maintainers  
Last reconciled: 2026-09-08

This is the execution bridge from the frozen local family to Bullet developing
Bullet through its own complete transaction. It refines the
[finish execution plan](execution-plan.md); it does not replace the
[G1–G18 register](product-gaps.md), [Waves 0–11](closure-roadmap.md),
[WP-01–WP-23](../workplan.md), or [OD-A–OD-J](../decisions/0013-operator-decision-register.md).
The executable profile check and admitted receipts always win over this page.

The September 8 execution order continues local engineering while the publication
App is unavailable. Exact-source publication and operational hosted CI remain a
parallel required track alongside the Codex–Claude–Cursor subscription loop. All three
providers are required initially; Antigravity follows. Four canonical source
repositories retain their authority, publishing exact reviewed objects/trees to
`neverhuman/bulletfarm` through App-authored review branches and human-approved
PRs. The [publication runbook](../runbooks/publication.md) records the component
implementation and remaining hosted prerequisites. Every ordinary, family,
scheduled, and independent certification campaign must execute on final subjects.

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

## 1. Finish lines

“Dogfood” has four non-substitutable finish lines. Work is planned against the
earliest honest one, while preserving the path to the complete target.

| Finish line | Meaning | Earliest gate | Current fact |
| --- | --- | --- | --- |
| Coordination dogfood | Bullet uses a distinct preserved development generation for claims, handoffs, commits, receipts, exact retries, and restart; historical recovery stays separate | Complete two-location admission, independent review, operator checkpoint, W0 | **BLOCKED**; Operating HOLD remains effective |
| Full-product offline dogfood | One Bullet change crosses Control, Execution/BulletGit, Verification, Delivery/integration, and Evidence/audit with all twelve fault boundaries | W7 `TRANSACTION_PROOF` | **BLOCKED**; a retained component bridge reaches purpose-signed fixture intent/evidence/proof and `MATCHED` Observation, exact Candidate delivery, local check, protected integration, reopen read-back, and post-exit retained-artifact reads, but its executor/keys/roles are harness-process fixtures, its outer receipt is unsigned/ineligible, and independent custody plus the twelve-boundary campaign are absent |
| Live self-hosted dogfood | The same product path adds separately admitted Codex, Claude, and Cursor subscription execution and protected Jeryu integration on the signed Ubuntu family | W8 `self-hosted-v1` | **BLOCKED** by W7 and OD-A/B/D/E |
| Complete documented target | Independent evolution, provider, forge, platform, team, and saga profiles also pass | W9–W11 and all eight finish conditions | **BLOCKED**; no narrower receipt may be borrowed |

Engineering owns the local consumers and proofs. The coordination transition
also requires its exact reviewed operator checkpoint; live and release work
requires the separately admitted account, signing, service, and forge custody. No percentage,
checkbox, local simulation, or chat line promotes an evidence class.

## 2. Critical path and stop rules

```text
local engineering and exact-source publication/hosted CI proceed in parallel
→ complete two-location preservation/admission and durable Genesis consumers
→ independent review and operator fresh-generation checkpoint
→ distinct development generation initialization and restart read-back
→ W0 four clean heads, family observation, first reviewed coordinated change
→ supervised schema 22→23 upgrade before account/run/quota migrations
→ W1 generated contracts, immutable subjects, and durable launch admission
→ W2 durable authority and W3 supervised Runner/service isolation
→ W4 production BulletGit and W5 Candidate verification/effect reconciliation
→ W6 durable operator commands, projections, and Portal reconciliation
→ fake provider through the real path, then twelve Codex–Claude–Cursor tasks
→ seven-day integrated-change survival observation
→ W7 signed twelve-boundary transaction and Ubuntu lifecycle
→ W8 admitted subscriptions + protected Jeryu self-hosted transaction
→ W9/W10/W11 complete the documented target
```

The upgrade path requires exclusive maintenance custody, a verified backup of
an authentic supported migration prefix, one migration transaction, final
reopen/read-back, external authority high-water, and quarantined rollback.
Startup recognition alone is not upgrade execution. Subsequent execution tables
must use this mechanism. `/api/v1/commands` remains the runtime mutation ingress:
revision/idempotency/authority checks, quota reservation, launch nonce consumption,
run allocation, and dispatch enqueue commit atomically. Runner must persist
supervision and canonical results, reconcile ambiguous starts without a second
invocation, and retain termination evidence before releasing capacity.

The first native campaign comprises twelve bounded tasks, four per subscription
CLI: Rust, TypeScript/React, tests, and documentation. Each selected account has
its own credential/runtime generations and private home. Vendor quota buckets
remain distinct from Bullet invocation use; warnings at 80%, critical alerts at
95%, and exhaustion pauses survive restart. Manual `OPERATOR_REPORTED` snapshots
expire within one hour or reset and need finite local allowances; they cannot
override later vendor exhaustion. Unknown actual charges remain `UNPRICED`.
Account selection and resume are explicit. Per-task limits remain one active invocation
per account, two implementation workers, two repairs, one escalation, eight
provider invocations, and 60 minutes; pauses and account changes reset none.

Every packet stops on dirty or changed subjects, failed/skipped/zero tests,
missing artifacts, stale generations, signer/identity substitution, an
unreconciled effect, or `UNKNOWN`. A retry first reads back by the original
request and desired-state identity. It never dispatches a second write merely
because a response was lost.

## 3. Phase R and W0: start useful dogfood safely

These packets are serialized. R4–R7 remain historical recovery obligations;
they do not initialize the selected distinct development generation. That
proposed route needs complete incident-derived admission from both locations,
final locked validation and Genesis persistence, independent review, and the
operator checkpoint under ADR 0015. The user-authorized source-maintenance
exception permits the sole integration owner to prepare review branches and
prove bootstrap subjects while the coordinator remains on HOLD. It grants no
coordinator, enrollment, provider, signing, or integration authority.

| Packet | Work and owner surface | Exit evidence | Active hold |
| --- | --- | --- | --- |
| DF-R4 | Hub recovery verifier/ledger: validate the published recovery tombstone, retired source, sibling absence, and sealed observation chain before and at final locked replay | Clean exact R3.1+R4 subject; all recovery/adoption hostile suites; strict lint; independent artifact review | Bounded logic is exact-copy green; `cargo clippy --locked --workspace --all-targets -- -D warnings` is 0/0 on this tree; R4.1 `canonical_hostile` 8/8 after the current path/attribute/dependency inventory pin. No live execution is authorized |
| DF-R5 | Hub private recovery facade and narrow CLI: exact normalized absolute inputs, descriptor-safe Linux admission, sole inner topology oracle, deterministic writer-wait/resume, typed non-Linux refusal | Facade publish/wait/resume/already-current tests; facade-level Exchange crash/restart; macOS/Windows compile and zero-mutation CLI refusal | Linux facade, Exchange restart, and static hosted-lane policy are green. This Linux host now fail-closes with `NATIVE_PLATFORM_EVIDENCE_UNAVAILABLE` (`scripts/platform-native-evidence.sh`); native `macos-15`/`windows-2025` compile/refusal remains unproved until those runners admit `ops/ci/platform-refusal.sh` |
| DF-R6 | Hub recovery producers: internally derived proof PASS, sealed independent APPROVE, canonical adoption request, explicit `adopt --request`, request-byte idempotency/conflict | Failed/SKIP/UNKNOWN/non-APPROVE cannot append; reviewer differs from orchestrator; exact proof set/watermark; retry invokes no clock/process/write | Linux component implemented: the [operator runbook](../runbooks/coordinator-recovery.md) covers the five closed CLI actions; focused model/backend/CLI chain is 5/5, public ingress 2/2, the complete coordinator library is 159/159, canonical hostiles are 8/8, and four canonical public suites are 19/19. The former 41 shared Hub lint findings are historical; exact Hub `d3c4db3600e6f1d93800d258e3ac7c1a2b4db4f3` passed `required`, including strict lint. Native-platform execution, R7 rehearsal, independent review, and the real incident remain blocked |
| DF-R7a | Fresh owner-0700 synthetic family: run rollover → proof → review → request → adopt → restart with injected crashes at every publication and append boundary | One immutable rehearsal bundle; deterministic redacted JSON; byte-identical rerun; independent review | Unsigned COMPONENT rehearsal producer (`scripts/recovery-rehearsal.sh`) is landed: 0700 parent, byte-identical rerun, live incident hard-false. Signed independent bundle review and policy-enabled recover-rollover remain open (`RECOVERY_POLICY_DISABLED` without operator keys) |
| DF-R7b | Frozen real incident: independently compare every live input hash to the reviewed rehearsal, execute once under supervision, adopt every reviewed break-glass group, restart and read back | Current schema-2 generation, complete watermark, no unexplained frozen claim, signed human review record | Compare/refuse producer (`scripts/recovery-incident-compare.sh`) is landed: missing APPROVE and frozen live source both refuse; the script cannot chmod, recover, or adopt. Execution waits on independent human approval |
| DF-W0a | Four repository owners close active changes, run atomic and standalone `required` lanes, then sole-writer commits reviewed path sets | Four clean immutable commit/tree pairs; no missing, skipped, zero-test, dirty, or orphan partition | Source preparation proceeds under the maintenance exception; coordinator use waits on complete two-location admission and the fresh-generation checkpoint, while historical recovery remains separate |
| DF-W0b | Family order: BulletGit build → Kernel component/family with exact daemon path → Portal component/real-farmd browser → Hub contracts/models/media | Deterministic unsigned `bullet.ci-observation.v1`; second identical family run | No signed Evidence is claimed |
| DF-DOG0 | First low-risk docs/test-only Bullet change uses status → exact claim → heartbeat → handoff → proof → sole-writer commit receipt → restart read-back | Stored request IDs and complete watermark reconstruct the whole loop after process restart | Any direct commit or inferred receipt invalidates the exercise |

DF-DOG0 is the earliest useful development dogfood. After it passes, all new
Bullet work should use the coordinator, while the product still describes that
use as `COMPONENT`, never `TRANSACTION`.

## 4. W1: one immutable language and family

W1 prevents every later service from inventing its own identity or receipt
meaning.

| Packet | Deliverable | Required negative proof | Dependency |
| --- | --- | --- | --- |
| DF-101 | Publish one recursively closed `bullet-wire-v1` source for Authority, Transaction, Forge, Evolution, Release, Candidate, Evidence, Effect, and receipt records; regenerate Rust, JSON Schema, OpenAPI, and TypeScript | Unknown fields, duplicate keys, unsafe numbers, aliases, wrong tagged OID, partial IDs, and generated drift fail | Clean W0; WP-03 |
| DF-102 | Canonical bytes and identity: RFC 8785 JSON, domain-separated BLAKE3, full-width IDs, exact request digests, one-use nonces | Cross-type/domain substitution, non-canonical equivalent bytes, overflow, replay, and digest aliasing fail | DF-101 |
| DF-103 | Trust admission: PASETO authority, Ed25519 receipts, signer roles, trusted time, revocation, high-water, exact-family and dependency closure | Self-selected policy/root, wrong role/family, expiry boundary, revocation, stale high-water, and generic-receipt substitution fail | DF-101/102; OD-E custody for release evidence |
| DF-104 | Schema-3 `family.lock`, external-component lock, signed immutable member/wire/Jeryu subjects, and non-circular Hub-last tag order | Branch, sibling path, mutable URL, missing OID/tree/digest, tag/lock cycle, and second-resolution drift fail | OD-D, reviewed Jeryu tag, DF-101–103 |
| DF-105 | Exactly two model locks—authority/lease/fence and effect/check/integration—plus generated traces bound to receipts | Model drift, missing trace, extra authoritative model, and trace/receipt mismatch fail | DF-101–103 |

W1 exits only when two fresh resolutions produce identical authenticated bytes
and every receipt kind has a hostile-tested semantic verifier. It closes the
language and subject boundary, not a transaction.

## 5. W2–W3: durable authority and isolated execution

### W2 work packets

| Packet | Owning code | Exit |
| --- | --- | --- |
| DF-201 normalized truth | Kernel domain, SQLite migrations, adapters | Mission, graph revision, Variant, Attempt, scope, policy/config generation, authority epoch, budgets, leases, fences, freeze, intervention, and outbox are constrained columns/relations rather than authoritative blobs |
| DF-202 atomic authority | Kernel application/adapters | One serialized write transaction commits state, event, audit link, hash chain, and outbox; concurrent acquire, expiry, zero-row renewal, and quota races cannot over-authorize |
| DF-203 capability custody | Kernel authority and Runner transport | Durable peer registry, operator-admitted signing key, nonce/grant high-water, exact `SO_PEERCRED` binding, product Runner client, and lost-response read-back survive restart |
| DF-204 storage continuity | Kernel SQLite/CAS/GC/backup | `synchronous=FULL`, safe GC, audit-root continuity, verified backup/restore, external high-water, and `SAFE_STOPPED` on corruption or ambiguity |
| DF-205 recovery mode | Kernel control and services | `RECOVERING` invalidates leases/grants/credentials, reconciles remote truth, and requires independent recovery approval before mutation resumes |

### W3 work packets

| Packet | Owning code/operations | Exit |
| --- | --- | --- |
| DF-301 identities | Service packaging and UDS boundaries | Distinct control, runner, BulletGit, verifier, broker, attestor, integrator, observer, auditor, and Jeryu users; no cross-role state or credential read |
| DF-302 S1 workcell | Runner, rootless `crun`, cgroup/seccomp/network policy | Read-only source/root, private HOME/tmp/cache, bounded writable roots, default-deny egress, resource limits, and full process-tree teardown pass hostile isolation tests |
| DF-303 lifetime/secret projection | Runner and credential broker | Monotonic kill timer starts before Git/provider/forge; short-lived role-scoped credentials are projected only after authority; canary never appears in logs/artifacts |
| DF-304 S2 boundary | Runner/Firecracker package | S2-required policy refuses before spawn until an exact guest image has its own containment receipt; no S2→S1 downgrade |

W2/W3 exit evidence must include crash, restore, UID spoof, socket swap, escape,
egress bypass, fork bomb, resource exhaustion, canary, and survivor hostiles. A
debug registry, ephemeral key, inherited HOME, or process-local recovery map is
not production authority.

## 6. W4–W5: exact Candidate to reconciled integration

| Packet | Deliverable | Exit proof | Current component fact |
| --- | --- | --- | --- |
| DF-401 mutation permit | Kernel durably reserves a one-use operation and BulletGit repeats the final online lease/fence/subject check before I/O, then settles or reconciles it | Stale/replayed/wrong-scope permit and authority loss perform no Git mutation | The retained bridge durably admits its ScopeGrant, uses peer-authenticated farmd/Runner, and requires a Kernel-issued exact Candidate grant plus final check before production Gitd prepares the one-use Candidate. This is private component authority, not an admitted release reservation. |
| DF-402 private generation | BulletGit applies one exact `PatchProposal` through dirfd/openat2 into a private generation with inode-safe locks, fsynced journal/CAS/tree, and atomic active-generation switch | Traversal, `.git`, symlink/reparse, hostile config/filter/attribute, binary substitution, crash, and ENOSPC hostiles fail closed or resume exactly | Production Gitd is on the retained path and stale fence is refused; the full filesystem, crash, ENOSPC, and immutable published-subject exit proof remains required. |
| DF-403 Candidate | Candidate and Integration manifests bind repository, lineage, graph, Attempt, scope, policy, environment, toolchain, gates, and distinct proof roots | Candidate reconstructs byte-for-byte; rebase/merge/result-OID changes invalidate identity | Kernel chooses the exact Candidate preparation subject, production Gitd returns the one-use Candidate, fixture verification refuses writer identity and then records PASS, and purpose-signed fixture intent/evidence/proof plus every effect retain the same Candidate/head/tree. The outer receipt remains unsigned and both eligibility flags are false. |
| DF-404 forge port | Capability handshake, expected-old-OID delivery, immutable-ref read-back, idempotent check/PR, protected integration, target read-back, observation, reconciliation; one active primary profile | `LocalBareForge` proves `UNKNOWN` and `ORPHANED_REMOTE` without overwrite or duplicate logical effect | The retained `LocalBareForge` chain delivers and authoritatively reads back the exact Candidate head, refuses stale fence, reconciles lost response `UNKNOWN` to `COMMITTED`, publishes/read-backs the exact-SHA check and actual ProofBundle root, performs protected expected-old-OID integration, purpose-signs authoritative target outcome `MATCHED`, and proves all subjects again after reopen. Private ordinary-Git source/Candidate/target artifacts and the ledger are retained; the shell independently reads the exact three Git subjects after child exit. These remain local fixture components, not external-forge evidence. |
| DF-501 independent verifier | Verifier-owned identity/workcell reconstructs the exact Candidate and signs Evidence/ProofBundle under immutable `GateSpecV1` | Author/writer cannot self-qualify; zero/skipped/flaky/timeout/infra/unknown never becomes PASS | The retained harness now issues purpose-separated PASETO v4.public/JCS `VerificationIntentV1`, `EvidenceV1`, and `ProofBundleV1`, reconstructs its ephemeral public subjects, and reverifies the canonical chain. The executor and signers still share the harness process; the keys are self-asserted `FIXTURE_KEY_ONLY` subjects with no trusted lifecycle, durable nonce consumption, distinct verifier UID/credentials, or independently owned verifier artifact custody, so independent-evidence eligibility stays false. |
| DF-502 five-authority effects | Distinct broker, attestor, integrator, and observer execute the legal order only | Candidate → ProofBundle → delivery/read-back → check/read-back → integration/read-back → Observation; every retry reads remote truth first | The purpose-signed fixture ProofBundle now feeds the complete local order and reopen; the caller-free fixture observer authoritatively reads the target, signs `ObservationV1` as `MATCHED`, and reconstructably reverifies it. Broker/attestor/integrator/observer are still not distinct OS identities or credential/key owners, so the signed Observation remains ineligible component truth. |
| DF-503 audit closure | Observer/auditor bind commands, identities, subjects, outcomes, and hash-chain anchors without receiving mutation credentials | A clean read reconstructs the effect history; audit loss or mismatch blocks completion | The nested verification/Observation chain is signed and private source/Candidate/target plus ledger artifacts are retained, but the containing JSON remains an unsigned `COMPONENT_PROOF`, not a signed audit or release receipt. |

From the `bullet-kernel` checkout, preserve the current component boundary with:

```bash
BULLET_GITD_BIN="${BULLET_GITD_BIN:?set exact absolute canonical daemon}" \
BULLET_GITD_SHA256="${BULLET_GITD_SHA256:?set exact lowercase SHA-256}" \
just proof-transaction-offline
```

The retained diagnostic is
`/tmp/bullet-offline-component-proof.observation-20260827T1340Z/COMPONENT_PROOF.receipt.json`,
SHA-256 `b0aeadaebc834dd20e6c8f885b48d1868c0b4005d34a05b500fd27e23b6e5eb2`.
It binds Candidate
`can_6a1825080083e84b1f6a834ba11281e13dcb5bc08ec57844531f98379e215e3a`,
base `571e25fe7171eb98d00d4477481a3223f8e45b32`, head
`c60e2d674cc27b8520e16ff4b961ade355cddc64`, and tree
`618430ce7ff8883985bf50af5c074b07626ddc4d` through the nested signed chain
and every local effect. The actual ProofBundle/check/protection root is
`prf_9f433d2583c534e27a5a8cb853be7b42d05c31981e673f0cbf3bc6efa8f514e4`.
Reconstructed ephemeral public subjects reverify the verification chain at
BLAKE3 `e562a67094f4c54d3fbf7943555df54d204500d64d4b24a9709a3269da20e216`
and the signed `MATCHED` Observation at BLAKE3
`571d1f416ae67d9fb50a3ba66374a82aa3beda3011fc62cbe62edc9f3b191855`.
After child exit, the shell independently reopens exact source HEAD, Candidate
HEAD/tree, and target HEAD under retained sibling `artifacts`; the regular
private ledger remains at `data/ledger.sqlite`. The outer receipt remains
`UNSIGNED_FIXTURE`, nested records remain `FIXTURE_KEY_ONLY`, and independent,
transaction, and release eligibility are hard false. After independent
key/nonce/UID/credential/artifact custody and semantic receipt admission land,
this same command is the next honest read-back; the harness fixture cannot
update an eligibility flag.

This is the center of full-product dogfood. W4 may create an exact Candidate;
only W5 can independently qualify and reconcile it.

## 7. W6–W7: operable product, transaction, and package

### W6 operator surface

- DF-601 serves operator commands only at `/api/v1` and workload commands over
  peer-authenticated `/internal/v1`; legacy `/v1` refuses without mutation.
- DF-602 requires idempotency key plus expected revision and returns a durable
  `CommandReceiptV1`; signed dispatch and typed read-back survive response loss.
- DF-603 provides bounded snapshots, pagination, resumable authenticated SSE,
  atomic watermarks, queue backpressure, and `RESYNC_REQUIRED` on gaps.
- DF-604 makes every selected Portal surface durable or explicitly
  `OUT_OF_PROFILE`; missing subjects and unavailable read-back stay visible.
- DF-605 embeds manifest-verified Portal bytes at the farmd origin with CSP,
  OIDC+PKCE/off-loopback TLS, origin/CSRF/RBAC controls, WCAG 2.2 AA automation,
  and a retained manual review.

Current W6 component fact: a retained exact-subject wrapper authenticates an
idempotent public `run_demo` POST, survives farmd restart, replays and polls the
same command/request through the Vite-preview Portal, dispatches it to a registered
same-UID `SO_PEERCRED` UDS Runner and bounded exact worker, admits the retained
fixture transaction receipt, atomically settles the same command/request/raw-
receipt BLAKE3 to durable `UNKNOWN`, and reads `NO_COMMAND` after worker restart.
The outer result remains `COMPONENT_PROOF` / `UNSIGNED_FIXTURE`, nested records
remain `FIXTURE_KEY_ONLY`, and every eligibility flag is hard false. This closes
no DF-60x exit: signed `CommandReceiptV1`, operator custody, distinct identities,
process-level response-loss and twelve-boundary chaos evidence, remaining
projection/SSE guarantees, and package/install admission are still absent.

### W7 proof and lifecycle

| Packet | Required artifact | Acceptance |
| --- | --- | --- |
| DF-701 vertical dogfood scenario | One low-risk Bullet docs/test task entered through farmd, executed by simulator under a real grant, written by production BulletGit, independently verified, integrated into `LocalBareForge`, observed, and projected in Portal | Exact IDs and subjects agree across all five authorities and after restart |
| DF-702 twelve-boundary campaign | Signed fault bundle for grant, Runner start, workspace open, provider completion, patch apply, checkpoint, Candidate prepare, verifier handoff, delivery, check, integration, and observation/cleanup | Death, timeout, response loss, stale authority, freeze, clock shift, ENOSPC, and restart never duplicate or falsely complete work |
| DF-703 `TRANSACTION_PROOF` | `just proof-transaction-offline` emits one signed exact-family `GateReceiptV1` plus sanitized artifacts | Kind-specific admission succeeds from an absolute registry; changed family/policy/toolchain/evidence fails |
| DF-704 Ubuntu package | Reproducible x86_64 Ubuntu 24.04 archive with embedded Portal and binaries, OCI/S2/Jeryu assets, checksums, CycloneDX/SPDX SBOM, provenance, signatures, and rollback bytes | Different-identity network-bounded rebuild produces identical admitted artifacts |
| DF-705 lifecycle | Two clean installs from the same schema-3 lock; activate, upgrade, backup, restore, rollback, uninstall-with-retention, and disaster recovery | Exact read-back, non-circular manifests, authority high-water preservation, no ambient tool/credential substitution |
| DF-706 assurance | Portable audit, advisories/licenses/sources, secret/workflow/CodeQL/fuzz/sanitizer/chaos/accessibility gates, exact paper/brief rebuild | Jankurai ≥90, zero caps/hard findings, no skipped/missing result, artifact hashes match documentation |

DF-703 is the first honest **full-product offline dogfood**. DF-705 is the
first stranger-installable lifecycle. Neither authorizes a live provider or
forge.

## 8. W8: first live self-hosted dogfood

1. Finish the local provider-policy/enrollment-anchor consumers, exact runtime
   probe, launch/egress/teardown receipts, budgets, and onboarding UX before
   requesting credentials.
2. Consume OD-D/E custody and each independently admitted OD-A Codex, Claude,
   and Cursor account enrollment. Complete each provider's native conformance
   and the twelve-task campaign on exact admitted subjects. Stop
   on runtime/model/profile drift, budget uncertainty, canary leak, or teardown
   ambiguity.
3. Finish the offline Jeryu semantic adapter and receipt admission before
   consuming OD-B. Admit the exact deployment passport, protected repository,
   and distinct broker/attestor/integrator/observer credential handles.
4. Send the same exact Candidate through protected Jeryu delivery, check,
   integration, target read-back, observation, backup/restore, and drift proof.
   A lost response remains `UNKNOWN` until authoritative read-back.
5. Admit provider, forge, package, operations, and two-install receipts through
   their kind-specific verifiers. Run `check release --profile self-hosted-v1`
   from the admitted absolute registry.
6. Only after PASS, record install and provider task media from those same signed
   subjects. Media is a projection of proof, never the proof itself.

## 9. W9–W11: all work beyond first GA

| Phase | Independent branches | Completion condition |
| --- | --- | --- |
| W9 evolution | CognitiveTask/SelectionGroup/Role/Fusion persistence, budgets/routing/struggle, immutable recipes, matched-compute study, holdout custody, shadow, rollback readiness; then OD-H bounded ≤1% R0/R1 canary, promotion and drift | `evolution-v1` passes independently; it is never implied by self-hosted or universal |
| W10 providers | Maintain independent Codex, Claude, and Cursor certifications from the initial campaign; add Antigravity exact runtime/enrollment/conformance under OD-A | Four provider receipts exist; none substitutes for another |
| W10 forges | GitHub App under OD-C, GitLab.com under OD-I, self-managed GitLab under OD-J | Jeryu plus three independent forge profiles pass exact protected integration and reconciliation |
| W10 platforms | Linux aarch64, macOS x86_64/arm64, Windows x64 build/install and certified containment or typed mutation refusal | Five platform slices install twice and pass their exact profile; Linux remains the mutation reference until separately certified |
| W10 research | Preregistered matched corpus with subjects, costs, failures, and receipts | No benchmark/superiority claim precedes admitted comparable evidence |
| W11 team | PostgreSQL authority, remote runners, workload mTLS/SPIFFE, object storage, durable stream, partition/failover fencing, distributed restore | `team-v1` passes before saga work receives credit |
| W11 saga | Staged multi-repository Candidates, dependency quarantine, forward repair, compensation as new effects, exact global read-back | `saga-v1` passes without rewriting prior history |

## 10. Completeness crosswalk

No registered work may disappear between documents and implementation.

| Closure band | Gaps | Workplan | Operator facts |
| --- | --- | --- | --- |
| R/W0 coordination and baseline | control prerequisite to all G rows | WP-01, 11, 20, 22 groundwork | none |
| W1 contracts/subjects | G1, G4, G5, G12 | WP-01, 03, 07, 19, 22 | OD-D/E |
| W2/W3 authority/isolation | G2, G3, G8, G10, G14 | WP-02, 12, 16, 21 | later service custody; no live credential |
| W4/W5 writer/verification/effects | G2, G4, G6, G7, G14, G16 | WP-02, 04–06, 16, 19 | live forge acts wait |
| W6/W7 product/package | G1, G2, G8, G9, G12–G14 | WP-01–05, 11, 13, 20, 22 | OD-D/E |
| W8 self-hosted | G5, G6, G9, G10, G12–G14 | WP-06–08, 12, 13, 19, 21, 23 | OD-A/B/D/E; OD-G only for public topology |
| W9 evolution | G11, G13, G15 | WP-15 | OD-H after offline prerequisites |
| W10 breadth | G5, G7, G9, G10, G12, G16 | WP-06, 09, 12, 15, 17, 21, 23 | OD-A/C/G/I/J |
| W11 distributed/saga | G17, G18 | future profile work routed by the gap register | distributed infrastructure custody |
| Active post-V1 and retained governance | no release gap may be cleared by it | WP-09/10/14/17; WP-18 retired | OD-F/G clear no release gate |

The W0 typed bidirectional inventory must fail if any G1–G18, WP-01–WP-23,
OD-A–OD-J, W0–W11, invariant, receipt kind, runtime owner, test, or gate lacks a
reverse link. This table is a human route, not that executable inventory.

## 11. Ownership and safe parallelism

- One owner at a time edits a trust-boundary seam. Recovery verifier/ledger,
  recovery CLI, receipt producers, and live incident execution remain
  serialized.
- Authorized source-maintenance lanes may prepare the baseline during Operating
  HOLD with manual path-exact custody. Coordinator use waits on the admitted
  development-generation transition; the exact family lane remains serialized
  in dependency order.
- After W1 freezes wire identity, Kernel W2, local packaging, Portal projection,
  and provider-onboarding refusal work may proceed on disjoint paths. They
  converge only on clean immutable subjects.
- W4 BulletGit authority and W5 verifier/effects may develop in parallel only
  against the same frozen contract fixtures; the vertical scenario does not run
  until both exact implementations are admitted.
- W10 provider, forge, and platform slices are independent. One failed slice
  blocks itself and `universal-v1`, not a previously admitted self-hosted build.
- Operator acts are requested only after their local consumer, negative tests,
  least-privilege scope, expiry, revocation, rollback, and read-back procedure
  are complete. No live secret waits inside an unfinished path.

## 12. Owner cadence and definition of done

Every packet handoff records exact files, commit/tree, cleanliness, commands,
tool versions, test counts, outcomes, artifact hashes, evidence class,
remaining holds, and independent reviewer. The owner board is reconciled after
every packet, not after a wave-sized batch.

The active engineering queue prepares publication/CI and DF-W0a/b subjects,
completes both-location admission and locked Genesis consumers, then obtains
independent review and the operator fresh-generation checkpoint. Only the
admitted transition and restart read-back permit DF-DOG0. DF-R4–DF-R7b remain
separate historical recovery obligations; this plan authorizes no incident
mutation or coordinator initialization. Once DF-DOG0 passes, the vertical
product queue is DF-101 through DF-105, DF-201 through DF-205, DF-301 through
DF-304, DF-401 through DF-503, DF-601 through DF-605, and DF-701 through DF-706.
W8 live self-hosted certification begins only from that admitted W7 subject;
local provider consumer engineering proceeds while external checkpoints wait.

“100%” still means all eight conditions in the
[finish execution plan](execution-plan.md#9-definition-of-finished) hold at
once. Until then, report the exact narrower evidence class and blocker.
