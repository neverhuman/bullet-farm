# Bullet Farm V1 closure plan

Status: **ACTIVE — release authority remains blocked**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-24  
Applies to: the four-repository, local-first, single-user V1

This is the dependency-ordered bridge from the red-teamed Bullet Farm design to
release evidence. It is a status and execution map, not an authority source.
Only generated contracts, the Kernel ledger, exact repository subjects, and
independently verifiable receipts can make a row green.

## Frozen V1 decisions

- The public name is **Bullet Farm**. Centerrail is historical design
  provenance.
- The product is a transaction processor for software changes and model
  cognition, not a collection of autonomous personas.
- Kernel owns mutable authority. BulletGit owns immutable change subjects.
  Providers are read-only proposal producers. Verifiers do not trust writer
  state. Jeryu/GitHub are effect targets, not lease authorities. The Portal is
  a sequence-bound projection.
- Source distribution is Jeryu-only initially. GitHub remains a required
  configurable effect adapter.
- Local V1 is single-user and loopback-only. Team distribution, PostgreSQL,
  remote runners, cross-repository sagas, and online-learning routing remain
  post-V1.
- Exactly two protocols are model-checked before property tests: lease/fence/
  reclaim and command/effect ambiguity.
- The frozen release plan requires conformant Claude, Codex, Cursor, and
  Antigravity adapters. Historical `TEAM.md` critique C8 recommended GA with
  any two. That recommendation is superseded for this V1 unless a later
  reviewed decision explicitly changes the release contract.

## Evidence already present

| Slice | Exact receipt | What it proves | What it does not prove |
| --- | --- | --- | --- |
| Signed wire authority | Hub `c07efb10639d500c3e82ccc282265090ff63a4aa` | Strict authority DTOs, canonical encodings, signatures, typed settlement, golden vectors | A running issuer or online authority service |
| Authority/provenance docs | Hub `4f3d028c261ba961f8f9a3511039690941b881fc` | Historical sources are separated from product authority | Runtime enforcement |
| Installer tool admission | Hub `68d0fb92b52df8d4631ac346428f341f0bb492bc` | Canonical Cargo/Node/npm/Bash subjects, cleared child environment, ephemeral setup state, fail-before-clone tests | Authenticated release bootstrap, Git/helper fingerprints, or signed packages |
| SQLite lease check | Kernel `b6e0a338dcbd8aeecaf4ac2952280451151e3d0c` | Coherent ten-field active-lease check with database time | The complete production ledger/recovery contract |
| BulletGit fail-closed gateway | BulletGit `79bf1e2129fbe50ed85d424fe6e4416407bb17f4` | Production clone refuses unavailable authority before I/O; replay ledger is durable and bound | A positive production authority path or Jeryu backend |
| Portal cursor truth | Portal `5aa21708c808e6afd39537e9f5148679a0ce674b` | Gaps remain STALE until a covering snapshot; ambiguous mutation is UNKNOWN | Command-ledger reconciliation or authenticated production API |
| Frozen generated consumers | Kernel `76a47e9d5c3a61c03d0585af99d134b3190f94ee`, BulletGit `7df926c16be3e3c42c3fffe3fc74f57aabceeb41`, Portal `1f6dfb9cc22ebb2402f1c9e56b5499b3dc496724` | Consumer schema bundles are byte-identical to the hub | Runtime compatibility outside mapped contract tests |

Component or synthetic proof never upgrades itself to transaction, live, or
release proof. The current family gate is red until the Kernel runner tests no
longer depend on the removed BulletGit production-success scaffold.

## Audited baseline gaps

The 2026-08-24 read-only audit at hub `68d0fb92`, Kernel `76a47e9d`,
BulletGit `7df926c1`, and Portal `1f6dfb9c` observed:

- `family-contract` failed with 240/246 Kernel tests passing. The six failures
  were three runner repair-loop cases, heartbeat stale, kill/retry, and the
  explicitly synthetic E2E. All reached the intended production
  `AUTHORITY_CONTRACT_UNAVAILABLE` refusal but still expected the deleted
  success scaffold.
- No Bullet repository has a configured remote or immutable current HEAD tag.
  The checked-in lock is the schema-2 alpha.4 diagnostic snapshot and carries
  no authenticated Jeryu URLs/slugs.
- SQLite has useful WAL, migration, lease, command, outbox, effect, and event
  components, but migration checksums, foreign-key activation, normalized
  Candidate/Evidence truth, database-owned lease time, CAS, backup/restore,
  retention, and audit anchoring are absent.
- Runner/verifier/effect protocols are not the required negotiated JSON-RPC
  2.0 boundaries. Model-supplied `tests_to_run`/shell gate commands still exist
  instead of admitted gate IDs. `just demo` still writes synthetic SHA/PASS
  receipts and is not transaction evidence.
- ADR 0001 still documents deferred Codex App Server and Cursor ACP adoption.
  That conflicts with the frozen adapter contract and must be corrected in the
  same reviewed change that lands the conformant implementations.
- The API has no command ledger endpoints, browser bootstrap/session/CSRF, or
  snapshot body envelope. The Portal client/validators are handwritten, most
  operational surfaces remain honest UNKNOWN, and the real-farmd test uses a
  Vite development server rather than an embedded production build.
- Required CI omits the transaction demo, real built Portal E2E, security,
  audit, exact release lock, workflow/license scans, packages, and recovery.
  Current Jankurai score/cap observations are hub 54/11, Kernel 59/16,
  BulletGit 58/10, and Portal 60/11; release requires at least 90 and zero
  caps/hard findings in every repository.

These are internal engineering gaps. Jeryu authentication/tagging, the GitHub
App test repository, provider credentials, and platform signing credentials are
separate external prerequisites and cannot excuse missing internal controls.

## Dependency-ordered critical path

No slice starts while its predecessor has a known safety counterexample.

### 0. Coordination and provenance

Goal: make concurrent engineering auditable before adding more concurrency.

Required work:

1. Use `bullet-family coord claim|heartbeat|handoff|status` for every edit.
2. Keep claims repository-relative, exact, non-overlapping, and short-lived.
3. Commit only handed-off changed paths; bind every commit to its claim receipt.
4. Append coordination/proof/blocker events to the family chat without
   rewriting historical entries.
5. Preserve mixed-provenance history through corrective commits; never hide it
   with a reset or broad staging operation.

Exit: a concurrent overlap test has one winner, corrupt/truncated coordination
records fail closed, and every release commit has an exact claim receipt.

### 1. Frozen cross-repository contracts

Goal: one vocabulary and one byte representation at every trust boundary.

Required work:

1. Keep `bullet-wire` pure Rust and free of repository/runtime dependencies.
2. Publish it from an immutable signed Jeryu tag. Development overrides may be
   generated under ignored `.fusion`; committed sibling paths are forbidden.
3. Remove remaining duplicate Candidate, digest, Evidence, effect, and state
   semantics from Kernel and BulletGit.
4. Generate OpenAPI, JSON Schema, Rust consumers, TypeScript types/client, and
   AJV runtime validators from the same DTO sources into a temporary directory;
   diff without altering tracked files.
5. Reject legacy demo databases with `UNSUPPORTED_SCHEMA` and explicit export/
   removal guidance.

Exit commands:

```bash
just contract
just family-contract
```

Required negative evidence: cross-repository golden JSON/hash mutation for
every bound field, unknown-field refusal, unsupported-schema refusal, and zero
generated drift.

### 2. Kernel authority, commands, and durable recovery

Goal: the Kernel ledger is the only mutable operational authority.

Required work:

1. Finish ordered checksummed migrations and normalized immutable identities,
   transitions, graph revisions, Candidates, Evidence, commands, leases,
   events, projections, effects, and outbox rows. Remove JSON-blob truth and
   unsafe destructive upgrade behavior.
2. Allocate lease plus never-reused fence in one SQLite `BEGIN IMMEDIATE`
   transaction using database time. Restart, expiry, supersession, and restore
   must not reuse a fence.
3. Implement an authenticated `POST /v1/commands` ledger and
   `GET /v1/commands/{id}` reconciliation with
   `PENDING|APPLIED|VERIFIED|FAILED|UNKNOWN`. Persist command result, state
   transition, event, and outbox intent atomically.
4. Mint short-lived PASETO v4.public capabilities only from durable active
   leases. Every mutation must perform an online final authority check.
5. Put SQLite WAL and filesystem CAS under the platform data directory. Add
   fsync/rename boundaries, verified backup/restore, restore epochs, retention
   classes, and orphan-safe garbage collection.
6. Remove hard-coded PASS, SHA, verified, and demo-success branches. Reads
   reconstruct exact stored receipts or return UNKNOWN.

Exit: full Kernel required passes after restart; crash injection at every
SQLite/CAS/outbox boundary yields exactly the old or complete new state; all
authority-field mutation/replay/expiry/outage cases cause zero unauthorized
mutation.

### 3. Production BulletGit and Jeryu capability service

Goal: providers never receive Git authority, while exact changes survive
failure and can be independently reconstructed.

Required work:

1. Implement online Kernel authority verification for every mutating daemon
   call. Unavailable authority remains fail-closed; test permits stay private.
2. Bind proposals to base checkpoint and per-operation preimages. Reject
   duplicate/conflicting paths, traversal, `.git`, backslashes, Unicode/case
   collisions, stale preimages, oversized content, and symlink/reparse escapes.
3. Apply a batch in a new workspace generation; fsync tree, journal, and CAS,
   then atomically publish the generation. Failure preserves the prior
   authoritative generation.
4. Harden Git configuration, attributes, filters, hooks, alternate objects,
   sequencers, cleanup targets, and partial-write recovery.
5. Produce complete provenance-bound Candidate manifests, separate Candidate
   and Integration proof roots, exact checkpoints, and daemon-issued
   preservation receipts.
6. Keep the current local Git backend simulator-only. Production uses a
   reviewed, immutable Jeryu tag exposing a versioned `jeryu-gitd` service.

Exit: BulletGit required plus hostile repository/fault suite passes, Candidate
identity is deterministic and sensitive to every bound field, rebase/subject
changes invalidate proof, and cleanup cannot occur without the exact
preservation receipt.

External prerequisite: an authorized canonical Jeryu lane must harden and tag
the missing capability surface. Bullet work must not modify the running forge.

### 4. Five-plane transaction proof

Goal: replace component scaffolds with one real offline transaction.

Required work:

1. Runner acquires authority, starts a private read-only provider generation,
   accepts only a validated patch proposal, applies it through BulletGit, and
   runs admitted gate IDs only. Permit at most two repair turns.
2. Heartbeat renewal failure freezes mutation, terminates the full provider
   process tree, preserves the workspace, and lets a successor fence continue
   from the exact checkpoint.
3. Verifier reconstructs the exact Candidate in a separate clean environment.
   Writer output cannot satisfy independent gates; zero tests, timeout, flaky,
   unsupported, infrastructure error, and UNKNOWN never equal PASS.
4. Effect broker durably records intent before dispatch, maps a lost response
   to UNKNOWN, performs identity-exact read-back, and adopts the original
   effect without a second write.
5. `just demo` uses real JSON-RPC child boundaries and a protected local forge
   simulator while remaining deterministic and credential-free.

Exit: one signed `TRANSACTION_PROOF` covers materialization replay, fences
`1→2`, stale rejection, provider-tree death, salvage, private clones, exact
Candidate, independent Evidence, UNKNOWN recovery, protected integration,
preservation-before-cleanup, and truthful projection.

### 5. API and projection truth

Goal: the browser can never manufacture authority or hide uncertainty.

Required work:

1. Snapshot bodies carry `{data, as_of_sequence, observed_at, source}` from one
   atomic ledger read; do not rely on an unbound convenience header.
2. SSE emits only generated default `EventEnvelope` data. Cursors are exclusive,
   ordered, bounded, retained, and followed by live tail plus keepalives.
3. Add one-time CLI bootstrap exchange, HttpOnly/SameSite session cookies,
   CSRF protection, loopback binding, origin checks, and no wildcard CORS.
4. Correlate every projection receipt to a command ID. Timeout becomes UNKNOWN;
   an older green receipt cannot satisfy a newer failed command.
5. Complete Mission Graph, Attempts/Sessions, Router/Quota, Context/Fusion,
   Workspace/Candidate, Verification/Effects, and Audit views as projections.
6. Run Playwright against the built embedded Portal and a real `bullet-farmd`;
   retain mocks only for focused unit tests.

Exit: the `1,2,4` gap and failed-snapshot suite remains STALE at cursor 2;
watermark at least 4 clears it, and authentication/CSRF/command-reconciliation
negative cases pass against the packaged daemon.

### 6. Role-based evolutionary cognition and provider conformance

Goal: maximize useful multi-agent parallelism without giving cognition control
of authority or completion.

Required work:

1. Persist typed Cognitive Tasks, role/capability/profile snapshots, context
   capsules, behavior rules, quota/budget reservations, routing provenance,
   struggle signals, escalation decisions, fusion records, and dissent.
2. Route with hard constraints first: authority envelope, task/risk class,
   provider capability maturity, verified quota, context budget, independence,
   and verifier capacity. Optimize cost/latency/quality only among eligible
   profiles. UNKNOWN paid capacity blocks ordinary dispatch; bounded read-only
   probes require explicit policy.
3. Treat roles as policy profiles, not identities with ambient permissions:
   planner decomposes, researchers produce cited observations, implementers
   submit patches, critics generate counterexamples, fusion records agreement
   and dissent, verifier supplies independent Evidence, broker performs effects.
4. Preserve evolutionary activity as immutable variants plus selection records.
   Never overwrite a losing branch into success; retain negative knowledge and
   explain why a Candidate was selected.
5. Certify provider adapters in order: Claude bidirectional stream JSON, Codex
   App Server JSONL, Cursor ACP, Antigravity headless structured mode. Runtime
   probing determines capability maturity.
6. Invoke absolute verified binaries with allowlisted environments, ephemeral
   HOME, minimal provider OAuth, provider-only network egress, and no SCM,
   cloud, SSH, or host secrets.

Exit: all four adapters pass the same crash/cancel/timeout/malformed/duplicate/
delayed-event, process-tree, quota, canary-secret, read-only, and exact-patch
suite. Routing/fusion decisions are replayable from persisted inputs.

### 7. Onboarding, CI, packaging, and release mechanics

Goal: one small public command surface, with no shell-owned product truth.

Required Rust commands:

```text
bullet-family doctor --json
bullet-family setup --root <path> --source jeryu [--offline]
bullet-family lock generate|verify
bullet-family checkout verify
bullet-family fuse --source local|lock
bullet-family check fast|required|release
bullet-family coord claim|heartbeat|handoff|status
```

Thin Bash/Just/workflow wrappers may dispatch these commands. Installer, lock,
generation, orchestration, and proof decisions stay in Rust/TypeScript.

Installer closure:

1. Distribute a signed prebuilt `bullet-family`; `scripts/setup.sh` remains a
   source-development convenience and is not authenticated installation proof.
2. Verify hub tag, non-circular release manifest, schema-3 family lock,
   authenticated Jeryu URLs/slugs, signed member tags, exact commit/tree OIDs,
   lockfile digests, artifact checksums, and admitted tool/helper subjects
   before mutation.
3. Create ordinary clones at exact OIDs, reject dirty/symlinked/conflicting
   destinations, use `cargo --locked` and `npm ci`, and verify generated drift.
4. Run twice from a hub-only clone in a fresh HOME; both runs end at exact clean
   member OIDs with zero tracked changes and no worktrees.

CI closure:

| Lane | Required contents |
| --- | --- |
| `fast` | format, strict Clippy/unit tests, TypeScript type/unit tests, production Portal build, generated drift, affected-path routing; warm target under 60 seconds |
| `required` | locked builds/tests/doc-tests, wire/adapter contracts, migrations, real farmd/Portal E2E, deterministic transaction demo, pinned security/license/secret/workflow scans, family lock, Jankurai non-regression; no skip-green |
| `release` | platform archives, installer smoke, Rust 1.95 and 1.97.1, SBOM, checksums, signatures/provenance, backup/restore, fault suite, protected Jeryu/GitHub integration, four provider receipts, Jankurai at least 90 with zero caps/hard findings |

Exit: clean hub-only setup twice plus all three lanes from exact tagged bytes;
generated checks run in temporary directories and leave zero tracked changes.

### 8. Credentialed live and release proof

Goal: convert the proven offline transaction into externally observed effects
without weakening any boundary.

Required order:

1. Operator restores Jeryu authentication; run read-only probes, then the exact
   protected integration/reconciliation scenario against the reviewed tag.
2. Configure a GitHub App test repository with branch protection; prove intent,
   dispatch, lost-response UNKNOWN, exact read-back, check attestation, and
   protected integration.
3. Run all four provider conformance profiles with registered credentials and
   exact binary/model/config subjects.
4. Build and smoke signed archives for Linux x86_64/aarch64, macOS
   x86_64/arm64, and Windows x64. Linux is the production runner; other
   platforms refuse mutation until equivalent containment passes.
5. Publish SBOM, checksums, signatures, provenance, and the final non-circular
   signed release manifest.

Missing optional credentials yield a neutral unregistered lane. Missing a
credential, tool, adapter, platform receipt, or signer required by the release
profile fails the release.

## Immediate execution queue

1. Restore Kernel required after the fail-closed BulletGit boundary without
   reintroducing production simulator authority.
2. Make `just family-contract` green at the exact four clean heads and record
   the receipt.
3. Finish Kernel command/API/storage/recovery before enabling positive
   BulletGit authority.
4. Land the reviewed Jeryu capability tag, then implement positive BulletGit
   online authority and generation-atomic state.
5. Produce the offline five-plane transaction receipt.
6. Complete authenticated API/Portal truth and provider/cognitive conformance.
7. Finish the Rust onboarding/CI command surface and supply-chain release lane.
8. Run the external Jeryu, GitHub, provider, platform, and signing gates.

## Terminal definition of done

V1 is done only when every release row in `docs/release.md` has an exact,
independently verifiable receipt from tagged bytes; `just setup`, `just demo`,
`just fast`, and the required/release lanes are green without skips; Jankurai
is at least 90 with zero caps/hard findings; all four provider receipts and both
forge receipts are current; all package signatures verify; every checkout is
clean; and no known safety counterexample remains. Until then the only honest
status is **pre-release, blocked**.
