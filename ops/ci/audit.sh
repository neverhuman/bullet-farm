#!/usr/bin/env bash
# Jankurai audit lane. Writes artifacts for hosted CI and ratchets upward only.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"

AUDIT_FLOOR=53
command -v jankurai >/dev/null 2>&1 || {
  echo "[ci] missing required tool: jankurai" >&2
  exit 1
}
mkdir -p .jankurai
rm -f .jankurai/repo-score.json .jankurai/repo-score.md
log "audit lane: jankurai audit (floor ${AUDIT_FLOOR})"
jankurai audit . --no-score-history --fail-under "$AUDIT_FLOOR" --fail-on critical \
  --json .jankurai/repo-score.json --md .jankurai/repo-score.md
[[ -f .jankurai/repo-score.json && -f .jankurai/repo-score.md ]] || {
  echo "[ci] audit artifacts missing" >&2
  exit 1
}
log "audit lane passed"
