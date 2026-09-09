# xbabe2 development closeout

Status: **Implementation detail; no production or release admission**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-09-09

This is subordinate to the [full-product plan](full-product-dogfood-plan.md),
[G1–G18 register](product-gaps.md) and existing typed profile inventory.
The [health checkpoint](health-checkpoint-20260909.md) records accepted subjects
and retained failures. Those authorities and the existing DF/WP identifiers remain
unchanged; this document supplies the detailed execution and verification order.

This work order refines the existing DF packets for everyday development on xbabe2.
It does not replace the gap register, change a profile definition, or treat a
native CLI demonstration as the production transaction. Recorder, browser capture,
renderer, verifier, fixtures and local commands are product source: they must be
tracked in the supporting members and included in `neverhuman/bulletfarm`.

## Current evidence and immediate corrections

| Subject | Established evidence | Remaining qualification |
| --- | --- | --- |
| Hub `258c8467`, tree `2cfe419a` | Complete required lane: 819 distinct passing tests, all five mapped lanes, pinned models and independent review. GNU time 22:41.98; maximum child RSS 1,792,340 KiB; new private target about 4.17 GB logical. | Later changes need their own mapped proof; this is local component evidence. |
| Kernel `02bf7c5`, tree `6fd94de4` | Complete standalone required pass: 1,075 standalone plus 34 contract identities. | New provider edits are uncommitted and require independent review, inventory regeneration and complete proof. |
| Hub publication `eb6c9cc` / `912dddf` | Source timeout budgets and GitHub destination integrated after independent review, 36 publication tests and 50 actual wrapper cases. | New exact-source aggregate, complete mapped proof and hosted activation remain required; 52 member jobs and five final refusals remain unavailable. |
| Four-member family retry | BulletGit, Kernel standalone/family and Portal standalone checks completed. The run then correctly refused changed Kernel source after stage 5. | No complete family PASS. Stages 6–7 did not run; do not attach earlier successes to the changed source. Preserve both the earlier temporary-directory failure and this source-drift failure. |
| Primary GitHub | Fresh 9 September read-back still identifies `f8ce28e6`, zero workflows/runs/rulesets/open PRs, unprotected main. | Full source publication, workflow execution, required checks, review protection and tested-main read-back. |
| Auditor | The immutable Jankurai 1.7.0 artifact has verified GitHub provenance. Its selected dependency closure omits reviewed fixes. | An admitted source closure, detector regressions, reproducibility/clean-environment execution and actual full audits. Provenance alone admits no score. |
| Media | Actual recorder, browser, RGB/master and GIF-timing component tests; retained synthetic browser frames and private historical captures. | Real application/backend/provider activity, a complete three-provider coding task, actual TUI capture, 1080p geometry, accessibility/color acceptance and export verification. |

The initial 165 findings and 28 cap occurrences remain triage inputs, not 193
confirmed defects. Every disposition needs the source subject, finding fingerprint,
reproducer, repair or auditor correction, independent review and acceptance receipt.

The first-turn gate-selection defect also needs direct closure: validate a nonempty,
bounded selection against the authoritative gate catalog before preparing or
launching any provider. Construct request/transcript validation before dispatch.
Unknown or stale gates must cause zero provider launches. A newly added error enum
must distinguish a failure before spawn from a failed capture after spawn. Preserve
failure evidence and uncertain cost for the latter, including read failures,
truncation, canaries, nonzero exit, timeout and missing terminal frames. A forked
process, a CLI exit and a billed model turn are separate observations.

## Packet order and acceptance contracts

Each row is a workstream implemented as separately reviewed packets of at most
four claimed files. A row is not permission to claim its entire surface. Root owns
integration; two workers implement disjoint packets; an independent reviewer
challenges both the implementation and the evidence. Canonical checkouts only.

| Order / governing packet | Concrete implementation | Required evidence before advancing |
| --- | --- | --- |
| H1 / DF-W0a | Finish the reviewed timeout, GitHub destination and documentation packets. Reconcile new Kernel changes; preserve all failed runs and uncommitted source. | Focused positive/negative tests, exact file hashes, independent review, clean committed subjects and regenerated inventories. No static success badge substitutes for a run. |
| H2 / DF-W0a/b | Complete clean member checks and dependency-ordered family proof using private targets and a short, private temporary root. Obtain all writers' freeze acknowledgement first. | Exact four commit/tree pairs, raw selected/completed IDs, fresh reports, no failed or skipped selected tests, exact declared profile selections/exclusions, no unexpected zero-test execution, final source/config/tool read-back, released proof locks. A source change invalidates the combined run. |
| H3 / DF-706 | Admit the canonical Jankurai family, including policy/exit/conformance consistency, inventory classification and safe/unsafe detector pairs. Replace the Python auditor launcher with the admitted Rust executable. | Immutable component tags and full source/dependency provenance; checksum and signature verification; clean-environment execution; tests retain unsafe-case detection. Keep old reports. |
| H4 / DF-706 | Close actual security/correctness findings, contract-purpose drift, generated/manual misclassification, owner/test omissions, large authored files and duplicated verifiers. Unify each repository's full-scan policy. | Actual full audits at floor 90, zero caps and zero hard findings; all remaining medium dispositions explicitly reviewed. No lower CLI override, fake generated header or broad path exclusion. |
| C1 / DF-W0a, DF-706 | Activate exact-source aggregate CI using the current 53-job/55-invocation catalog and existing lane scripts. Preserve timeouts, dependencies, matrices, triggers and artifacts. | Every implemented job executes its real lane and validates retained artifacts; unsupported profiles fail explicitly. Complete applicable PR and deliberately triggered scheduled campaigns pass on the exact candidate. |
| C2 / DF-104, DF-502 | Synchronize accepted member refs, immutable tags and the primary aggregate. Use a new publication request for changed subjects or destination. | Fresh remote ancestry/base read-back, expected-old writes, complete history scan, exact source tags/review branch, PR, tested merge subject, approval, protected integration and resulting main/run/artifact read-back. |
| R1 / DF-R4–R7, DF-DOG0 | Complete both-location preserved-generation admission and final locked validation; persist admission references and exact retry/restart handling. | Reviewed retained inputs, authenticated independent review and the actual operator checkpoint. Preserve the incident separately. An omitted optional argument is not admission. |
| R2 / DF-204/205 | Implement supervised schema 22→23 upgrade and subsequent execution migrations through it. | Exclusive maintenance custody, prefix-aware verified backup, one transactional migration, reopen/read-back, external authority high-water, interrupted retry, collision handling, quarantine-preserving rollback. |
| T1 / DF-101–103, DF-201/202, DF-602 | Define accounts, enrollments, quota buckets, reservations, launch grants, operational runs, proposals and operator commands in the authoritative wire source; generate Rust/JSON Schema/OpenAPI/TypeScript. | Closed schemas, canonical bytes, unknown/duplicate/stale/overflow refusals, generated drift checks and contract agreement across all consumers. |
| T2 / DF-202/203 | Connect `POST /api/v1/commands` to one durable admission transaction: authority/revision/idempotency, capacity reservation, nonce consumption, run allocation and dispatch outbox. | Concurrent submissions, duplicate keys, lost response, process death at every write boundary and restart return one persisted command/run without duplicate dispatch or capacity. |
| T3 / DF-302/303 | Connect production Runner to exact account/model/effort/runtime/snapshot/checkpoint/policy/gates/reservation/deadline; persist supervision and results. | Hostile startup/configuration, resource exhaustion, ambiguous start, acknowledged interruption and process-tree teardown proofs. Capacity remains held until termination or authoritative reconciliation is evidenced. |
| T4 / DF-204, DF-401 | Repair termination ordering and durable stop ownership. Success, cleanup and lease release must follow verified termination; unresolved execution must not expire into redispatch. | Failed/false/missing acknowledgement, capture failure, supervisor death, heartbeat expiry and restart cannot report success or authorize a successor while execution may survive. Test both immediate cleanup and delayed TTL reclaim. |
| T5 / DF-401/503 | Complete preimage-bound proposals and atomic Candidate finalization: preserved Candidate, Attempt transition, lease settlement, audit event and verifier outbox. | Invalid proposal validation occurs before consuming a one-use mutation permit where no effect has occurred; stale bases, limits, interruption and response-loss proofs preserve original identities. Rebase/repair creates a new Candidate and invalidates affected review. |
| T6 / DF-501/502 | Connect independent verifier and governed human integration. | Writer cannot issue its own accepted review; exact Candidate/ProofBundle/check/merge bindings; missing, stale, skipped or failed evidence refuses; post-integration reconstruction matches source and target. |
| U1 / DF-601–605 | Add task graphs, dependencies, acceptance criteria, exact path ownership, context capsules, handoffs, review/cancel/freeze/resume/account commands and an atomic Shift Brief. | Durable command IDs survive response loss, reload, reconnect and server restart; malformed/stale/gapped SSE triggers reconciliation; organization isolation, session/CSRF/RBAC and accessibility tests pass. |
| D1 / DF-701 | Run a fake provider through the same real API/ledger/Runner/containment/BulletGit/supervisor/artifacts/verifier path before credentialed coding. | No alternate simulator-only ingress or fixture self-signing can promote this to production acceptance; retained failure matrix and independent reconstruction pass. |
| P1 / DF-303, W8 | Independently qualify Codex, Claude and Cursor subscription/runtime/account closures on xbabe2. | Each provider completes the real transaction with exact model provenance, quota evidence, safe startup, interruption, isolation and failure/restart receipts. A successful provider cannot qualify another. |
| M1 / DF-604/706 | Build and verify real 1080p TUI and web recordings locally using tracked Rust/TypeScript tooling. | Native frames, actual production task IDs and timestamps, exact tool/source subjects, pixel/timing checks, original lossless masters and an independently checked export. |
| D2 / W8 internal campaign | Run twelve bounded accepted implementations, four per provider, plus mixed-provider collaboration using Bullet on Bullet. | Exact task/attempt/model/account/runtime/source/proposal/review/test/merge/receipt chain; failed attempts retained; operators can interrupt, recover and explain every outcome. |
| D3 / W8 observation | Observe the integrated tasks for seven days against comparable manually coordinated work. | Measured human effort, time to integration, recovery/review effort, duplicates, repairs, quota and escaped defects; actual surviving changes, no synthetic productivity claim. |

T3 is provider-capable wiring only: it admits no live execution before T4
durable stop/reclaim guarantees, D1 same-stack fake-provider proof and the
required operational/account checkpoints. M1 tooling is implemented and tested
before D2; actual recordings span D2 and are retained through D3 observation.

T4 cannot be closed by moving an ignored `terminate()` error to a later return.
Current cleanup and heartbeat expiry can still release/requeue work. Paused states
also expire; retaining an absorbing quarantined Attempt with its lease can obstruct
global expiry, while releasing that lease removes writer custody. Define and test
the durable stop/redispatch ownership contract first. Reuse the existing retained
worker state and exact settlement machinery where suitable; introduce any new
execution tables only through the supervised upgrade mechanism.

## Rapid CI without weakening the acceptance gate

Fast feedback and complete acceptance are distinct jobs using the same reviewed
scripts. Do not create another CI engine or silently substitute a smaller suite.

| Campaign | Execution and target | Required output / failure behavior |
| --- | --- | --- |
| Local packet loop | Formatting, type checks and the owning focused Rust or Portal tests, with a private build target. Measure warm and cold latency. | Exact selected/completed identities and failures. This speeds editing and does not grant the full required check. |
| Every PR | Source scan first, mapped fast/lint/contract/security/docs partitions, generated contracts, publication reconstruction and changed-seam adversarial proofs. | Full expected job/matrix set, actual aggregate event SHA, member subjects, tool/workflow hashes, run/attempt and artifact digests. Missing tools, tests or reports fail. |
| Family candidate | Dependency-ordered BulletGit → Kernel standalone/family → Portal standalone/real-farmd → Hub contracts/models; admitted daemon path and hash. | A clean four-subject observation plus all stage reports. No concurrent source/config mutation; no inherited old observation. |
| Scheduled candidate | History secrets, links, advisories/licenses, coverage, portable Jankurai, native-platform jobs and fuzz/sanitizer/fault campaigns required by the profile. | Deliberately dispatch against the candidate, retain run/attempt/artifacts, and verify each applicable campaign. Ordinary PR success cannot stand in for it. |
| Credentialed xbabe2 | Dedicated isolated worker for provider qualification, real coding tasks, interruption/restart, account/quota and native capture. | Private credential custody, sanitized derived outputs and exact execution receipts. Ordinary PRs receive no subscription, signing or publication credentials. |
| Privileged/lifecycle/native | Separate admitted workers for containment, service identities, upgrade/install/rollback and five-platform certification. | Named infrastructure blocker until actual execution. Cross-compilation or a workflow definition does not certify a platform. |

The next small CI adapter is BulletGit's actual `source-scan` lane, reusing the
checksum-pinned scanner, closed job context and existing report validators.
Its staged directory differs from the Hub's and must be handled explicitly.
Workflow activation follows verified transfer/reconstruction of the bootstrap
verifier because hosted jobs do not share a filesystem. Avoid rebuilding the entire
bootstrap independently in every source-scan job. Keep unsupported profiles and
the final incomplete-campaign gate failing until their execution is implemented.

Admit Rust toolchains/MSRV and Cargo configuration, Node 22.23.2, npm 10.9.8,
browser binaries and immutable tool subjects. Cache keys include member identity,
toolchain, lockfiles, lane/profile and trust boundary; an untrusted PR cache must
not supply release authority. Measure elapsed time, maximum memory, disk and
artifact volume before choosing worker capacity. The observed Hub maximum child
RSS is not a measurement of aggregate parallel memory. Serial cold builds remain
the local default until measurements justify a different budget.

The stable final required check rejects every missing, skipped, cancelled, neutral,
malformed, stale or failed predecessor or artifact. It verifies exact matrix
identities and selected/completed test sets rather than trusting a job name or
green process exit. Preserve real `GITHUB_SHA`; record source commit/tree separately.
Configure required checks, human approval, stale-review dismissal and no force-push
or deletion on primary main after the complete checks actually operate. Read back
both the tested merge subject and the resulting protected main.

## Three separate subscription qualifications

| Provider | Required protocol/runtime work | Minimum negative proofs |
| --- | --- | --- |
| Codex | Inspect the installed App Server schema; explicit account, model, effort, cwd and session identity; structured proposals, correlated notifications, acknowledged interruption and multiple vendor quota buckets. | Wrong/changed model or schema, duplicate/late events, response loss, malformed proposal, tool permission refusal, interruption without terminal acknowledgement, exhausted/contradictory quota and restart. |
| Claude | Durable stream-JSON dispatch with explicit model/effort; qualify the installed schema and structured-output mechanism; safe configuration/startup with subscription authentication preserved. | Empty/stale gates before spawn, changed runtime, unknown authority-bearing fields, hostile settings/hooks/MCP/plugins, malformed tool/result frames, missing cost, post-spawn capture failure, timeout and cancellation. |
| Cursor agent | Actual ACP initialization and authentication, session/model/mode selection, streaming, permission requests, blocking extensions and cancellation. | Unsupported initialization/extensions/mode, rejected or malformed permission, stale session, changed model, missing terminal completion, interrupted start and cancellation ambiguity. |

Use actual retained native protocol observations and official installed schemas;
synthetic fixtures remain regression inputs with explicit provenance. A single new
transcript does not justify accepting every unknown field. Classify new fields and
retain corresponding unsafe-case detection. Root-stage only the specifically
admitted executable closure, with digest, ownership, loader/libraries/runtime
files and a verifiable passport. A provider auto-update requires a new runtime
generation and requalification; it must not inherit the old passport.

Each account has a private home, credential generation, serialized refresh and
one active invocation. Repository-controlled startup code must not execute before
the policy permits it. Vendor quota remains independent of local invocation usage:
sparse updates, external usage, expiry, contradictory readings and exhaustion must
survive restart. Retain the existing 80% warning, 95% alert and exhaustion pause.
Manual snapshots are explicitly `OPERATOR_REPORTED`, finite, expire within one
hour or reset and cannot override later vendor exhaustion. Unknown charge is
`UNPRICED`, not zero. Provider failure cannot silently move work to another account.

## Twelve tasks and one demonstrable collaborative change

For each of Codex, Claude and Cursor, select one small Rust implementation, one
TypeScript/React implementation, one meaningful test change and one documentation
implementation from the live Bullet backlog. Freeze each task's baseline, accepted
paths, expected behavior, negative case and review criteria before invocation.
Use exact account/model/runtime identities and retain rejected proposals as well
as successful ones. Complete tasks through the product commands and integration
path, not a separate terminal agent whose output is later pasted into Bullet.

At least one accepted change must visibly cross providers: for example a Rust
contract/backend change, its generated TypeScript/React consumer and an independent
adversarial review. Assign those roles explicitly, preserve dependencies and
handoffs, and have a human integrate the reviewed Candidate. The implementation
workers and reviewer must not share completion authority. Include a duplicate
submission, stale base, provider failure, quota fallback, explicit account change,
bounded repair/escalation and a restart in the retained campaign.

Limits remain one active invocation per account, two implementation workers,
two repairs, one escalation, eight provider invocations per task and a 60-minute
deadline. Pauses, nested delegation or account changes reset none of them. Publish
the actual model provenance and failures; do not claim comparative productivity
until the seven-day observation and matched manual baseline support it.

## Real 1080p capture as maintained source

1. Keep capture, rendering and verification implementation in the tracked main
   source distribution. Preserve the current `just demo-gif-record`,
   `just demo-gif-render` and `just demo-gif-check` entrypoints or migrate them
   explicitly with documentation and compatibility tests. The final aggregate
   must contain their complete source and fixtures; a private scratch helper is
   not delivery of this requirement.
2. Move owned process supervision, terminal recording, artifact manifests and
   validation from Python to Rust. Keep browser recording and web behavior in
   TypeScript with the actual Vite/React product. Thin shell launchers may invoke
   the admitted tools; they must not become a second policy or state authority.
   Prove Rust behavior against the retained recorder interruption, EOF, custody,
   output-bound and timing regressions before replacing the old implementation.
3. Provide an actual bright/high-contrast product theme for both TUI and web.
   Use sharp text and saturated, distinguishable status colors with textual
   labels; measure contrast and keyboard/focus behavior. Capture the selected
   real theme rather than brightening or recoloring frames afterward.
4. Capture native 1920×1080 frames. Fix and record browser viewport, device pixel
   ratio, browser/fonts and terminal grid/font/raster geometry. No upscaling,
   dim overlays, interpolated motion or substituted screenshots. Check decoded
   geometry and pixels, including text edges and bright status colors.
5. Start recorders before the real product command and retain actual timestamps
   continuously through task admission, provider/account/model selection, parallel
   work, proposal, independent review, tests, human integration and final read-back.
   Include genuine failure/reconciliation behavior where it occurs. An explanation
   prompt or five staged screenshots cannot satisfy the coding demonstration.
6. Bind both views to the same task/run/Attempt/Candidate/review/merge identities
   and ledger watermarks. The browser must use actual production API/SSE and
   durable state; the TUI must show the real CLI/session. A task on Bullet's own
   source is required for the self-dogfood demonstration.
7. Preserve original PNGs and terminal stream/transcript privately, with a
   pixel-exact FFV1 master and hashes. GIF has a 256-color palette and centisecond
   timing: accept a lossless GIF only when an independent decoder proves exact
   source-pixel equality and the admitted timing bounds. Otherwise fail strict
   lossless GIF acceptance and retain the exact master; never label a quantized
   derivative lossless. Any final-frame hold is an explicit display policy.
8. Test failure paths: absent or drifted tools, renderer error, dropped/colliding
   frames, wrong dimensions, timing flattening, truncation, disk exhaustion,
   interrupted capture, incomplete provider termination and mismatched task IDs.
   Refuse export on missing evidence. Retain failures instead of overwriting a
   successful-looking output file.
9. Separate private capture from reviewed public export. Credentials, private
   account homes and raw secrets never become committed assets. Select a bounded
   public task whose real output can be exported; preserve any redaction history
   and original private evidence. Exported media, transcript and reproduction
   manifest must identify their exact public source subjects.
10. Verify local reproduction from the accepted aggregate with admitted tools:
    rebuild the capture/render/check code, reproduce the encoded output from
    retained inputs and independently check it. A new provider execution has its
    own task/run identity; deterministic rendering does not imply deterministic
    model output or repeat billing authorization.

## Exit checkpoints and final read-back

The earliest **proper internal development** milestone requires clean local health,
working exact-source CI and protected GitHub delivery, admitted operational custody,
the real durable coding transaction, three separately qualified subscriptions,
reviewable collaborative work, interruption/restart recovery and retained evidence.
Useful dogfood acceptance additionally requires twelve accepted tasks and seven-day
survival. Live media may document an admitted internal campaign with that narrower
scope; it must not claim `self-hosted-v1` or universal certification early.

Full completion still requires the signed twelve-boundary campaign, production
custody separation and historical recovery, Ubuntu packaging and two clean installs,
all lifecycle/backup/upgrade/rollback proofs, durable cognition/routing/evolution,
Jeryu self-hosting, Antigravity, remaining forge adapters, five platform slices,
then `team-v1` and `saga-v1`, plus active post-V1 obligations and retired dispositions.
All 18 product profiles must pass independently under the existing typed inventory.
Do not remove a required profile to make the release page green.

For every accepted packet, reconcile this plan, the health checkpoint and G1–G18
register with the typed inventory; unchanged blocked statuses may retain identical
canonical JSON bytes. Record exact source commits/trees, review/PR/merge subjects,
workflow digests, run/attempt/matrix identities, selected/completed tests, artifact
and package digests, provider/account/runtime generations, task receipts and profile
receipts. Final read-back must come from actual local/remote consumers, not this
document or a chat claim. Forecast dates only after two measured implementation
cycles, separating infrastructure waits from the mandatory observation window.
