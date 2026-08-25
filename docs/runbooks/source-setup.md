# Source setup and installation boundary

Status: **contributor setup available; trusted public installation blocked**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-08-25

This runbook distinguishes three surfaces that must not be conflated.

| Surface | Current authority |
| --- | --- |
| Existing canonical family | contributor development and local proof |
| `scripts/setup.sh` | source wrapper that starts Cargo before Rust admission |
| Signed prebuilt installer | required for public release; not published |

## Existing canonical family

The family root contains four ordinary independent clones named by
`repos.manifest.toml`. Never create Git worktrees. Before running family proof:

```bash
cargo run --locked --quiet --bin bullet-family -- doctor --json
cargo run --locked --quiet --bin bullet-family -- checkout verify
just fast
```

Run these from the Hub checkout. `doctor` and `checkout verify` are diagnostics;
they do not repair dirty/missing subjects or grant install authority.

For local development fusion, use `bullet-family fuse --source local`. The
ignored `.fusion` output may contain local path overrides; committed manifests
must never contain sibling `path = "../..."` dependencies.

## Source bootstrap wrapper

`scripts/setup.sh` resolves absolute Cargo/Node/npm subjects and invokes:

```text
bullet-family setup --root <family-root> --source jeryu
```

It is not a curl-pipe installer or release trust root. It launches the local
Cargo toolchain before the Rust setup mechanism can verify a signed prebuilt
binary. The checked-in alpha lock is schema 2, so a hub-only clone currently
returns `UNSUPPORTED_SCHEMA` with regeneration guidance before creating member
directories or running dependency tools. `--offline` narrows dependency/network
behavior; it cannot supply missing signed source authority.

## Future trusted installation

Do not publish a public install command until all inputs exist:

1. a signed prebuilt `bullet-family` for the target platform;
2. an allowed-signers policy and verified non-circular release manifest;
3. a signed schema-3 family lock with authenticated Jeryu URL/slug, tag,
   commit/tree, dependency-lock, generated-artifact, and checksum subjects;
4. five package archives with SBOM, provenance, checksums, and signatures; and
5. installer smoke receipts from clean supported hosts.

The existing Linux `bullet-family release verify` command only verifies an
already materialized bundle. A separate `release extract` command can safely
materialize one verified archive at an absent destination. Neither command
downloads, builds, activates, rolls back, provisions signers, or interprets
binary/SBOM/provenance semantics.

## Setup transaction rules

The Rust setup mechanism must continue to:

- validate every fallible input before no-replace publication;
- retain the admitted family-root descriptor and recheck pathname identity
  around path-based external commands;
- use descriptor-relative 0700 staging, no-replace member/manifest publication,
  and fsync boundaries;
- create ordinary canonical clones at exact OIDs;
- reject dirty, symlinked, non-empty, or conflicting destinations;
- run `cargo --locked` and `npm ci` through bounded admitted tool paths;
- generate contracts into temporary directories and reject drift;
- publish the outer completion manifest last; and
- be idempotent: two runs in a fresh home end with exact clean OIDs and no
  tracked changes.

After a crash or refusal, run `doctor --json` and `checkout verify` before any
retry. Do not delete, overwrite, or move partially published directories merely
to force success. Preserve them for diagnosis; setup recovery must prove the
state is exactly prior or complete next.

The current containment is strongest at publication and cleanup boundaries. It
does not eliminate active same-UID swap-and-restore races inside path-based Git
execution. Bounded cleanup intentionally preserves an orphan if identity,
depth, or entry limits prevent a safe removal. An error reported after the
outer manifest was published is indeterminate until the same exact setup and
verification commands reconcile the durable family.

## Platform boundary

Linux is the initial production runner. Packages for macOS and Windows must
refuse real mutation until equivalent native containment has release evidence.
An archive existing for a platform does not authorize execution there.
