#!/usr/bin/env bash
# Unsigned loopback operator console: farm init + farmd + Vite Portal +
# command-worker loop. Reuses scripts/dogfood/serve.sh --leave-bootstrap so
# the one-time token stays on disk for `bullet auth login`. Never prints the
# token, cookie, or CSRF.
set -euo pipefail
umask 077

HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
KERNEL="$FAMILY/bullet-kernel"
GIT="$FAMILY/bullet-git"
PORTAL="$FAMILY/bullet-portal"
# shellcheck source=ops/ci/toolchain-pins.sh
source "$HUB/ops/ci/toolchain-pins.sh"

usage() {
  cat <<'EOF'
usage: operator-console.sh --data-dir <abs dir under $HOME>
                           [--bind 127.0.0.1:7420]
                           [--portal-origin http://127.0.0.1:5173]
                           [--bullet <abs>] [--farmd <abs>]
       operator-console.sh --stop --data-dir <abs dir>
       operator-console.sh --help

Unsigned local console only. Not a trusted installer. HOLD remains.

--data-dir must be an absolute path under $HOME, outside this clone, and
not under /tmp. There is no implicit default.

Next commands (token is never printed):
  bullet auth login --farmd <farmd> --origin <portal-origin> --stdin < bootstrap_file
  bullet            (TTY; same as bullet tui)
  open <portal-origin>  (paste the token only if login has not consumed it)
EOF
}

refuse() {
  printf 'OPERATOR_CONSOLE_%s: %s\n' "$1" "$2" >&2
  exit 1
}

data_dir=""
bind="127.0.0.1:7420"
portal_origin="http://127.0.0.1:5173"
bullet_bin="${BULLET_BIN:-}"
farmd_bin="${BULLET_FARMD_BIN:-}"
stop=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --data-dir) data_dir="${2:-}"; shift 2 ;;
    --bind) bind="${2:-}"; shift 2 ;;
    --portal-origin) portal_origin="${2:-}"; shift 2 ;;
    --bullet) bullet_bin="${2:-}"; shift 2 ;;
    --farmd) farmd_bin="${2:-}"; shift 2 ;;
    --stop) stop=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) refuse ARG_UNKNOWN "$1" ;;
  esac
done

[[ -n "$data_dir" ]] || refuse DATA_DIR_REQUIRED "--data-dir is required; there is no default"
[[ "$data_dir" == /* ]] || refuse DATA_DIR_NOT_ABSOLUTE "$data_dir"
case "$data_dir" in
  /tmp|/tmp/*) refuse DATA_DIR_TMP "$data_dir" ;;
esac
[[ "$data_dir" == "$HOME"/* ]] || refuse DATA_DIR_NOT_UNDER_HOME "$data_dir"
[[ "$data_dir" != "$HOME" ]] || refuse DATA_DIR_UNTRUSTED "$data_dir"
case "$data_dir" in
  "$FAMILY"|"$FAMILY"/*|"$HUB"|"$HUB"/*) refuse DATA_DIR_INSIDE_CLONE "$data_dir" ;;
esac

if [[ "$stop" -eq 1 ]]; then
  worker_pid_file="$data_dir/worker.pid"
  if [[ -f "$worker_pid_file" ]]; then
    worker_pid="$(<"$worker_pid_file")"
    if [[ "$worker_pid" =~ ^[0-9]+$ ]] && kill -0 "$worker_pid" 2>/dev/null; then
      kill -TERM -- "-$worker_pid" 2>/dev/null || kill -TERM "$worker_pid" 2>/dev/null || true
    fi
    rm -f -- "$worker_pid_file"
  fi
  portal_pid_file="$data_dir/portal.pid"
  if [[ -f "$portal_pid_file" ]]; then
    portal_pid="$(<"$portal_pid_file")"
    if [[ "$portal_pid" =~ ^[0-9]+$ ]] && kill -0 "$portal_pid" 2>/dev/null; then
      kill -TERM -- "-$portal_pid" 2>/dev/null || kill -TERM "$portal_pid" 2>/dev/null || true
    fi
    rm -f -- "$portal_pid_file"
  fi
  exec bash "$HUB/scripts/dogfood/serve.sh" --stop --data-dir "$data_dir"
fi

[[ "$bind" =~ ^127\.0\.0\.1:[0-9]+$ ]] || refuse BIND_NOT_LOOPBACK "$bind"
[[ "$portal_origin" =~ ^http://(127\.0\.0\.1|localhost):[0-9]+$ ]] \
  || refuse PORTAL_ORIGIN_NOT_LOOPBACK "$portal_origin"
[[ -d "$KERNEL" && -d "$PORTAL" ]] || refuse FAMILY_LAYOUT "need bullet-kernel and bullet-portal siblings"

for tool in cargo curl node npm setsid; do
  command -v "$tool" >/dev/null 2>&1 || refuse TOOL_MISSING "$tool"
done
[[ "$(node --version)" == "v$PINNED_NODE_VERSION" ]] \
  || refuse NODE_PIN "expected Node v$PINNED_NODE_VERSION, found $(node --version)"
[[ "$(npm --version)" == "$PINNED_NPM_VERSION" ]] \
  || refuse NPM_PIN "expected npm $PINNED_NPM_VERSION, found $(npm --version)"

built_kernel=0
if [[ -z "$farmd_bin" || -z "$bullet_bin" ]]; then
  ( cd "$KERNEL" && cargo build --locked \
      -p bullet --bin bullet --bin transaction_offline \
      -p bullet-farmd --bin bullet-farmd \
      -p bullet-runner --bin bullet-runner --bin bullet-command-worker \
      -p bullet-verifier --bin bullet-verifier-fixture \
      --features bullet-verifier/fixture-executor ) \
    || refuse BUILD_FAILED "cargo build --locked console worker subjects"
  built_kernel=1
  [[ -n "$bullet_bin" ]] || bullet_bin="$KERNEL/target/debug/bullet"
  [[ -n "$farmd_bin" ]] || farmd_bin="$KERNEL/target/debug/bullet-farmd"
fi
[[ "$bullet_bin" == /* && -x "$bullet_bin" ]] || refuse BULLET_BIN_INVALID "$bullet_bin"
[[ "$farmd_bin" == /* && -x "$farmd_bin" ]] || refuse FARMD_BIN_INVALID "$farmd_bin"
bullet_bin="$(realpath -e -- "$bullet_bin")"
farmd_bin="$(realpath -e -- "$farmd_bin")"

if [[ ! -e "$data_dir" ]]; then
  mkdir -m 0700 -- "$data_dir"
fi
if [[ ! -e "$data_dir/logs" ]]; then
  mkdir -m 0700 -- "$data_dir/logs"
fi
export BULLET_DATA_DIR="$data_dir"
"$bullet_bin" farm init || refuse FARM_INIT_FAILED "$data_dir"

(
  cd "$PORTAL"
  node ops/ci/preinstall-scan.mjs
  npm ci --ignore-scripts --no-audit --no-fund
)

# serve.sh admits the 0700 data dir and starts farmd with the token left in place.
serve_out="$(mktemp)"
if ! bash "$HUB/scripts/dogfood/serve.sh" \
  --data-dir "$data_dir" \
  --bind "$bind" \
  --portal-origin "$portal_origin" \
  --farmd "$farmd_bin" \
  --leave-bootstrap >"$serve_out"; then
  kill -TERM -- "-$portal_pid" 2>/dev/null || true
  rm -f -- "$data_dir/portal.pid"
  cat "$serve_out" >&2 || true
  rm -f -- "$serve_out"
  refuse FARMD_START_FAILED "see $data_dir/logs"
fi
serve_report="$(cat "$serve_out")"
rm -f -- "$serve_out"
printf '%s\n' "$serve_report"

setsid bash "$HUB/scripts/portal.sh" >"$data_dir/logs/portal.log" 2>&1 &
portal_pid=$!
printf '%s\n' "$portal_pid" >"$data_dir/portal.pid"

farmd_url="$(printf '%s\n' "$serve_report" | sed -n 's/^farmd=//p' | tail -n 1)"
[[ "$farmd_url" =~ ^http://127\.0\.0\.1:[0-9]+$ ]] || farmd_url="http://${bind}"
printf 'portal=%s\n' "$portal_origin"

kernel_debug="${CARGO_TARGET_DIR:-$KERNEL/target}/debug"
worker_bin="${BULLET_COMMAND_WORKER_BIN:-$kernel_debug/bullet-command-worker}"
runner_bin="${BULLET_RUNNER_BIN:-$kernel_debug/bullet-runner}"
verifier_bin="${BULLET_VERIFIER_FIXTURE_BIN:-$kernel_debug/bullet-verifier-fixture}"
transaction_offline_bin="${BULLET_TRANSACTION_OFFLINE_BIN:-$kernel_debug/transaction_offline}"
gitd_bin="${BULLET_GITD_BIN:-$GIT/target/debug/bullet-gitd}"
loop_sh="$HUB/scripts/dogfood/worker-loop.sh"
if [[ "$built_kernel" -eq 1 && -f "$GIT/crates/bullet-gitd/Cargo.toml" && ! -x "$gitd_bin" ]]; then
  ( cd "$GIT" && cargo build --locked -p bullet-gitd --bin bullet-gitd ) \
    || refuse GITD_BUILD_FAILED "cargo build --locked -p bullet-gitd --bin bullet-gitd"
  gitd_bin="$GIT/target/debug/bullet-gitd"
fi
worker_reason=""
if [[ ! -x "$loop_sh" ]]; then
  worker_reason=WORKER_LOOP_MISSING
elif [[ ! -f "$data_dir/session.json" ]]; then
  worker_reason=SESSION_FILE_MISSING
else
  missing=""
  for labeled in "WORKER:$worker_bin" "RUNNER:$runner_bin" "GITD:$gitd_bin" \
      "VERIFIER:$verifier_bin" "TRANSACTION_OFFLINE:$transaction_offline_bin"; do
    label="${labeled%%:*}"
    path="${labeled#*:}"
    if [[ ! -x "$path" ]]; then
      missing="${missing:+$missing,}$label"
    fi
  done
  if [[ -n "$missing" ]]; then
    worker_reason="WORKER_MANIFEST_UNBOUND:$missing"
  fi
fi
if [[ -z "$worker_reason" ]]; then
  worker_bin="$(realpath -e -- "$worker_bin")"
  runner_bin="$(realpath -e -- "$runner_bin")"
  gitd_bin="$(realpath -e -- "$gitd_bin")"
  verifier_bin="$(realpath -e -- "$verifier_bin")"
  transaction_offline_bin="$(realpath -e -- "$transaction_offline_bin")"
fi
if [[ -n "$worker_reason" ]]; then
  printf 'worker=UNBOUND reason=%s\n' "$worker_reason"
else
  manifest="$data_dir/worker/binary-manifest.json"
  mkdir -m 0700 -p -- "$data_dir/worker"
  setsid bash "$loop_sh" \
    --data-dir "$data_dir" \
    --manifest "$manifest" \
    --worker "$worker_bin" \
    --transaction-offline "$transaction_offline_bin" \
    --farmd "$farmd_bin" \
    --runner "$runner_bin" \
    --gitd "$gitd_bin" \
    --verifier "$verifier_bin" \
    >"$data_dir/logs/worker-loop.stdout" 2>"$data_dir/logs/worker-loop.stderr" &
  worker_pid=$!
  printf '%s\n' "$worker_pid" >"$data_dir/worker.pid"
  sleep 0.2
  if ! kill -0 "$worker_pid" 2>/dev/null; then
    wait "$worker_pid" 2>/dev/null || true
    printf 'worker=UNBOUND reason=WORKER_START_FAILED see=%s/logs/worker-loop.stderr\n' "$data_dir"
    rm -f -- "$data_dir/worker.pid"
  else
    printf 'worker=started pid=%s\n' "$worker_pid"
    printf 'manifest=%s\n' "$manifest"
  fi
fi

printf 'next=bullet auth login --farmd %s --origin %s --stdin < bootstrap_file\n' \
  "$farmd_url" "$portal_origin"
printf 'next=bullet tui\n'
printf 'next=bullet\n'
printf 'note=just dev cannot create a session; this wrapper can.\n'
printf 'note=unsigned local console; HOLD remains; not VERIFIED.\n'
printf 'note=coding stop is STOP_UNIMPLEMENTED; Ctrl+C detaches the TUI only.\n'
