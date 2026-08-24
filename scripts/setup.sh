#!/usr/bin/env bash
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

need rustc
need cargo
need node
need npm
need git
need rustup
need just

if [[ ! -f "$FAMILY/repos.manifest.toml" ]]; then
  echo "setup blocked: expected split-family manifest at $FAMILY/repos.manifest.toml" >&2
  echo "this alpha cannot install sibling repositories from a hub-only clone; run 'cargo run --locked --quiet --bin bullet-family -- doctor --json' for the exact blockers" >&2
  exit 1
fi
for member in bullet-farm bullet-kernel bullet-git bullet-portal; do
  if [[ ! -d "$FAMILY/$member/.git" ]]; then
    echo "setup blocked: $FAMILY/$member is missing or is not an ordinary Git checkout" >&2
    echo "restore the canonical split-family checkout without Git worktrees, then rerun setup" >&2
    exit 1
  fi
done

rustup component add rustfmt clippy >/dev/null

echo "generating and syncing canonical family contracts"
(cd "$HUB" && cargo run --locked --quiet -p bullet-wire --bin bullet-contract -- generate --root .)
bash "$HUB/scripts/sync-family-contracts.sh" write

echo "fetching rust crates"
(cd "$FAMILY/bullet-kernel" && cargo fetch --locked)
(cd "$FAMILY/bullet-git" && cargo fetch --locked)

echo "installing portal dependencies"
(cd "$FAMILY/bullet-portal" && npm ci --no-fund --no-audit)
(cd "$FAMILY/bullet-portal" && npx --no-install playwright install chromium)

echo "generating portal client from kernel contract"
(cd "$FAMILY/bullet-kernel" && cargo run --locked --quiet -p bullet -- contracts generate)
cp "$FAMILY/bullet-kernel/contracts/generated/api.ts" \
  "$FAMILY/bullet-portal/src/generated/api.ts"

echo "setup complete"
echo "next: just demo"
