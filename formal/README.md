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
