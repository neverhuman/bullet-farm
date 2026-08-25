# Release build (x86_64-unknown-linux-gnu)

Status: **component producer only; every release gate remains BLOCKED**
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-25

`bullet-family release build` produces one unsigned Linux x86_64 release bundle
from an exact committed four-repository subject. It is not a release, an
installer, a signer, or a package matrix. V1 requires five signed archives; this
command produces one of them, unsigned, and says so in its own final line.

## What it produces

```
<out>/
  family.lock                                   copy of the hub lock, at the path the manifest schema binds
  release-build-manifest.json                   canonical JSON; binds every subject and digest, never its own
  SIGNING.txt                                   the exact operator commands for OD-E
  x86_64-unknown-linux-gnu/
    bullet-farm-<tag>-x86_64-unknown-linux-gnu.tar.zst        the archive
    bullet-farm-<tag>-x86_64-unknown-linux-gnu.cdx.json       CycloneDX 1.6 SBOM
    bullet-farm-<tag>-x86_64-unknown-linux-gnu.intoto.jsonl   unsigned in-toto provenance
    bullet-farm-<tag>-x86_64-unknown-linux-gnu.checksums.json BLAKE3 over every archive entry and bundle file
  .scratch/                                     build scratch: the Portal clone and the cargo/npm caches
```

The archive layout is exactly what `src/release/archive.rs` admits: one
`bullet-farm/` root, byte-sorted unique ASCII paths, regular files and
directories only, parents before children.

```
bullet-farm/
bullet-farm/LICENSE
bullet-farm/bin/{bullet,bullet-effects,bullet-family,bullet-farmd,bullet-gitd,bullet-runner,bullet-verifier}
bullet-farm/share/family.lock
```

`bullet-farmd` is built with `--features embedded-portal`, so the Portal is
inside the daemon binary, not shipped as loose files.

## What it does not produce

- **No signature.** Signing is operator decision OD-E. The build prints the exact
  `ssh-keygen -Y sign` commands into `SIGNING.txt` and fabricates nothing.
- **No `release-manifest.toml`.** The frozen schema in `src/release/schema.rs`
  requires all five byte-sorted targets *and* a schema-3 `family.lock`. This host
  can honestly build one target, and the checked-in lock is schema 2. Writing a
  manifest would require inventing four archives and a lock version, so none is
  written. `bullet-family release verify` therefore refuses this bundle, which is
  the correct result.
- **No SPDX document.** `docs/release.md` calls for both SBOM formats, but the
  frozen manifest schema has exactly one SBOM slot per package and requires
  `.cdx.json`. A second document could not be bound or signed, so it is not
  emitted. Reconciling that is a schema change, not a builder change.
- **No SHA-256 checksums.** BLAKE3 is the family digest and the only algorithm
  the verifier, the family lock, and the Portal bundle manifest accept. No
  SHA-256 implementation is pinned in `Cargo.lock`; adding one would be an
  unpinned dependency, not evidence.

## Preconditions

1. All four member checkouts are ordinary clones (never worktrees) and **clean**.
   Any tracked, untracked, or index change refuses with `DIRTY_SOURCE`.
2. `git`, `cargo`, `rustc`, `node`, and `npm` resolve to executable regular files.
   Anything missing refuses with `RELEASE_TOOLCHAIN_MISSING`. The `cargo`/`rustc`
   pathnames are deliberately *not* canonicalized: they are rustup shims that
   dispatch on their own `argv[0]`.
3. `--out` names an absolute path that does not exist. The build creates it and
   never replaces an existing byte.

## Run

```bash
bullet-family release build \
  --target x86_64-unknown-linux-gnu \
  --out /absolute/absent/bundle \
  --family-root /absolute/family/root   # optional; discovered from the working directory
  # --offline                            # cargo --offline and npm ci --offline
  # --cache-dir /absolute/cache          # reuse cargo target and npm caches between runs
echo EXIT=$?
```

Ordered stages, each of which refuses instead of continuing:

1. admit the target, the toolchain, every member subject, and the lock;
2. clone the committed Portal subject into `<out>/.scratch/bullet-portal`, then
   `npm ci --ignore-scripts`, `npm run build`, `npm run bundle:generate`, and
   `npm run bundle:check` there — never in the tracked checkout and never into a
   tracked `dist`;
3. re-read every emitted Portal file against that bundle manifest before the
   bytes reach the Rust build script;
4. `cargo build --locked --release` for all seven binaries, into a scratch target
   directory so no member checkout is written to;
5. write the deterministic `tar.zst` (uid/gid 0, mtime 0, ustar, one Zstandard
   frame);
6. write the CycloneDX SBOM and admit every component semantically;
7. write the unsigned in-toto provenance statement;
8. write the checksum manifest, then **re-open, re-parse, and re-hash** every
   subject it names;
9. write the non-circular build manifest and re-read it;
10. re-extract the archive through the committed extractor into
    `<out>/.scratch/extracted` and compare every byte to the checksum manifest.

## SBOM admission

Every component must carry a name, a version, a package URL, and a license.

- Cargo components come from `cargo metadata --locked` for all three Rust
  workspaces. Each is admitted against the committed `deny.toml` allow-list of
  *every* workspace whose locked graph contains it. SPDX `OR`/`AND`/`WITH`,
  parentheses, and the legacy `A/B` slash form are all evaluated.
- npm components come from the Portal `package-lock.json` bound by the bundle
  manifest. Shipped (non-`dev`) components are admitted against the union of the
  reviewed Rust allow-lists. Build-only components must declare a license but are
  marked `scope: excluded` and are not gated, because the family has no committed
  npm license policy file. **Adding one is an open supply-chain item.**

A component with no license fails the build. It is never a warning.

## Refusals

| Code | Cause |
| --- | --- |
| `UNSUPPORTED_RELEASE_TARGET` | any target other than `x86_64-unknown-linux-gnu`; the message names the other four archives the V1 contract requires |
| `DIRTY_SOURCE` | any member with tracked, untracked, or index changes |
| `RELEASE_TOOLCHAIN_MISSING` | `git`, `cargo`, `rustc`, `node`, or `npm` absent or not executable |
| `RELEASE_OUTPUT_EXISTS` | `--out`, a bundle file, or the Portal scratch clone already exists |
| `RELEASE_PORTAL_BUNDLE_INVALID` | the Portal bundle manifest disagrees with its own files, its lock, or the admitted Git subject; also carries a failed `npm` step's typed output |
| `RELEASE_SBOM_LICENSE_REFUSED` | a component declares no license, or one outside the governing `deny.toml` allow-list |
| `RELEASE_CHECKSUM_MISMATCH` | a byte subject changed between writing and re-reading it, or an extracted entry differs |
| `INVALID_RELEASE_BUILD_MANIFEST` | the build manifest would bind its own digest |
| `RELEASE_BUILD_FAILED` | a locked release compile failed; the child's exact exit status and bounded output are reported |

## After the build

```bash
bullet-family release verify --bundle /absolute/absent/bundle \
  --allowed-signers /absolute/path/to/allowed_signers; echo EXIT=$?
```

This **must refuse**: the bundle carries no signed `release-manifest.toml`.
Recording that refusal is the honest end of this lane. `release extract` refuses
for the same reason, because extraction is gated on complete verification. The
build's own step 10 is the only extraction proof available today, and it runs
through the same committed `src/release/archive.rs` code path.

## Known defects this lane records but does not fix

- `src/release/archive.rs` marks exactly one archive path executable,
  `bullet-farm/bin/bullet-family`. Every other packaged binary materializes at
  mode `0644` and cannot be run from an extracted tree. A five-archive release
  needs all seven. `src/release/build/tests.rs` asserts the current behavior so
  the defect cannot be lost.
- There is no `bullet-family --version`; the binary answers `USAGE`.

## Gate status

This command clears nothing. `release.package-matrix`, `release.checksums`,
`release.sbom`, `release.manifest-non-circular`, and `release.provenance` remain
`BLOCKED`, because one unsigned target from a schema-2 lock is not the
five-archive contract. See [`../release.md`](../release.md).
