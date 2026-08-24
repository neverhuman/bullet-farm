# 0001 — Provider execution mode: read-only providers, kernel applies patches

Status: Accepted
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: bullet-kernel (harness-*, runner), bullet-git (bullet-gitd)

## Decision

Every provider turn runs READ-ONLY inside the private clone. The model proposes; the kernel writes.

- claude 2.1.241: `claude -p --permission-mode plan --output-format stream-json --json-schema <PatchProposal> --session-id <uuid> --max-budget-usd <cap>` (no --cwd: spawn with cwd; no --max-turns).
- codex 0.149.0: `codex exec -C <clone> --sandbox read-only --ask-for-approval never --json --output-schema patch-proposal.json -o <file> --skip-git-repo-check` (no --full-auto; -p = --profile).
- cursor-agent 2026.08.11: `cursor-agent -p --workspace <clone> --mode plan --output-format stream-json --trust`.
- agy 1.0.7: `agy -p --sandbox --print-timeout 10m` — text only; `structured_output_schema = Unsupported`; excluded from structured dispatch.

The only accepted structured output is a `PatchProposal`
(bullet-kernel/contracts/schemas/patch-proposal.json): full-file contents per changed path
(create|modify|delete), plus tests_to_run, claims, uncertainties, done.

The runner validates every path against the granted Change Intent BEFORE mutation (W4/W5 in real
time), then calls `bullet-gitd apply_change` — the sole writer of the workspace — runs the declared
deterministic gate (E1; PASS only from a typed gate result), and resumes the provider session with
the results (claude --resume <uuid> / codex exec resume <id> / cursor-agent --resume <chatId>),
bounded by max_repair_loops = 2 and per-run invocation/time caps.

## Consequences

- The provider process needs no write access, sees no remote and no credential; its own sandbox
  flags are defence in depth, not the boundary. The spec's interactive Tool Gateway (§21) is not
  required for V1; app-server/ACP transports can add it later.
- Argv builders HARD-DENY -w/--worktree/--worktree-base/--tmux (claude, cursor create git worktrees;
  the family zero-new-worktree rule is absolute), with unit tests asserting the denial.
- Enclave env: inherit HOME plus ~/.claude, ~/.codex, ~/.cursor, ~/.gemini (subscription OAuth —
  API keys are not configured); strip GH_TOKEN, GITHUB_TOKEN, SSH_AUTH_SOCK, GIT_*; the private
  clone has no remote and no credential helper, so a model-issued `git push` cannot succeed.
- Spend bounds until a quota subsystem exists: wall-clock timeout, max-invocations-per-run,
  claude --max-budget-usd, BULLET_PROVIDER_KILL=1 kill switch.

## Spec readings fixed by this ADR (so parallel lanes build compatible enums)

- Evidence tiers: E0–E4. Gate outcomes: the 11 values of spec §22.3.
- Routing tiers: D0/M1/M2/M3/M4 (M4 represented, unused in V1).
- Behavior catalog: spec §17 table is normative (rule IDs FS/GT/SC/TL/NW/SE/CD/TS/CP/CX/AG/EF/CL);
  the §17 action names are the EnforcementAction enum.
- SUPERSEDED is a normal terminal state for scope succession (§20.6), not only an exception.
