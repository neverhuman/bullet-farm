# Health checkpoint after the production audit

Status: **Kernel and media corrective packets accepted locally; production and release remain incomplete**

Last updated: 2026-09-09 05:03 UTC

This checkpoint follows the [deep audit](deep-audit-20260909.md) under the
[full-product plan](full-product-dogfood-plan.md) and [G1–G18 register](product-gaps.md).
The audit remains a frozen observation of its original subjects.

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
| `3f976ab43741c3b68abca96052b0ca8c2d98f563` | Retain each private capture and its true exit, interruption and cleanup outcome; preserve failed attempts; distinguish a forked child from verified provider execution. | All 11 focused recorder tests passed, including signals, missing executable, EOF, limits, custody and synthetic campaign failures. Applicable strict lint, syntax and exact test-identity checks passed. |
| `3bb772c72302ccd75161584b76bf8880e7f7ef9f` | Preserve capture inputs, create exact-color Portal masters, and independently check GIF pixels, geometry, timing and artifact identity using explicitly selected local tools. | All three focused renderer tests passed with actual local FFmpeg and agg, including exact versus quantized colors, tampering, transposed geometry, timeout and child-process cleanup. Focused lint, syntax and exact identities passed. |

The respective integration trees are
`82d081386c5e77d9a441768072db4e7a76104b6e` and
`300cdee789e5b33758df8bb8ab824d70b3d25554`. Tests ran against the recorded source
files before integration; independent reviews verified their exact bytes and
retained 811 recorder and 641 renderer artifact entries. These 14 focused test
identities are not a complete Hub or family run. The Hub's total inventory still
needs reconciliation from actual enumeration after the next capture packet.

Portal masters preserve decoded RGB and geometry; GIF colors are explicitly
`EXACT` or `QUANTIZED`, with original timestamp JSON retained separately from GIF
centisecond timing. Terminal casts are preserved byte-for-byte but supply no
original RGB master. Tool-file hashes and observed host fonts are not a complete
portable runtime or font closure. Outputs remain private and require review
before export. No provider, account, real-browser or production admission follows
from these tests. The [capture runbook](../demo-gif/README.md) records the actual
interfaces and limits; continuously sampled Portal recording is a separate
implementation packet, still pending qualification.

The usage correction in `3c9b823f2879bebe4ce385911f8d9d1ecd5c232c` removes
unverified public media claims, routes explicit arguments through Justfile, and
states the selected primary GitHub delivery target. Its focused documentation
checks passed. The mapped `just docs` attempt on retained Hub subject `43d601c`
exited 1 before rustdoc: the existing Hub policy rejected xbabe2's Boolean
`net.git-fetch-with-cli` setting. The raw Cargo configuration stayed unchanged,
the failed observation was retained, and the proof lock was released. The
configuration repair and full mapped docs/lint acceptance remain open.

## Outstanding delivery and production work

At **04:53 UTC**, GitHub `neverhuman/bulletfarm` main remained
`f8ce28e6b0583160519e5898250904f63eee753d`: zero workflows, zero runs, zero
rulesets and unprotected main. The earlier 03:51 read-back also found zero open
PRs. Expected-old-absent pushes of the exact
audit and Kernel review commits were both rejected because the available OAuth
token lacks `workflow` scope. Authoritative read-back found neither review
branch. Workflow bytes and source history were preserved.

Historical offline GIFs and the accepted local media components do not establish
a continuous browser screencast, real-account coding session or production
Bullet transaction. The production transaction must still be connected and
individually qualified with Codex, Claude and Cursor on xbabe2 before genuine
coding demonstrations and the retained dogfood campaign can be accepted.

Canonical Jankurai artifact admission, full score-90/zero-cap/zero-hard scans,
remaining repairs, clean family proof, complete hosted CI and governed
publication remain required. Coordinator admission, supervised upgrades, the
durable coding transaction, individual account qualification, twelve real tasks
and seven-day survival remain open. All G1–G18 remain `DESIGNED`; all 18 product
and two diagnostic profiles remain `BLOCKED` in the unchanged typed inventory.

## Retained evidence

The local September 8 health implementation directory on xbabe2 retains:

- `formal-json-repair-r1/`: source packet, focused logs, independent reviews,
  integration commit and rejected push/read-back.
- `formal-json-metadata-r1/`: exact old/new identity sets, retained failed attempt,
  successful enumeration/checker, independent review and integration commit.
- `kernel-required-r5/`: complete command log, before/after source and tool
  subjects, produced artifacts and result. Command log SHA-256:
  `90cbb3ff540616c4b080821545e6c6e22ad09f9e515d9e7f3f6e5ac8991b47f4`.
- `deep-audit-20260909/`: the frozen audit bundle, four GitHub read-backs,
  publication refusals, `mapped-docs-r1/`, `recording-repair-r1/`,
  `hq-rendering-repair-r1/` and `media-usage-r1/`, including source packets,
  raw results, independent reviews, integration receipts and failed attempts.

These are local proof records. Hosted runs, packages and product-profile
receipts require their own executed, exact-subject admission.
