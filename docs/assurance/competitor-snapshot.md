# Gas Town and Gas City comparison snapshot

Status: **RESEARCH INPUT — not release evidence**
Observed: 2026-08-25 UTC
Owner: Bullet Farm maintainers

This bounded snapshot prevents the design comparison from silently tracking a
moving branch. It records public upstream subjects and extracts mechanisms to
test. It is not a benchmark result and makes no superiority claim.

## Exact upstream subjects

| Project | Subject observed | Source |
| --- | --- | --- |
| Gas Town | release `v1.2.1`, peeled commit `319d33a91b2deca59bba6dd26be6b9daf8eaacf6` | [release](https://github.com/gastownhall/gastown/releases/tag/v1.2.1), [commit](https://github.com/gastownhall/gastown/commit/319d33a91b2deca59bba6dd26be6b9daf8eaacf6) |
| Gas City release | release `v1.4.1` (2026-08-15), tag object `610666314a884cd2db7386c1df3b6ef7c521706e`, peeled commit `58ef17e3bd685fd5cf7f21286277b208d3324590` | [release](https://github.com/gastownhall/gascity/releases/tag/v1.4.1), [commit](https://github.com/gastownhall/gascity/commit/58ef17e3bd685fd5cf7f21286277b208d3324590) |
| Gas City development | `main` commit `2cd07e018bf3680d24b037b509e6a4bad5e623ba` observed 2026-08-25 | [repository](https://github.com/gastownhall/gascity), [commit](https://github.com/gastownhall/gascity/commit/2cd07e018bf3680d24b037b509e6a4bad5e623ba) |

Gas Town's tag remains its released product baseline. Gas City's released tag
is the product baseline; its separate branch row is a dated source observation
only. Every later comparison must record a new tag or commit and date instead
of silently moving either subject.

## Mechanisms worth preserving

Gas Town demonstrates durable named work, explicit operational roles, provider
plurality, proactive handoff, health patrol, merge-queue specialization, and a
visible factory metaphor. Gas City generalizes much of that into six declarative
primitives—Agent, Bead, Formula, Rig, Pack, and Event—with configurable roles,
formula fan-out, multiple runtime providers, an append-only event surface, and
controller reconciliation.

Gas City 1.4.1 raises the concrete comparison bar beyond those primitives. Its
release line includes typed and paginated run/session APIs, warm projections,
worker-boundary session convergence, formula-v2 retry/fan-out/drain/scope
controls, explicit degraded storage outcomes, release-candidate gates using
real inference, fenced dead-session cleanup, and platform artifacts with
checksums, SBOMs, and attestations. These are observed mechanisms to test, not
evidence that either system is safer or faster.

Bullet Farm should preserve the product leverage while measuring these exact
properties:

| Property | Required Bullet measurement |
| --- | --- |
| Durable work | Restart from every transaction boundary retains one exact Mission/Attempt/Candidate history |
| Role flexibility | A role is typed policy/configuration, not hidden authority or a hard-coded persona |
| Parallel fan-out | Receipts report useful surviving outcomes, contention, cost, latency, and verifier backlog |
| Provider plurality | Identical proposal/evidence contracts pass independently for Claude, Codex, Cursor, and Antigravity |
| Health reconciliation | Lease/fence and observation state—not terminal names or silence—drive recovery |
| API and projection scale | Typed pagination, replay watermarks, warm-read behavior, and degraded-state truth remain correct under a measured run/session corpus |
| Declarative workflow control | Retry, fan-out, drain, cancellation, and scope transitions preserve one exact graph/effect identity under faults |
| Review and gap filling | Writer, verifier, and effect attestor remain independent exact-subject principals |
| Portability | One signed family lock and package manifest reproduces clean ordinary clones from a hub-only start |
| Release evidence | Archives, checksums, SBOMs, attestations, real-inference gates, and installer receipts are compared by exact subject rather than feature presence |

## Deliberate Bullet boundaries

Gas City's primitive test—keep judgment out of transport code and prefer
primitives that remain useful as models improve—is compatible with Bullet Farm.
Bullet adds a stricter rule: transport success, task closure, session state, and
model judgment cannot confer mutation, verification, or integration authority.

The resulting split is intentional:

- Kernel is the durable transaction and lease authority.
- BulletGit is the exact repository-subject and Candidate authority.
- providers are read-only proposal producers;
- a clean verifier owns independent Evidence;
- an effect broker owns forge credentials and ambiguity reconciliation; and
- Portal is a sequence-bound projection only.

These boundaries cost more machinery than a role/session orchestrator. The cost
is justified only if receipts show fewer destroyed changes, false completions,
duplicate effects, unrecoverable sessions, and escaped regressions at acceptable
latency and spend.

## Fair benchmark contract

Run the same repository/task corpus, tool budget, wall-clock limit, provider
profiles, retry cap, and integration policy. Report distributions and failures,
not a single success rate. At minimum capture:

- candidate survival and protected-integration rate;
- escaped-defect and revert rate;
- false-completion and zero-test rate;
- duplicate/ambiguous effect rate;
- crash recovery time and lost-work bytes;
- model/tool spend, elapsed time, and human interventions; and
- p50/p95 verifier backlog and end-to-end latency.

No benchmark begins while Bullet still substitutes synthetic proof for its own
five-plane transaction. No public comparison is published without raw task
subjects, configuration, receipts, exclusions, and exact upstream commits.

## Refresh procedure

1. Resolve the released Gas Town and Gas City tags (including annotated tag
   objects and peeled commits) plus any explicitly compared development branch.
2. Record observation date, URLs, configurations, and benchmark corpus digest.
3. Review upstream architecture and release notes for changed primitives.
4. Add a new dated snapshot or append a clearly delimited revision; do not
   silently replace the subjects used by an existing result.
5. Re-run only after Bullet's exact build and live receipts are current.
