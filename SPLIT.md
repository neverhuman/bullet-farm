# bullet-farm

Status: first-build split-family hub
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: bullet-farm

## Role

Public hub, installer, family manifest, lock, fusion script, onboarding, and
Centerrail design corpus.

## Repositories

- Local authoritative repo: `root/bullet-farm`
- Public mirror target: `neverhuman/bullet-farm`
- Release tag pattern: `bullet-farm-v0.1.0-split.0`

## Split Rules

- Jeryu remains the local forge; GitHub is the public mirror.
- Release builds depend on immutable tags, not branches.
- Local development uses `scripts/fuse.sh` output under `.fusion/`.
- Committed manifests must not depend on sibling checkout paths.
- Generated outputs are regenerated from their source contracts or build commands.

## Required Local Check

```bash
bash scripts/ci-local.sh required
```
