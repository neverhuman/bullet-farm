# 0001 — Provider execution mode: read-only providers, kernel applies patches

Status: Accepted architecture; execution quarantined at Wave 0
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: bullet-kernel (harness-*, runner), bullet-git (bullet-gitd)

## Decision

Every provider turn runs READ-ONLY inside the private clone. The model proposes; the kernel writes.

The command lines below are compatibility evidence, not executable admission. Until the Wave-2
signed validator and sandbox are green, the shared argv boundary returns
`LIVE_ADMISSION_UNAVAILABLE` for every known provider executable. Environment flags, OAuth state,
and an opt-in test feature cannot bypass that denial.

- claude 2.1.241: `claude -p --permission-mode plan --output-format stream-json --json-schema <PatchProposal> --session-id <uuid> --max-budget-usd <cap>` (no --cwd: spawn with cwd; no --max-turns).
- codex 0.149.0: `codex exec -C <clone> --sandbox read-only --ask-for-approval never --json --output-schema patch-proposal.json -o <file> --skip-git-repo-check` (no --full-auto; -p = --profile).
- cursor-agent 2026.08.11: `cursor-agent -p --workspace <clone> --mode plan --output-format stream-json --trust`.
- agy 1.1.19: `agy --sandbox --mode plan --print-timeout 10m -p='<prompt>'` — flags **before** `-p=`.
  The 1.0.7 line `agy -p --sandbox …` is wrong on 1.1.19 (`-p` consumes `--sandbox` as the prompt).
  `--json-schema` exists; V1 still treats `structured_output_schema` as Unsupported and excludes
  agy from structured dispatch until the conformance ladder says `CONTRACT_PASS` for schema.
  Host OAuth state is not admitted by the Wave-0 runtime.

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
- Wave 2 replaces inherited host state with a provider-specific short-lived credential projection,
  private HOME/XDG/cache, an immutable binary digest, and policy egress.
- Wall-clock and invocation limits remain defense in depth. They are not admission, quota truth, or
  spend authorization; unknown quota abstains.

## Spec readings fixed by this ADR (so parallel lanes build compatible enums)

- Evidence tiers: E0–E4. Gate outcomes: the 11 values of spec §22.3.
- Routing tiers: D0/M1/M2/M3/M4 (M4 represented, unused in V1).
- Behavior catalog: spec §17 table is normative (rule IDs FS/GT/SC/TL/NW/SE/CD/TS/CP/CX/AG/EF/CL);
  the §17 action names are the EnforcementAction enum.
- SUPERSEDED is a normal terminal state for scope succession (§20.6), not only an exception.
