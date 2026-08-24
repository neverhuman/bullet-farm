# Fleet runbook

Status: Active
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: all bullet repos

Coordination is rooted at the outermost ancestor containing `repos.manifest.toml`; no public file
contains a host-specific absolute path. Human decisions remain append-only in
`<family-root>/AGENT_CHAT.md`. Machine claims are append-only in the ignored
`<family-root>/.bullet-family/coord/events.jsonl` ledger and are changed only through:

```bash
bullet-family coord claim --agent codex-a --lane L4 --repo bullet-kernel --path crates/runner
bullet-family coord heartbeat --claim clm_... --agent codex-a --note proof-started
bullet-family coord handoff --claim clm_... --agent codex-a \
  --proof 'cargo test --locked' --exit-code 0 --changed-path crates/runner/src/lib.rs
bullet-family coord status --json --all
```

The CLI takes an exclusive file lock across replay, overlap detection, and one append. Active claims
overlap when either repository-relative path contains the other on a segment boundary. Expired
claims stop blocking and cannot be revived; claim again. Handoff requires green proof and rejects
every changed path outside the claim. Use `--commit <oid>` when the orchestrator records a commit
receipt. Run a heartbeat at least every five minutes and on proof, blocker, commit, or handoff.

Only the orchestrator commits. Proof commands are per-repo
`bash scripts/ci-local.sh required` plus the lane-specific Cargo/npm commands. Zero Git worktrees,
zero pushes, zero new remotes.
Provider CLI execution is quarantined. `BULLET_LIVE_PROVIDERS`, provider OAuth state, and forge
tokens do not authorize a run. A later signed admission validator must bind the exact provider,
binary, profile, budget, policy, epoch, request, and expiry before any live lane can exist.
