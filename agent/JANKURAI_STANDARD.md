# Jankurai Standard Binding

Standard version: `0.9.0`
Target stack: Rust core, TypeScript/React/Vite product surface, SQLite local
truth (PostgreSQL later), generated contracts. No Python product truth.

Read the family copy of the operating standard before editing:

- `/home/ubuntu/jankurai-split/jankurai-standard/docs/agent-native-standard.md`
- `/home/ubuntu/jankurai-split/jankurai-standard/docs/audit-rubric.md`

Hard rules for this repository:

- Keep files small. Split before 500 LOC; prefer under 300.
- Do not hand-edit generated zones.
- Do not create Git worktrees.
- One-command setup and one-command validation must exist.
- Public API clients are generated from contracts, never handwritten DTOs.
- Authority never lives in the portal, README, or a provider session.
