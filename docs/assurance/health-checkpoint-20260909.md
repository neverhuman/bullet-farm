# Health checkpoint after the production audit

Status: **Corrective packets accepted locally; production and release remain incomplete**

Last updated: 2026-09-09 06:56 UTC

This checkpoint follows the [deep audit](deep-audit-20260909.md),
[full-product plan](full-product-dogfood-plan.md) and [G1–G18 register](product-gaps.md).
The audit remains a frozen observation of its original subjects. Its 101-document
coverage has been followed by a 103-document inventory and review of subsequent
changes, including the two new audit/checkpoint documents.

## Accepted Kernel work

| Commit | Reviewed change | Executed evidence |
| --- | --- | --- |
| `79cae6e7084d1a651e001967f44cbacb9dd3f054` | Restore strict raw JSON parsing, consume the unchanged authoritative formal fixtures, and restore truthful generated/manual ownership metadata. | Three formal tests and one schema agreement test passed. Invalid comments, malformed JSON and raw string newlines are rejected; valid escaped newlines retain their meaning. |
| `02bf7c5ad0aea398a6b82b9d98ff60dbdde2a09c` | Reconcile the actual test inventory and active fixture instructions. | Enumeration found 1,121 total and 1,075 standalone identities, adding exactly the new parser test. The existing inventory checker passed. |

The complete Kernel `bash scripts/ci-local.sh required` then passed on
`02bf7c5ad0aea398a6b82b9d98ff60dbdde2a09c`, tree
`6fd94de4da0514be66dce7dc79360078225fa4cd`, in **504.998 seconds**.
Its 1,075 standalone and 34 contract tests passed, together with the mapped lint,
security and documentation checks. The three host-dependent egress and nine
family tests are outside this standalone campaign. Peak resident memory was
697,800 KiB. Source, selected tools and raw Cargo configuration stayed unchanged;
the checkout was clean and the proof lock was released.

The run supplied `CARGO_NET_GIT_FETCH_WITH_CLI=true` and offline Cargo explicitly.
An earlier inventory attempt failed its configuration guard and is retained.
The successful warm enumeration fixed the sole known configuration difference
with an explicit override; it was inventory evidence, not a Rust test run.

BF-A11's active consumer is repaired. Unused commented formal copies and the
declaration-only binding introduced by the earlier workaround remain for a
separate reviewed cleanup. This checkpoint supplies no blanket acceptance of
earlier commits, auditor scores or other member trees.

## Accepted private capture and rendering

| Commit | Reviewed change | Executed evidence |
| --- | --- | --- |
| `3f976ab43741c3b68abca96052b0ca8c2d98f563` | Preserve private captures, true exits, interruptions and cleanup results; distinguish a forked child from provider execution. | All 11 recorder tests passed, including signals, missing executable, EOF, limits, custody and synthetic campaign failures. |
| `3bb772c72302ccd75161584b76bf8880e7f7ef9f` | Preserve inputs and exact-color Portal masters; independently check GIF pixels, geometry and artifact identity with selected local tools. | All three renderer tests passed. Subsequent continuous capture exposed a GIF timing defect; earlier acceptance covers RGB/geometry and retained source timestamps. |
| `742a771d5786b81925867fb419f7677e74436094` | Sample Portal frames serially between eight landmarks, retain actual timing/gaps, and drain capture writes before closing the browser. | Six actual Chromium tests plus all 11 recorder regressions passed; focused lint, syntax and 17 selected/completed identities agreed. Synthetic pages and API responses were used. |

`8248c45c61fee68921761900e06210f2e00632e6`, tree
`c462ff9fbf88f2dcfefc31c6ba3237938202be72`, subsequently repairs native
timestamp measurement and GIF encoding. All three existing renderer identities,
focused Clippy and syntax checks passed. Independent review verified 957 retained
artifacts; the total remains 20 distinct media test identities.

The corresponding integration trees are `82d081386c5e77d9a441768072db4e7a76104b6e`,
`300cdee789e5b33758df8bb8ab824d70b3d25554` and
`369044a78c7a0d1bb82e8e204e65c3474a32310a`. Independent reviews verified the
respective 811, 641 and 390 retained artifact entries. These are **20 distinct
focused test identities**, with the 11 recorder regressions repeated during
continuous capture verification. They are not a complete Hub or family run.

The latest synthetic browser tour retained 17 original 1920 × 1080 PNGs over
6.062 seconds, with an observed 2.752 frames/second and actual gaps. Requested
4 Hz is a ceiling, not a guaranteed frame rate. The first Rust browser attempt
passed five cases and failed one Chromium screenshot operation. That failure
remains retained; a reviewed fixture paint-readiness precondition preceded the
successful six-case run. Screenshot failures still abort capture.

A separate actual render and independent check of those 17 PNGs passed in
6.524 and 3.420 seconds. All 112 artifacts and 40 successful child-command records
were reviewed. Original RGB and geometry match the lossless master; GIF pixels
are explicitly `QUANTIZED`. Original input JSON remains exact.

**The encoded timing defect is repaired and independently verified locally.**
The original 112-artifact roundtrip above remains historical RGB/geometry evidence:
it flattened variable gaps to 35–36 centiseconds. The corrected renderer preserves
native timestamp offsets, checks source-derived master PTS, and validates actual
GIF graphic-control delays and the final display hold. Coherently rehashed but
retimed masters, shifted origins, flattened GIFs and altered final holds refuse.

A new render/check of the same 17 original PNGs passed in 5.996/3.367 seconds.
The master is byte-identical to the earlier lossless master. Maximum measured
source timestamp error is 1.225 microseconds for the master and 4.883 milliseconds
for the GIF, whose absolute timestamps use centisecond rounding. Its final
34-centisecond hold repeats the last encoded gap; this is an explicit display
policy, not an observed source duration. Encoded intervals shorter than two centiseconds, or colliding after rounding,
refuse instead of dropping frames. These bounds do not claim temporal
losslessness. Original inputs, failed attempts and corrected outputs remain separate.

No real Portal application, subscription, coding transaction or public export
is admitted by these synthetic browser tests. The [capture runbook](../demo-gif/README.md)
records the actual interfaces and limits. Native CLI illustration prompts request
explanations without tools or edits; genuine coding demonstrations still require
the production transaction and provider campaign. Observed tool files and host
fonts do not establish a complete portable runtime/font closure.

## Inventory, documentation and review corrections

`49b95c23764514efeb166584c55aae7521569f21`, tree
`cecc4ef6be0fe0546ca090d5d32e314fe2c251a9`, changes only four inventory literals.
Actual Nextest enumeration found **819 total identities: 600 Hub and 219 wire**.
All previous 799 remain, with exactly the 11 recorder, three renderer and six
browser additions; no removals, ignored tests or partition overlap were found.
The unchanged inventory checker passed in 34.421 seconds. Enumeration does not
mean that all 819 tests were executed.

`db91f0d1cc87ef582634462ada031d88375953a6` admits only Boolean
`net.git-fetch-with-cli` configuration while retaining compiler/dependency/alias
controls. The full existing compiler-control canary passed in 48.747 seconds,
including 16 new valid/invalid configuration cases. The global Cargo configuration
was preserved. The earlier `just docs` refusal on `43d601c` remains recorded.

The complete mapped **`just docs` passed in 217.835 seconds** on
`4cf7a98794517145299d2ca674e9c0f23cc9bd99`, tree
`9c447224678de954eaa5b8324d38c8108be50456`, plus the recorded continuous-capture
source overlay. It ran rustdoc, the existing zero-doctest suites, 90-file link
checks, dogfood/release-truth checks, strict BLOCKED doctor readback and two
independent component-media reconstructions. All 870 source, ten Cargo-config,
15 selected-tool and 177 sysroot-file guards stayed unchanged; the proof lock
was released. This was mapped documentation proof on a checkout containing
retained untracked media, not clean complete Hub/family or hosted CI acceptance.

`6298124` and `a5d81f7` withdraw the newly introduced fast wrapper and competing
closeout page after independent review. The wrapper could count one checkout
four times, hid Cargo-home inputs, and removed private-target selection; the
page supplied invalid commands and weaker production/CI exit criteria. Original
commit `e0286b9` and all preimages remain preserved. The existing plan and lane
commands remain authoritative.

`4cf7a987` corrects historical media documentation. The unadmitted live-media
hook and false authentication claim were restored to the existing component
boundary after preserving their bytes. The live checker still refuses when its
required collection is absent and still reconstructs a valid collection when
present. `bc1176d038dccedb614b0c4ca517ab906f9449c9` adds explicit private tool-receipt
selection against a reviewed source digest; `962dadf` documents that interface.
All 16 rendering and 15 normalization fixture groups passed. The actual canonical
receipt consumer also admitted the real private input, 26 selected executables
and 107 runtime files in 1.587 seconds; after relocation it returned identical
metadata in 1.242 seconds with the old canonical receipt absent. This exercised
receipt selection, without image execution, media generation or provider calls.

All 57 previously untracked files (13,479,239 bytes) now have verified private
copies and retained originals in durable custody. Fifteen separate batches of at
most four paths used same-filesystem, no-replace rename, with unchanged tracked
source, index and HEAD. No bytes or Git objects were deleted. The historical
theme remains explicitly authored historical configuration; it was not labeled
generated or hidden with an exclusion. Source-pin selection keeps the current
receipt usable outside the checkout. Seven filesystem tests and independent
copy/procedure reviews preceded relocation. Interrupted operations require
reconciliation; this one-off transfer is not a production crash campaign.
Full mapped lint and a clean complete Hub/family run remain open.

## Outstanding delivery and production work

At **06:26 UTC**, GitHub `neverhuman/bulletfarm` main remained
`f8ce28e6b0583160519e5898250904f63eee753d`: zero workflows, zero runs, zero
rulesets and unprotected main. Supporting Hub PR 2 merged as `55e86b2` and
Kernel PR 2 as `6070a05`; these reconstructed subjects do not inherit proofs
bound to different canonical commits. The earlier exact audit/Kernel branch
pushes were rejected because the available OAuth token lacks `workflow` scope.
Their branches remained absent. Source/workflow history was preserved; the
repository-scoped publication App and complete aggregate CI remain required.

The installed Jankurai remains **1.6.11**. Subsequent remote discovery found the
published 1.7.0 Linux archive and its checksum/provenance assets, plus all three
selected dependency tags. An initial tag absence was caused by a local URL
rewrite and is not evidence of remote absence. The archive was downloaded and
hashed without installation or execution; attestation shape was inspected,
without cryptographic trust verification.

Source readback establishes a remaining admission defect: selected Core
`8505079e47597225e1f2bf65f57d41b0d050bfe7` has the common-base versions of
`audit/mod.rs` and `commands/witness.rs`, and lacks `audit/outcome.rs` and
`audit/release_proof.rs` from reviewed Core `2e0c395`. These four implementation
paths prove that those reviewed fixes are absent from the selected source;
ancestry divergence alone was not used to make that finding. A newer release
banner and successful release build do not establish the required policy,
conformance and process-exit agreement. Score-90/zero-cap/zero-hard acceptance
remains open until the source selections and auditor qualification are repaired.

Finish health and complete CI, then admit the preserved coordinator generation
and supervised 22→23 upgrade before the durable account/reservation/run/dispatch
transaction. Connect Runner supervision, termination reconciliation and atomic
Candidate finalization; reconcile browser state after lost responses and restart.
Qualify Codex, Claude and Cursor independently on xbabe2, then execute the twelve
real tasks, mixed-provider collaboration and seven-day survival observation.
The remaining custody, lifecycle, platform, evolution, forge, distributed and
post-V1 obligations stay in the full plan. All G1–G18 remain `DESIGNED`; all
18 product and two diagnostic profiles remain `BLOCKED` in the unchanged typed
inventory. The coordinator Operating HOLD remains effective.

## Retained evidence

The local September 8 health implementation directory on xbabe2 retains original
source packets, raw logs, failed attempts, independent reviews and integration
receipts, including:

- `formal-json-repair-r1/`, `formal-json-metadata-r1/` and `kernel-required-r5/`.
  Kernel required log SHA-256: `90cbb3ff540616c4b080821545e6c6e22ad09f9e515d9e7f3f6e5ac8991b47f4`.
- `jankurai-admission-delta-r2/` and the deep-audit directory's document coverage,
  GitHub readbacks and publication refusals.
- Deep-audit `recording-repair-r1/`, `hq-rendering-repair-r1/`, `continuous-portal-r1/`,
  `continuous-render-integration-r1/`, `continuous-timing-diagnosis-r1/` and
  `renderer-timing-repair-r1/`. Timing review SHA-256:
  `8b6d993f07a039b0166fec6d9378c704ecb47deb946508284ee4fcd82aa50806`.
- Deep-audit `cargo-transport-config-r1/`, `hub-media-inventory-r1/`, `media-routing-r1/`,
  external-closeout review/restoration packets, and both `mapped-docs-r1/` and
  `mapped-docs-r2/`. Successful docs log SHA-256:
  `aab8eb7f8c56d554c5f5316c46db0ba3b874bef808dbb7d1a2467ead936e2fc1`.

- `jankurai-admission-remote-r1/`, and deep-audit `private-live-receipt-r1/`,
  `private-receipt-actual-consumer-r1/` and `untracked-media-custody-transfer-r1/`.
  The durable private transfer retains both verified payloads and original files.

Hosted runs, packages and product-profile receipts require their own executed,
exact-subject admission. Local observations do not substitute for them.
