# Bullet Farm Safety-Complete V1 closure plan

Status: **ACTIVE — pre-release; release authority is blocked**
Owner: Bullet Farm maintainers
Last reconciled: 2026-08-25
Scope: four repositories, local-first, single-user V1
This dependency graph and evidence register is not runtime authority. Only exact
generated/ledger/Git/Evidence/effect/release subjects can make a gate green.

## Status vocabulary

| Status | Meaning |
| --- | --- |
| `COMPLETE` | Exact committed subject and mapped receipt exist for this bounded claim |
| `IN PROGRESS` | Claimed work or focused evidence exists, but no completed commit receipt exists |
| `LOCAL-BLOCKED` | Implementable offline work remains or a predecessor safety gate is not green |
| `EXTERNAL-BLOCKED` | Promotion needs operator-controlled service, credential, signer, or platform evidence |

A newer subject invalidates its receipt until the mapped gate is replayed.

## Frozen Safety-Complete V1 contract

- The public name is **Bullet Farm**. Centerrail and `TEAM.md` are historical
  design provenance. Where they conflict with this reviewed plan, this plan's
  frozen V1 choices win; historical bytes remain preserved and hashed.
- Kernel owns Missions, immutable graph revisions, commands, leases/fences,
  routing and policy snapshots, the event log, and the outbox. It never owns
  Git credentials or unverified engineering truth.
- BulletGit owns private clones, atomic patch application, journal/CAS,
  checkpoints, exact Candidates, and Candidate proof roots. It never schedules
  Missions, holds provider credentials, or grants protected-ref authority.
- Providers are read-only proposal producers. Runner supervises them and may
  execute only admitted gate IDs. Verifier reconstructs the exact Candidate in
  an independent clean environment. The effect broker alone holds forge
  credentials. Portal is a sequence-bound projection and never an authority.
- Initial source distribution is Jeryu-only from immutable signed tags. No
  `neverhuman/bullet-*` GitHub namespace is assumed. GitHub remains a required,
  separately configured effect adapter, not source authority.
- The shared wire uses RFC 8785 canonical JSON, domain-separated BLAKE3, full
  256-bit lowercase IDs, and algorithm-tagged Git OIDs. `ContentId` is distinct
  from provenance-bound `CandidateId`; Candidate and Integration proof roots
  are distinct.
- Public mutations use authenticated `POST /v1/commands`, initially return
  `202 PENDING`, and reconcile through `PENDING|APPLIED|VERIFIED|FAILED|UNKNOWN`.
  Transport success never implies verification.
- Runner, verifier, effects, and `bullet-gitd` use negotiated, bounded,
  versioned JSON-RPC 2.0 over JSONL stdio. Stdout is protocol-only.
- Local V1 is single-user and loopback-only. PostgreSQL, distributed teams,
  remote runners, cross-repository sagas, semantic merge synthesis, and online
  learning are post-V1; versioned ports may exist, speculative code does not.
- Exactly **two** protocols are model-checked: lease/fence/reclaim and command/
  effect ambiguity under timeout. No third formal model is a V1 gate.
- GA requires conformant **Claude, Codex, Cursor, and Antigravity** adapters;
  `TEAM.md` critique C8's any-two recommendation is superseded for this V1.
- Release has exactly **five** archives: Linux x86_64/aarch64, macOS x86_64/
  arm64, and Windows x64. Non-Linux mutation fails until containment passes.
- Pre-1.0 schemas are disposable. Unknown/legacy databases fail with typed
  `UNSUPPORTED_SCHEMA` plus explicit export/removal guidance.
- Product comparison is [commit/date-pinned](competitor-snapshot.md); no performance
  claim is valid before the same receipt-bearing corpus runs on both systems.

## Reconciled repository subjects and receipts

The committed subjects observed immediately before this plan edit were:

| Repository | Commit | Tree | Checkout truth |
| --- | --- | --- | --- |
| Hub | `4b93a04a4fde0293df994386e1c86f84a9c8f1cc` | `51bd23dbdc4fb44c3583210c4cfe8f3bda5acf43` | Setup, receipt, onboarding, lock vocabulary, and literal argv controls committed; this documentation edit is excluded |
| Kernel | `365bb5d32ac31791f338a58c9c9d0b94b0b74f18` | `64d15aa7e6c901a2531d88ae9b4e1b19d1f756c5` | Command correlation and bounded verifier transport committed; newly announced disjoint work is excluded |
| BulletGit | `f55173622613e7ce55d9e1366ee6434c7e32158e` | `e4bf780dfd67a53810e03f465ba34f2ac25e3bb4` | Clean at reconciliation |
| Portal | `181cd00cc6f9d20b079bdcecea88eebde70c47c3` | `3738a39a588e3356c45403c985b4c751072ecb1c` | Exact SSE pair handling and real-farmd UNKNOWN reconciliation committed; clean |

There is no family transaction or release receipt: `check release` names the
missing transaction, live, recovery, package, security, and signing evidence.

| Receipt | Status | Exact evidence | Boundary |
| --- | --- | --- | --- |
| Pure signed wire | `COMPLETE` | Hub `c07efb10639d500c3e82ccc282265090ff63a4aa` | DTO/canonical/signature/golden proof; no running issuer or published immutable tag |
| Tool admission | `COMPLETE` | Hub `68d0fb92b52df8d4631ac346428f341f0bb492bc` | Bounded admitted Cargo/Node/npm/Bash component path; not signed installation |
| Bounded JSON-RPC session contract | `COMPLETE` | Hub `b0b9be55199d7d58bc795c5252b27106a5c310b3`; wire 57/57, hostile IPC 11/11 | Pure hello/frame/deadline/cancel/correlation state machines; runtime consumers still use legacy boundaries |
| Executable exact-subject checks | `COMPLETE` | Hub `24d05af9db762a72bd4a54cddbb1807c9800ea64`; local fusion `17aa92885b2fdd1807100ad8b1ab335de8b72e5b` | Fixed bounded commands and unchanged subjects; synthetic/component results cannot promote release |
| Same-origin development | `COMPLETE` | Hub `59675c8`; launcher regression and Hub required pass | Vite development proxy only; no browser command/auth or embedded production proof |
| Checksummed SQLite schema | `COMPLETE` | Kernel `9d0e5c2232342789dc889d25b34c4035059d6e4b`; 260/260 required | Migration/FK/disposable-schema component proof; not full normalized recovery |
| Database-clock lease authority | `COMPLETE` | Kernel `63285a0`; 271/271 required, clock 6/6, farmd lease 4/4 | TTL `1..=15`, DB-owned windows, restart fence, bounded heartbeat; no public command/auth or live final check |
| Atomic lease command/outbox | `COMPLETE` | Kernel `544f43ff50b92ab864cb4bcfc31c2f0d880c36f6`; required 277/277 | Fence, Attempt, lease, graph, event, exact result, and correlated outbox commit once; public wire command/auth remains open |
| Admitted gates and authoritative snapshots | `COMPLETE` | Kernel `cdfd6f2a085faeb8201a52745bec13ce444047db`, `20032074526605e1708aa72789defbd87fa75b58`, `fef4aba1f687d67107e5bafffa9adba8545becc3`; required 293/293 | Provider text cannot name shell; atomic SQLite projections carry source/time/watermark; empty ready is verified `data:null` |
| Provider process admission | `COMPLETE` | Kernel `03baa0ed7bd6746f7e7458cca6f56a60ddfda617`; focused 55/55 plus strict checks | Absolute binary/digest, 0700 HOME, 0400 OAuth copy, positive environment, canary scan, cleanup; dispatch remains blocked without signed authority and egress |
| Authenticated command ingress | `COMPLETE` | Kernel `19d1d47cb03956eb92dfa3f27e409c87d1ab5203`; required 307/307 | Loopback bootstrap/session/origin/CSRF and atomic PENDING command/outbox/event |
| Operation-specific local authority | `COMPLETE` | Kernel `e697f8aa457ed6289e82cf51428f5e9de809436d`; required 311/311 | Exact typed mutation/terminal/preservation decisions; signed request capabilities, daemon receipt validation, and online cleanup settlement remain open |
| Authenticated offline command reconciliation | `COMPLETE` | Kernel `77a0ecd0079d030e944ebf3a7b9077b7d64aabcc`; required 317/317 | Exact-ID worker settles one command/outbox/event to honest UNKNOWN/FAILED; no dispatch, APPLIED, VERIFIED, provider, verifier, or effect path |
| Receipt-bound backup/quarantined restore | `COMPLETE` | Kernel `798f0c814cc4dde1fc510eadc46ce14653380772`; backup 9/9, migrations 14/14, CLI 1/1 | WAL-consistent exact-size/digest copy and verified restore epoch; restored data stays quarantined because authenticity and production admission remain open |
| Four offline provider message subsets | `COMPLETE` | Kernel provider commits `ca376e4`, `c34d578`, `ea89929`, `5badc85`; strict recursive JSON `1bb32bd`; required 330/330 | Pure bounded parsers with public dispatch blocked; RFC 8785 identity, signed runtime isolation, native Cursor proposal extension, and every live receipt remain open |
| BulletGit fail-closed gateway | `COMPLETE` | BulletGit `79bf1e2129fbe50ed85d424fe6e4416407bb17f4`; 82/82 required at consumer head `7df926c` | Production refuses unavailable authority; no positive checker or Jeryu backend |
| Atomic generations and preservation | `COMPLETE` | BulletGit `61bf76dd06753df1ce37715582ab56fbf5d75cff`, `9d527b9e2d8da2fe4ca76c45787851f6d6dab8c2`; required 105/105 | Prior-or-complete-next generation plus sealed exact-state salvage before cleanup; positive online authority and Jeryu remain open |
| Exact mutation reservation | `COMPLETE` | BulletGit `f8121142cd337e243bdc97cdeec9dacea9554b04`; required 107/107 | Durable reservation binds request/envelope/Attempt/fence/workspace generation; production checker and Kernel settlement remain unavailable |
| Projection and real-process browser truth | `COMPLETE` | Portal `cfba6f72f6fd55cc0477182b74b63ade49821d07`; unit 51/51, mocked browser 10/10, real farmd browser 1/1 | Strict snapshots, STALE recovery, server provenance, ready-null; packaging remains open |
| Strict browser command reconciliation | `COMPLETE` | Kernel `35b64847459aefb88ef17c37427d9b7b9754ae97`; Portal `181cd00cc6f9d20b079bdcecea88eebde70c47c3`; Kernel 342/342, Portal 67/67, real farmd 1/1 | Sole exact outbox/submitted/reconciled truth; SSE ID/sequence conflicts stay STALE; authenticated worker proves correlated `PENDING→UNKNOWN`, never green. Vite preview is not packaged/embedded evidence |
| Transaction-safe source setup | `COMPLETE` | Hub `5148a52a122da46e749be3e2169f81bd6d4b8116`; Hub required 34/34 plus integrations | Fallible validation precedes no-replace publication and final durable manifest; signed schema-3 inputs and prebuilt installer remain open |
| Pinned assurance controls | `COMPLETE` | Hub `0440d446190c20c2be24620fa1d91d1b39fa3073`; required 34/34 | Pinned secret/dependency/license/workflow scans and release blocker report; passing components do not promote a release |
| Signed five-target bundle verification | `COMPLETE` | Hub `352f963c75ce1939898a26d94d39be13de321f86`; required 39/39 plus 3/3 bundle integration | Linux read-only exact-byte/signature verification; no package build, semantics, extraction, install, signer provisioning, or intermediate-directory race proof |
| Safe signed archive extraction | `COMPLETE` | Hub `ba0905604b6c743306f245837a0621781478e4cf`; hostile archive/publication proof | Exact signed bytes materialize descriptor-relatively at one absent destination; no activation, rollback, semantic package admission, or installer receipt |
| Descriptor-relative setup publication | `COMPLETE` | Hub `94b6549aa24ee4bc2110627c994b4d476042864a`; focused 13/13, required and fast | Retained root/staging identity, private staging, no-replace publication, bounded no-follow cleanup; schema-3/prebuilt install and same-UID path-based Git containment remain open |
| Signed release-receipt contract | `COMPLETE` | Hub `143f8b963586ad162a97fcd0d5ca7f18fa034796`; focused 5/5 and Rust 1.95 strict proof | Canonical TOML, exact policy digest, signer/namespace/interval, and sealed-input verification only; external policy, trusted time/revocation/custody, semantic adjudication, registry/replay, and real receipts remain open |
| BulletGit recovery and wire-shaped subjects | `COMPLETE` | BulletGit `274fd6d6655ce88979bcaa80eca758c44886963e`, `f55173622613e7ce55d9e1366ee6434c7e32158e`; required 121/121 | Freeze/recovery, full IDs, tagged OIDs, strict manifests; provenance-bound Candidate identity, shared immutable wire tag, and production Jeryu remain open |
| Generated Kernel-to-Portal runtime contract | `COMPLETE` | Kernel `043b8fddd59cef8a67ad98f45d9c190fc11bd94f`; Portal `c294ec7bddb7dd217eb4bd360b6c810c0391a31d`; Hub `601cb82c9a5f66cc627677244e230f289e9acc65` | Dependency-closed JSON Schema and AJV for consumed command/mission/readiness DTOs; raw events and absent Candidate/Evidence/Effect DTOs remain predecessors |
| Shared admitted gate and clean verifier | `COMPLETE` | Kernel `528348fae6038def88ecd6d6b4f4f54e78747cd4`; required 339/339, contract plus 3 simulators | Caller/model shell and timeout authority removed; exact gate/argv/timeout/base/head/tree Evidence. One fixture gate exists; executable digest, framing/source-path admission, and multi-gate aggregation remain open |
| Bounded verifier transport | `COMPLETE` | Kernel `365bb5d32ac31791f338a58c9c9d0b94b0b74f18`; required/contract 345/345 plus 3 simulators | Strict one-shot 64 KiB request and single bounded Evidence frame; contaminated/unknown/lossy/overflow output refuses and infinite writer is killed/reaped. JSON-RPC, signed source admission, and process-tree supervision remain open |

Hub Jankurai is 58/raw 58 with 10 caps and 40 findings (25 high, 15 medium); all
25 high findings are classified `hard`. It fails the V1 >=90, zero-cap/zero-hard gate.
Hosted CI also lacks a portable checksum-pinned Jankurai artifact; a machine-local
binary cannot be converted into a skip-green workflow.

## Dependency graph

```text
V1-S0 coordination/provenance
  -> V1-S1 wire, schemas, IPC
       -> V1-S2 Kernel durable authority
            -> V1-S3 BulletGit durable subjects + Jeryu service
                 -> V1-S4 runner/verifier/effect transaction
                      -> V1-S5 authenticated API + truthful Portal
                           -> V1-S6 cognitive plane + four providers
                                -> V1-S8 live promotion/release
       -> V1-S7 installer/CI/docs/package mechanics ----------------^
```

`V1-S7` may build after `V1-S1`, but cannot exit before `V1-S2..S6`. No slice
advances while it or a predecessor has a known safety counterexample.

## V1-S0 — coordination and provenance

Status: `COMPLETE` locally; release receipts remain continuous obligations.

Remaining work:

1. Require exact, non-overlapping `coord claim|heartbeat|handoff|status` records
   for every edit and short-lived heartbeat at least every five minutes and on
   every proof, blocker, commit, or handoff.
2. The orchestrator stages only completed claimed paths and records claim IDs
   against exact commit path sets. Never stage an entire dirty checkout.
3. Preserve mixed provenance through corrective commits. Append coordination
   history; never reset, rewrite, or treat a lane label as proof.

Exit gates:

```bash
bullet-family coord status --json
bash scripts/ci-local.sh required
```

Mandatory negatives: concurrent overlapping claims have one winner; expired,
corrupt, truncated, replayed, path-traversing, or mismatched-repository records
fail closed; a commit containing an unclaimed path cannot receive a receipt.

## V1-S1 — frozen cross-repository contracts and IPC

Status: `LOCAL-BLOCKED`. Wire/IPC component machines exist, but no immutable
Jeryu publication exists and consumers duplicate or use legacy semantics.

Required work:

1. Publish `bullet-wire` from an immutable signed Jeryu tag. Generate ignored
   `.fusion` development overrides only; committed sibling `path` dependencies
   remain forbidden.
2. Make Kernel and BulletGit consume the authoritative validated Candidate,
   digest, Evidence, effect, proposal, checkpoint, and proof semantics; delete
   duplicate local meanings rather than translating around them.
3. Replace model-owned `tests_to_run` and shell command fields with admitted
   `gate_ids`. A proposal binds producing Attempt, base checkpoint ID+digest,
   per-operation preimages, scope, and bounded content.
4. Generate OpenAPI, JSON Schema, Rust consumers, TypeScript client/types, and
   AJV validators into a temporary directory and diff tracked artifacts without
   mutating them.
5. Land negotiated JSON-RPC 2.0 hello/version/frame/deadline/cancel/request-ID
   conformance before any trust-boundary process is treated as production.

Exit gates:

```bash
just contract
just family-contract
```

Mandatory negatives: mutate every ID/digest/authority/Candidate bound field;
unknown fields, duplicate keys, non-canonical JSON, algorithm-confused Git OIDs,
oversized frames, missing hello, duplicate request IDs, late events, and legacy
schemas all refuse. Golden JSON/hash bytes match in every repository and
generation leaves zero tracked drift.

## V1-S2 — Kernel durable authority, commands, and recovery

Status: `LOCAL-BLOCKED`. Checksummed migrations/FKs, database lease time,
atomic lease-command/event/outbox, admitted gates, and authoritative snapshots
are complete components. Authenticated public command admission and typed local
operation decisions, an UNKNOWN/FAILED-only worker, and receipt-bound backup
with quarantined restore are complete; normalized truth, signed capabilities,
restore admission, fault-complete recovery, and CAS remain.

Required work:

1. Normalize immutable identities/transitions, graph revisions, Candidates,
   Evidence, commands, leases, events, projections, effects, and outbox rows.
   Remove JSON-blob truth and `INSERT OR REPLACE` authority.
2. Preserve lease/fence allocation in one SQLite `BEGIN IMMEDIATE` transaction
   using database time. Fence counters never rewind across replay, expiry,
   supersession, restart, backup, or restore.
3. Preserve authenticated admission and the exact-ID offline worker. Extend it
   only through signed dispatch; persist each result, transition, event, and
   outbox settlement atomically, returning `UNKNOWN` when truth is unavailable.
4. Mint short-lived PASETO v4.public capabilities from the durable active lease
   only. Bind audience, operation/request digest, Mission/repository/graph/
   package/Variant/Attempt/fence/runner/workspace/scope/context/configuration/
   policy/routing, authority epoch, freeze generation, expiry, and nonce.
   Lease and token maximums are separately enforced at 15 seconds.
5. Put WAL and filesystem CAS under platform data directories. Preserve the
   exact backup/restore receipt and quarantine, then add receipt authenticity,
   restore admission, fault boundaries, retention, orphan-safe GC, and anchors.
6. Preserve operation-specific decisions and delete every remaining hard-coded
   SHA/PASS/demo-success branch.

Exit gates: Kernel `bash scripts/ci-local.sh required`, locked Rust 1.95 strict
Clippy, generated drift, backup/restore/fault suites, and Hub `just model-check`
with exactly the two pinned models.

Mandatory negatives: simultaneous acquire, replay, TTL 0/16, exact expiry,
heartbeat-versus-expiry, supersession, freeze, restore, every capability-field
mutation, wrong operation/request, nonce reuse, malformed/corrupt persisted
values, authority outage/timeout/zero-row, and crash at every SQLite/WAL/CAS/
event/outbox boundary yield zero unauthorized mutation and exactly old or
complete-new durable state.

## V1-S3 — BulletGit durable subjects and Jeryu capability service

Status: `LOCAL-BLOCKED`. Immutable CAS/journal/checkpoints, prior-or-complete-next
generations, and preservation-bound cleanup are complete. Complete shared-wire
manifests and online authority remain. `jeryu-gitd` promotion is
`EXTERNAL-BLOCKED` on a reviewed capability tag and operator authentication.

Required work, in order:

1. Preserve the committed CAS-first journal and exact checkpoint identity.
   Reopen must continue to reject a missing or altered referenced object.
2. Preserve generation-atomic publication and sealed-cleanup negative coverage.
3. Consume the immutable `bullet-wire` proposal and enforce base checkpoint,
   absent/digest preimages, duplicate/conflict/path/scope/content constraints.
4. Harden an absolute verified Git binary, hostile config/attributes/filters/
   hooks/alternates, all sequencer states, exact `-z` status parsing, no-follow
   writes, validated IDs, and safe cleanup targets.
5. Complete Checkpoint/Candidate/Integration proof manifests; retain the sealed
   preservation receipt binding Attempt/fence/nonce/tree/dirty/journal/CAS and
   an external destination.
6. Add Kernel online final check and durable settlement immediately around each
   mutation. Keep the current local Git backend simulator-only. Production calls
   versioned `jeryu-gitd` from a separately reviewed immutable Jeryu tag.

Exit gates: BulletGit required/contract, hostile-repository and deterministic
fault suites, strict toolchain/audit checks, then the exact-head family contract.

Mandatory negatives: duplicate/case/NFC-colliding paths, absolute/dot/backslash/
`.git` paths, ADS/trailing-dot-space, stale/missing preimages, symlink/reparse
escape, oversized content, non-UTF-8/newline Git status, hostile filters or
alternates, incomplete sequencers, partial writes, crash at allocate/write/
file-sync/publish/dir-sync/generation-switch, proof mutation/rebase, forged or
mismatched preservation, unsafe cleanup, and authority outage all preserve the
prior authoritative generation or return typed `UNKNOWN`; none yields success.

## V1-S4 — runner, verifier, effects, and offline transaction

Status: `LOCAL-BLOCKED` on `V1-S2/S3`; bounded transport exists, but farmd serves none of Runner's
acquire/heartbeat/release/attempt-advance routes, and Runner plus `/v1/demo/run` remain synthetic.

Required work:

1. Runner acquires authority, starts a private read-only provider generation,
   validates a proposal, applies only through BulletGit, executes admitted gates,
   allows at most two repair turns, checkpoints, and prepares an exact Candidate.
2. Failed or zero-row heartbeat freezes mutation, terminates the entire provider
   tree, preserves the workspace, and permits a successor fence to resume only
   from the exact checkpoint.
3. Verifier reconstructs the Candidate independently. Writer state or
   writer-produced proof cannot satisfy independent gates; oracle-modifying
   diffs receive distinct review and holdout treatment.
4. Effect broker records intent before dispatch, authorizes, dispatches, records
   ambiguous response as `UNKNOWN`, reads authoritative remote state, and adopts
   only the exact desired subject without a second write.
5. `just demo` stays credential-free and deterministic but uses real child
   boundaries and a protected local forge simulator, ending in one signed
   `TRANSACTION_PROOF`.

Exit gate: `just demo` plus Kernel/BulletGit required and an exact-head family
contract proves materialization replay, fences `1→2`, stale rejection, process
death/salvage, isolated clones, exact Candidate, independent Evidence, ambiguous
effect recovery, protected integration, preservation-before-cleanup, and
truthful projection.

Mandatory negatives: provider crash/cancel/timeout/malformed/duplicate/delayed
events, missed heartbeat, surviving grandchild, stale fence, zero admitted
tests, timeout/flaky/unsupported/infra/UNKNOWN verification, writer-modified
oracle, lost effect response, conflicting remote OID, reconciliation restart,
and cleanup-before-preservation never become PASS or cause a duplicate effect.

## V1-S5 — authenticated API, SSE, and projection truth

Status: `LOCAL-BLOCKED`. Atomic snapshots, strict Portal validation, exact-pair
SSE recovery, authenticated commands, and real farmd `PENDING→UNKNOWN` are
components. There is no APPLIED/VERIFIED dispatch or embedded/package-served Portal.

Required work:

1. Snapshot responses carry `{data,as_of_sequence,observed_at,source}` from one
   atomic ledger read. SSE emits only generated default `EventEnvelope` values;
   `after` and `Last-Event-ID` are exclusive, conflicting cursors fail, replay is
   bounded/ordered, and live tail includes keepalives.
2. Preserve the one-time bootstrap, HttpOnly/SameSite session, CSRF,
   loopback/origin, and no-wildcard-CORS boundary.
3. Preserve generated client/AJV validation. Correlate each displayed
   receipt to its command ID; timeout is `UNKNOWN`, and an old green receipt
   cannot satisfy a new failed command.
4. Implement Mission Graph, Attempts/Sessions, Router/Quota, Context/Fusion,
   Workspace/Candidate, Verification/Effects, and Audit views as projections.
5. Embed the built Portal in the Rust distribution and run Playwright against a
   real packaged farmd. Vite/mock lanes remain focused component tests only.

Exit gates: Portal required/build, generated drift, and packaged farmd/browser
E2E with authenticated command reconciliation.

Mandatory negatives: SSE `1,2,4` plus failed snapshot stays STALE at cursor 2;
watermark `>=4` clears it. Cover first-event gaps, reconnect, retention gap,
malformed/oversized/CRLF streams, body timeout, conflicting cursors, forged or
missing session/CSRF/origin, command timeout, mismatched command receipt, older
green result, and farmd restart. None may render false green.

## V1-S6 — cognitive plane and four provider adapters

Status: `LOCAL-BLOCKED`. Fail-closed offline Claude, Codex, Cursor, and
Antigravity message subsets plus strict recursive JSON are committed. Provider
execution remains blocked without signed authority/isolation, native
conformance, egress, supervision, and live receipts.

Required work:

1. Persist typed Cognitive Tasks, role/capability/profile snapshots, context
   capsules, behavior rules, budget/quota reservations, routing provenance,
   struggle/escalation, fusion, dissent, selection, and negative knowledge.
2. Treat roles as constrained policy profiles: planner decomposes, researchers
   cite observations, implementers submit proposals, critics create
   counterexamples, fusion preserves agreement/dissent, verifier supplies
   Evidence, and broker performs effects. No role receives ambient authority.
3. Route hard constraints before optimization: risk/authority envelope,
   capability maturity, exact profile, quota, context budget, independence, and
   verifier capacity. UNKNOWN paid capacity blocks ordinary dispatch; a bounded
   read-only probe requires explicit policy.
4. Preserve the bounded offline Claude stream-JSON, Codex App Server JSONL,
   Cursor ACP, and Antigravity headless subsets while closing decoder/native-
   extension gaps. Runtime probing, including Antigravity `-p=` ordering and
   schema support, determines capability.
5. Execute absolute verified binaries with allowlisted environments, ephemeral
   HOME, minimum provider OAuth, provider-only egress, and no SCM/cloud/SSH/host
   secrets. Persist exact binary/model/config/profile receipts.

Exit gate: all four adapters pass one common conformance matrix and routing/
fusion decisions replay from persisted inputs.

Mandatory negatives: missing/unknown quota, expired reservation, profile or
model substitution, capability downgrade, malicious output, malformed/
duplicate/delayed events, cancel/timeout/crash, resume/fork mismatch, process
escape, direct filesystem/Git mutation, network escape, and canary secrets in
environment/output/patch/log all fail closed. A losing Variant remains immutable
and cannot be rewritten into the selected Candidate.

## V1-S7 — installer, checks, CI, packaging, and documentation

Status: `LOCAL-BLOCKED`. Coordination/checks/fusion, strict schema-3 verification,
descriptor-relative setup, signed bundle verification, and safe extraction exist.
Schema 2 refuses; no deterministic Portal bundle, embedded farmd, package builder,
activation/rollback installer, authenticated prebuilt installer, signer, or release exists.

Required command surface:

```text
bullet-family doctor --json
bullet-family setup --root <path> --source jeryu [--offline]
bullet-family lock generate|verify
bullet-family checkout verify
bullet-family fuse --source local|lock
bullet-family check fast|required|release
bullet-family coord claim|heartbeat|handoff|status
```

Installer closure, in dependency order:

1. Decide and implement public signed Jeryu objects or a short-lived,
   destination-bound credential channel that never enters URL/argv/env/logs.
2. Define an exact Portal bundle manifest/root, stage it into an opt-in Kernel
   build through `OUT_DIR`, then publish signed `bullet-family` and embedded
   `bullet-farmd`; never use a sibling include or tracked `dist` as authority.
3. Verify hub tag, non-circular manifest, schema-3 lock, authenticated member
   URL/slug, signed tags, exact commit/tree, lockfiles, artifacts, and canonical
   Git/Bash/Cargo/Node/npm binary digests and admitted versions before mutation.
4. Preserve the committed rule that dependency/generated/exact-family checks
   finish before no-replace member publication and the final manifest marker.
5. Create ordinary clones at exact OIDs; reject dirty/symlinked/conflicting
   paths; use `cargo --locked` and `npm ci`; generate/diff in temporary trees.
   Preserve validated Rust fusion and its byte-idempotent ignored output.
6. Run hub-only setup twice in a fresh HOME and crash-inject every clone,
   checkout, fsync, publish, dependency, and drift boundary. End at exact clean
   member OIDs with no worktree and either prior or complete setup state.

CI closure:

| Profile | Required executable evidence |
| --- | --- |
| `fast` | warm under 60s; fmt, strict Clippy/unit, TypeScript type/unit, production Portal build, generated drift, affected-path routing |
| `required` | locked build/test/doc-test, contracts, migrations, real packaged browser E2E, transaction demo, pinned secret/dependency/license/workflow scans, family lock, Jankurai ratchet; no skip-green |
| `release` | five archives, installer smoke, Rust 1.95 and pinned 1.97.1, SBOM/checksum/signature/provenance, backup/restore/faults, Jeryu/GitHub integration, four providers, Jankurai >=90 with zero caps/hard findings |

Jeryu CI must run the same local commands through workflow IR. GitHub workflows
are a portable mirror, not release evidence until a public mirror exists.
Nightly exists only for meaningful fuzz/soak/live work. Missing optional live
registration is neutral; missing a required tool or receipt fails.

Documentation closure follows executable ownership:

- `docs/release.md`: concise stable release contract and blockers;
- this file: stable slice graph, exact subjects, receipts, and next gates;
- generated check report: current machine status, never manually copied scores;
- README: separate contributor use from future signed hub-only install;
- ADRs: accepted choices changed only with implementing evidence;
- runbooks: tested setup recovery, upgrade/rollback/uninstall, signer rotation,
  schema removal, backup/restore, SAFE_STOPPED, effect reconciliation, and
  platform refusal after the typed commands exist;
- `docs/spec/` and `TEAM.md`: immutable hashed historical provenance.

Exit gates: `bullet-family check fast|required|release`, signed installer smoke
for all five archives, two-run hub-only setup, generated drift in a temporary
directory, docs/link/source/license/workflow scans, and zero tracked changes.

Mandatory negatives: schema-2/future/corrupt lock, bad signature/checksum/tag/
tree/lockfile/tool digest, PATH shim, credential/canary leak, offline cache miss,
dirty/symlinked/non-empty destination, partial setup/crash, rerun conflict,
unsupported platform mutation, package mismatch, circular manifest, missing
scanner, zero routed tests, optional-neutral promoted to required PASS, broken
links, machine-local paths, or generated drift all block with repair guidance.

## V1-S8 — credentialed live proof and release promotion

Status: `EXTERNAL-BLOCKED`; it cannot begin before `V1-S0..S7` are green from
tagged bytes.

Required order:

1. An authorized Jeryu lane reviews, tags, and publishes the capability service.
   The operator restores authentication; run read-only probes before protected
   integration and UNKNOWN/read-back reconciliation. Never alter the running
   forge to work around missing capability.
2. Configure a GitHub App test repository with branch protection. Separate
   delivery credentials from evidence-attestation credentials; green checks are
   reconstructed from the exact signed proof bundle and Candidate SHA.
3. Register all four provider profiles and run the same exact-subject transaction
   and canary-isolation conformance suite.
4. Build/smoke the five signed archives, publish SBOM/checksums/signatures/
   provenance, verify backup/restore and containment, then sign the final
   non-circular release manifest.

Exit gate: `bullet-family check release` passes from exact signed tags with
current Jeryu, GitHub, Claude, Codex, Cursor, Antigravity, platform, security,
recovery, package, and signer receipts.

Mandatory negatives: lost remote response becomes `UNKNOWN`; read-back adopts
the original exact OID without a second write. Wrong fence/OID/check/proof root,
expired or missing credential, protected-ref refusal, provider/profile drift,
platform containment absence, signature/provenance mismatch, revoked signer,
or unavailable required service blocks promotion. Optional unregistered lanes
may be neutral only when the requested release profile does not require them.

## Immediate closure queue

1. Extend offline command reconciliation into signed dispatch, effect read-back,
   and independent verification without synthetic APPLIED or VERIFIED.
2. Complete `V1-S1` immutable publication/runtime-consumer convergence and
   `V1-S2` normalized truth, capability, CAS, backup/restore, and fault receipts.
3. Complete shared-wire manifests and positive online BulletGit authority, then
   consume the reviewed tagged `jeryu-gitd` capability.
4. Produce the `V1-S4` credential-free five-plane transaction receipt; replace,
   rather than rename, synthetic success.
5. Embed Portal; persist cognitive routing/fusion; close all four offline
   provider gaps, then obtain separately admitted live conformance receipts.
6. Publish signed schema-3 install subjects and a prebuilt installer, then build
   five packages with installer smoke, SBOM, checksums, signatures, provenance,
   containment, and Jankurai/security evidence.
7. Run `V1-S8` only with operator-provided Jeryu, GitHub App, provider, platform,
   and signing authority.

## Terminal definition of done

V1 is done only when every `V1-S0..S8` gate has a current independently verifiable receipt from the same signed subjects; setup/demo/fast/required/release and two pinned models pass;
Jankurai reaches 90 with zero caps/hard findings; every provider, forge, package, signature, recovery, fault, installer, and containment receipt is current; every checkout is clean;
Portal has no synthetic authority; and no safety counterexample remains. Until then: **pre-release, blocked**.
