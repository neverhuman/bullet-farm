#!/usr/bin/env bash
# Run the first-slice simulator demonstration and print receipts.
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
KERNEL="$FAMILY/bullet-kernel"
PORTAL="$FAMILY/bullet-portal"
DATA="${BULLET_DATA_DIR:-$KERNEL/target/demo}"
mkdir -p "$DATA"

echo "== Bullet Farm demo =="
echo "kernel: $KERNEL"
echo "data:   $DATA"

if [[ ! -f "$KERNEL/Cargo.toml" ]]; then
  echo "bullet-kernel checkout missing at $KERNEL" >&2
  exit 1
fi

(cd "$KERNEL" && BULLET_DATA_DIR="$DATA" cargo run -q -p bullet -- demo)

if [[ "${BULLET_DEMO_PORTAL:-0}" == "1" ]]; then
  echo "== portal smoke =="
  (cd "$PORTAL" && npm test --silent)
fi

echo "== demo complete =="
