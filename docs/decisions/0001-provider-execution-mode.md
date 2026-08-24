# 0001 — Provider execution mode: providers propose, BulletGit writes

Status: Accepted architecture; live execution remains quarantined
Owner: Bullet Farm maintainers
Last reviewed: 2026-08-24
Applies to: bullet-kernel (provider harness, runner), bullet-git (`bullet-gitd`)

## Decision

Every provider turn is read-only inside a private workspace generation. A provider produces typed events and a
patch proposal; it never receives Git, effect, cloud, SSH, or general workspace-write authority. Kernel owns the
lease, policy, routing, budget, and gate authority. Runner supervises the read-only process. BulletGit is the only
component that may apply an authorized proposal to a workspace.

Provider discovery and `--version`/`--help` output are capability observations, not admission. Runtime probing of
the exact binary controls capability maturity; a provider name or stale documentation never does. Until the
provider validator, containment, credential projection, protocol conformance, and canary-secret gates pass, the
production boundary returns `LIVE_ADMISSION_UNAVAILABLE`. No environment flag, cached OAuth state, or test feature
may upgrade a probe into live authority.

## Frozen V1 adapter order and runtime observations

The adapter order is normative:

1. Claude bidirectional stream JSON.
2. Codex stable App Server JSON-RPC method set over stdio JSONL.
3. Cursor Agent Client Protocol (ACP).
4. Antigravity headless structured mode.

V1 GA requires current conformance receipts for **all four** adapters. `TEAM.md` critique C8's earlier suggestion
that any two providers could gate GA is historical provenance and is superseded by the frozen V1 closure plan.

On 2026-08-24, sanitized local `--version`/`--help` probes of absolute resolved binaries observed:

| Provider | Installed subject | Observed compatibility surface | Maturity consequence |
| --- | --- | --- | --- |
| Claude | Claude Code 2.1.241 | `--print`, `--input-format stream-json`, `--output-format stream-json`, `--permission-mode plan`, `--json-schema`, `--session-id`, `--resume`, and `--max-budget-usd` | Bidirectional stream JSON is the first implementation target; probe-only, not certified |
| Codex | Codex CLI 0.149.1 | `codex app-server` defaults to `stdio://` and exposes generated protocol schemas; this installed CLI still labels the command experimental | Use the frozen stable App Server JSON-RPC method set over JSONL, not `codex exec --json`; local maturity is not certified until the conformance suite passes |
| Cursor | Cursor Agent 2026.08.11-e8db854 | Top-level help exposes plan/ask and print JSON modes but does not advertise an ACP entry point | ACP remains the required transport; the local invocation is `UNKNOWN`, so stream JSON must not be substituted or admitted |
| Antigravity | `agy` 1.1.19 | Headless print mode exposes plan mode, sandboxing, text/JSON/stream-JSON input and output, `--json-schema`, and `--print-timeout` | Structured-schema capability is present in the runtime surface; conformance and containment are still unproved |

These observations deliberately make no credential, network, live-session, read-only-containment, resume, quota,
or contract-pass claim. An absent or ambiguous entry point is typed `UNKNOWN`, never guessed from another version.

Antigravity 1.1.19 retains an ordering-sensitive compatibility form:

```text
agy --sandbox --mode plan --print-timeout 10m --json-schema <schema> -p='<prompt>'
```

Put every flag before the final `-p=` prompt. The older `agy -p --sandbox ...` form is invalid for this subject
because `-p` consumes the following token as prompt text. The observed `--json-schema` surface removes the stale
ADR claim that structured output is unsupported; it does not make Antigravity certified without the shared live
conformance receipt.

## Proposal and gate authority

The only patch input accepted across the write boundary is the validated `bullet-wire::PatchProposal`. It binds:

- schema and proposal identity;
- the producing Attempt;
- base checkpoint ID and digest;
- each repository-relative operation, its exact absent/content-digest preimage, and its write/delete mutation;
- policy-admitted `gate_ids`.

Before mutation, the write gateway must reject unknown fields, duplicate or conflicting paths,
invalid/traversal/`.git` paths, stale preimages, and oversized content. Proposal application still requires
operation-specific Kernel authority and an online final active-lease/fence check; parsing a valid proposal grants
no authority by itself.

Provider text such as `tests_to_run`, a shell command, a claimed PASS, or a suggested check name is never shell or
completion authority. A proposal may reference only gate IDs already admitted by policy. Runner must resolve those
IDs through the trusted gate catalog and execute the corresponding fixed argv; it never executes model-authored text.
Zero tests, timeout, flaky, unsupported, infrastructure error, and `UNKNOWN` never equal PASS.

At most two bounded repair turns may receive typed gate results. Resume/fork support is used only when the exact
runtime descriptor reports a conformant capability; otherwise the adapter starts a new read-only turn or returns a
typed unsupported/unknown outcome. Provider output is evidence input, never independent verification.

## Process and protocol boundary

Production invocation must use an absolute verified binary path, allowlisted environment, ephemeral HOME/XDG state,
minimum provider-only OAuth projection, provider-only network egress, and no SCM/cloud/SSH credentials. Stdout is
reserved for the provider protocol. Runner must normalize native messages into `ProviderDescriptor`,
`ProviderEvent`, `SessionHandle`, `InvocationReceipt`, and `QuotaObservation`, while preserving
unknown/malformed/duplicate/delayed events as explicit outcomes.

Provider sandbox flags are defense in depth, not the authority boundary. Argv admission denies worktree creation,
ambient write modes, dangerous bypass flags, unapproved MCP/plugin/config injection, and inherited credential or
proxy state. Heartbeat loss, cancellation, timeout, malformed protocol, or provider death must freeze mutation and
terminate the complete provider process tree before salvage.

## Consequences

- Claude lands first, but no adapter is promoted by documentation or probe output.
- Codex exec JSON and Cursor print stream JSON are not fallback V1 transports for App Server JSONL and ACP.
- Antigravity's installed schema flag is eligible for use only after the same validator, containment, and
  conformance gates.
- Quota observations remain typed; `UNKNOWN` paid capacity blocks ordinary dispatch unless an explicit read-only
  probe policy permits a bounded measurement.
- The provider cannot write, select arbitrary tests, attest evidence, dispatch effects, or declare completion.
- Wave-0 quarantine remains honest until exact runtime receipts close every required gate.

## Stable shared vocabulary

- Evidence tiers: E0–E4. Writer output cannot satisfy an independent tier.
- Gate outcomes come only from the generated enum; only a runtime-valid `PASS` may satisfy a gate.
- Routing maturity is capability-specific: unsupported, experimental, supported, certified, or unknown.
- `SUPERSEDED` is a normal terminal state for scope succession, not only an exception.
