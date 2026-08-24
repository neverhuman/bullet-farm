#!/usr/bin/env bash
# Run the end-to-end demonstration: planning council, fenced runner on a real
# private clone, independent verifier process, and a brokered local-forge
# effect with a read-back receipt. BULLET_PROVIDER=sim runs it fully offline.
# Live provider execution stays default-denied until an admission validator
# grants it; refusals are typed in the receipt, never painted green.
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
KERNEL="$FAMILY/bullet-kernel"
BULLETGIT="$FAMILY/bullet-git"
DATA="${BULLET_DATA_DIR:-$KERNEL/target/demo-live}"
PROVIDER="${BULLET_PROVIDER:-claude}"
GITD_BIN="${BULLET_GITD_BIN:-$BULLETGIT/target/debug/bullet-gitd}"

echo "== Bullet Farm demo-live =="
echo "kernel:   $KERNEL"
echo "data:     $DATA"
echo "provider: $PROVIDER"

if [[ ! -f "$KERNEL/Cargo.toml" ]]; then
  echo "bullet-kernel checkout missing at $KERNEL" >&2
  exit 1
fi

# The workspace daemon is the sole writer of every private clone.
if ! (cd "$BULLETGIT" && cargo build -q -p bullet-gitd); then
  if [[ -x "$GITD_BIN" ]]; then
    echo "WARNING: bullet-gitd build failed; using existing binary $GITD_BIN" >&2
  else
    echo "bullet-gitd build failed and no binary at $GITD_BIN" >&2
    exit 1
  fi
fi

(cd "$KERNEL" && cargo build -q -p bullet -p bullet-verifier)

# The kernel CLI surface decides what is admitted: prefer demo-live; fall
# back to the simulator-only synthetic scaffold when live is not admitted.
HELP="$(cd "$KERNEL" && cargo run -q -p bullet -- --help 2>/dev/null || true)"
if grep -q "demo-live" <<< "$HELP"; then
  SUBCOMMAND=(demo-live --provider "$PROVIDER")
elif grep -q "demo-synthetic" <<< "$HELP"; then
  if [[ "$PROVIDER" != "sim" ]]; then
    echo "NOTE: live provider execution is default-denied (no admission" >&2
    echo "validator); running the simulator-only synthetic scaffold." >&2
  fi
  SUBCOMMAND=(demo-synthetic)
else
  echo "bullet CLI exposes neither demo-live nor demo-synthetic" >&2
  exit 1
fi

mkdir -p "$DATA"
(cd "$KERNEL" &&
  BULLET_DATA_DIR="$DATA" \
  BULLET_GITD_BIN="$GITD_BIN" \
  BULLET_VERIFIER_BIN="$KERNEL/target/debug/bullet-verifier" \
  cargo run -q -p bullet -- "${SUBCOMMAND[@]}")

echo "== demo-live complete =="
