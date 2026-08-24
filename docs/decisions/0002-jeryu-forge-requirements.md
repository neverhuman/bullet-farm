# 0002 — Jeryu as the Bullet Farm effect target: requirements and do-not-disturb rules

Status: Accepted (requirements list to be completed by the effects lane)
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: bullet-kernel (effects), bullet-farm (hub)

## Decision

The live effect target for demo-live is the running Jeryu forge at http://127.0.0.1:8787
(GitHub-compatible REST; `GH_HOST=127.0.0.1:8787` / `gh --hostname 127.0.0.1:8787`). A local bare
repository (`LocalBareForge`) backs all offline unit/fault tests behind the same `ForgeEffects` port.
GitHub (neverhuman/bullet-*) uses the same code path later.

## Do-not-disturb rules (hard)

Many repo families depend on this Jeryu instance. Therefore:
- NEVER stop, restart, upgrade, or reconfigure the `jeryu serve` process (pid-owned by the operator).
- NEVER edit ~/.jeryu/, ~/.local/share/jeryu/, its --split-manifest files, or any
  /home/ubuntu/jain-split/jeryu-split source as part of Bullet Farm work.
- Additive REST calls only: create root/bullet-* repositories, push branches, open PRs, post
  check-runs. Probe capabilities first; anything unsupported is recorded as Unsupported in the
  ForgeEffects descriptor — never worked around by touching the forge.
- The stored gh token for 127.0.0.1:8787 is INVALID; the operator must run
  `gh auth login -h 127.0.0.1:8787` before the live effects lane runs. Note `gh auth status`
  exits 2 because of this host — preflights must use `gh auth status --hostname github.com`.

## Feature requirements to verify against the running forge (fill in with probe results)

| Feature (git_role.md / spec §23.3) | Needed for | Jeryu status |
|---|---|---|
| Create repo via REST | registering root/bullet-* | TBD |
| Push branch `refs/heads/bullet/candidate/<id>` | Candidate export | TBD |
| Expected-old-OID push semantics (or --force-with-lease honored) | exact-subject delivery | TBD |
| Read ref back via REST | Effect receipt | TBD |
| Create/update PR idempotently | protected delivery | TBD |
| Check-run bound to exact SHA (`Bullet Farm / Proof Complete`, proof_root payload) | proof surface | TBD |
| Branch protection / required checks | protected integration | TBD |
| Merge queue / merge-group subject | Phase-later integration | TBD |
| Immutable annotated tags | family release pins | TBD |

Gaps become a proposal for a NEW Jeryu release rolled out to git.neverhuman.org (work happens in the
jeryu-split family, not here), so bullet users can create free accounts or self-host their own binaries.
