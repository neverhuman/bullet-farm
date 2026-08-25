#!/usr/bin/env bash
# Run the first-slice simulator demonstration and print receipts.
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
KERNEL="$FAMILY/bullet-kernel"
PORTAL="$FAMILY/bullet-portal"
if [[ -n "${BULLET_DATA_DIR:-}" ]]; then
  DATA="$BULLET_DATA_DIR"
  mkdir -p "$DATA"
else
  DEMO_ROOT="$KERNEL/target/demo"
  mkdir -p "$DEMO_ROOT"
  DATA="$(mktemp -d "$DEMO_ROOT/run.XXXXXXXX")"
fi

echo "== Bullet Farm demo =="
echo "evidence_class: SYNTHETIC_PROOF"
echo "release_gate_eligible: false"
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
