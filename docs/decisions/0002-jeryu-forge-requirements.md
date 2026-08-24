# 0002 — Jeryu as the Bullet Farm effect target: requirements and do-not-disturb rules

Status: Accepted target; credentialed access quarantined at Wave 0
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: bullet-kernel (effects), bullet-farm (hub)

## Decision

Local Jeryu is the first future production forge after Waves 1–2. A local bare repository
(`LocalBareForge`) backs offline component and fault tests behind the same `ForgeEffects` port.
GitHub certifies separately later. Wave 0 performs no authenticated probe, repository creation,
push, check, pull request, or integration.

## Do-not-disturb rules (hard)

Many repo families depend on this Jeryu instance. Therefore:
- NEVER stop, restart, upgrade, or reconfigure the `jeryu serve` process (pid-owned by the operator).
- NEVER edit ~/.jeryu/, ~/.local/share/jeryu/, its --split-manifest files, or any
  /home/ubuntu/jain-split/jeryu-split source as part of Bullet Farm work.
- Additive REST calls only: create root/bullet-* repositories, push branches, open PRs, post
  check-runs. Probe capabilities first; anything unsupported is recorded as Unsupported in the
  ForgeEffects descriptor — never worked around by touching the forge.
- Existing host tokens are outside the trust boundary and do not authorize Bullet Farm. Re-login is
  not requested during quarantine.

## Feature requirements to verify against the running forge (fill in with probe results)

| Feature (git_role.md / spec §23.3) | Needed for | Jeryu status |
|---|---|---|
| Create repo via REST | registering root/bullet-* | UNPROBED — QUARANTINED |
| Push branch `refs/heads/bullet/candidate/<id>` | Candidate export | UNPROBED — QUARANTINED |
| Expected-old-OID push semantics (or --force-with-lease honored) | exact-subject delivery | UNPROBED — QUARANTINED |
| Read ref back via REST | Effect receipt | UNPROBED — QUARANTINED |
| Create/update PR idempotently | protected delivery | UNPROBED — QUARANTINED |
| Check-run bound to exact SHA (`Bullet Farm / Proof Complete`, proof_root payload) | proof surface | UNPROBED — QUARANTINED |
| Branch protection / required checks | protected integration | UNPROBED — QUARANTINED |
| Merge queue / merge-group subject | Phase-later integration | UNPROBED — QUARANTINED |
| Immutable annotated tags | family release pins | UNPROBED — QUARANTINED |

Gaps become a proposal for a NEW Jeryu release rolled out to git.neverhuman.org (work happens in the
jeryu-split family, not here), so bullet users can create free accounts or self-host their own binaries.

## Probe results (2026-08-24, unauthenticated GET)

The forge is healthy: `/` serves the SPA (200) and `/api/v3` answers with GitHub-style JSON
(`401 {"documentation_url":"/docs/rest","message":"Requires authentication"}`), as does the git
smart-HTTP endpoint (`/git/<org>/<repo>.git/info/refs` → 401). Every REST capability probe in the
table above would require authenticated admission. This historical observation is not a current
capability receipt. The effects lane remains on `LocalBareForge` and the table stays quarantined.
