# bullet-farm

Status: pre-transaction split-family hub; not release-ready
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-25
Applies to: bullet-farm

## Role

Public hub, installer, family manifest and lock, fusion script, onboarding,
frozen wire contracts (`crates/bullet-wire`), canonical policy (`policy/`),
release verification (`bullet-family release verify|extract|receipt-verify`),
family coordination (`bullet-family coord`), the fail-closed release-truth
report (`bullet-family check release --report`, `just release-truth`), and
historical Centerrail design provenance.

## Repositories

- Initial source authority: Jeryu repository `root/bullet-farm`
- Public GitHub mirror: not configured; no namespace is assumed
- Release tag pattern: `bullet-farm-v0.1.0-split.0`

## Split Rules

- Jeryu is the initial source forge; GitHub is a configurable effect adapter,
  not source authority.
- Release builds depend on immutable tags, not branches.
- Local development uses `scripts/fuse.sh` output under `.fusion/`.
- Committed manifests must not depend on sibling checkout paths.
- Generated outputs are regenerated from their source contracts or build commands.

## Required Local Check

```bash
bash scripts/ci-local.sh required
```
