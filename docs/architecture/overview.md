# Architecture

Bullet Farm is a transaction processor for software changes and model cognition.

```text
Mission → immutable Plan Revision → Work Packages / Variants
  → incarnation-fenced Attempt → exact Candidate → Evidence
  → independent Review → brokered Effect → protected Integration
  → Observation Window → surviving Outcome
```

Three sovereign truths:

| Domain | Canonical system |
| --- | --- |
| Mutable operational authority | Kernel SQL ledger |
| Immutable engineering history | BulletGit |
| Remote integration | GitHub or Jeryu forge |

The portal is a projection. It is never an authority source.

See `docs/spec/` for historical Centerrail design provenance, the Gastown risk
audit, the BulletGit note, and the IEEE paper abstract. Those documents explain
the design lineage; generated contracts and the Kernel ledger are runtime
authority.
