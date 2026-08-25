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

- `scripts/ci-local.sh fast|required|contract|security|audit` and the thin
  repository `just`/workflow wrappers are component merge lanes. A green local
  `required` proves the checks that repository currently implements; it is not
  product readiness.
- `bullet-family check fast|required|release` is the family/product gate
  runner. It executes only its sealed command catalog, adds immutable missing-
  receipt inventory, and stays nonzero while a required product receipt is
  absent. Today, component CI can be green while product `required` and
  `release` correctly remain `BLOCKED`.

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
There is intentionally no green no-op nightly. Platform packages may be built
for Linux x86_64/aarch64, macOS x86_64/arm64, and Windows x64, but non-Linux
mutation remains fail-closed until equivalent containment passes.

## Local verification order

From a clean canonical family checkout:

```bash
bullet-family checkout verify
bullet-family lock verify --tag <version>
bullet-family check fast
bullet-family check required
bullet-family check release
```

`release` is currently a diagnostic and must remain nonzero while required live,
package, signing, recovery, security, or platform receipts are absent. Current
subjects, receipt counts, and blockers are maintained in the
[V1 closure plan](assurance/v1-closure-plan.md) and [release index](release.md).
