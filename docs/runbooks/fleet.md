# Fleet runbook

Status: Active
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: all bullet repos

Coordination file: /home/ubuntu/bullet/AGENT_CHAT.md (append-only; rules in its header).
Lanes and paths: see the lane table there. One agent per lane; lanes are disjoint path sets; only
the orchestrator commits. Proof commands are per-repo `bash scripts/ci-local.sh required` plus the
lane-specific cargo/npm commands. Zero git worktrees, zero pushes, zero new remotes.
Provider CLI testing: live lane is opt-in via BULLET_LIVE_PROVIDERS; every live run logs a receipt
(provider, version, session id, usage/cost, wall time, exit code) to AGENT_CHAT.md.
