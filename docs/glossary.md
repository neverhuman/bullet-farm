# Glossary

Status: **one definition per term, with its authoritative source**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-25  
Applies to: bullet-farm `d762f86`, bullet-kernel `0109a90`, bullet-git `236f4ef`, bullet-portal `95108e3`

A glossary entry explains a word; it never makes a claim true. Where two documents define a term
differently, the entry records both, names the one chosen here, and says why (see "Adjudicated
inconsistencies" at the end).

## Evolutionary vocabulary

Source: [`architecture/evolutionary-control.md`](architecture/evolutionary-control.md) "Evolutionary
vocabulary"; identities in `crates/bullet-wire/src/ids.rs`.

| Term | Definition |
| --- | --- |
| Mission | Durable user objective, constraints, repository subjects, and acceptance policy (`mis_` id). Owned by the Kernel. |
| Graph Revision | Immutable dependency graph and package decomposition admitted for the Mission (`grf_`). |
| Selection Group | Bounded set of Variants evaluated against the same declared objective and subject (`sel_`); declares maximum Variants, concurrent Attempts, repair loops, tokens, cost, wall time, context bytes, and verifier backlog. |
| Variant | Immutable hypothesis plus role/profile/routing/context/configuration snapshot and parent lineage (`var_`); the genotype. At most one active fenced writer per Variant. |
| Attempt | One execution incarnation of a Variant under a unique lease and a permanent, never-reused fence (`atm_`); Kernel `AttemptState` per spec §24.2 (`crates/domain/src/states.rs`). |
| Proposal | Provider-produced typed patch data (`PatchProposal`, `crates/bullet-wire/src/proposal.rs`, keyed by a `ContentId`); never shell, Git, Evidence, or effect authority. Only BulletGit applies it. |
| Candidate | Immutable software-change phenotype constructed by BulletGit from an applied proposal: exact base/head/tree OIDs and patch digest (`can_`). See the CandidateId inconsistency below. |
| Evidence | Independently reproduced result over the exact Candidate and admitted gate definitions (`Evidence` in `crates/bullet-wire/src/outcome.rs`: candidate, exact head/tree, gate id, outcome, verifier identity, `verifier_is_independent`, environment/toolchain/proof-bundle digests). Only `PASS` from an independent verifier satisfies a requirement; `EvidenceOutcome` also carries `FAIL`, `NOT_RUN`, `TIMED_OUT`, `FLAKY`, `UNSUPPORTED`, `INFRA_ERROR`, `INVALIDATED`, `UNKNOWN`. |
| Fitness Record | Policy evaluation over Evidence, cost, risk, and declared observations; a vector, not one opaque number; hard constraints filter before any ranking. |
| Fusion | A new Variant derived from named parent Candidates with a persisted dissent/fusion decision, fresh Attempt/fence, new Candidate identity, and no inherited PASS. |
| Outcome | Observed post-integration result; it never rewrites historical fitness. |
| TeamRecipe | The immutable, content-addressed unit of team evolution: typed roles and contracts, communication edges, certified provider/model/profile choices, bounded budgets, and deterministic stopping/escalation/fusion rules. It cannot contain or evolve credentials, authority, safety rules, evidence floors, or hard ceilings, and never inherits a live session or workspace (`evolutionary-control.md` "Dependency-gated team evolution"). |

## Evidence classes

Source: [`release.md`](release.md) "Evidence classes".

| Class | Proves | Never proves |
| --- | --- | --- |
| `COMPONENT_PROOF` | one crate, service, or portal surface passed its mapped tests | cross-process transaction safety |
| `SYNTHETIC_PROOF` | deterministic simulator behaviour | a provider, forge, or production mutation |
| `TRANSACTION_PROOF` | one exact offline five-plane transaction with independent receipts | external provider or forge conformance |
| `LIVE_PROOF` | an admitted provider or effect adapter passed the same exact-subject transaction | a release on every platform |
| `RELEASE_PROOF` | packages, installer, recovery, security, signatures, provenance, and required live profiles from tagged bytes | future versions or untested environments |

An exit code, model statement, HTTP success, branch push, or pull request is never evidence by itself.
`UNKNOWN`, timeout, zero tests, unsupported, skipped, flaky, or infrastructure error never equals `VERIFIED`.

## Status vocabulary

| Term | Definition | Source |
| --- | --- | --- |
| `COMPLETE` | exact committed subject and mapped receipt exist for a bounded claim | [`assurance/v1-closure-plan.md`](assurance/v1-closure-plan.md) |
| `IN PROGRESS` | claimed work or focused evidence exists; no completed commit receipt | same |
| `LOCAL-BLOCKED` | implementable offline work remains, or a predecessor safety gate is not green | same |
| `EXTERNAL-BLOCKED` | promotion needs operator-controlled service, credential, signer, or platform evidence | same |
| `BLOCKED` (gate) | a `check release` gate with no registered receipt; every row is one of `LOCAL (closable offline)` or `EXTERNAL (needs operator credential, signer, service, or platform)` | [`release.md`](release.md), [`assurance/release-truth.generated.md`](assurance/release-truth.generated.md) |
| `BLOCKED` / `PASS` (doctor) | per-check diagnostic status of `bullet-family doctor --json`; `BLOCKED` carries a `repair` string and grants nothing | `src/doctor` |
| neutral (`78`) | a live lane that could not run because its optional registration or tooling is absent; distinct from success and from failure | `ops/ci/nightly.sh`, `ops/ci/egress.sh`, `apps/bullet/src/provider.rs` |

## Command and projection states

| Term | Definition | Source |
| --- | --- | --- |
| `PENDING` | command admitted and durable; one correlated outbox row; nothing executed | Kernel `CommandPhase`; closure-plan frozen contract |
| `APPLIED` | dispatched to an execution adapter; not read back | same (no adapter can set it today) |
| `VERIFIED` | read-back proved the exact intended effect; the only state the Portal renders green | same |
| `FAILED` | durably refused; nothing ran | same |
| `UNKNOWN` (persisted) | adapter or response lost; the effect may or may not have happened; settled only by read-back reconciliation | same; [`runbooks/effect-reconciliation.md`](runbooks/effect-reconciliation.md) |
| `UNKNOWN` (local, Portal) | a status read whose subject conflicts, whose transport timed out, or that regressed `APPLIED`→`PENDING`; rendered unknown, never healthy, never "empty" | `bullet-portal/docs/architecture.md` "Status vocabulary" |
| `CONFIRMED` | spec §25 term listed by the Portal for a verified effect; the wire has no such phase — `VERIFIED` is the Kernel name and the only green | same; see inconsistencies |
| `STALE` | the event stream detected a sequence gap; badge until replay or covering watermarks fill it | same |
| `CONTRADICTORY` | observations that disagree (`ObservationKind::contradictory`); never rendered as a value; C5 gives it fence-mediated exits | same; `product-gaps.md` C5 |
| `OUTCOME_UNKNOWN` | effect-broker state after a lost push response; `dispatch` refuses `RETRY_WITHOUT_RECONCILE`; only `reconcile` (adopt / retry once on proven non-execution / quarantine) leaves it | `bullet-kernel/crates/effects/src/broker.rs` |

## Identities

Sources: `crates/bullet-wire/src/ids.rs` (hub wire), `bullet-git/crates/bullet-git-types/src/{ids,change}.rs`,
`bullet-git/docs/architecture.md`. All are full 256-bit lowercase prefixed ids; Git OIDs are algorithm-tagged
(`sha1:`/`sha256:`).

| Term | Definition |
| --- | --- |
| ChangeId (`chg_`) | The logical change a series of checkpoints and Candidates belongs to. |
| CheckpointId (`ckp_`) | An exact journal checkpoint a proposal is based on (`base_checkpoint_id` + digest in `PatchProposal`). |
| ContentId (`cnt_`) | Digest identity of content bytes alone — e.g. `PatchProposal.proposal_id`. Identical bytes give the identical id regardless of who produced them or where. |
| CandidateId (`can_`) | Identity of a Candidate. In BulletGit today: `CandidateId::from_content(change, tree, head)` — content-derived from change + tree + head, so two trees under one Change never collide. Lineage subject and environment digest are optional and **outside** the id. The frozen V1 contract wants it provenance-bound and distinct from ContentId; see inconsistencies. |
| ProofRoot | Merkle binding of proof claims to an exact Candidate (`bullet-git-types::ProofRoot`); computed with length-framed fields. The wire distinguishes `CandidateProofRoot` (`cpr_`) from `IntegrationProofRoot` (`ipr_`): a Candidate's proof and an integration's proof are different roots. |

## Authority and admission

| Term | Definition | Source |
| --- | --- | --- |
| Five trust planes | control; execution/repository writer; independent verification; effect/attestation/integration; observation/audit. Broker, attestor, integration worker, and observer are separate principals; Portal and evolution engine hold no mutation credential. A plane consumes only grants addressed to its authenticated principal. | [ADR 0003](decisions/0003-five-trust-planes.md) |
| Lease | The durable, database-clock-bounded right of one Attempt to write; TTL `1..=15` s, acquired in a single ledger transaction, renewed by heartbeat, lost on expiry or supersession. A point-in-time lease read is an observation, not authority. | `bullet-kernel/crates/application/src/leases.rs`; closure plan "Database-clock lease authority"; ADR 0005 |
| Fence | Permanent, monotonically increasing epoch bound to an Attempt's lease; never reused. A successor gets a higher fence; a stale writer cannot publish. `AuthorityToken.attempt_fence`. | `bullet-kernel/crates/domain/src/authority.rs`; `evolutionary-control.md` |
| Launch grant | `SignedLaunchGrantV1`: a PASETO v4.public token binding the durable active lease, the evaluated provider admission (exact executable path and BLAKE3 digest, profile, model, protocol), the policy (snapshot digest, generation, sandbox/environment digests, gate ids), and the budget. Audience `provider-runner`, operation `launch-provider`, TTL ≤ 15 s, single-use nonce. Authorizes one process launch and nothing else. | [ADR 0011](decisions/0011-signed-launch-grant-and-egress-isolation.md) |
| Egress receipt | `EgressReceipt` sealed by `bullet-harness-egress` after the provider namespace is built: allowlist, nftables ruleset, tool records, and probe digests proving direct internet, the host forge port, decoy ports, and DNS were refused before the child started. Clears `EGRESS_ISOLATION_UNAVAILABLE` only when every containment probe was refused. | ADR 0011; `bullet-kernel/crates/harness-egress/src/receipt.rs` |
| Admission blocker | A typed reason the Kernel will not dispatch a provider: `SIGNED_ADMISSION_UNAVAILABLE`, `EGRESS_ISOLATION_UNAVAILABLE`, `PROTOCOL_NONCONFORMANT`, `CAPABILITY_NONCONFORMANT` (`AdmissionBlocker`). Each is cleared by exactly one piece of evidence. Policy refusal (`POLICY_LIVE_ADMISSION_DISABLED`) is upstream of admission. | `bullet-kernel/crates/harness-core/src/admission/receipt.rs`; ADR 0011 |
| Nonce | A 256-bit single-use value. Authority claims carry one (ADR 0005); a launch grant carries `grant_nonce`, persisted and spent by the issuer so a grant verifies at most once, plus a `workspace_nonce_digest` binding the workspace. | ADR 0005; `bullet-kernel/crates/harness-core/src/launch_grant/claims.rs` |
| Policy generation | Monotone counter in `PolicySnapshotV1`; generation 1 is the committed Gate 0 offline policy and can never admit a provider; live admission needs `>= 2` under `v1alpha2` with an active `provider-runner` key. | [ADR 0012](decisions/0012-policy-v1alpha2-live-admission.md) |
| Operator decision | A fact only custody can create (ratified policy, authenticated forge, protected key, mutable test repository), registered once in [ADR 0013](decisions/0013-operator-decision-register.md). | ADR 0013 |

## Adjudicated inconsistencies

| Term | Definition A | Definition B | Chosen here, and why |
| --- | --- | --- | --- |
| CandidateId | "provenance-bound immutable software-change phenotype" (`docs/architecture/evolutionary-control.md`); "`ContentId` is distinct from provenance-bound `CandidateId`" (`docs/assurance/v1-closure-plan.md` frozen contract) | content-derived from change + tree + head; lineage subject and environment digest optional and outside the id (`bullet-git/crates/bullet-git-types/src/ids.rs`, `bullet-git/docs/architecture.md`) | Both are recorded: B is what the code computes today, A is the frozen V1 target. `release.md` already lists "provenance-complete Candidate identity" as open, so the documents agree that A is not yet implemented; the glossary entry says so rather than picking one. |
| Key algorithm string | `paseto-v4-public` (`docs/runbooks/live-conformance.md` §2 step 1; family-root `TEAM_PLAN_CLAUDE.md` §10.5) | `paseto-v4.public` (`crates/bullet-wire/src/policy.rs` serde rename; `bullet authority keygen` output; ADR 0012) | `paseto-v4.public` — it is the byte the validator accepts. The hyphenated spelling in `live-conformance.md` is a prose typo and should be corrected by its owner. |
| `CONFIRMED` | Portal lists spec §25 vocabulary `PENDING, CONFIRMED, FAILED, UNKNOWN, STALE, CONTRADICTORY` (`bullet-portal/docs/architecture.md`) | Kernel wire phases are `PENDING, APPLIED, VERIFIED, FAILED, UNKNOWN` (`apps/bullet-farmd/src/commands.rs`) | `VERIFIED` is the only green and the only wire name; `CONFIRMED` is retained as the spec's historical word for the same state and is not a wire value. |
| `BLOCKED` | closure-plan slice statuses `LOCAL-BLOCKED` / `EXTERNAL-BLOCKED` | gate status `BLOCKED` with a separate `LOCAL`/`EXTERNAL` closability column (`check release`), and doctor's per-check `BLOCKED` | Same partition expressed at three granularities (slice, gate, diagnostic); no contradiction, but never sum them: only `check release` counts gates. |
| `UNKNOWN` | persisted Kernel phase / `OUTCOME_UNKNOWN` / `EvidenceOutcome::Unknown` | Portal local `UNKNOWN` from transport or subject conflict | One epistemic meaning ("not proven either way") over different subjects; the entry keeps both because a local UNKNOWN is never written to the ledger. |
