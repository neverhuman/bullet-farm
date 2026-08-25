#!/usr/bin/env bash
# Render the portable Release Truth page from machine facts and write or check
# the committed generated zone docs/assurance/release-truth.generated.md.
# The renderer keeps the release decision exit code (3 = BLOCKED); only a
# rendering failure (usage or I/O) stops this script.
set -euo pipefail

HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="${1:-write}"
TARGET="$HUB/docs/assurance/release-truth.generated.md"
STAGING=""

finish() {
  if [[ -n "$STAGING" ]]; then
    rm -f -- "$STAGING"
  fi
}
trap finish EXIT

if [[ "$MODE" != "write" && "$MODE" != "check" ]]; then
  echo "usage: release-truth.sh [write|check]" >&2
  exit 2
fi

mkdir -p -- "$(dirname "$TARGET")"
STAGING="$(mktemp "${TARGET}.XXXXXX")"
status=0
(cd "$HUB" && cargo run --locked --quiet --bin bullet-family -- check release --report --portable) \
  >"$STAGING" || status=$?
case "$status" in
  0|1|3) ;;
  *)
    echo "release-truth: renderer exited $status" >&2
    exit "$status"
    ;;
esac

if [[ "$MODE" == "check" ]]; then
  if [[ ! -f "$TARGET" ]] || ! cmp -s -- "$STAGING" "$TARGET"; then
    echo "generated release-truth drift: docs/assurance/release-truth.generated.md (repair: just release-truth)" >&2
    exit 1
  fi
  echo "release truth check passed (decision exit $status)"
  exit 0
fi

if [[ -f "$TARGET" ]] && cmp -s -- "$STAGING" "$TARGET"; then
  echo "release truth unchanged (decision exit $status)"
  exit 0
fi
mv -- "$STAGING" "$TARGET"
STAGING=""
echo "release truth written (decision exit $status)"
