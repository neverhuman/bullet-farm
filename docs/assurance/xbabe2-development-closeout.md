# Xbabe2 development dogfood close-out

Status: **ACTIVE operator/engineering sequence; planning only**  
Last reconciled: 2026-09-09  
Owner: Bullet Farm maintainers

This page is the shortest honest path from the current xbabe2 family to
**using Bullet for our own development** with production Codex, Claude, and
Cursor accounts. It does not replace the
[full-product plan](full-product-dogfood-plan.md), [G1–G18 register](product-gaps.md),
[closure roadmap](closure-roadmap.md), [health checkpoint](health-checkpoint-20260909.md),
or [ADR 0015](../decisions/0015-dogfood-track.md). Executable checks and admitted
receipts always win.

It is not a second audit. Dated observations stay in
[deep-audit-20260909](deep-audit-20260909.md). This page is the close-and-verify
queue.

Language lock: **Rust**, **Vite + TypeScript + React**, and the existing Bash
CI wrappers that already invoke those trees. No new Python product-truth. No
new CI engine. Recorder/render Python remains capture tooling, not the product.

## 1. Two finish lines that must not be conflated

| ID | Honest claim when closed | Required for daily xbabe2 coding? | Current fact |
| --- | --- | --- | --- |
| **XD-M0** | Human-launched Codex, Claude, and Cursor agents coordinate real Bullet work through Bullet claims, heartbeats, and sole-writer commits | **Yes, for “use Bullet to develop Bullet” as the coordination board** | **BLOCKED.** Operating HOLD. `bullet-family check dogfood --json` is fail-closed (`COORD_UNAVAILABLE`, `DOGFOOD_POLICY_MISSING`). Agents must not write enrollments, keys, or operator-decision lines |
| **XD-D2** | One fake provider completes the real `/api/v1/commands` → ledger → Runner → containment → BulletGit → supervisor → artifacts → verifier path, with duplicate, response-loss, process-death, and restart negatives | **Yes, before Bullet itself launches those CLIs** | **BLOCKED.** Dispatch is `run_demo`; production Runner is `sim`; BF-A01–A06 remain open |
| **XD-D3** | Each of Codex, Claude, and Cursor independently completes that same production route on this host under a labeled coding-harness exception | **Yes, to prove the three-account loop** | **BLOCKED** by XD-D2, HOLD, and per-CLI qualification |
| **XD-GA** | `check release --profile self-hosted-v1` is green from an admitted registry | **No.** First GA is later than useful dogfood | **BLOCKED.** All 18 product profiles plus two diagnostics remain blocked |

Use XD-M0 for human-launched development **as soon as the operator recovers the
coordinator**. Use XD-D2/D3 when Bullet must spawn and supervise those CLIs.
Do not wait for XD-GA, evolution, team, or saga before starting XD-M0 or XD-D2.

A coding-harness exception may label a native-account illustration. It does not
confer production admission, flip `UNKNOWN` to `VERIFIED`, or set
`live_admission_enabled`.

## 2. Live bind (re-read before executing)

Re-read `/home/ubuntu/bullet/AGENT_CHAT.md` physical EOF and these commands
before treating this table as current.

| Subject | Observation at plan write |
| --- | --- |
| Hub canonical inode | `83434247` at `/home/ubuntu/bullet/bullet-farm` |
| Hub HEAD | `54a0145` `docs: record scoped media verification and remaining CI blockers` on `cursor/jankurai-90-bullet-farm` |
| Hub dirty | Other-lane media/config/portal-tour files. Do not mix them into this packet |
| Kernel HEAD | `02bf7c5` / `79cae6e` — local `required` accepted (checkpoint). GitHub still `8c9b8e8a` |
| BulletGit HEAD | `6cf10e7` — matches GitHub `9600d668` snapshot lineage |
| Portal HEAD | `e76eeb4` — matches GitHub `22359616` snapshot lineage |
| GitHub `neverhuman/bulletfarm` | `f8ce28e6`, **0 workflows**, unprotected `main` |
| GitHub member mains | Farm `54860587`, Kernel `8c9b8e8a`, Git `9600d668`, Portal `22359616` — **0 workflows each** |
| Publication dest | Still JeRYu in `publication/config.json` |
| `gh` token | `repo`, `read:org`, `gist` — **no `workflow`**. Cannot publish or update Actions files |
| Codex | `0.153.4` at `~/.npm-global/lib/node_modules/@openai/codex/bin/codex.js` |
| Claude | `2.1.266` at `~/.local/bin/claude` |
| Cursor agent | `2026.09.08-6caf4ff` at `~/.local/bin/cursor-agent` |
| Local Jankurai 1.6.11 | Farm 92 / Kernel 95 / Git 92 / Portal 92 on the scored trees; not hosted `release.jankurai-90` |

Kernel `02bf7c5` repairs BF-A11’s active consumer. Sixteen Hub
`generated/zones/*.txt` stubs remain declaration-only (BF-A10). Do not treat
those scores as release health.

## 3. Language and proof rules

- Product code: Rust workspaces (Hub, Kernel, BulletGit) and Portal
  Vite/TypeScript/React. Tests stay in those languages plus the existing Bash
  `scripts/ci-local.sh` lanes.
- Do not add a second orchestrator, a Python product crate, or a 15th Playwright
  spec to chase a metric.
- Stop rules: dirty or changed subjects, failed/skipped/zero tests, missing
  artifacts, stale generations, signer substitution, unreconciled effect, or
  `UNKNOWN` painted green.
- Retry reads back by original request identity. Never dispatch a second write
  because a response was lost.
- No Hub path swaps. No shared `~/.cargo/config.toml` edits. Use
  `CARGO_NET_GIT_FETCH_WITH_CLI=true` / `cargo --config` only.
- No `bullet-family coord` verbs until HOLD lifts. No provider spawn from an
  unclaimed lane.

## 4. Close-out packets

Packets are serialized where marked. Parallelism is allowed only on disjoint
paths after the named freeze.

### XD-0 — Stop the theater (now, any unclaimed owner)

| Work | Verify |
| --- | --- |
| Leave claimed recorder/config/media paths until handoff | `git -C bullet-farm status --porcelain` names only the claiming lane’s files |
| Do not commit unverified README GIFs or host-local tool manifests | Family README does not call text-replay GIFs “authenticated sessions” without the audit’s limit |
| Do not add zone stubs or comment-strip canonical JSON to move a score | BF-A10/A11 stay closed or stay named open; no new stubs |
| Collapse citations to the active four: this page, `full-product-dogfood-plan.md`, `product-gaps.md`, `closure-roadmap.md` | Do not execute `launch-plan.md`, `path-to-100.md`, `CLOSE-100-*`, or `OWNER-CLOSE-PLAN.md` |

**Exit:** working trees that this lane owns are clean; no new health workarounds.

### XD-1 — Rapid local CI (engineering, this packet)

Reuse existing lanes. Do not invent a second engine.

| Lane | Command | When to run | Not |
| --- | --- | --- | --- |
| Hub component | `cd /home/ubuntu/bullet/bullet-farm && just dogfood-rapid` (Hub `fast` only) or `just fast` | Every Hub source packet | `just check` / family / Hub `required` unless you hold the sole-writer slot |
| Kernel standalone | `cd /home/ubuntu/bullet/bullet-kernel && bash scripts/ci-local.sh required` | After Kernel source; private target; no inherited `CARGO_TARGET_DIR` | Concurrent with another Kernel cold build |
| BulletGit | `cd /home/ubuntu/bullet/bullet-git && bash scripts/ci-local.sh required` | After Git source | Family lock unless family owner |
| Portal | Node **22.23.2** / npm **10.9.8** via nvm, then `bash scripts/ci-local.sh required` | After Portal source | Extra Playwright specs |
| Family order | `just family` from the Hub **only** with four clean heads and sole family custody | After XD-2 clean heads | A diagnostic over dirty trees |
| Dogfood board | `cargo run --locked --quiet --bin bullet-family -- check dogfood --json` | Status only | Treating exit 1 as a surprise; it is the current truth |
| Jankurai | Pinned `/home/ubuntu/.jeryu/bin/jankurai` **1.6.11** `--full` into a private out dir | After health packets | Deleting prior output; wrapper floor overrides |

`just dogfood-rapid` with explicit absolute member roots runs each member’s
existing `scripts/ci-local.sh fast` in Git → Kernel → Portal → Hub order. That
is a **component smoke**, not `required`, not family Evidence, not hosted CI.

Hosted Actions cannot turn green until an operator token or App has the
`workflow` scope and publication dest is GitHub. Local `ci.yml` already calls
these same scripts. Do not strip workflow files to force a snapshot push.

**Exit:** the packet’s own `fast` or `required` lane is green on the exact
commit/tree, with tool versions recorded. Dirty foreign files are not hidden.

### XD-2 — Publish accepted source without workflow theater

| Work | Owner | Verify |
| --- | --- | --- |
| Kernel `79cae6e` + `02bf7c5` onto GitHub `neverhuman/bullet-kernel` `main` as a source-only PR (no workflow-file add vs current GitHub main) | this lane / integrator | PR URL; merge only if the tree is the reviewed repair; hosted checks may be empty until workflows exist |
| Do not snapshot Hub `54a0145` onto GitHub by deleting `.github/workflows` | everyone | Root already rejected that workaround |
| Operator: grant `workflow` scope or a repo-scoped App; protect `main` on `bulletfarm` and the four members | operator | `gh api repos/neverhuman/bulletfarm/rulesets` is non-empty; Actions run on the merge subject |
| Point `publication/config.json` dest at `https://github.com/neverhuman/bulletfarm.git` through a **new** publication request | publication owner after dest change | Expected-old and read-back; historical JeRYu request identities retained |

**Exit:** accepted Kernel repair is on GitHub; Hub/Git/Portal publication waits
on workflow-capable authority rather than a stripped snapshot.

### XD-3 — Clean four heads (DF-W0a, after foreign claims freeze)

| Work | Verify |
| --- | --- |
| Integrate reviewed recorder/renderer/config packets or leave them uncommitted | Hub `git status --porcelain` empty on the accepted subject |
| Remove or replace BF-A10 declaration-only zone stubs | Sync checker fails if a real generated zone is absent or corrupt |
| Four standalone `required` lanes on immutable commit/tree pairs | No skipped/zero-test/dirty partition |
| One family observation (`just family`) | Unsigned `bullet.ci-observation.v1`; second identical run |

**Exit:** four clean heads. Still no coordinator use.

### XD-4 — Coordinator recovery and XD-M0 (operator-gated)

Agents implement only local consumers already specified in
`docs/runbooks/coordinator-recovery.md`. They do not chmod ledgers, create
`CURRENT`, write policy, or run the live incident.

| Step | Closer | Verify |
| --- | --- | --- |
| Two-location admission, independent review, omitted-input refusal, final-lock revalidation | operator + recovery owners | Recovery runbook exits; rehearsal bundle reviewed |
| Operator checkpoint (ADR 0015 / OD-K) | operator only | Written decision; no agent-authored OD line |
| Distinct development generation; restart read-back | operator after checkpoint | `check dogfood --track coordination` names no `COORD_*` blocker |
| DF-DOG0: one docs/test change through status → claim → heartbeat → handoff → sole-writer commit → restart | recovered coordinator | Stored request IDs reconstruct the loop after process restart |

**This is the first day we develop Bullet on Bullet.** Direct commits after
DF-DOG0 invalidate the exercise. Evidence class remains `COMPONENT`.

### XD-5 — Durable fake-provider coding command (XD-D2, Kernel + Portal)

Engineering. No live credentials. Language: Rust (Kernel/Hub) and
TypeScript/React (Portal). Implement in this order; each row has a failing
test first.

| ID | Change | Negative proof |
| --- | --- | --- |
| BF-A01 | Extend `/api/v1/commands` so one transaction admits account, expected revision, launch nonce, quota reservation, and allocated run. Keep `run_demo` as a distinct kind | Concurrent identical requests allocate once; changed payload, stale revision, exhausted quota, or invalid authority leave no partial row |
| BF-A02 | Production Runner accepts one non-`sim` fake-provider path carrying account, model/effort, runtime, snapshot, paths, gates, reservation, deadline | Frozen provider probes do not enter this route until the fake path is green |
| BF-A03 | Propagate `adapter.terminate` uncertainty; do not record success when teardown fails | Successful task + failed terminate; cancelled task + surviving child |
| BF-A04 | Supervisor journal and checkpoint writes are fallible and durable | Inject write/rename/sync/malformed/restart failures |
| BF-A05 | Persist Candidate finalization before workspace cleanup | Kill at each boundary; same Candidate reconstructs; rebase creates a new Candidate |
| BF-A06 | Control Tower persists the scoped pending envelope before send | Commit-then-drop-response, reload, exact retry, conflicting digest, org switch |
| BF-A07 | Event-stream projections refresh by sequence; Shift Brief is one atomic read | Contiguous updates, gaps, reorder, reconnect, restart |

**Exit:** one fake provider through the real API and ledger. Portal reload does
not mint a new operation key. This is still not live admission.

### XD-6 — Qualify the three installed CLIs (XD-D3, after XD-5)

Qualify the **installed** subjects, not docs. Isolated credential worker later;
do not point untrusted PR code at subscription homes.

| CLI | Must observe | Must not |
| --- | --- | --- |
| Codex 0.153.4 | App Server schema, explicit model/effort/cwd, native session, interruption, multiple rate-limit buckets | Treat missing quota as unlimited |
| Claude 2.1.266 | stream-JSON, explicit model/effort, structured proposal, subscription auth | Use `--bare` as a subscription-preserving shortcut |
| Cursor agent 2026.09.08-6caf4ff | ACP init, `cursor_login`, session/model/mode, permission replies, cancellation | Leave a permission prompt unanswered and call the session qualified |

Persist vendor quota separately from Bullet invocation use. Warn 80%, alert
95%, pause at exhaustion. `OPERATOR_REPORTED` snapshots expire within one hour.
Unknown charges stay `UNPRICED`. Limits: one active invocation per account, two
implementation workers, two repairs, one escalation, eight invocations, 60
minutes.

**Exit:** each CLI has one create-once operational observation on the fake-then-
native path. `live_admission_enabled` remains false. No release gate turns green.

### XD-7 — Twelve bounded self-tasks (useful dogfood)

After XD-M0 **or** XD-D3 (prefer M0 for human-launched; D3 when Bullet spawns):

Four tasks per subscription CLI: one Rust, one TypeScript/React, one test, one
documentation change **inside this family**. Each task:

1. Exact owned paths and acceptance criteria.
2. Account and model selected explicitly.
3. Progress, one interruption, and reconciliation.
4. Typed `PatchProposal` → Candidate → independent review → human integrate.
5. Exact resulting tests and commit.
6. Retain at least one failed attempt.

Include one mixed-provider handoff. Then measure seven-day survival of the
integrated changes. Media from those same subjects may be recorded only after
the repaired capture/render packets land; GIF is a palette derivative of a
lossless master, never Evidence.

Bonus (coding-harness exception): one of the twelve is a real Bullet Farm fix
so the product visibly improves itself. Label it. Do not call it production
admission.

**Exit:** twelve accepted tasks, retained failures, exact model/source evidence.
Still not `self-hosted-v1`.

### XD-8 — Hosted CI and first GA (later; do not block XD-4/5)

1. Operator workflow authority + protected `main`.
2. Render publication jobs as real `scripts/ci-local.sh` steps (BF-A12). Final
   gate rejects skipped, cancelled, stale, and zero-test lanes.
3. Checksum-pinned Jankurai 1.6.11 in scheduled CI. Exit 78 stays red.
4. Then W7 `TRANSACTION_PROOF` and W8 `self-hosted-v1` per the full-product plan.

## 5. Operator-only unblocks (agents document, never perform)

- Workflow-capable token or GitHub App; protect `main` on five repos.
- HOLD decision, two-location admission, dogfood policy/binding files (no
  committed secrets).
- OD-A enrollments for the three accounts when XD-6 is otherwise ready.
- Leave `live_admission_enabled=false`.
- Do not ask agents to chmod coordinator ledgers.

## 6. What this sequence deliberately skips until later

Adaptive councils, fusion dashboards, learned routing, team/saga, five-platform
archives, GitLab adapters, and Nightshift-scale Shift Brief beyond one atomic
brief. Those are Waves 6/9/10/11. They must not precede XD-4 or XD-5.

## 7. Definition of “we can develop on xbabe2”

Report the earliest true sentence; do not skip ahead.

1. **Engineering loop:** four member `fast`/`required` lanes and Portal/farmd
   are usable on clean subjects. (Almost here; dirty claimed Hub files block a
   family proof.)
2. **Coordination dogfood:** XD-4 DF-DOG0 passed. Daily work uses Bullet claims.
3. **Spawned-provider dogfood:** XD-5 + XD-6 passed. Bullet launches the three
   CLIs through the real command path.
4. **Useful self-host:** XD-7 twelve tasks + seven-day survival.
5. **Ship to a stranger:** XD-8 / `self-hosted-v1`.

Until (2) or (3), “proper dogfooding” is not true. Local Jankurai scores,
offline GIFs, and GitHub snapshots without Actions do not close (2) or (3).
