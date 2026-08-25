# Bullet Farm documentation map

Status: **pre-release; release authority remains blocked**  
Last reviewed: 2026-08-25

This file is the hub documentation index. Do not add `docs/INDEX.md`.
A document can explain a decision, but it cannot make a command, receipt,
Candidate, Evidence result, effect, or release true.

## Current authority and status

| Question | Authoritative source |
| --- | --- |
| What can cross a trust boundary? | Generated schemas and Rust DTOs under `contracts/`, `policy/`, and `crates/bullet-wire/` |
| What is durably authorized or complete? | Kernel ledger state plus exact signed receipts; never a portal projection or prose claim |
| Which repository subjects form a family release? | A verified signed `family.lock` and its signed tags |
| Which gates are still blocked? | [`release.md`](release.md) |
| What is still missing as a product? | [`assurance/product-gaps.md`](assurance/product-gaps.md) (G1–G15 index; `check release` wins) |
| Which of the 26 `check release` gates is which product gap? | [`assurance/product-gaps.md`](assurance/product-gaps.md#the-26-check-release-gates) |
| Which controls have executable enforcement? | [`assurance/invariant-registry.md`](assurance/invariant-registry.md) and the generated [`assurance/invariant-crosswalk.generated.md`](assurance/invariant-crosswalk.generated.md) |
| What is the rendered release decision? | [`assurance/release-truth.generated.md`](assurance/release-truth.generated.md) (generated projection, exit 3; `check release` wins) |
| Which architecture decisions are current? | [`decisions/`](decisions/) |
| What is the shortest normative architecture map? | [`architecture.md`](architecture.md) |
| Where are trust, repository, credential, and evidence boundaries? | [`boundaries.md`](boundaries.md) |
| What does a stable Hub error mean and how is it repaired? | [`errors.md`](errors.md) |
| What runs in the current dependency order? | [`assurance/phase-1-dependency-map.md`](assurance/phase-1-dependency-map.md) and the repository test maps |
| Which competitor subjects are pinned? | [`assurance/competitor-snapshot.md`](assurance/competitor-snapshot.md) |
| How do agents coordinate? | [`runbooks/fleet.md`](runbooks/fleet.md) |
| Which operator procedures exist? | [`runbooks/README.md`](runbooks/README.md) |
| Which operator decisions are still open? | [`decisions/0013-operator-decision-register.md`](decisions/0013-operator-decision-register.md) |
| What does a term mean here? | [`glossary.md`](glossary.md) |
| What is post-V1? | [`phase-9-10.md`](phase-9-10.md) |
| Which paper-driven opportunities remain? | [`workplan.md`](workplan.md) (non-authoritative; the V1 closure plan wins) |
| What is the canonical byte pipeline? | [`assurance/canonicalization.md`](assurance/canonicalization.md) |

The release index is deliberately fail-closed. `BLOCKED`, `UNKNOWN`, a missing
tool, a skipped test, a zero-test run, or a simulator receipt does not become
green through documentation.

## Explanatory documents

- [`architecture.md`](architecture.md) is the canonical short architecture
  entrypoint and current proof boundary.
- [`boundaries.md`](boundaries.md) maps repository, authority, credential,
  evidence, effect, and projection crossings to their fail-closed behavior.
- [`errors.md`](errors.md) maps stable Hub error classes to bounded repair and
  escalation steps; `UNKNOWN` remains an outcome state, never a green result.
- [`paper/`](paper/) is the IEEEtran arXiv preprint source. A compile is not
  a release, installer, or benchmark receipt.
- [`workplan.md`](workplan.md) is an opportunity backlog for paper evidence,
  forge publication, and post-V1 hardening. It cannot change V1 scope or gate
  status; [`assurance/v1-closure-plan.md`](assurance/v1-closure-plan.md) wins.
- [`brand/mascots/`](brand/mascots/) is sticker-first image-generator briefs.
  Generated art is not a receipt.
- [`architecture/overview.md`](architecture/overview.md) is the concise system
  orientation.
- [`architecture/evolutionary-control.md`](architecture/evolutionary-control.md)
  defines roles, Variants, evidence-bound fitness, selection, fusion, budgets,
  and the V1 adaptation boundary.
- [`testing.md`](testing.md) maps test profiles, evidence classes, ownership,
  negative cases, and live-lane admission.
- [`runbooks/`](runbooks/) describes operator procedures; a runbook does not
  bypass an API or policy gate.
- [`runbooks/source-setup.md`](runbooks/source-setup.md) separates contributor
  bootstrap from the blocked signed release installer.
- [`runbooks/backup-restore.md`](runbooks/backup-restore.md) covers receipt-bound
  SQLite snapshots and the mandatory restore quarantine.
- [`runbooks/README.md`](runbooks/README.md) is the runbook index.
- [`runbooks/fleet.md`](runbooks/fleet.md) is the agent coordination runbook:
  claims, heartbeats, handoffs, receipts, receipt corrections, and the
  orchestrator-only commit rule.
- [`runbooks/live-conformance.md`](runbooks/live-conformance.md) is the operator
  act that ratifies a generation-2 policy outside the repositories and runs the
  Kernel live-conformance lane. No `LIVE_PROOF` receipt exists yet.
- [`assurance/`](assurance/) maps claims to code, schemas, and tests.
  Start at [`assurance/product-gaps.md`](assurance/product-gaps.md) for the
  remaining-gap index.
- [`assurance/v1-closure-plan.md`](assurance/v1-closure-plan.md) is the
  dependency-ordered implementation and proof map for the remaining V1 work.
- [`assurance/product-gaps.md`](assurance/product-gaps.md) is the remaining-gap
  index (G1–G15, V1-S leftovers, C1–C12 product status). Documentation closed
  the visibility gap; implementation remains. `bullet-family check release
  --json` wins if they disagree.
- [`assurance/release-truth.generated.md`](assurance/release-truth.generated.md)
  is the generated, drift-checked portable rendering of `check release
  --report --portable` (`just release-truth`). It is a projection with decision
  exit 3, never a receipt.
- [`assurance/invariant-crosswalk.generated.md`](assurance/invariant-crosswalk.generated.md)
  is generated from `policy/v1alpha1/invariant-registry.json` by
  `just contract-generate`; hand edits are drift.
- [`decisions/`](decisions/) records reviewed design choices and their status.
- [`decisions/0013-operator-decision-register.md`](decisions/0013-operator-decision-register.md)
  lists the decisions only an operator can make (ratification, key custody,
  forge topology); an agent cannot close one.
- [`glossary.md`](glossary.md) defines the terms these documents use.
- [`spec/README.md`](spec/README.md) indexes the historical Centerrail/Bullet
  Farm design provenance under [`spec/`](spec/). It is useful context and never
  runtime or release authority.

## Executable local evidence

Run commands from the public hub checkout:

```bash
just ci-doctor            # missing local tools before a lane starts (fast|required|contract|security|audit|all)
just fast
just contract-check       # generated policy/schema/client byte drift
just model-check          # exactly two pinned TLC models
just contract
just check-family
just family-contract
just security
just audit
just release-truth        # regenerate docs/assurance/release-truth.generated.md (decision exit 3 preserved)
cargo run --locked --quiet --bin bullet-family -- check release --report [--portable]
```

The meaning and limitations of those lanes are defined in
[`testing.md`](testing.md) and [`release.md`](release.md). `check release
--report` renders the 26-gate decision as an operator brief and keeps exit 3
while every gate is `BLOCKED`; `just release-truth` writes its portable variant
and the `required` lane refuses a stale copy. Live-provider, live-forge,
package, signing, and release evidence must use their separately admitted lanes
and exact subjects; the commands above do not substitute for them.

## Maintenance rule

When an implementation changes, update its generated contract or executable
test first. Change a status document in the same reviewed transaction that
adds the receipt supporting the new status. Preserve historical sources
verbatim when practical and refresh `spec/HISTORICAL_ARTIFACTS.sha256` whenever
a tracked historical Markdown source intentionally changes.
