# Setup recovery drill

Status: **drill runnable on Linux GNU; the positive setup path is refused by design (schema 2), so recovery today is diagnosis, not repair**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-25  
Applies to: bullet-farm `d762f86` (`src/setup.rs`, `src/setup/transaction.rs`, `src/doctor`, `src/checkout`);
transaction rules in [`source-setup.md`](source-setup.md)

Run this drill after any setup crash, refusal, power loss, or "it printed an error after it said it
published". Its purpose is to prove which of exactly two states the family is in — **prior** or **complete
next** — and to preserve everything else for diagnosis. It never deletes.

## 1. The two admissible states

`bullet-family setup` (`src/setup.rs::run`) validates every fallible input first (platform containment,
admitted family-root descriptor, hub location, lock schema, Cargo/Node/npm subjects, environment), then runs
one transaction (`src/setup/transaction.rs`): members are cloned into a `0700`, descriptor-relative staging
directory named `.bullet-family-setup.<random>` beside the members, published with no-replace rename, fsynced,
and finally the outer completion marker — the family-root `repos.manifest.toml`, byte-equal to the signed hub
manifest — is published last.

| State | Definition | Evidence |
| --- | --- | --- |
| **prior** | no new member directory, no staging directory, family-root `repos.manifest.toml` unchanged | `doctor --json` `family_layout` and the directory listing are what they were before the run |
| **complete next** | every locked member is an ordinary clean clone at its exact OID and the family-root manifest equals the hub manifest | `doctor --json` all `PASS` and `checkout verify` clean under a schema-3 lock |
| anything else | a `.bullet-family-setup.*` directory, a member directory without the manifest, a manifest without all members, or an orphan left by bounded cleanup | **partial — preserve; do not retry over it** |

An error reported *after* the manifest was published is indeterminate until the same exact `setup` and
`checkout verify` reconcile it; an error *before* is prior state, because publication is no-replace and the
manifest is last.

## 2. The drill

Run from the hub checkout. Every command is read-only.

1. **Diagnose.**

   ```bash
   cargo run --locked --quiet --bin bullet-family -- doctor --json; echo EXIT=$?
   ```

   Observed on this host (2026-08-25, hub `d762f86`): `EXIT=0`, `"status": "BLOCKED"`. The check ids are
   `hub_checkout`, `toolchain`, `source_metadata`, `family_layout`, `member_oids`, `clean_checkouts`,
   `exact_family_authority`. Here `hub_checkout`, `toolchain`, and `family_layout` were `PASS`;
   `source_metadata` and `exact_family_authority` were `BLOCKED` ("family.lock schema 2 is diagnostic-only";
   "the diagnostic schema-2 lock cannot authenticate an install"); `member_oids` and `clean_checkouts` were
   `BLOCKED` because sibling members sit ahead of the schema-2 lock's pins and one member had active-lane
   edits (`DIRTY_CHECKOUT`). Read the `repair` field of each blocked check; the `clean_checkouts` repair is
   explicit: "finish and hand off active claims; do not clean, reset, or stage another agent's changes".

2. **Verify the family against the lock.**

   ```bash
   cargo run --locked --quiet --bin bullet-family -- checkout verify; echo EXIT=$?
   ```

   Observed: `UNSUPPORTED_SCHEMA: family.lock schema 2 is not installable; remove it or regenerate schema 3
   from authenticated signed tags`, `EXIT=4`. That is the expected negative under the checked-in lock; with an
   admitted schema-3 lock this command is the clean-family verdict. Record the refusal, do not "fix" the lock
   ([`schema-removal.md`](schema-removal.md)).

3. **Look for partial publication.**

   ```bash
   ls -a "$(dirname "$PWD")" | grep -F '.bullet-family-setup.' ; echo "staging dirs above (none = ok)"
   ```

   Observed: none. If one exists, it is a partial transaction or a cleanup-bounded orphan: leave it, record
   its name and mtime, and hand the family to the orchestrator.

4. **Re-run the refused setup to prove it is still pre-mutation.**

   ```bash
   (cd /tmp && /abs/path/to/bullet-farm/scripts/setup.sh --offline); echo EXIT=$?
   ```

   Observed: the same `UNSUPPORTED_SCHEMA` line, `EXIT=4`, no staging directory created, `git status --short`
   in the hub unchanged. This is exactly what `ops/ci/required.sh` asserts. Running the binary directly needs
   the full argument set: `setup --root ABS --source jeryu --cargo-bin ABS --node-bin ABS --npm-cli ABS
   [--offline]` (the wrapper resolves those subjects; observed `EXIT=4` as well). On a non-Linux-GNU host the
   very first check answers `UNSUPPORTED_PLATFORM_CONTAINMENT` instead ([`platform-refusal.md`](platform-refusal.md)).

5. **Confirm the hub itself is intact.**

   ```bash
   cargo run --locked --quiet --bin bullet-family -- hub check; echo EXIT=$?
   ```

   Observed: `hub-check: ok`, `EXIT=0`.

## 3. What you must not do

- Do not delete, move, or overwrite a partially published member, a staging directory, or an orphan to
  force a rerun; setup is idempotent only over prior or complete-next state, and the orphan is the evidence
  of which boundary failed.
- Do not `git reset`, `git clean`, or re-clone a dirty shared checkout to satisfy `clean_checkouts`; that is
  another agent's active claim. `doctor` says so in its repair text.
- Do not edit `family.lock` or the family-root `repos.manifest.toml` by hand; both are authority inputs and
  the transaction compares the root manifest byte-for-byte with the signed hub manifest.
- Do not treat `doctor` as repair: it diagnoses and grants no install authority.

## 4. Limits of this drill

- The positive path (a real two-run setup in a fresh home ending in complete-next state) is component proof
  only: the fixture uses `LocalTransport` and a test-only validator (`tests/setup.rs`, `tests/setup_signed.rs`),
  not production `JeryuTransport`/`SetupValidator`. No crash-injected recovery has been run from tagged bytes
  on a fresh host; gate `release.installer-twice` is `BLOCKED`.
- With the schema-2 lock, `checkout verify` cannot return a clean verdict on any host, so the "complete next"
  branch of this drill is currently unreachable in the canonical family. That is honest, not a defect of the
  drill.
- Same-UID mutation during the path-based Git child remains outside the descriptor boundary
  ([`source-setup.md`](source-setup.md)); a drill cannot detect that race after the fact.
