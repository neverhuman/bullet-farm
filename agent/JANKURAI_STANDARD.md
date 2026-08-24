# Jankurai Standard Binding

Standard version: `0.9.0`
Target stack: Rust core, TypeScript/React/Vite product surface, SQLite local
truth (PostgreSQL later), generated contracts. No Python product truth.

The repository binding is this file plus `agent/standard-version.toml`,
`agent/owner-map.json`, `agent/test-map.json`, and
`agent/generated-zones.toml`. An installed auditor may expose its full
operating standard through `jankurai doctor`; never commit a machine-local
auditor checkout path as public project authority.

Hard rules for this repository:

- Keep files small. Split before 500 LOC; prefer under 300.
- Do not hand-edit generated zones.
- Do not create Git worktrees.
- One-command setup and one-command validation must exist.
- Public API clients are generated from contracts, never handwritten DTOs.
- Authority never lives in the portal, README, or a provider session.
