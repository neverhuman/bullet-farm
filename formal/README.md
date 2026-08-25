# Gate 0 formal assurance

Status: Enforced
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: Gate 0

This corpus intentionally contains exactly two bounded models:

- `LeaseFence.tla` covers acquisition, permanent fences, expiry, reclaim, scope-revision
  acknowledgement, freeze, and restore epoch invalidation.
- `EffectCheck.tla` covers durable intent, ambiguous dispatch, exact read-back, third-party remote
  state, proof-before-check, policy expiry, freeze, crash, and restore.

`toolchain.lock.json` pins the stable TLC 1.7.4 release asset by its published SHA-1 and a locally
recorded SHA-256. `model-lock.json` pins every module/config hash and the deterministic
single-worker state counts. `model-check.sh` downloads only that exact asset, verifies both
digests, requires Java 21, rejects a third model, and fails on any source/config/state drift.

The JSON traces under `formal/traces/` are executable conformance fixtures, not model-check
success claims. Rust tests replay the same refusal/adoption decisions. Run `just model-check`.

## Exactly what is checked

Both configurations set `CHECK_DEADLOCK FALSE`. No liveness or fairness property is checked; the
one property below is a safety (action) property, and the pinned counts are for exactly these
configurations.

- `LeaseFence.cfg` (`Runners = {r1, r2}`, `MaxFence = 3`, `MaxTime = 4`, `MaxScope = 3`,
  `MaxOps = 2`, `MaxEpoch = 2`, `MaxFreeze = 3`) — INVARIANTS `TypeOK`,
  `BarrierHasNoInFlightApply`, `RestoredAuthorityNeedsFreshAcquire`; no PROPERTIES.
- `EffectCheck.cfg` (`MaxDispatches = 2`, `MaxEpoch = 2`) — INVARIANTS `TypeOK`,
  `AtMostOneLogicalEffect`, `VerifiedEffectWasReadBack`, `CheckRequiresDurableProof`,
  `ThirdPartyStateIsNeverAdopted`; PROPERTIES `NoNewDispatchAfterStop`.

`NoNewDispatchAfterStop == [][(~policyLive \/ frozen) => UNCHANGED <<effectDispatches,
checkDispatches>>]_vars` states the guarantee the protocol actually gives: after policy expiry or
freeze, no *new* dispatch leaves. `DispatchEffect` and `DispatchCheck` are the only actions that
raise a dispatch counter and both are guarded by `policyLive /\ ~frozen`, so the property is the
regression guard on those two guards.

`NoDispatchAfterStop` stays defined in `EffectCheck.tla` and stays unchecked, with the reason in
the module. As a state predicate it is false, and TLC refutes it at depth 4 (`Init` →
`PersistEffectIntent` → `DispatchEffect` → `ExpirePolicy`, and symmetrically `Freeze`): a policy
expiry or freeze may arrive while `effectPhase = "dispatching"`, because a stop cannot recall a
request already on the wire. Listing it as an invariant would assert something false; weakening it
would hide the boundary. An in-flight dispatch surviving a stop is therefore a modelled reality the
read-back path must absorb, not a case the model rules out.

Re-pinning after a model change: run the exact `model-check.sh` TLC invocation, take the
`N states generated, M distinct states found` line and the reported depth, and write those with the
new `sha256sum` of the module and config into `model-lock.json`. The V1 contract of exactly two
models is unchanged; `model-check.sh` still refuses a third.
