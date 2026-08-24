# Bullet Farm Safety-Complete V1 closure plan

Status: **ACTIVE — pre-release; release authority is blocked**
Owner: Bullet Farm maintainers
Last reconciled: 2026-08-24
Scope: four repositories, local-first, single-user V1

This document is the dependency graph, current evidence register, and closure
checklist for V1. It is not runtime authority. Generated contracts, exact Git
subjects, the Kernel ledger, independently reconstructed Evidence, observed
effect receipts, and signed release receipts are the only things that can make
a gate green. HTTP success, a passing component test, model output, or prose
never promotes itself to transaction, live, or release proof.

## Status vocabulary

| Status | Meaning |
| --- | --- |
| `COMPLETE` | Exact committed subject and mapped receipt exist for this bounded claim |
| `IN PROGRESS` | Claimed work or focused evidence exists, but no completed commit receipt exists |
| `LOCAL-BLOCKED` | Implementable offline work remains or a predecessor safety gate is not green |
| `EXTERNAL-BLOCKED` | Promotion needs operator-controlled service, credential, signer, or platform evidence |

`BLOCKED`, `UNKNOWN`, zero tests, timeout, skipped, flaky, unsupported, or
infrastructure error never equals `PASS`. A newer subject invalidates an older
receipt until the mapped gate is replayed against the newer subject.

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
  learning are post-V1; versioned ports may exist, speculative implementations
  do not.
- Exactly **two** protocols are model-checked: lease/fence/reclaim and
  command/effect ambiguity under timeout. No third formal model is a V1 gate.
- GA requires conformant **Claude, Codex, Cursor, and Antigravity** adapters.
  `TEAM.md` critique C8's any-two recommendation is superseded for this V1.
- Release has exactly **five** archives: Linux x86_64, Linux aarch64, macOS
  x86_64, macOS arm64, and Windows x64. Linux is the production runner; other
  packages fail closed on mutation until equivalent native containment passes.
- Pre-1.0 schemas are disposable. Unknown/legacy databases fail with typed
  `UNSUPPORTED_SCHEMA` plus explicit export/removal guidance.

## Reconciled repository subjects and receipts

The committed subjects observed immediately before this plan edit were:

| Repository | Commit | Tree | Checkout truth |
| --- | --- | --- | --- |
| Hub | `24d05af9db762a72bd4a54cddbb1807c9800ea64` | `4b7207a14a58a2e23862e7164b084b0d1727df2a` | Clean before this claimed documentation edit |
| Kernel | `544f43ff50b92ab864cb4bcfc31c2f0d880c36f6` | `e47e7f5698a0f5831e3b4ba50c496f5758a06677` | Clean before the active admitted-gate claim |
| BulletGit | `dff6dd143fd91d539c0c98b0fb2cd0ad6dd643ba` | `1bd7e7f7ba6f5fa89abea9871cc91d339a829487` | Clean before the active generation-publication claim |
| Portal | `3ef6e7f4458a3ad2982f2231efae410f555d5eda` | `2fe6fe842c50f194bcd75c738279771b62b23b1c` | Clean |

The latest exact-subject family component receipt is **R4**. On the four heads
above, `check fast --json` passed seven bounded gates within 60 seconds; required
ran the family contract and `SYNTHETIC` demo, then exited 3/`BLOCKED` on six
missing receipt classes. It is not a transaction or release receipt.

| Receipt | Status | Exact evidence | Boundary |
| --- | --- | --- | --- |
| Pure signed wire | `COMPLETE` | Hub `c07efb10639d500c3e82ccc282265090ff63a4aa` | DTO/canonical/signature/golden proof; no running issuer or published immutable tag |
| Tool admission | `COMPLETE` | Hub `68d0fb92b52df8d4631ac346428f341f0bb492bc` | Bounded admitted Cargo/Node/npm/Bash component path; not signed installation |
| Bounded JSON-RPC session contract | `COMPLETE` | Hub `b0b9be55199d7d58bc795c5252b27106a5c310b3`; wire 57/57, hostile IPC 11/11 | Pure hello/frame/deadline/cancel/correlation state machines; runtime consumers still use legacy boundaries |
| Executable exact-subject checks | `COMPLETE` | Hub `24d05af9db762a72bd4a54cddbb1807c9800ea64`; R4 fast PASS and required BLOCKED on six named receipts | Fixed bounded commands and unchanged clean subjects; synthetic/component results cannot promote release |
| Same-origin development | `COMPLETE` | Hub `59675c8`; launcher regression and Hub required pass | Vite development proxy only; no browser command/auth or embedded production proof |
| Checksummed SQLite schema | `COMPLETE` | Kernel `9d0e5c2232342789dc889d25b34c4035059d6e4b`; 260/260 required | Migration/FK/disposable-schema component proof; not full normalized recovery |
| Database-clock lease authority | `COMPLETE` | Kernel `63285a0`; 271/271 required, clock 6/6, farmd lease 4/4 | TTL `1..=15`, DB-owned windows, restart fence, bounded heartbeat; no public command/auth or live final check |
| Atomic lease command/outbox | `COMPLETE` | Kernel `544f43ff50b92ab864cb4bcfc31c2f0d880c36f6`; required 277/277 | Fence, Attempt, lease, graph, event, exact result, and correlated outbox commit once; public wire command/auth remains open |
| BulletGit fail-closed gateway | `COMPLETE` | BulletGit `79bf1e2129fbe50ed85d424fe6e4416407bb17f4`; 82/82 required at consumer head `7df926c` | Production refuses unavailable authority; no positive checker or Jeryu backend |
| Immutable CAS/journal/checkpoint | `COMPLETE` | BulletGit `dff6dd143fd91d539c0c98b0fb2cd0ad6dd643ba`; required 100/100 | CAS-first journal objects and exact checkpoint tree/root; active generation switch, daemon authority, and Jeryu remain open |
| Cursor/projection truth | `COMPLETE` | Portal `5aa21708c808e6afd39537e9f5148679a0ce674b`; current consumer/docs head `3ef6e7f` passes 43/43 plus build | STALE/watermark and honest component prose; no command reconciliation or packaged E2E |

Last recorded Jankurai component scores are Hub 54, Kernel 58, BulletGit 58,
and Portal 60. They meet local ratchet floors only. They are not a current
release receipt and do not meet V1's at-least-90, zero-cap, zero-hard gate.

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

`V1-S7` may build offline in parallel after `V1-S1`, but its release exit cannot
pass before `V1-S2..S6`. A slice does not advance while a known safety
counterexample remains in that slice or a predecessor.

## V1-S0 — coordination and provenance

Status: `COMPLETE` as a local coordination substrate; release receipts remain
continuous obligations.

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

Status: `LOCAL-BLOCKED`. Hub wire and JSON-RPC session machines are component
evidence; no immutable Jeryu publication exists, and consumers still duplicate
semantics or use legacy framing.

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

Status: `LOCAL-BLOCKED`. Checksummed migrations/FKs, database lease time, and
the atomic lease-command/event/outbox transaction are complete components;
public wire commands, normalized truth, capabilities, recovery, and CAS remain.

Required work:

1. Normalize immutable identities/transitions, graph revisions, Candidates,
   Evidence, commands, leases, events, projections, effects, and outbox rows.
   Remove JSON-blob truth and `INSERT OR REPLACE` authority.
2. Preserve lease/fence allocation in one SQLite `BEGIN IMMEDIATE` transaction
   using database time. Fence counters never rewind across replay, expiry,
   supersession, restart, backup, or restore.
3. Add authenticated command admission/reconciliation. Persist command result,
   state transition, event, and outbox intent atomically; reads reconstruct the
   exact stored receipt or return `UNKNOWN`.
4. Mint short-lived PASETO v4.public capabilities from the durable active lease
   only. Bind audience, operation/request digest, Mission/repository/graph/
   package/Variant/Attempt/fence/runner/workspace/scope/context/configuration/
   policy/routing, authority epoch, freeze generation, expiry, and nonce.
   Lease and token maximums are separately enforced at 15 seconds.
5. Put WAL and filesystem CAS under platform data directories; add fsync/rename
   boundaries, restore epochs, verified backup/restore, retention classes,
   orphan-safe GC, and append-only audit anchors.
6. Delete every hard-coded SHA, PASS, verified, or demo-success branch.

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

Status: `LOCAL-BLOCKED`. Immutable CAS, journal bindings, and exact checkpoint
identity are complete; generations, manifests, preservation, and the wire
consumer remain. Authority and `jeryu-gitd` promotion are `EXTERNAL-BLOCKED` on
a reviewed capability tag and operator authentication.

Required work, in order:

1. Preserve the committed CAS-first journal and exact checkpoint identity.
   Reopen must continue to reject a missing or altered referenced object.
2. Stage changes in a new workspace
   generation, fsync tree/journal/CAS, and atomically publish the active pointer.
3. Consume the immutable `bullet-wire` proposal and enforce base checkpoint,
   absent/digest preimages, duplicate/conflict/path/scope/content constraints.
4. Harden an absolute verified Git binary, hostile config/attributes/filters/
   hooks/alternates, all sequencer states, exact `-z` status parsing, no-follow
   writes, validated IDs, and safe cleanup targets.
5. Emit complete Checkpoint/Candidate/Preservation manifests, separate Candidate
   and Integration proof roots, and daemon-issued unforgeable preservation
   receipts bound to Attempt/fence/nonce/tree/dirty/untracked/journal/CAS and an
   external destination.
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

Status: `LOCAL-BLOCKED` on `V1-S2` and `V1-S3`. Current Runner safety simulation
and `/v1/demo/run` are component/synthetic evidence only.

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

Status: `LOCAL-BLOCKED`. SSE gap recovery and same-origin development are
complete components; the browser still calls direct `/v1/demo/run` and has no
command ledger, session, CSRF, or packaged-production proof.

Required work:

1. Snapshot responses carry `{data,as_of_sequence,observed_at,source}` from one
   atomic ledger read. SSE emits only generated default `EventEnvelope` values;
   `after` and `Last-Event-ID` are exclusive, conflicting cursors fail, replay is
   bounded/ordered, and live tail includes keepalives.
2. Add one-time CLI bootstrap exchange, HttpOnly/SameSite session, CSRF tokens,
   loopback/origin checks, and no wildcard CORS.
3. Generate the TypeScript client and AJV validators. Correlate each displayed
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

Status: `LOCAL-BLOCKED` for durable cognition/conformance harnesses and
`EXTERNAL-BLOCKED` for registered live provider receipts.

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
4. Certify in order: Claude bidirectional stream JSON; Codex App Server JSONL;
   Cursor ACP; Antigravity headless structured mode. Keep ADR 0001's frozen
   targets and runtime observations current; conformance maturity advances only
   with implementation receipts. Runtime probing, including Antigravity `-p=`
   ordering/schema support, determines observed capability.
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

Status: `LOCAL-BLOCKED`. Rust coordination, exact-subject checks, strict
schema-3 verification, bounded ordinary-clone setup, and a signed two-run fixture
exist. Schema 2 correctly returns `UNSUPPORTED_SCHEMA`; no authenticated binary
or release exists. Dependency/drift work still follows publication, and
multi-member crash recovery lacks a prior-or-complete-next receipt.

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
2. Publish a signed prebuilt `bullet-family`. Keep `scripts/setup.sh` a source
   bootstrap, never an installation trust root.
3. Verify hub tag, non-circular manifest, schema-3 lock, authenticated member
   URL/slug, signed tags, exact commit/tree, lockfiles, artifacts, and canonical
   Git/Bash/Cargo/Node/npm binary digests and admitted versions before mutation.
4. Separate prebuilt install from source dependency preparation; use a verified
   family-owned cache instead of deleting a successful Cargo cache. Stage all
   fallible work before no-replace publication or persist a typed setup result
   with deterministic retry/cleanup.
5. Create ordinary clones at exact OIDs; reject dirty/symlinked/conflicting
   paths; use `cargo --locked` and `npm ci`; generate/diff in temporary trees.
   Replace shell-owned fuse behavior with validated Rust atomic publication.
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

1. Complete the active BulletGit generation-atomic publication, Kernel
   `gate_ids` admission, and Portal gap/watermark truth slices; replay R4 on the
   resulting exact committed subjects.
2. Complete `V1-S1` immutable publication and runtime-consumer convergence,
   then `V1-S2` public
   command/auth/CAS/backup-recovery before enabling positive BulletGit authority.
3. Complete `V1-S3` generation/proposal/proof/preservation
   chain, then consume the reviewed tagged `jeryu-gitd` capability.
4. Produce the `V1-S4` credential-free five-plane transaction receipt; replace,
   rather than rename, synthetic success.
5. Land `V1-S5` authenticated command/SSE/snapshot semantics and packaged
   Portal E2E, then `V1-S6` cognitive routing/fusion and four-provider harnesses.
6. In parallel where dependencies permit, move dependency/drift work before
   installer publication or persist typed retry truth, prove crash recovery,
   replace shell-owned fuse, then build five packages and supply-chain evidence.
7. Run `V1-S8` only with operator-provided Jeryu, GitHub App, provider, platform,
   and signing authority.

## Terminal definition of done

V1 is done only when every required `V1-S0..S8` exit gate has a current,
independently verifiable receipt from the same signed subjects; `just setup`,
`just demo`, `just fast`, family required, and release checks pass without
skips; exactly two formal models pass their pinned counts; Jankurai is at least
90 with zero caps/hard findings in every repository; all four provider and both
forge receipts are current; all five packages and signatures verify; backup,
restore, fault, installer, and containment receipts are current; every checkout
is clean; the Portal displays no synthetic authority; and no known safety
counterexample remains. Until all of that is true, the honest status is
**pre-release, blocked**.
