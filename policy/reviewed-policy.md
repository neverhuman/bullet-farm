# Gate 0 policy adoption

Status: Accepted for v1alpha1 Gate 0
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: offline and simulator execution only

This is the sanitized human review surface for the canonical machine policy. It restates the
adopted decisions without preserving control sequences or treating source prose as executable
instructions.

- T0 is the universal incumbent and deterministic abstention target.
- R0/R1 may become eligible for bounded automation only after later certification and canary gates.
- R2 and above require an exact signed human approval artifact.
- Unknown quota is not headroom. Only a separately typed capped probe may measure capacity later.
- Evolution may select among certified recipes; it never selects authority, evidence, attestation,
  effects, integration, credentials, risk, or safety policy.
- Author evidence is not independent evidence. Unknown never satisfies a gate.
- Linux S1 is the production containment reference; arbitrary shell gates and live admission are
  disabled in this policy generation.
- The first deployment target is one self-hosted Linux host with SQLite and local Jeryu. GitHub is
  a separate later certification gate. Multi-tenant SaaS is outside scope.

The binding form is generated `policy/v1alpha1/policy.json`, whose schema-bundle and invariant
registry hashes are recomputed by `bullet-wire`. This prose cannot grant, widen, or waive authority.
