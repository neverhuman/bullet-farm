# Fleet runbook

Status: Active
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-25
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
bullet-family coord receipt --claim clm_... --orchestrator codex-root \
  --commit 0123456789abcdef0123456789abcdef01234567 \
  --committed-path crates/runner/src/lib.rs
bullet-family coord receipt-group --claim clm_a... --claim clm_b... \
  --orchestrator codex-root --commit 0123456789abcdef0123456789abcdef01234567
bullet-family coord status --json --all
```

The CLI takes an exclusive file lock across replay, overlap detection, and one append. Active claims
overlap when either repository-relative path contains the other on a segment boundary. Expired
claims stop blocking and cannot be revived; claim again. Handoff requires green proof and rejects
every changed path outside the claim. After an exact-path commit, the orchestrator records either a
single-claim `receipt` with every committed path or a `receipt-group` whose handed-off path union
exactly matches the commit. Run a heartbeat at least every five minutes and on proof, blocker,
commit, or handoff.

Only the orchestrator commits. Proof commands are per-repo
`bash scripts/ci-local.sh required` plus the lane-specific Cargo/npm commands. Zero Git worktrees,
zero pushes, zero new remotes.
Provider CLI execution is quarantined. `BULLET_LIVE_PROVIDERS`, provider OAuth state, and forge
tokens do not authorize a run. A signed launch-grant validator already exists (ADR 0011). Live
dispatch stays policy-disabled: committed policy is v1alpha1 / generation 1 /
`live_admission_enabled=false`. ADR 0012 ratification plus a Kernel loader mirror are operator
acts, not environment variables.
