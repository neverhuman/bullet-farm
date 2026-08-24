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
need python3
need git

rustup component add rustfmt clippy >/dev/null

echo "fetching rust crates"
(cd "$FAMILY/bullet-kernel" && cargo fetch)
(cd "$FAMILY/bullet-git" && cargo fetch)

echo "installing portal dependencies"
(cd "$FAMILY/bullet-portal" && npm install --no-fund --no-audit)

echo "generating portal client from kernel contract"
python3 "$FAMILY/bullet-kernel/scripts/generate-types.py"
cp "$FAMILY/bullet-kernel/contracts/generated/api.ts" \
  "$FAMILY/bullet-portal/src/generated/api.ts"

echo "setup complete"
echo "next: just demo"
