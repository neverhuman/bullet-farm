# ADR 0013: Operator decision register

Status: Accepted (register)
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-25
Applies to: every gate whose closer is an operator act rather than code

## Decision

There is exactly one list of open operator decisions, and it is this file. It supersedes the lists formerly
kept in the family-root `README.md` ("Open operator decisions"), in
[`../assurance/product-gaps.md`](../assurance/product-gaps.md) ("Operator decisions that are not code"), and
in `TEAM_PLAN_CLAUDE.md` §10.5 (OD-1). Those locations point here or will be redirected by the orchestrator;
if any of them disagrees with this register, this register wins and the other is stale.

An operator decision is a fact only a person with custody can create — a ratified policy generation, an
authenticated forge, a protected key, a repository the family may mutate. Agents must not manufacture,
simulate, or flip any of them. Each entry names the gate ids it unblocks
([`../assurance/release-truth.generated.md`](../assurance/release-truth.generated.md); all 26 are `BLOCKED`
at hub `d762f86`), the procedure that consumes it, the exact `AGENT_CHAT.md` line that ratifies it, and its
status. A decision is `OPEN` until its ratification line exists in the family-root `AGENT_CHAT.md`; the log
line is the authority for the running configuration, never an agent edit
([`../runbooks/live-conformance.md`](../runbooks/live-conformance.md) §2.3).

Ratification lines use the family log's heading form, with `operator` as the actor:

```text
## <UTC ISO 8601 Z> — operator — <DECISION-TAG> — RATIFIED: <exact facts>
```

## Register

### OD-A — Live provider admission: policy generation 2 and the launch-grant key

- Decision: generate the `provider-runner` authority-signing key on the host that will run providers
  (`bullet authority keygen`), write a v1alpha2 policy at `policy_generation = 2` that registers the printed
  `IssuerKeyV1` and sets `sandbox_policy.live_admission_enabled = true` with `route_policy.evolutionary_authority`
  still `false`, store it outside the repositories, and ratify it.
- Why not code: ADR 0012 makes the schema able to express live admission; generation 1 can never admit a
  provider, and the committed `policy/v1alpha1/policy.json` stays generation 1. The key's private half is
  operator custody (`0600`, never committed).
- Unblocks: `release.provider.claude`, `release.provider.codex`, `release.provider.cursor`,
  `release.provider.antigravity` (LIVE_PROOF), by making the live-conformance lane able to reach `PONG`
  instead of the neutral `POLICY_LIVE_ADMISSION_DISABLED`. Also a predecessor of the offline five-plane
  `release.transaction-demo` once dispatch exists.
- Procedure: [`../runbooks/live-conformance.md`](../runbooks/live-conformance.md) §2–§3; rotation in
  [`../runbooks/signer-rotation.md`](../runbooks/signer-rotation.md) §1.
- Ratification line (verbatim format from `live-conformance.md` §2.3):
  `## <UTC> — operator — POLICY-GENERATION-2 — RATIFIED: live_admission_enabled=true with key <key_id>; evolutionary authority stays false.`
- Prerequisite landed: the Kernel policy loader mirrors the v1alpha2 rule since Kernel `0d848f6`
  (`crates/application/src/policy_snapshot/live.rs`); only the operator act stands between the neutral
  refusal and a receipt.
- Status: `OPEN` — no `— operator — POLICY-GENERATION-2 — RATIFIED` line exists in `AGENT_CHAT.md` as of 2026-08-25 (the only `RATIFIED` word in the log is an orchestrator claim arbitration, not an operator decision).

### OD-B — Jeryu test authority: authentication and a scoped test repository

- Decision: configure the local forge host entry with `jeryu gh-setup --host http://127.0.0.1:8787
  --token-file ~/.jeryu/secrets/merge-token`, name one scoped test repository the family may push candidate
  refs to and read back from, and ratify both. The running forge must not be modified to work around a
  missing capability.
- **Do not run `gh auth login`, `gh auth refresh`, or any credential-store token hunting against a Jeryu
  host.** All three are on Jeryu's own do-not-run list, published by the running instance at
  `GET http://127.0.0.1:8787/.jeryu/capabilities` under `gh_auth_policy.do_not_run`, and repeated in
  `jeryu-deploy/docs/errors.md`: "GitHub.com auth and local Jeryu host auth are separate; do not run
  gh auth login for Jeryu hosts." A login flow against a Jeryu host returns a guided refusal, not a device
  flow, so the older instruction to use `gh auth login -h 127.0.0.1:8787` could never have succeeded.
  If `gh` later reports a stale or invalid token for the host, the repair is to rerun the same `gh-setup`
  command — never a login or refresh flow.
- Why not code: credentials and repository custody. Every Jeryu integration path today refuses without
  positive online authority (BulletGit fail-closed gateway; effect broker read-back).
- Unblocks: `release.forge.jeryu` (LIVE_PROOF: protected integration plus UNKNOWN/read-back reconciliation
  receipt); it is also the first input to OD-D and to the live half of
  [`../runbooks/effect-reconciliation.md`](../runbooks/effect-reconciliation.md).
- Procedure: closure plan V1-S8 step 1 (read-only probes before protected integration, then
  read-back/reconciliation). No runbook yet; it is written after the first receipt. The one procedural step
  that is fixed today is producing the token file **without the token ever entering argv, an environment
  variable, or a log**:

  ```bash
  install -d -m 700 "$HOME/.jeryu" "$HOME/.jeryu/secrets"
  ( umask 077
    IFS= read -r -s -p 'Jeryu host token: ' token </dev/tty
    printf '%s\n' "$token" > "$HOME/.jeryu/secrets/merge-token" )
  jeryu gh-setup --host http://127.0.0.1:8787 --token-file "$HOME/.jeryu/secrets/merge-token"
  ```

  `read -s` does not echo; `printf` is a shell builtin, so no process is ever spawned with the token in its
  `argv`; nothing is exported, so it never reaches a child's environment; `umask 077` makes the file `0600`
  before the first byte is written; the subshell drops the variable on exit; and shell history records the
  command text, never what `read` consumed. Only the *path* is passed to `gh-setup`. Two consequences the
  operator must accept, both read off `jeryu-cli/src/commands/gh_setup.rs`: `gh-setup` writes the token in
  cleartext into the GitHub CLI's `hosts.yml`, so that file becomes secret material with the same custody as
  the token file; and `--print` renders the same entry — token included — to stdout, so it must never be used
  in a logged or shared session. `--token <value>` is available and must not be used: it puts the secret in
  `argv`.
- Ratification line:
  `## <UTC> — operator — JERYU-TEST-AUTHORITY — RATIFIED: 127.0.0.1:8787 host entry configured via jeryu gh-setup --token-file as <principal>; scoped test repository <owner/slug>; read-back only until the integration receipt is registered; forge unmodified.`
- Status: `OPEN`.

### OD-C — GitHub App test repository

- Decision: create or designate a GitHub App with separate delivery and attestation credentials, install it
  on one protected test repository the family may mutate, and ratify the identifiers.
- Why not code: an external account, App registration, and branch-protection settings. GitHub is a required,
  separately configured effect adapter, never source authority (ADR 0002, ADR 0008).
- Unblocks: `release.forge.github-app` (LIVE_PROOF: exact-subject integration and reconciliation receipt).
- Procedure: none written; the adapter is specified, not certified
  ([`../assurance/product-gaps.md`](../assurance/product-gaps.md) G7).
- Ratification line:
  `## <UTC> — operator — GITHUB-APP-TEST-REPO — RATIFIED: App <app id> installed on <owner/repo>; branch protection enabled; delivery and attestation credentials are distinct; the family may mutate this repository only.`
- Status: `OPEN`.

### OD-D — Signed schema-3 lock inputs

- Decision: supply the authenticated Jeryu URL and member slugs, the signer identity that will sign member
  tags and the hub tag, and the exact member subjects (tag, commit/tree, dependency locks, generated-artifact
  digests, checksums), so that `bullet-family lock generate --tag VERSION` can produce a schema-3
  `family.lock` and `checkout verify` can return a clean verdict.
- Why not code: authenticated source metadata and a tag signer. The checked-in lock is schema 2 and is refused
  before mutation by design ([`../runbooks/schema-removal.md`](../runbooks/schema-removal.md)).
- Unblocks: `release.installable-lock`, then `release.installer-twice` and `release.manifest-non-circular`
  (which bind the schema-3 lock); indirectly every RELEASE_PROOF gate that starts "from tagged bytes".
- Procedure: [`../runbooks/source-setup.md`](../runbooks/source-setup.md) "Future trusted installation";
  [`../runbooks/setup-recovery.md`](../runbooks/setup-recovery.md) for the two-run proof afterwards.
- Ratification line:
  `## <UTC> — operator — SCHEMA-3-LOCK-INPUTS — RATIFIED: Jeryu <url>; slugs <hub, kernel, git, portal>; tag signer <principal> <fingerprint>; member tags <tag>; lock generate may run against these subjects.`
- Status: `OPEN` (depends on OD-B for the authenticated forge).

### OD-E — Release-signing key custody

- Decision: provision the release signing key (Ed25519, OpenSSH allowed-signers form) under protected
  custody, publish the signer policy that scopes it to receipt kinds and a validity interval (namespace
  `bullet-farm-release-receipt-v1`), update `release/allowed_signers`, and ratify the fingerprint and
  custody location.
- Why not code: key custody and the external signer policy (ADR 0005 distinct purposes; ADR 0010 explicit
  policy input). The verifiers exist; the key does not.
- Unblocks: `release.signatures`, `release.receipt-contracts`, `release.provenance` (RELEASE_PROOF).
- Procedure: [`../runbooks/signer-rotation.md`](../runbooks/signer-rotation.md) §2 records what exists and the
  invariants a future procedure must satisfy; no signing procedure is written before the key exists.
- Ratification line:
  `## <UTC> — operator — RELEASE-SIGNING-CUSTODY — RATIFIED: release signing key <fingerprint> held at <custody>; signer policy <path or digest>; allowed_signers updated in hub commit <oid>.`
- Status: `OPEN`.

## Consequence

- A gate named above moves only when its receipt is registered, never when its decision is ratified;
  ratification is the precondition, the lane run is the evidence.
- Adding a decision means adding an entry here with a gate id and a ratification format, in the same
  reviewed change that adds the consuming procedure. Removing one means its ratification line exists and the
  entry is marked `RATIFIED <UTC>` with the log reference; entries are not deleted.
- No agent writes a `— operator —` line. The orchestrator quotes it; the operator writes it.

### OD-F — Release scope: Linux-only preview or amend the five-archive rule

Status: OPEN (added 2026-08-25 by the launch plan). `docs/assurance/v1-closure-plan.md` freezes the V1 release at exactly
five archives (Linux x86_64/aarch64, macOS x86_64/arm64, Windows x64). Everything agents can build on this host produces
a Linux x86_64 archive only (`release.package-matrix` and `release.platform-containment` stay BLOCKED without the other
hosts). Decide one of: (a) ship milestone M4 as an explicitly labelled **Linux preview outside the release gate** (no
release tag, no "V1" wording; the release-truth page keeps both gates BLOCKED), or (b) amend the five-archive rule by a
reviewed ADR so a Linux-only release can be a V1 release. Unblocks: how `docs/assurance/launch-plan.md` M4 may be
described; nothing else. Ratification line: `## <UTC> — operator — OD-F — RATIFIED: (a) Linux preview outside the gate | (b) amend five-archive rule`.

### OD-G — Public names, endpoints, and deployment identity

Status: OPEN (added 2026-08-25). `git.neverhuman.org`, `github.com/neverhuman/*`, Jeryu endpoint/DNS/TLS/backup, and the
deployment identity are unratified (`docs/workplan.md` WP-08/WP-14/WP-17). No lock, credential, publication, or receipt
may use them before ratification. Unblocks: public mirror topology, immutable paper/source permalinks, hosted family CI
provisioning (R-49). Ratification line: `## <UTC> — operator — OD-G — RATIFIED: <names/endpoints>`.

