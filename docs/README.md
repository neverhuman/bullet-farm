# Bullet Farm documentation map

Status: **pre-release; release authority remains blocked**  
Last reviewed: 2026-08-25

This index separates executable product authority from design provenance. A
document can explain a decision, but it cannot make a command, receipt,
Candidate, Evidence result, effect, or release true.

## Current authority and status

| Question | Authoritative source |
| --- | --- |
| What can cross a trust boundary? | Generated schemas and Rust DTOs under `contracts/`, `policy/`, and `crates/bullet-wire/` |
| What is durably authorized or complete? | Kernel ledger state plus exact signed receipts; never a portal projection or prose claim |
| Which repository subjects form a family release? | A verified signed `family.lock` and its signed tags |
| Which gates are still blocked? | [`release.md`](release.md) |
| Which controls have executable enforcement? | [`assurance/invariant-registry.md`](assurance/invariant-registry.md) and its generated crosswalk |
| Which architecture decisions are current? | [`decisions/`](decisions/) |
| What runs in the current dependency order? | [`assurance/phase-1-dependency-map.md`](assurance/phase-1-dependency-map.md) and the repository test maps |

The release index is deliberately fail-closed. `BLOCKED`, `UNKNOWN`, a missing
tool, a skipped test, a zero-test run, or a simulator receipt does not become
green through documentation.

## Explanatory documents

- [`architecture/overview.md`](architecture/overview.md) is the concise system
  orientation.
- [`architecture/evolutionary-control.md`](architecture/evolutionary-control.md)
  defines roles, Variants, evidence-bound fitness, selection, fusion, budgets,
  and the V1 adaptation boundary.
- [`testing.md`](testing.md) maps test profiles, evidence classes, ownership,
  negative cases, and live-lane admission.
- [`runbooks/`](runbooks/) describes operator procedures; a runbook does not
  bypass an API or policy gate.
- [`assurance/`](assurance/) maps claims to code, schemas, and tests.
- [`assurance/v1-closure-plan.md`](assurance/v1-closure-plan.md) is the
  dependency-ordered implementation and proof map for the remaining V1 work.
- [`decisions/`](decisions/) records reviewed design choices and their status.
- [`spec/`](spec/) is historical Centerrail/Bullet Farm design provenance. It
  is useful context and never runtime or release authority.

## Executable local evidence

Run commands from the public hub checkout:

```bash
just fast
just contract
just check-family
just family-contract
just security
just audit
```

The meaning and limitations of those lanes are defined in
[`testing.md`](testing.md) and [`release.md`](release.md). Live-provider,
live-forge, package, signing, and
release evidence must use their separately admitted lanes and exact subjects;
the commands above do not substitute for them.

## Maintenance rule

When an implementation changes, update its generated contract or executable
test first. Change a status document in the same reviewed transaction that
adds the receipt supporting the new status. Preserve historical sources
verbatim when practical and refresh `spec/HISTORICAL_ARTIFACTS.sha256` whenever
a tracked historical Markdown source intentionally changes.
