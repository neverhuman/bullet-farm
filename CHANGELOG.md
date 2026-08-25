# Changelog

All notable Bullet Farm changes are recorded here. A release entry is added
only when a signed tag and its release manifest exist; working-tree progress
belongs under `Unreleased`.

## Unreleased

### Added

- Canonical v1alpha1 policy and wire contracts with hostile fixtures, generated
  consumer bindings, and two bounded formal models.
- Rust family coordination, lock verification, doctor, hub validation, and
  repository-path dependency checks.
- Explicit component, synthetic, transaction, live, and release evidence
  classes with self-hosted, separately certified GitHub, and later distributed
  deployment stages.
- Signed launch-grant wire contract (`SignedLaunchGrantV1`, hub `a2d6b2a`) and
  Kernel signed launch-grant admission with Linux provider egress isolation
  (`bullet-harness-egress`; kernel `d388733`); ADR 0011.
- Policy `v1alpha2` operator-ratified live-admission rule and its five reason
  codes (hub `bf5c642`); Kernel loader mirror (kernel `0d848f6`); ADR 0012. The
  committed policy stays `v1alpha1`, generation 1, live admission disabled.
- Fail-closed release-truth report `check release --report [--portable]`,
  `just release-truth`, and the drift-checked generated page
  `docs/assurance/release-truth.generated.md` (`0cc7eec`).
- Release bundle verification (`release verify`, `352f963`), constrained
  one-target extraction (`release extract`, `ba09056`), and the signed receipt
  verifier (`release receipt-verify`, `143f8b9`).
- Receipt-admission path for exactly one release gate,
  `release.rust-msrv-1-95`, from a root-owned descriptor outside the
  repository (`d762f86`); its absence keeps the gate `BLOCKED`.
- Sealed setup tool subjects for the source-setup transaction (`7efe2f3`).
- Five read-only farmd projections — fleet, sessions, merge rail, quality lab,
  audit (kernel `529bad1`) — and the Portal views over them (portal `95108e3`).
- End-to-end policy-gated live-conformance path `bullet provider
  live-conformance` with sealed thirteen-step receipts (kernel `ba485d5`),
  nightly real-binary mode `BULLET_LIVE_REAL=1` (kernel `b4735da`), and the
  operator runbook `docs/runbooks/live-conformance.md` (`48cca46`).

### Security

- Fail-closed secret/dependency and ratcheted Jankurai audit lanes.
- BulletGit repository-local command configuration admission, including a
  clean-filter canary regression.
- Portable-path ancestor collisions are refused in `bullet-wire` proposal
  validation (`65a5ea7`).

### Known limitations

- No V1 release candidate is authorized. Hub-only source installation,
  production transaction storage, admitted live providers/forges, recovery,
  packaging, signing, provenance, and the Jankurai 90 release threshold remain
  blocked as listed in `docs/release.md`. `bullet-family check release`
  reports 26/26 gates `BLOCKED` with 0/26 receipts.
- Everything above is `COMPONENT_PROOF` at most. No `TRANSACTION_PROOF`,
  `LIVE_PROOF`, or `RELEASE_PROOF` receipt exists: no provider has a live
  conformance receipt, no policy generation 2 has been ratified, and the
  launch-grant, egress, and projection components have admitted no real
  provider or effect.
