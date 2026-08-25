# Bullet Farm opportunity workplan

Status: **non-authoritative backlog; not release authority**
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-25

This workplan records opportunities exposed by the architecture paper and its
evidence audit. It does not compete with the authoritative
[`assurance/v1-closure-plan.md`](assurance/v1-closure-plan.md), generated
contracts, the Kernel ledger, or `bullet-family check release --json`. If this
file disagrees with any of them, this file loses. `COMPLETE` here would mean
only that the named acceptance receipt exists; it cannot promote a family gate.

## Frozen V1 topology and operator-blocked proposals

- Initial source distribution is Jeryu-only from immutable signed tags. No
  authenticated Jeryu URL is admitted by the manifest, and no GitHub namespace
  or public mirror is selected or assumed.
- The only permitted local Jeryu family is
  `/home/ubuntu/jain-split/jeryu-split`. Never recreate
  `/home/ubuntu/jeryu-split`. The branded hub owns onboarding, its manifest,
  and pinned component mapping; component repositories remain independent.
- Users are encouraged to run localhost Jeryu so CAS, read-back, protected
  refs, offline CI, and local context services can be certified together.
  GitHub and GitLab stay optional effect adapters. WP-19 installs or connects
  to a signed Jeryu runtime artifact under XDG data/config paths; it never
  creates a Jeryu source checkout or family member under this tree.
- `git.neverhuman.org`, `github.com/neverhuman/jeryu`, and
  `github.com/neverhuman/jankurai` are unratified future topology proposals,
  not current authority, authenticated endpoints, or public front doors. An
  operator decision must precede any use of those names in a lock, credential,
  publication, or receipt.
- If an operator later ratifies a public mirror, it remains a separately
  configured effect adapter rather than source authority. Publication must use
  expected-old-OID CAS, post-write read-back, divergence quarantine, and
  backups proved independently of the mirror.
- Jankurai remains the monorepo at `/home/ubuntu/jankurai`. No public front door
  is assumed; any future external-history reconciliation requires a
  preservation-first RFC and explicit ancestry receipts, never a force-push.

## Backlog

Priority is `P0` (release/safety predecessor), `P1` (important follow-on), or
`P2` (research/product opportunity). Class is `V1` or `post-V1`. Status values
are descriptive only: `BACKLOG`, `IN PROGRESS`, `EXTERNAL-BLOCKED`, or
`COMPLETE WITH RECEIPT`.

| ID | Priority | Class | Dependencies | Owning subsystem | Current evidence | Acceptance receipt | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| WP-01 | P1 | V1 | Stable family subjects; paper toolchain | Hub assurance | Hand-curated dated snapshot and deterministic TeX build | CI regenerates `bullet.paper-evidence.v1`, competitor pins, macros, and byte-identical PDFs from clean signed subjects | IN PROGRESS |
| WP-02 | P0 | V1 | V1-S1 through V1-S4 | Kernel, Runner, Verifier, BulletGit, Effects | Component receipts only; no connected proof | Signed `TRANSACTION_PROOF` for one exact offline five-plane transaction, followed by a matched receipt-bearing benchmark corpus | BACKLOG |
| WP-03 | P0 | V1 | Signed shared wire tag; Jeryu authority | Hub, Kernel, BulletGit | Local wire tests; no immutable publication | Signed immutable `bullet-wire` tag is independently resolved and consumed without committed sibling paths | EXTERNAL-BLOCKED |
| WP-04 | P0 | V1 | Setup admission and hostile-filesystem fixtures | Hub installer, BulletGit | Sealed tool subjects and partial descriptor-relative operations | Receipts admit the Git binary by digest, parse status as NUL-delimited bytes, mutate descriptor-relatively/no-follow, bind stronger lock identity, and prove support or typed refusal on every target OS | BACKLOG |
| WP-05 | P0 | V1 | Candidate/Evidence wire publication | BulletGit, Verifier, Effects | Strict local manifests; one fixture gate | Candidate and integration manifests prove invalidation across rebase, merge, and merge-queue result OIDs | BACKLOG |
| WP-06 | P0 | V1 | Authenticated Jeryu service and broker credentials | Effects, BulletGit, Jeryu | Local bare-forge component only | Protected integration receipt binds authenticated push, expected-old OID, required checks, server read-back, and observed protected ref | EXTERNAL-BLOCKED |
| WP-07 | P1 | V1 | Jeryu governance review | Jeryu hub, release operations | Documentation assigns overlapping release roles to `jeryu-release-ops` and `jeryu-deploy` | Accepted authority RFC plus generated owner/credential crosswalk with one release principal per effect | BACKLOG |
| WP-08 | P0 | V1 | Operator ratification of a Jeryu endpoint, DNS, TLS, auth, protection, backup, deployment | Jeryu operations | No endpoint selected; no authenticated URL or live receipts | Dated operator decision plus TLS chain, authenticated protection policy, deployment identity, backup digest, destructive restore drill, and independent read-back receipts for the ratified endpoint | EXTERNAL-BLOCKED |
| WP-09 | P1 | post-V1 | WP-06, WP-08, and a separately ratified public mirror | Jeryu mirror broker | No public mirror selected; no component-mirror proof | If ratified, each Jeryu component tag mirrors one way under expected-old-OID CAS; divergence is quarantined without overwrite | BACKLOG |
| WP-10 | P1 | post-V1 | Preservation RFC and complete ref inventories | Jankurai governance | No public mirror selected; preservation/reconciliation RFC absent | Signed RFC, full bundle backups, ancestry map, rehearsal receipts, and an approved non-destructive reconciliation before any mirror decision | BACKLOG |
| WP-11 | P0 | V1 | Reproducible Jankurai build and hosted runner | Hub assurance, Jankurai | Machine-local audit only; release score blocked | Portable signed Jankurai artifact digest and pinned hosted audit receipt from exact source and policy | BACKLOG |
| WP-12 | P0 | V1 | Provider adapters, live policy, containment | Runner, Providers, Effects | Offline parsers and bounded component paths; live admission false | Four-provider protocol-conformance receipts plus signed launch, exact executable, egress, teardown, and no-canary-leak evidence | EXTERNAL-BLOCKED |
| WP-13 | P0 | V1 | Five target builders and signing custody | Release engineering | Verifier/extractor components; no produced package | Deterministic archives, checksums, SBOM, provenance, signatures, and two-run schema-3 installation receipts from the same family lock | BACKLOG |
| WP-14 | P1 | V1 | Operator-ratified immutable publication endpoint and paper preflight | Hub documentation, Jeryu | Local PDF/source only; no public endpoint selected | Immutable source and PDF permalinks whose hashes match the checked evidence manifest before any external announcement | EXTERNAL-BLOCKED |
| WP-15 | P2 | post-V1 | WP-02 matched corpus | Research/evaluation | Capability comparison only | Preregistered workloads, costs, failures, exact subjects, and comparable receipts; superiority claims are admitted only from this corpus | BACKLOG |
| WP-16 | P0 | V1 | Existing G3/G4; signed shared wire tag | Kernel, BulletGit | Signed lease-transport and fail-closed production clone are component-only; no durable Mutation reservation | One Kernel reservation write that repeats the active-lease check, issues a one-use permit, and is verified by production BulletGit before I/O | BACKLOG |
| WP-17 | P1 | post-V1 | WP-08, WP-09, operator topology RFC | Jeryu operations, Hub | No public Git endpoint; GitHub and `git.neverhuman.org` are unratified names | Dated decision: GitHub as public index and/or backup; independent Jeryu Git as hosted front door for selected families; neither becomes source authority without CAS, read-back, and backup receipts | BACKLOG |
| WP-18 | P1 | post-V1 | Operator family-root RFC | Jeryu hub | Permitted local family remains `/home/ubuntu/jain-split/jeryu-split` | Extract Jeryu to a ratified independent family root without recreating `/home/ubuntu/jeryu-split`; update pins, never committed sibling path dependencies | BACKLOG |
| WP-19 | P0 | V1 | WP-03, WP-08, OD-D; must not wait on WP-18 | Hub installer, family container | Dedicated forge-topology audit; family-root `NEXT_EVOLUTION_PLAN.md` remains non-authoritative until corrected | Keep source only in `/home/ubuntu/jain-split/jeryu-split`. Bind a signed Jeryu runtime artifact in a non-circular external-component lock; support read-only `connect-existing` and isolated `managed` modes under versioned XDG paths; never copy, symlink, vendor, restart, upgrade, or reconfigure the existing source family or shared service. Default effect profile is localhost Jeryu; GitHub/GitLab remain optional, separately certified adapters. Freeze `/git/{owner}/{repo}.git`, REST, capability, upgrade/rollback, and receipt contracts before implementation. This row is not a G6 live receipt | BACKLOG |

## Maintenance

Every update must retain all eight row fields, link the exact receipt when one
exists, and state the boundary it does not prove. V1 items must also update the
authoritative closure plan in the same reviewed transaction; changing this
backlog alone never changes product truth.
