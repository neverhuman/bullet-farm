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

## Contributor quick start

Prerequisites: Rust stable, Node 22+, `just`, Git, and the four ordinary sibling
checkouts in `repos.manifest.toml`.

```bash
cd /path/to/bullet/bullet-farm
cargo run --locked --quiet --bin bullet-family -- doctor --json
just fast
just demo
```

Install the repository-owned pre-push proof hook once per clone:

```bash
just hooks-install
```

The hook runs the same deterministic fast lane used by CI. `just ci-doctor`
reports missing local tools before a lane starts; `just ci-doctor audit`
additionally requires the exact locally admitted Jankurai 1.6.11 binary.

## Installation status

This alpha requires the four ordinary sibling checkouts listed in `repos.manifest.toml`. A hub-only
clone is diagnosis-only because the checked-in `family.lock` is the legacy schema-2 snapshot and
does not carry install authority. The Rust source-setup and `checkout verify` commands require a
signed schema-3 lock with an authenticated Jeryu URL/slug, exact commit/tree, signer,
dependency-lock digest, and generated-artifact manifest for every non-hub member. On Linux, setup
publishes through retained directory descriptors, rejects replaced/symlinked components, and uses
no-replace renames plus directory fsync; its signed fixture also proves a clean idempotent second
run. Until real release inputs are published, `doctor` reports `BLOCKED` and `just setup` returns
`UNSUPPORTED_SCHEMA` before creating member directories or running dependency tools. Do not create
Git worktrees or infer source locations from local paths. A future published lock is checked with
`bullet-family lock verify --tag <version>`; the current schema-2 lock cannot pass that check.

Accordingly, there is no trusted public install command yet. `scripts/setup.sh`
is a contributor source-bootstrap wrapper around the Rust setup mechanism; it
is not an authenticated installer and currently reaches the same schema-2
refusal. A release install starts only from a signed prebuilt `bullet-family`,
a verified schema-3 lock, and a verified five-platform release bundle.

On Linux, a local operator can verify an already-materialized bundle without
mutating it:

```bash
bullet-family release verify \
  --bundle /absolute/path/to/bundle \
  --allowed-signers /absolute/path/to/allowed_signers
```

That command verifies exact manifest, lock, payload, detached-signature, and
signer subjects. On Linux/glibc, the same verified component can safely extract
one exact signed target into a new, absent destination:

```bash
bullet-family release extract \
  --bundle /absolute/path/to/bundle \
  --allowed-signers /absolute/path/to/allowed_signers \
  --target x86_64-unknown-linux-gnu \
  --destination /absolute/path/to/new-directory
```

Extraction re-verifies the pinned archive subject, admits only the constrained
archive shape, fully syncs a staging tree, and publishes with a no-replace
rename. Neither command builds, downloads, produces, activates, or installs a
package, and neither supplies absent schema-3 release authority.

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
| Gate 0 contracts | Canonical v1alpha1 policy/schema bundle, hostile fixtures, invariant registry, and exactly two bounded models |
| Hub source setup | Descriptor-relative publication and a signed two-run local fixture are implemented on Linux; real schema-3 Jeryu lock/tag publication remains blocked |
| Release bundle components | Signed five-target integrity verification and constrained one-target extraction are implemented on Linux/glibc; package production, activation, semantic artifact validation, and installer smoke remain blocked |
| Signed launch grant | `SignedLaunchGrantV1` contract and Kernel verification with a single-use nonce (ADR 0011) are `COMPONENT_PROOF`; no grant has admitted a real provider |
| Provider egress isolation | Linux user+net namespace, nftables default-drop, and host CONNECT proxy (`bullet-harness-egress`; Kernel `egress` lane, neutral 78 without tools) are `COMPONENT_PROOF` on this host class only |
| Live-conformance refusal | `bullet provider live-conformance` refuses at step `POLICY` with exit 78 under the committed generation-1 policy; the thirteen-step path is exercised only against a fake provider process (component evidence); `LIVE_PROOF` is absent for every provider |
| farmd projections and Portal views | Fleet, sessions, merge rail, quality lab, and audit are read-only projections (`COMPONENT_PROOF`); a projection holds no authority and seven designed Portal surfaces remain `UNKNOWN` |
| Trusted public installer | Not available; no published schema-3 authority, signed package set, or activation transaction exists |
| Transaction-ready | Not yet achieved; requires the signed V1-S4 `TRANSACTION_PROOF` (the historical "Wave 4" offline receipt) |
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
./scripts/fuse.sh --source local
.fusion/dev.sh build
```

Jeryu is consumed through pinned tags from the operator-configured canonical `jeryu-split` family.
The family `AGENTS.md` policy identifies that checkout for local development; do not create a
substitute checkout or commit sibling path dependencies.

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
just contract-check # generated policy/schema/client byte drift
just model-check    # exactly two pinned TLC models and state locks
just contract       # hub-only canonical contract + model gate
just check-family   # hub plus every member required lane
just family-contract # family required lanes + canonical contract + models
just release-truth  # regenerate docs/assurance/release-truth.generated.md (decision exit 3 kept)
cargo run --locked --quiet --bin bullet-family -- check release --report   # 26-gate operator brief, exit 3 while BLOCKED
```

Gate 0 sources live under `policy/`, `contracts/v1alpha1/`, `fixtures/`, and `formal/`. Reviewed
prose is never runtime authority. The generated policy keeps live admission disabled, and Gate 0
does not substitute for the signed authority, API authentication, sandbox, vector-budget, freeze,
audit-anchor, and restore gates of the later V1-S stages (historically "Wave 2").

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
just coord heartbeat --claim clm_... --agent codex-a --note proof-started
just coord handoff --claim clm_... --agent codex-a --proof 'bash scripts/ci-local.sh required' \
  --exit-code 0 --changed-path docs/README.md
just coord receipt --claim clm_... --orchestrator codex-root --commit <40-hex> --committed-path docs/README.md
just coord receipt-group --claim clm_a... --claim clm_b... --orchestrator codex-root --commit <40-hex>
just coord correct-receipt --claim clm_... --orchestrator codex-root --previous-commit <40-hex> \
  --commit <40-hex> --committed-path docs/README.md --reason 'amended commit'
```

The coordinator keeps a locked append-only ledger at the discovered family root and rejects active
path overlaps before product edits begin. `handoff` requires green proof and refuses changed paths
outside the claim; `receipt`/`receipt-group` bind a claim to the exact path set of one commit;
`correct-receipt` rebinds an already-receipted claim to a replacement commit only when
`--previous-commit` equals the recorded OID and the new commit's actual paths equal
`--committed-path`, appending a reasoned correction record rather than rewriting history.

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
