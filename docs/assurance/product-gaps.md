# Product gap register

Status: **operator index; not runtime or release authority**  
Last reviewed: 2026-08-25  
Owner: Bullet Farm maintainers

This page answers “what is still missing before Bullet Farm is a product?”
It does not make a gate green. Authoritative status remains
[`release.md`](../release.md) and the receipt rows in
[`v1-closure-plan.md`](v1-closure-plan.md). A newer commit invalidates a row
until those documents are replayed.

There is no remaining *undocumented* V1 product gap: every blocked capability
below already has an owner document, a fail-closed checker, and a typed
refusal. The remaining work is implementation, operator ratification, and
receipts — not missing prose. Closing a row in this file is not closing the
product.

## How to read a row

| Field | Meaning |
| --- | --- |
| Gap | The product capability an operator still cannot honestly claim |
| Why it is still open | The exact missing subject, not a vibe |
| What already exists | Component proof that must not be promoted |
| Closer | Who or what can close it |
| Authority | Document that may flip the status |

`bullet-family check release --json` is the executable form of the release
blockers. If this page and that command disagree, the command wins.

## What an agent may close versus what it may not

Gaps are closed only by a receipt. Prose cannot close G1–G15. Agents may close
a gap only when every closer in the row is code or a mapped test **and** no
operator secret, signer, or policy generation flip is required.

| Class | IDs | Agent action |
| --- | --- | --- |
| Operator-blocked | G1, G5, G6, G7 | Document the exact act. Do not invent a lock, flip `live_admission_enabled`, or patch Jeryu/GitHub to look green. |
| Engineering, predecessor-blocked | G2, G3, G4, G9, G12, G13, G14, G15 | Implement only behind an unexpired `coord claim`. A component receipt does not clear the family gate. |
| Quality / platform | G8, G10 | Reduce hard findings; add a native backend. A local Jankurai binary is not CI evidence. |
| Explicitly post-V1 | G11 | Leave `evolutionary_authority=false`. Do not start a campaign. |

A gap that is *fully specified, fail-closed, and indexed* is a **documented
open gap**, not a missing product definition. That is the only sense in which
documentation can “close” G1–G15 today.

## Remaining V1 product gaps

| ID | Gap | Why it is still open | What already exists | Closer | Authority |
| --- | --- | --- | --- | --- | --- |
| G1 | Hub-only signed install | Checked-in lock is schema 2 and is refused on purpose. The source wrapper launches Cargo before admission, Git remains path-based, and no production Jeryu/validator two-run, package lifecycle, or signed prebuilt exists | Descriptor-relative setup, no-replace publish, sealed Linux Cargo/Node/Bash/npm subjects, two-run component fixture | Operator publishes schema-3 lock + signed prebuilt `bullet-family`; engineering pins/isolate Git and replays production setup | [`release.md`](../release.md), [`runbooks/source-setup.md`](../runbooks/source-setup.md) |
| G2 | Connected five-plane transaction | No signed `TRANSACTION_PROOF` covering crash, salvage, verify, ambiguous effect, integration, portal truth | Atomic lease/command/outbox; fail-closed `bullet-gitd`; fixture E2; Portal `PENDING→UNKNOWN` | Kernel + BulletGit + Portal owners land one exact offline saga | [`v1-closure-plan.md`](v1-closure-plan.md) V1-S4 |
| G3 | Production Kernel write path | Signed full-subject lease transport, durable reservation, JSON-RPC, provider/effect dispatch, CAS/GC, production restore remain | DB-clock leases; authenticated ingress; UNKNOWN/FAILED worker; launch-grant + Linux egress components | Kernel V1-S2/S4; do not remount unauthenticated `HttpLeaseClient` | [`release.md`](../release.md), [ADR 0011](../decisions/0011-signed-launch-grant-and-egress-isolation.md) |
| G4 | Production BulletGit write path | Public `clone` still returns `AUTHORITY_CONTRACT_UNAVAILABLE`; no published immutable `bullet-wire` tag | Dissociate clone, hostile-git, generations, preservation, honest cleanup UNKNOWN | Operator publishes wire tag; Kernel online reservation/settlement | [`v1-closure-plan.md`](v1-closure-plan.md) V1-S3 |
| G5 | Live provider conformance | Committed policy is v1alpha1 / generation 1 / `live_admission_enabled=false`. Kernel snapshot loader still accepts v1alpha1 only | Four offline parsers; signed launch grant; live-tested Linux egress probes; ADR 0012 validator | Operator ratifies v1alpha2 + runner key; kernel lane mirrors the loader; then one-turn canaries | [ADR 0012](../decisions/0012-policy-v1alpha2-live-admission.md) |
| G6 | Jeryu live effect | No authenticated read-back/reconciliation receipt | Local bare-forge component; Jeryu adapter is typed quarantine | Operator restores scoped Jeryu auth on an unmodified forge | [`release.md`](../release.md) |
| G7 | GitHub live effect | No App-test-repo integration receipt | Effect adapter is specified, not certified | Operator configures a GitHub App test repository | [`release.md`](../release.md), [ADR 0002](../decisions/0002-jeryu-forge-requirements.md), [0008](../decisions/0008-forge-gates.md) |
| G8 | Security release floor | Hub Jankurai 58 (raw 58), 10 caps, 29 hard findings; no portable CI artifact | Pinned local scan that fails closed | Hard findings to zero; score ≥90; checksum-pinned CI binary | [`release.md`](../release.md) |
| G9 | Signed five-target release | No package builder, SBOM/provenance, protected signing, installer smoke | Linux verify + safe extract of an already-signed archive; Portal bundle manifest | Release engineering from tagged bytes | [`release.md`](../release.md), [ADR 0010](../decisions/0010-supply-chain-policy.md) |
| G10 | Non-Linux containment | Mutation on macOS/Windows fails until a native backend passes | Linux is the strong-isolation reference | Platform owners | [ADR 0007](../decisions/0007-sandbox-secret-taint.md) |
| G11 | Evolutionary runtime | Recipes, archive, promotion are design-only | [`evolutionary-control.md`](../architecture/evolutionary-control.md); policy `evolutionary_authority=false` | After G2; never by flipping the policy bit first | [`phase-9-10.md`](../phase-9-10.md) |
| G12 | Family `check release` | 26/26 gates `BLOCKED`; the generated truth report records 0/26 receipts | Read-only negative inventory plus deterministic `--report` projection; no skip-green nightly | Receipts for G1–G10 | `bullet-family check release --json`; [`release-truth.generated.md`](release-truth.generated.md) |
| G13 | Portal product surfaces | Eleven of fifteen spec surfaces have no farmd projection and stay explicit UNKNOWN; Portal is not packaged/embedded | Control Tower, Mission Graph, Live Attempt, Incidents; CSRF/202; `PENDING→UNKNOWN`; SSE STALE | Portal + farmd owners after G2/G3 expose the remaining projections | Portal architecture; V1-S5 |
| G14 | farmd production API | Public surface is the authenticated command/snapshot/SSE subset, not the designed ~80-route control plane | Loopback origin, no-wildcard CORS, command 202, ready/outbox/missions snapshots | Kernel API after signed dispatch exists | V1-S5 |
| G15 | Cognitive persistence | CognitiveTask / SelectionGroup / Role / Fusion are IDs or design, not durable scheduler objects | Wire shapes; routing provenance records; offline provider parsers | Kernel V1-S6 after G2 | [`evolutionary-control.md`](../architecture/evolutionary-control.md) |

## The 26 `check release` gates

These IDs are the static negative inventory in `src/check/prerequisites.rs`.
Every row is `BLOCKED`. A green component crate cannot clear any of them.

| Gate ID | Product gap | Class |
| --- | --- | --- |
| `release.installable-lock` | G1 | Release |
| `release.installer-twice` | G1 | Release |
| `release.transaction-demo` | G2 | Transaction |
| `release.fault-suite` | G2, G3 | Release |
| `release.backup-restore` | G3, G9 | Release |
| `release.provider.claude` | G5 | Live |
| `release.provider.codex` | G5 | Live |
| `release.provider.cursor` | G5 | Live |
| `release.provider.antigravity` | G5 | Live |
| `release.forge.jeryu` | G6 | Live |
| `release.forge.github-app` | G7 | Live |
| `release.jankurai-90` | G8 | Release |
| `release.scan.dependency` | G8, G9 | Release |
| `release.scan.license` | G8, G9 | Release |
| `release.scan.secret` | G8, G9 | Release |
| `release.scan.workflow` | G8, G9 | Release |
| `release.checksums` | G9 | Release |
| `release.manifest-non-circular` | G9 | Release |
| `release.package-matrix` | G9 | Release |
| `release.provenance` | G9 | Release |
| `release.receipt-contracts` | G9 | Release |
| `release.rust-msrv-1-95` | G9 | Release |
| `release.rust-pinned-1-97-1` | G9 | Release |
| `release.sbom` | G9 | Release |
| `release.signatures` | G9 | Release |
| `release.platform-containment` | G10 | Release |

G4, G11, G13, G14, and G15 are product gaps that are not themselves a named
`release.*` id. They still block G2 and therefore `release.transaction-demo`.
G12 is the inventory of this table.

## How to verify each gap is still open

From the hub checkout:

```bash
bullet-family doctor --json          # G1: BLOCKED / UNSUPPORTED_SCHEMA is honest
bullet-family check release --json   # G12: 26/26 BLOCKED until receipts exist
just fast && just contract           # component lanes; never G2–G15
```

Do not convert a green `just fast` into a closed G-row.

## Which command proves what

| Command | Proves | Does not prove |
| --- | --- | --- |
| `just fast` | Mapped component lanes on this checkout | G2–G15, live, install, release |
| `just contract` | Generated wire/schema identity | A running issuer or published tag |
| `bullet-family doctor --json` | Honest refusal of schema-2 hub-only install | That schema-3 exists |
| `bullet-family check release --json` | The 26-gate negative inventory | Any gate is green |
| Archived 2026-08-24 live demo | That one past tree spawned under then-policy | HEAD conformance |

`check required` adds six more static blockers (`required.installable-lock`, `required.jankurai-ratchet`, `required.packaged-browser-e2e`, `required.pinned-scans`, `required.recovery-faults`, `required.transaction-proof`). They are the same gaps, not a second product list.

## One-hop operator answers

| Question | Answer |
| --- | --- |
| How do I install from a hub-only clone? | You cannot, honestly. Schema 2 is refused. Contributor bootstrap is [`runbooks/source-setup.md`](../runbooks/source-setup.md); signed install is G1. |
| Can I turn on live Claude/Codex/Cursor/Antigravity? | Not from this tree. Ratify ADR 0012 with a real runner key (G5). The v1alpha2 validator is not a live policy. |
| Why does `doctor` fail? | The checked-in lock is schema 2. That refusal is the product. |
| Did the white paper close the product? | No. The paper inventories G1–G15. Closing prose is not a receipt. |
| Is `just fast` enough to ship? | No. It is a component lane. `check release` stays 26/26 `BLOCKED`. |
| What is the same-UID install hole? | The Rust boundary now seals Cargo/Node/Bash/npm bytes. G1 still includes the source wrapper's pre-admission Cargo launch and path-based Git; a signed prebuilt plus pinned/isolated Git closes those surfaces. |
| Does ADR 0012 mean the kernel loads v1alpha2? | No. The Kernel snapshot loader still accepts v1alpha1 only. |
| Where is `docs/INDEX.md`? | It must not exist. This family's index is [`../README.md`](../README.md). |

## Slice leftovers (V1-S0..S8)

This is the same work as G1–G15, indexed by the closure-plan slices so an
implementer cannot “lose” a leftover by reading only the G-table.

| Slice | Status class | Leftover that still blocks a product claim |
| --- | --- | --- |
| V1-S0 | Local complete; release continuous | Exact-path commits and claim receipts remain an orchestrator obligation |
| V1-S1 | LOCAL-BLOCKED | Immutable published `bullet-wire` tag; consumers still carry duplicate or legacy semantics; production JSON-RPC hello/version/frame contract |
| V1-S2 | LOCAL-BLOCKED | Normalized full truth, signed capabilities, CAS/GC, production restore admission, fault-complete recovery |
| V1-S3 | LOCAL-BLOCKED | Positive online authority; complete Candidate/Integration manifests; reviewed tagged `jeryu-gitd` |
| V1-S4 | LOCAL-BLOCKED | Signed internal lease transport; runner/verifier/effect saga; credential-free `TRANSACTION_PROOF` |
| V1-S5 | LOCAL-BLOCKED | APPLIED/VERIFIED dispatch; remaining eleven Portal surfaces; packaged farmd-served Portal |
| V1-S6 | LOCAL-BLOCKED | Durable cognitive objects; four-provider live conformance; routing/fusion replay from persisted inputs |
| V1-S7 | LOCAL-BLOCKED | Schema-3 lock, prebuilt installer, five archives, SBOM/provenance, hosted Jankurai artifact, docs that wait on typed commands |
| V1-S8 | EXTERNAL-BLOCKED | Operator-issued Jeryu, GitHub App, provider profiles, platform, signer receipts from tagged bytes |

V1 is done only when every `V1-S0..S8` gate has a current independently
verifiable receipt from the same signed subjects. “Agents ran” is not that
receipt.

## Operator decisions that are not code

These are the only remaining non-code closers. Agents must not flip them.

1. **Live admission.** Ratify [ADR 0012](../decisions/0012-policy-v1alpha2-live-admission.md): policy generation ≥2, a registered `provider-runner` authority-signing key, `live_admission_enabled=true`, `evolutionary_authority` still false. Record the act in `AGENT_CHAT.md`.
2. **Jeryu test authority.** Authenticate to the local forge and name a scoped test repository for read-back. Do not patch the forge to paper over a missing capability.
3. **Schema-3 lock inputs.** Authenticated Jeryu URL/slug, signer identity, and exact member subjects for a hub-only install.
4. **GitHub App test repository.** Branch protection, separate delivery vs attestation credentials, and a repo the family may mutate.

## Centerrail C1–C12: product status

The family `TEAM.md` red-team is historical provenance. Living control is
[`evolutionary-control.md`](../architecture/evolutionary-control.md) plus this
register. Disposition of each critique:

| ID | Adopted meaning | Product status |
| --- | --- | --- |
| C1 | Every invariant is T1 schema / T2 gateway / T3 test | Registry exists; most later-wave rows remain `planned` |
| C2 | Oracle-modifying diffs + required holdouts for R2+ | Designed; implemented verifier is one fixture E2 |
| C3 | Two-track scope expansion ([ADR 0004](../decisions/0004-scope-amendment-tracks.md)) | Accepted decision; not a live Attempt path |
| C4 | Attestor ≠ broker; reconstructible check from proof bundle | Designed; live forge blocked |
| C5 | `CONTRADICTORY` / prolonged `UNKNOWN` has fence-mediated exits | Designed |
| C6 | Bounded probe reservation; `unknown` is never headroom | Designed |
| C6b | One seat-equivalent per named human | Designed; not enforced in the kernel |
| C7 | Formal-model exactly two protocols in Phase 0 | Adopted and component-complete |
| C8 | GA = kernel + any two certified providers | Adopted as schedule rule; live still operator-gated |
| C9 | Identity-exact effect adoption (fence + desired OID) | Command idempotency component; graph mint not a live path |
| C10 | Verifier dwell is writer-admission backpressure | Designed |
| C11 | Freeze chip shows recorded vs enforced-on-N/M runners | Portal honesty component; freeze countdown designed |
| C12 | Multi-repo saga quarantines blast radius, not the fleet | Out of V1; [`phase-9-10.md`](../phase-9-10.md) |

Rejected critiques stay rejected: do not drop the verifier plane, do not
mandate Postgres for V1, do not collapse the five planes, do not replace
forge sovereignty with an internal merge queue.

## Documentation that is closed (do not reopen as a gap)

These used to look like missing product definition. They are defined and
fail-closed.

| Topic | Where it is closed |
| --- | --- |
| Public name, five planes, providers-propose | [`architecture/overview.md`](../architecture/overview.md), [ADR 0001](../decisions/0001-provider-execution-mode.md), [0003](../decisions/0003-five-trust-planes.md) |
| Competitor pins (Gas Town, Gas City, DeepSeek, Omnigent) | [`competitor-snapshot.md`](competitor-snapshot.md) |
| IEEE preprint source | [`../paper/`](../paper/) |
| What “evolutionary” may mean in V1 | [`evolutionary-control.md`](../architecture/evolutionary-control.md) |
| Evidence classes and skip-green ban | [`testing.md`](../testing.md) |
| C1–C12 / TEAM.md red-team | This page + paper Section “Red-Team Disposition”; `TEAM.md` is provenance |
| Mascot / brand briefs | [`../brand/mascots/`](../brand/mascots/) |
| Family-root `/docs/*.md` copies | Not authority. Hub `bullet-farm/docs/` wins |

## Documentation that must wait on typed commands

These are not missing definitions. Writing them now would invent a CLI that
does not exist. They become runbook work after the named command is real.

| Deferred doc | Blocked on |
| --- | --- |
| Upgrade / rollback / uninstall runbook | Signed prebuilt installer (G1, G9) |
| Signer rotation and schema-removal runbook | Release signing keys (G9) |
| SAFE_STOPPED / freeze-enforced operator card | Runner ack generation (C11, G3) |
| Effect-reconciliation operator card | Identity-exact adoption path (C9, G2) |
| Platform-refusal after mutation attempt | Native backends (G10) |
| Live-admission operator recipe | ADR 0012 ratification (G5) |
| Jeryu restore / read-back recipe | Operator forge auth (G6) |
| GitHub App test-repo recipe | Operator App + protected repo (G7) |
| Launch-grant keygen recipe | ADR 0011 + operator key (G5) |
| Jankurai CI pin admission | Portable checksum-pinned binary (G8) |
| `bullet-wire` / `jeryu-gitd` publication | Operator tags (G4) |
| Generated `check release` report as a committed score | Forbidden; the command wins |

## What this page will never say

- That a green `just fast` or a green component crate is a release.
- That the archived 2026-08-24 live demo certifies HEAD.
- That Bullet Farm is measured faster, cheaper, or safer than Gas Town,
  DeepSeek Harness, or Omnigent.
- That an agent may enable live admission or invent a schema-3 lock.
- That finishing the white paper, or this register, closed G1–G15.
