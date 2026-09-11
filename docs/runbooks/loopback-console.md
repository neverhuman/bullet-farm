# Unsigned loopback operator console

Status: **contributor procedure; not installation or release authority**  
Owner: Bullet Farm maintainers  
Last reviewed: 2026-09-11

This is the stranger path: clone [neverhuman/bulletfarm](https://github.com/neverhuman/bulletfarm),
build from source, start loopback farmd, log in, and open the HOLD-honest TUI
and Portal. It is not G1. It does not replace [`source-setup.md`](source-setup.md)
or authorize `just setup`.

Evidence class: local observation. A successful walk is not VERIFIED, not a
trusted installer, and not a Claude replace. Operating HOLD remains.

## Prerequisites

| Need | Pin / rule |
| --- | --- |
| OS | Linux (farm init and farmd refuse other platforms) |
| Rust | 1.95.0 (`rust-toolchain.toml` in each Rust member) |
| Node | 22.23.2 |
| npm | 10.9.8 |
| Clone | The **bulletfarm** aggregate (four sibling trees). A hub-only clone is not enough. |
| Data dir | Absolute path under `$HOME`, mode 0700, **not** inside the clone, **not** under `/tmp` |

Do not copy an existing operator `session.json`. Do not paste bootstrap tokens
into issues, AGENT_CHAT, or README examples.

## What you should see

| Step | Honest result |
| --- | --- |
| `just preview` | `doctor` **BLOCKED**, exit **3**. That is success for the schema-2 lock. |
| `just console` | Non-secret lines only: `origin=`, `farmd=`, `pid=`, `data_dir=`, `bootstrap_file=`, `session_file=`, `portal=`, and either `worker=started` or a typed `worker=UNBOUND reason=`. |
| `bullet` (TTY) or `bullet tui` | CONNECTING, then `HOLD · LIVE n · UNBOUND · HEAD_RUNTIME_BINDING_REQUIRED · STOP_UNIMPLEMENTED`. Empty fleet is zero rows, not a green fleet. |
| Portal Shift Brief | `RELEASE DECISION: unknown`. |
| Portal Head | Send stays blocked (`HEAD_RUNTIME_BINDING_REQUIRED`). |
| Portal Control Tower | Same command ids the TUI shows. A submit can leave `PENDING` and become an attempt **or** a typed harness/manifest refusal. LIVE n is not “agents finished”. |

`just dev` starts HTTP shells **without** a bootstrap token. It cannot create a
session. Use `just console`.

## Procedure

From the aggregate root (the directory that contains `bullet-farm/`,
`bullet-kernel/`, `bullet-git/`, and `bullet-portal/`):

```bash
cd bullet-farm
just preview
# doctor BLOCKED / exit 3 is the expected install diagnosis

just console -- --data-dir "$HOME/.local/state/bullet-operator-console"
```

The wrapper builds `bullet`, `bullet-farmd`, the command-worker subjects, and
sibling `bullet-gitd` when those bins are unset, runs `farm init`, starts farmd
on `127.0.0.1:7420` with `--leave-bootstrap`, starts Vite on `127.0.0.1:5173`
with a matching `--portal-origin`, and starts `scripts/dogfood/worker-loop.sh`
with a `bullet.command-worker-binary-manifest.v1` when every subject exists.
If `$data_dir/harness/harness.env` exists, the wrapper passes `--env-file`.
`COMMAND_CODING_HARNESS_UNBOUND` is idle-backoff, not a dead loop. Missing
gitd or a binary is `worker=UNBOUND`, not a hang. It never prints the token.
`--stop` signals this invocation's worker, Portal, and farmd groups.

If `127.0.0.1:7420` is already bound, pass `--bind 127.0.0.1:<free-port>`
(and a matching free Portal origin if 5173 is taken). Do not restomp an
existing operator farmd unless you intend to.

Login consumes the one-time token. Use a redirect; do not `cat` the file:

```bash
cd ../bullet-kernel
# Use the farmd= and origin= / bootstrap_file= lines printed by just console.
# After login, no-args `bullet` is the TUI (same as `bullet tui`).
./target/debug/bullet auth login \
  --farmd http://127.0.0.1:7420 \
  --origin http://127.0.0.1:5173 \
  --stdin < "$HOME/.local/state/bullet-operator-console/custody/bootstrap.token"
./target/debug/bullet
```

Open <http://127.0.0.1:5173>. If login already consumed the token, the browser
needs its own cookie: use **Check session** only when this browser already
authenticated, or provision a new bootstrap file with
`bullet-farmd --provision-bootstrap-token` (never print it) and paste it into
the Portal token field. Head Send stays blocked. `bullet coding stop` stays
`STOP_UNIMPLEMENTED`. Ctrl+C detaches the TUI; farmd work continues.

After a `coding submit`, `bullet coding harness-bind` produces the three
ledger identities. `prepare-harness.sh` still writes the other bindings.
`harness-check` is BOUND only when all seventeen `BULLET_HARNESS_*` names are
present.

Stop:

```bash
cd bullet-farm
just console -- --stop --data-dir "$HOME/.local/state/bullet-operator-console"
```

## Cannot

- Sign or verify a schema-3 `family.lock`, or make `just setup` succeed
- Lift Operating HOLD or set `live_admission=true`
- Execute Portal Head Send or native `bullet talk` / `ask` / `head`
- Settle production `coding submit` as a Candidate / Evidence / Effect
- Claim VERIFIED, a trusted installer, or that LIVE n means agents finished
- Treat a Cursor text ping or a Codex `{stdout, exit}` pair as a turn

Stage-one VHS tapes stay under `docs/readme-media/`. Real xbabe2 GIFs live in
[`../../media/operator-console/`](../../media/operator-console/) and are not
replayable from this runbook.
