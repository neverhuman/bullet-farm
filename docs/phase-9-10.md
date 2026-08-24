# Phase 9–10 — later, gated

Do not start these while any Wave A–D exit has a known counterexample (spec §34).

## Phase 9 — team distribution

Build only after the three Wave-4 transaction entrypoints emit independently verifiable signed
receipts and Wave 5 is green:

- PostgreSQL ledger (`bullet-adapters-postgres`; today `from_env()` is `NotConfigured` without `DATABASE_URL`)
- S3-compatible CAS
- mTLS runners and runner epochs
- RBAC/OIDC
- replicated farmd projections
- high-isolation verifier pools

Exit: partition/failover chaos creates no double authority or accepted stale Effect.

## Phase 10 — advanced optimization

Only after empirical proof from Phase 9:

- contextual routing learner (today `bullet-router` is deterministic D0)
- larger councils and code races
- integration batching / bisection
- cross-repository sagas
- S2/S3 sandbox expansion

Exit: statistically valid uplift, no guardrail regression, every adaptive decision explainable.

## Current scaffold

`crates/adapters-postgres` compiles in required CI and refuses to open a connection unless `DATABASE_URL` is set. That is the entire Phase 9 code until Wave A–D are green.
