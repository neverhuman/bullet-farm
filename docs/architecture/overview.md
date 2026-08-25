# Architecture

Status: **normative orientation; implementation remains pre-release**  
Last reviewed: 2026-08-25

Bullet Farm is a local-first transaction processor for software changes and
model cognition. It lets several specialized agents explore competing ideas
without letting model confidence become write, verification, or release
authority.

```text
Mission → immutable Graph Revision → Selection Group / Variants
  → fenced Attempt → typed PatchProposal → exact Candidate
  → independent Evidence → selection/fusion → brokered Effect
  → protected Integration → Observation Window → Outcome
```

## Authority planes

| Plane | Owns | Cannot prove |
| --- | --- | --- |
| Kernel | Missions, graphs, commands, leases/fences, policy, routing, event log, outbox | Git state or engineering correctness |
| BulletGit | private clones, atomic generations, checkpoints, Candidates, preservation, proof subjects | scheduling, provider identity, or protected integration |
| Runner | admitted provider supervision, heartbeat, gate execution | write authority outside its fenced Attempt |
| Verifier | clean reconstruction and exact-Candidate Evidence | writer state or integration |
| Effect broker | authorized dispatch, read-back, reconciliation, integration receipts | model execution or Candidate correctness |
| Portal | sequence-bound projections | any authoritative state transition |

Jeryu is the source forge and mandatory local effect adapter. GitHub is a
separate effect adapter, not Kernel or source authority. Cross-boundary values
come from generated `bullet-wire` contracts; prose and TypeScript types do not
define the wire.

## Evolutionary control

Agents receive capability-scoped roles, immutable task/context/configuration
snapshots, and budgets. A Selection Group may create isolated Variants; each
Variant has at most one active fenced writer. Candidates are immutable
phenotypes of those inputs. Selection uses independently reproduced evidence
and a deterministic vector policy. Fusion creates a new lineage rather than
rewriting a winner. See
[`evolutionary-control.md`](evolutionary-control.md) for the normative model.

The V1 records routing and outcome provenance but does not learn online or tune
itself. Adaptive routing, larger councils, and distributed execution remain
gated by [`../phase-9-10.md`](../phase-9-10.md).

## Truth and evidence

Mutable operational authority is the Kernel ledger. Immutable engineering
subjects live in BulletGit. Remote integration exists only when an effect
receipt is read back and reconciled. A timeout becomes `UNKNOWN`; zero tests,
unsupported, skipped, flaky, infrastructure error, or writer-produced proof
never becomes `VERIFIED`.

The current implementation and its gaps are indexed in
[`../assurance/v1-closure-plan.md`](../assurance/v1-closure-plan.md),
[`../release.md`](../release.md), and
[`../assurance/product-gaps.md`](../assurance/product-gaps.md). Historical
Centerrail material under `docs/spec/` explains design provenance only.
