# Bullet Farm

**Many minds. One verified line to main.**

Bullet Farm is an open-source platform for ambitious software projects where
most work is broken into actions a weaker model can finish, and the hard work
(planning, deep R&D, fusion, review) uses frontier models — even several
providers at once — without ever letting a model grant itself authority.

It is a **transaction processor** for software changes, model cognition, and
consequential external effects. It is designed to replace the operational role
of [Gas Town](https://github.com/gastownhall/gastown) without inheriting its
authority model.

```text
Mission → immutable Plan → fenced Attempt → exact Candidate
  → independent Evidence → brokered Effect → protected Integration
  → observation → surviving Outcome
```

A model saying “done,” a terminal going idle, a process exiting zero, or a
pull request opening has **no completion authority**.

## 5-minute start

Prerequisites: Rust stable, Node 22+, `just`, Git.

```bash
git clone https://github.com/neverhuman/bullet-farm
cd bullet-farm
just setup
just demo
```

`just demo` runs a deterministic ledger simulator (no provider process, forge
credential, or network effect). It demonstrates component behavior only:

1. One Mission materializes once.
2. A released Variant receives a higher, never-reused fence on reacquisition.
3. A stale Attempt is refused.
4. Ambiguous effect execution remains `UNKNOWN`.

Receipts print to the terminal and are written under `bullet-kernel/target/demo/`.

Readiness is intentionally explicit:

| Surface | Current meaning |
| --- | --- |
| Component tests | Individual lease, workspace, verifier, broker, and portal primitives |
| `just demo` | Deterministic ledger simulation; not a five-plane transaction |
| `bullet demo-synthetic` | Simulator-only integration scaffolding with `transaction_gate_eligible=false` |
| Transaction-ready | Not yet achieved; requires the signed Wave-4 offline receipt |
| Production-ready | Not yet achieved; live providers and credentialed forges are quarantined |

Then start the local control plane and portal:

```bash
# terminal 1
just farmd

# terminal 2
just portal
```

Open http://127.0.0.1:5173. The Control Tower shows pending versus verified
mutations. `UNKNOWN` renders as unknown. It is never painted healthy.

## Why this exists

Models and harnesses commoditize. Orchestrators that infer meaning from tmux
names, worktrees, Beads rows, and “the agent said done” fail in the same ways:
false delivery, destroyed work, and green dashboards for questions they did not
resolve.

Bullet Farm keeps three non-overlapping truths:

| Domain | System |
| --- | --- |
| Leases, fences, commands, quota, unsettled effects | Kernel ledger |
| Change identity, Candidates, proof, lineage | BulletGit |
| Refs, pull requests, checks, merge queue | GitHub or Jeryu |

Gastown’s best product ideas stay — persistent work, provider plurality,
escalation, cost learning, a merge rail, operator visibility. They become
**portal views and policies**, not personas with write authority.

| Gas Town role | Bullet Farm |
| --- | --- |
| Mayor | Control Tower policy view |
| Witness | Audit and incident projection |
| Refinery | Merge Rail over exact Candidates |
| Polecat / worker | Fenced Attempt on a private clone |

## Repository family

This hub is the public clone and pin surface. Product source lives in four
repos, declared in `repos.manifest.toml` and pinned by `family.lock`.

| Repo | Role |
| --- | --- |
| `bullet-farm` | Hub, installer, onboarding, spec corpus |
| `bullet-kernel` | Domain, ledger, router, `bullet-farmd`, runner/verifier/effects bins |
| `bullet-git` | BulletGit capability API, journal, proof roots |
| `bullet-portal` | Vite + React operations portal |

Local fusion (the only place sibling path patches may appear):

```bash
./scripts/fuse.sh --source local --all
.fusion/dev.sh build
```

Jeryu is consumed through pinned tags. The only permitted Jeryu family is
`/home/ubuntu/jain-split/jeryu-split`. Do not recreate `~/jeryu-split`.

## What we will not claim

- 100% autonomy
- Zero regressions
- Exactly-once physical side effects across the network
- That a provider session is canonical memory
- That a GitHub App token enforces Bullet Farm fences

Uncertainty is explicit: observations are `VALUE`, `EMPTY`, `UNKNOWN`, or
`CONTRADICTORY`. Unknown quota is not capacity. A timeout is not proof of
non-execution.

## Proof

```bash
just fast          # hub onboarding checks
bash scripts/ci-local.sh required
```

There is deliberately no `demo-live` command. Later proof entrypoints are
`proof-transaction-offline`, `proof-transaction-jeryu`, and
`proof-transaction-github`; they are unavailable until their prerequisite
waves can emit independently verifiable signed receipts.

Member repos use the same Jankurai shape: `AGENTS.md`, owner-map, test-map,
generated zones, `ops/ci/*.sh`, and `just fast`.

Concurrent agents claim exact repository-relative paths through the Rust coordinator:

```bash
just coord status --json --all
just coord claim --agent codex-a --lane docs --repo bullet-farm --path docs
```

The coordinator keeps a locked append-only ledger at the discovered family root and rejects active
path overlaps before product edits begin.

Target stack: Rust control plane, TypeScript/React/Vite portal, SQLite WAL
locally (PostgreSQL in team mode later), generated contracts. No Python
product truth. No writable Git worktrees.

## Design corpus

- [Centerrail engineering spec](docs/spec/CENTERRAIL_FINAL_ADAPTIVE_MULTI_FRONTIER_ENGINEERING_SPEC.md)
- [Gastown risk audit](docs/spec/GASTOWN_OPEN_ISSUES_RISK_AUDIT_FOR_CENTERRAIL.md)
- [BulletGit / git role](docs/spec/git_role.md)
- [IEEE paper abstract](docs/spec/paper.md)
- [Architecture](docs/architecture/overview.md)

## License

Apache-2.0
