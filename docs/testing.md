# Test and evidence strategy

Status: **normative; pre-release gaps remain**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-25

Tests establish bounded facts about exact subjects. They do not authorize a
provider, forge, installation, effect, or release merely because a process
exited successfully.

## Profiles

| Product profile | Purpose | Required behavior |
| --- | --- | --- |
| `fast` | warm developer feedback under 60 seconds | format, lint/unit, portal typecheck/unit/build, generated drift, affected-path routing |
| `required` | V1 acceptance inventory | locked component/family contracts plus registered install, Jankurai, packaged-browser, scan, recovery, and exact transaction evidence |
| `release` | exact tagged release | both required toolchains, packages, installer smoke, SBOM/signatures/provenance, recovery/fault suites, live Jeryu/GitHub, all four provider receipts |

`required` has no skip-green behavior. A missing required tool, zero tests,
ignored required test, timeout, unsupported result, flaky result, infrastructure
error, or `UNKNOWN` fails that profile. Missing credentials may produce a
neutral unregistered live lane only when that lane is not part of the requested
release profile.

There are two deliberately different command layers:

- `scripts/ci-local.sh <lane>` and the thin repository `just`/workflow
  wrappers are component merge lanes. A green local `required` proves the
  checks that repository currently implements; it is not product readiness.
- `bullet-family check fast|required|release` is the family/product gate
  runner. It executes only its sealed command catalog
  (`src/check/catalog.rs`: `fast` = 5 commands, the four member `fast` lanes
  plus `scripts/sync-family-contracts.sh check`; `required` = 2 commands,
  `ops/ci/family-contract.sh` and `scripts/demo.sh`; `release` executes
  nothing and inventories 26 gates), adds immutable missing-receipt inventory,
  and stays nonzero while a required product receipt is absent. Today,
  component CI can be green while product `required` and `release` correctly
  remain `BLOCKED`.

### Lanes

| Lane | Where | What it proves |
| --- | --- | --- |
| `fast`, `required`, `contract`, `audit` | every repository `scripts/ci-local.sh` | Component checks of that repository only |
| `security` | every repository `scripts/ci-local.sh security` | gitleaks (`--no-git --redact`) in all four repositories; in bullet-farm, bullet-kernel and bullet-git also `cargo deny --locked check licenses advisories bans sources` against that repository's committed `deny.toml`, preceded by a lane-side freshness proof of the RustSec advisory database (`ADVISORY_DB_ABSENT` / `ADVISORY_DB_UNREADABLE` / `ADVISORY_DB_STALE`, exit 1, limit 14 days); in bullet-portal also `npm audit --omit=dev`; `zizmor .` in all four. Component hygiene only: none of it registers a license, dependency, secret, or workflow scan receipt, and `release.scan.*` stays BLOCKED |
| `family` | hub only (`just check-family`) | Hub plus every member `required` lane from the sibling checkouts |
| `family-contract` | hub only (`just family-contract`; also `required.family-contract` in the sealed `check required` catalog) | Family `required` lanes plus the canonical contract and the two pinned models. The checked set is exactly: LeaseFence's three invariants, and EffectCheck's five invariants plus the `NoNewDispatchAfterStop` action property (hub `3728e798`) |
| `egress` | bullet-kernel `ops/ci/egress.sh` | Linux user+net namespace, `slirp4netns` uplink, in-namespace nftables default-drop, host CONNECT proxy; exits 78 (neutral) when a tool or unprivileged namespaces are absent, never green without running the probes |
| `nightly` | bullet-kernel `ops/ci/nightly.sh` | Per provider in `BULLET_LIVE_PROVIDERS`: the feature-gated refusal test plus the positive live-conformance half. Default marker mode points the positive half at a marker script, so any spawn under the committed policy fails the lane and `POLICY_LIVE_ADMISSION_DISABLED` (exit 78) is the expected neutral outcome. `BULLET_LIVE_REAL=1` with an absolute `BULLET_POLICY_PATH` is operator real mode: the resolved real binary is used and receipts are kept under `target/live/<provider>/<utc>/` |
| `toolchain-msrv`, `toolchain-pinned` | bullet-kernel and bullet-git `just toolchain-msrv` (Rust 1.95.0); hub `just toolchain-pinned` (Rust 1.97.1) | The second toolchain named by the [release contract](release.md). Each repository's `rust-toolchain.toml` and hosted CI pin exactly one toolchain (hub 1.95.0; Kernel and BulletGit 1.97.1), so these explicit local lanes are the only place the other required toolchain is exercised. Each runs the exact receipt argv and environment from `src/check/release_evidence/verify.rs` (`cargo build --workspace --all-targets --locked`, then `cargo test --workspace --all-targets --locked --no-fail-fast`, with `CARGO_INCREMENTAL=0`, `CARGO_NET_OFFLINE=true`, `RUSTC=<absolute rustc>`, `RUSTUP_TOOLCHAIN=<version>`) in an isolated `target/toolchain-<version>/`, then writes the ignored machine-local observation `.bullet-family/toolchain-<version>-<repository>.json` beside its two raw output logs. A missing rustup toolchain, `b3sum` 1.8.2, or `jq` is a typed refusal (exit 1), never a skip; a red build or test fails the lane after the observation is written. Compile and test only: no fmt, clippy, or contract step. The observation is an input for a future operator-signed `release.rust-msrv-1-95` / `release.rust-pinned-1-97-1` receipt and is never itself a receipt; both gates stay `BLOCKED` |
| `release-truth` | hub `just release-truth` (`scripts/release-truth.sh write`) | Renders `check release --report --portable` into `docs/assurance/release-truth.generated.md`; `scripts/release-truth.sh check` runs inside the hub `required` lane as a drift gate. The decision exit (3 = `BLOCKED`) is preserved; the page is a projection, not a receipt |

`nightly` and `egress` have no hub or family wrapper; run them from the Kernel
checkout. The toolchain lanes have no hosted job and are not in any `check`
catalog: they are operator/developer lanes that produce observations only. Neither produces `LIVE_PROOF` today: no provider has a live receipt.

A contract check generates into a temporary directory and diffs tracked
output; it does not repair drift while claiming to verify it. Neither command
layer may translate a skipped, missing, unsupported, flaky, infrastructure, or
`UNKNOWN` result into success.

The `release receipt-verify` component validates canonical receipt/policy
bytes, the exact policy digest, an admitted OpenSSH Ed25519 signer, signature
namespace, and validity interval. Its success message explicitly says
`contract only`; it does not provide trusted time, revocation/custody,
kind-specific semantic adjudication, registry/replay protection, or gate
registration. Those independent subjects must exist before a verified receipt
can clear any product gate.

## Evidence ownership

| Subject | Producer | Independent check |
| --- | --- | --- |
| Rust/TypeScript unit | owning repository | required profile with locked dependencies |
| Wire contract | DTO/schema generator | cross-repository golden JSON/hash vectors |
| Candidate | BulletGit | verifier clean reconstruction and proof-root recomputation |
| Provider contract | pure adapter transcript | frozen official/runtime schema fixture plus adversarial mutation suite |
| Provider live | admitted supervised runner | exact provider/version/profile/environment/canary receipt |
| Effect | effect broker | remote read-back and reconciliation |
| Portal | generated client and projection | built embedded portal against a real farmd |
| Package/install | release builder | separate verifier and fresh-host installer smoke |

Writer state, provider prose, portal color, HTTP 2xx, Git push success, or a
simulator cannot satisfy an independent gate.

## Mandatory suites

Every relevant boundary has deterministic negative coverage for:

- lease races, fence replay, expiry, supersession, and authority outage;
- crash injection at SQL, CAS, journal, and active-generation switch points;
- duplicate/conflicting paths, stale preimages, traversal, symlink/reparse and
  Unicode/case collisions, hostile Git configuration/filters, and unsafe cleanup;
- Candidate sensitivity for every bound field and proof invalidation after
  rebase or other subject change;
- provider malformed, duplicate, delayed, cancelled, timed-out, and oversized
  events, process-tree cleanup, and canary-secret absence;
- effect success with lost response, exact read-back adoption, and no second
  write;
- SSE gaps, failed snapshots, exclusive cursor replay, reconnect, retention
  gaps, malformed 200 responses, and command timeout reconciliation; and
- hub-only setup twice in a fresh home with exact clean OIDs and zero tracked
  changes.

The deterministic demo must use real child-process boundaries and a protected
local forge simulator. It proves only synthetic/component behavior until the
same exact-subject scenario is replayed through admitted production adapters.

## Provider conformance

Offline provider tests are pure transcript machines. They may bind a frozen
native message subset and reject everything else. They must not spawn a CLI,
read OAuth state, inherit ambient credentials, use the network, write a
workspace, convert free text to a proposal, or mark model output verified.
Enabling a `live` Cargo feature without signed admission must run a non-ignored
refusal test, not a hidden quota-spending smoke test.

Live conformance requires all of:

1. absolute canonical executable path, exact digest/version, and runtime probe;
2. signed short-lived authority binding provider, profile, policy, request,
   scope, budget/quota, environment, runner, Attempt, fence, and expiry;
3. ephemeral HOME and the minimum read-only provider credential material;
4. allowlisted environment, provider-only egress, no SCM/cloud/SSH secrets;
5. bounded JSONL frames, deadlines, cancellation agreement, and full process-
   tree termination;
6. schema-valid `PatchProposal` with exact admitted gate IDs; and
7. receipts proving canary host secrets are absent from environment, output,
   proposal, artifacts, and logs.

Each provider/version/profile certifies only itself. Claude, Codex, Cursor, and
Antigravity all need separate conformant live receipts for V1 GA.

### Implemented path

The positive live-conformance path is implemented in
`bullet-kernel/crates/application/src/live_conformance/` (`mod.rs`, `steps.rs`;
kernel `ba485d5`, loader mirror `0d848f6`) and driven by the operator entry
point `bullet provider live-conformance --data-dir <abs> --provider
{claude,codex,cursor,agy} [--executable <abs>]`. It records thirteen ordered
step statuses, never collapsed: `POLICY`, `OPERATOR_KEY`, `LEASE`, `ADMISSION`,
`MINT`, `VERIFY_GRANT`, `ADMIT_SIGNED`, `EGRESS_PREPARE`, `ADMIT_EGRESS`,
`REQUIRE_DISPATCH`, `DISPATCH`, `CANARY_SCAN`, `PONG_MATCH`; a step that did not
run is `NOT_RUN`. Exit 0 is a sealed `PONG` receipt; exit 78 is a neutral policy
refusal (`POLICY_LIVE_ADMISSION_DISABLED` under the committed generation-1
policy, before any key read, probe, namespace, or spawn); any other exit names
the failing step in the receipt. The policy step reads the production loader
(v1alpha1, or an operator-ratified v1alpha2 generation per ADR 0012 loaded from
`BULLET_POLICY_PATH`). The Kernel suite exercises the full path only against a
fake provider process; no `LIVE_PROOF` receipt exists for any provider. The
operator procedure is [`runbooks/live-conformance.md`](runbooks/live-conformance.md).

## Live forge and effect tests

Live tests never alter the running Jeryu service to make a test pass. They use a
separately authorized test repository, exact idempotency key, protected target,
read-back, and reconciliation. GitHub tests use an installed GitHub App with
least-privilege repository credentials; a personal token is not equivalent
release evidence.

A lost response after remote success records `UNKNOWN`. The test must prove
read-back adopts that exact effect and that the broker performs no duplicate
write. Jeryu is mandatory for local production proof; GitHub is the second
configurable effect adapter.

## CI ownership and portability

Jeryu CI executes the same repository-local commands through its workflow IR.
GitHub workflows are a portable mirror, not release evidence before a public
mirror exists. Actions and downloaded tools are pinned by immutable subject and
checksum. Thin shell/Just/workflow wrappers may dispatch; policy, lock,
generation, proof, and release decisions remain in Rust/TypeScript.

Nightly exists only for meaningful fuzz, soak, or admitted live-adapter work.
There is intentionally no green no-op nightly; the Kernel `nightly` lane above
fails on any provider spawn under the committed policy. Platform packages may
be built for Linux x86_64/aarch64, macOS x86_64/arm64, and Windows x64, but
non-Linux mutation remains fail-closed until equivalent containment passes.

## Local verification order

From a clean canonical family checkout:

```bash
bullet-family checkout verify
bullet-family lock verify --tag <version>
bullet-family check fast
bullet-family check required
bullet-family check release
bullet-family check release --report
```

`release` is currently a diagnostic and must remain nonzero while required live,
package, signing, recovery, security, or platform receipts are absent;
`--report` renders the same 26-gate decision as an operator brief and keeps
exit 3. Current
subjects, receipt counts, and blockers are maintained in the
[V1 closure plan](assurance/v1-closure-plan.md) and [release index](release.md).
