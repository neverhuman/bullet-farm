#!/usr/bin/env bash
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
cd "$HUB"

resolve_on_path() {
  local name="$1"
  local resolved
  resolved="$(command -v -- "$name")" || {
    printf 'setup: required tool is unavailable: %s\n' "$name" >&2
    exit 4
  }
  [[ "$resolved" = /* ]] || {
    printf 'setup: required tool is not an absolute path: %s\n' "$name" >&2
    exit 4
  }
  printf '%s\n' "$resolved"
}

cargo_launcher="$(resolve_on_path cargo)"
cargo_resolved="$(/usr/bin/readlink -f -- "$cargo_launcher")"
cargo_bin="${BULLET_SETUP_CARGO_BIN:-}"
if [[ -z "$cargo_bin" ]]; then
  if [[ "${cargo_resolved##*/}" == rustup ]]; then
    rustup_launcher="$(resolve_on_path rustup)"
    cargo_bin="$("$rustup_launcher" which cargo)"
  else
    cargo_bin="$cargo_resolved"
  fi
fi
node_bin="${BULLET_SETUP_NODE_BIN:-$(/usr/bin/readlink -f -- "$(resolve_on_path node)")}"
npm_cli="${BULLET_SETUP_NPM_CLI:-$(/usr/bin/readlink -f -- "$(resolve_on_path npm)")}"

exec "$cargo_launcher" run --locked --quiet --manifest-path "$HUB/Cargo.toml" \
  --bin bullet-family -- setup --root "$FAMILY" --source jeryu \
  --cargo-bin "$cargo_bin" --node-bin "$node_bin" --npm-cli "$npm_cli" "$@"
