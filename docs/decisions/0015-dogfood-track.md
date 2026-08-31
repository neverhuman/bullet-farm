# 0015 — The dogfood track: `DOGFOOD_RUN` operational observations and `dogfood-local-v0`

Status: **Proposed — pending operator ratification (OD-K) AND the engineering predecessors named below.**
An independent review on 2026-08-28 held this ADR and its runbook on eight findings; the corrections are
folded in. No dogfood operational record exists yet — `DOGFOOD_RUN` and `dogfood-local-v0` appear in no Rust
source — every release profile remains `BLOCKED`, and this ADR cannot change that.
Owner: Bullet Farm maintainers
Related: 0001 (providers propose, they never write), 0003 (five trust planes), 0005 (signed authority and
key lifecycle), 0011 (signed launch grants; `BULLET_LIVE_ADMISSION` rejected), 0012 (policy v1alpha2 live
admission), 0013 (operator decision register — OD-K lives there)

## Context

The release-assurance program is honest and it is working: 0 of 43 gates are receipted, `check release`
is fail-closed, and every lane correctly refuses to promote fixture-grade evidence. That program answers
one question — *may we ship this to a stranger?*

It has been asked to answer a second, different question — *may we use this ourselves, today, on this
machine, to do our own work?* — and it answers "no" to that too, because it has no vocabulary for it.
The consequence is visible in the family's own record: the plan of record places real providers at
Wave 8, councils at Wave 9 and the non-Claude providers at Wave 10, all after first GA, while
`docs/runbooks/dogfood.md` redefines dogfood as using the coordinator as a claims board. Meanwhile the
operator's own directive is to accelerate by running real frontier models through the loop now.

Both answers can be true at once, and conflating them costs the family the feedback that would make the
release program cheaper to finish. This ADR proposes that the second question receive its own vocabulary, its own
proposed admission path, and its own permanent ceiling.

## Decision

`DOGFOOD_RUN` is **not a sixth evidence class.** The ladder in
[`../assurance/execution-plan.md`](../assurance/execution-plan.md) §2 and
[`../release.md`](../release.md) says every evidence assertion uses exactly one of five classes
(`DESIGNED`, `COMPONENT`, `TRANSACTION`, `LIVE`, `RELEASE`), and an earlier draft of this ADR contradicted
that by inventing a sixth. Corrected: **`DOGFOOD_RUN` is a purpose-separated, non-evidence operational
observation.** Its future record would describe what an operator's own loop did on one host. It would
assert no evidence class and could never be an input to one.

The proposed observation would be produced under the operational profile name `dogfood-local-v0`, which
must **not** implement `ReleaseProfile` and must never be selectable by `check release`.

**Not yet implemented — required before any operator act.** None of the following exists in code today;
this section is a specification, not a description:
- `DOGFOOD_RUN` and `dogfood-local-v0` appear in no Rust source.
- `DOGFOOD_RUN` must **not** be added to the `GateClass` enum in `src/check/model.rs` — that enum is the
  release-gate vocabulary and a variant there would make a dogfood observation structurally selectable by
  a release gate, which is exactly the confusion this ADR exists to prevent.
- Because it also must not extend `bullet-wire`'s release receipt-kind vocabulary, the Kernel→Hub record
  needs its own **purpose-separated, generated, non-release** wire record with explicit bounds and
  unknown-field refusal. It must not be signed with the `authority-signing` / `provider-runner` key: that
  key admits provider launch, not evidence.
- A hostile fixture test must feed a `DOGFOOD_RUN` operational record to the release receipt registry and require
  refusal. Until that test exists, the separation is a convention, not a guarantee.

A future `DOGFOOD_RUN` operational record would be limited to this assertion and nothing more:

> On one Linux host, under one operator UID, with an operator-owned generation-2 policy admitted by that
> operator, a change went from a materialized Mission through a read-only provider proposal, an exact
> `PatchProposal` applied by production `bullet-gitd`, a sealed-catalog gate, a verifier chain, and a
> local bare-forge delivery with authoritative read-back — and here are the real subjects it touched.

### What a future `DOGFOOD_RUN` record may claim

- Real, reproducible Git subjects: base, head, tree, Candidate id, delivered ref, read-back result.
- Real gate outcomes from the sealed catalog, including honest `FAIL`, `TIMED_OUT` and `INFRA_ERROR`.
- Real provider facts: enrolled executable path and digest, observed version, invocation count, and
  either a provider-reported cost or an explicit `UNPRICED`.
- That the loop is operable end to end on this host, and — via `check dogfood` — that the family board
  is in a workable state.

### What a future `DOGFOOD_RUN` record may never claim

- Any release, transaction, or independent-evidence eligibility. The future purpose-separated record must
  make all three structurally inexpressible or hard `false` **by type**, not by convention; its validator
  must reject any conflicting value, and the release registry must refuse a `DOGFOOD_RUN` record. Those
  guarantees do not exist until the generated record, validator, and hostile refusal test land.
- Any gate in the release inventory, any `ReleaseProfile` condition, or any scorecard floor.
- Independence of any kind. Executor, verifier and observer would share one process and one UID on this
  host; the future record must label that social custody `OPERATOR_LOCAL_KEY_SAME_UID`, never
  `FIXTURE_KEY_ONLY` (which would understate the key's durability) and never a trusted-custody label (which
  would overstate it).
- Provider liveness for the release program. `release.provider.claude` is closed by OD-A and its
  conformance receipt, not by a dogfood run, however many succeed.
- Recovery of the 2026-08-26 coordinator incident. See "Coordinator" below.

## Admission

The proposed admission would remain the operator's decision. Runtime scope checks must be mechanical, but
operator provenance remains social on this same-UID host. No admission exists until all of these future
requirements and the engineering predecessors below are implemented and independently safety-reviewed:

1. A separately typed dogfood audience/operation binding that the general live and release paths refuse.
   The current global `live_admission_enabled` bit cannot satisfy this requirement.
2. An operator-owned top-level v1alpha2 policy at an absolute path **outside every repository**, mode 0600,
   with `policy_generation >= 2`, `sandbox_policy.live_admission_enabled = true`, and an
   `authority-signing` / `paseto-v4.public` issuer key carrying the `provider-runner` audience — exactly
   the ADR 0012 rule, unchanged and unweakened — plus the separate dogfood binding above. Every nested
   policy `schema_version` stays `v1alpha1`. A separately admitted producer must encode the resulting
   `PolicySnapshotV1` as RFC 8785 canonical bytes, install it create-once, reopen it, and require byte-exact
   canonical read-back; pretty or sorted output from plain `jq` is not an admissible policy subject.
   `route_policy.evolutionary_authority` stays `false`.
3. The private half of that key under operator custody in the data directory, plus exact provider enrollment,
   service identity, credential projection, invocation/spend bounds, validity, revocation, and containment.
4. The canonical OD-K social witness in the family log, naming every field required by ADR 0013. The line
   itself supplies no runtime authority and cannot mechanically distinguish its same-UID author.
5. A durable, complete incident-inventory record and a typed W0 subject that binds exactly the Hub, Kernel,
   BulletGit, and Portal commit and tree OIDs, their clean states, the zero-active-claim high-water, and the
   independent W0 review. The fresh-Genesis mutation must consume both record digests and re-read every
   subject inside its locked transition, refusing any path, inventory, claim, commit, tree, index, worktree,
   or untracked-file drift. Neither record nor that consumer exists today.

There is no environment-variable admission. ADR 0011 rejected `BULLET_LIVE_ADMISSION` and this ADR does
not reintroduce it under another name: a shared token is not custody.

**Two limits of that admission, stated plainly rather than implied away.**

*The policy is not scoped to this profile.* `PolicySnapshotV1` has no profile or audience field for
dogfood (`crates/bullet-wire/src/policy.rs:286-343`) and `validate_live_admission` checks only a global
boolean, the generation, and the presence of a provider-runner key (`policy/live.rs:44-64`). Setting
`live_admission_enabled = true` therefore clears the POLICY step for **every** guarded live route, not
only a dogfood command. A separately typed dogfood audience/operation binding that the general live and
release paths refuse is an **engineering predecessor of OD-K**, and OD-K is therefore *not*
predecessor-free. Until it exists, attempting to ratify this ADR would widen execution authority more than
its name suggests, and that is the reason the operator kit is marked not-executable.

*Operator provenance on this host is social, not cryptographic.* An earlier draft claimed a consuming
command independently rejects an agent-created policy or key. That is false here: Kernel policy loading
checks regularity, size, stable inode/length and canonical content but **not** owner, mode, signature, or
any OD-K witness (`crates/application/src/policy_snapshot/load.rs:62-103`); key loading checks mode 0600
and the current UID, which every agent on this host shares; and no Hub or Kernel source consumes OD-K or
`AGENT_CHAT.md` at all. A forged `— operator —` line is refused by *people reading the log*, not by the
code. Closing this needs an operator trust root or an identity agents cannot reach; until then the honest
description is custody by convention on a single-UID host.

## What does not change

- **ADR 0001 must hold without amendment.** A future dogfood provider must run read-only and propose;
  `bullet-gitd` must remain the sole writer; `-w/--worktree/--tmux` and every documented escape hatch must
  stay hard-denied. A dogfood run would have strictly the same write topology as a release-grade run — only
  its operational purpose and assurance ceiling would differ.
- **The Jeryu forge must not be touched.** The proposed first dogfood merge target would be a local bare
  mirror under the operator data directory. OD-B would not be consumed, no credential would be minted, and
  the running instance at `127.0.0.1:8787` would be neither modified nor authenticated against.
- **Bullet must never merge into a real repository.** It would deliver a Candidate ref into the mirror and
  read it back. A human would fetch and merge.
- **The release program is unchanged.** No gate, profile, receipt kind, invariant, or scorecard input is
  added, removed, or relaxed by this ADR.

## Coordinator

The dogfood track requires a working coordinator; the 2026-08-26 incident left the live ledger frozen
(`events.jsonl`, mode 0400, no `CURRENT`), and its sanctioned recovery requires an independent reviewer
distinct from the operator — which a single-operator host cannot supply honestly. Before any retirement,
an admitted descriptor-bound producer must persist a create-once canonical inventory of every retained
relative path, file type, owner, mode, link count, size, and regular-file digest, then read it back and bind
its domain-separated digest. Console `find` listings and three individual file hashes are not a durable or
complete inventory and cannot substitute.

If the typed predecessors land, a fresh safety review passes, and the operator then ratifies OD-K, that
decision would permit a **fresh Genesis generation**, with the frozen generation retained, unmodified and
unreadable-by-accident, as incident evidence. `GenesisManifestBody` currently binds only one bootstrap
commit, so the fresh-Genesis command must additionally consume the exact four-repository W0 subject above
and persist its digest in a create-once adjacent authority record (or extend the typed manifest contract)
before the procedure can become executable. This would not be recovery and could not be mistaken for it:
`GenesisManifestBody` carries no lineage, parent-generation, or trusted-record-count field — only
`RecoveryManifestBody` does — so no Genesis generation can represent the frozen claims as recovered, and
no claim state would carry forward. DF-R7a and DF-R7b remain open packets, owed in full, on their own lane.

## Consequences

- Required future surface: `bullet-family check dogfood --json`, a typed board that would report coordinator
  state, per-repo head and dirtiness, and policy admission — and **exit non-zero when the loop is broken**.
  It must never read a release receipt or write an observation. It would replace
  `scripts/dogfood-board.py`, which always exits 0 and therefore cannot fail an operator.
- Required future authority inputs: an admitted RFC 8785 policy producer/read-back, a canonical complete
  incident inventory, and an exact four-repository W0 subject. Fresh Genesis must reject missing records,
  digest mismatch, an incomplete inventory, a non-clean repository, any changed commit/tree or active-claim
  high-water, and any drift between admission and its locked append.
- Required future refusal: `check release --profile dogfood-local-v0` must return typed
  `NOT_A_RELEASE_PROFILE`, so the mistake this ADR exists to prevent is refused by the tool rather than by
  a reader's memory.
- Required future record: `DOGFOOD_RUN` must be a purpose-separated, non-evidence operational observation.
  `bullet-wire`'s release receipt-kind vocabulary must **not** be extended; adding a dogfood kind there
  would create exactly the ambiguity this ADR forbids.
- Future dogfood operational observations could cheaply expose faults, costs, and latency that guide the
  W7 chaos campaign and W5 custody-split work. They would be planning inputs, never evidence or substitutes
  for the proof those waves require.

## Falsification

This decision is wrong, and should be reverted, if any of the following is ever observed: a
`DOGFOOD_RUN` operational record admitted by a release gate or profile; a dogfood operational observation
cited as independent or transaction evidence in any document or handoff; a dogfood run performing a
repository mutation outside `bullet-gitd`; a live provider dispatched without the operator policy and launch
grant; or the frozen
coordinator generation being modified, adopted, or described as recovered.
