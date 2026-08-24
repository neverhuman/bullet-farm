#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export REPO_ROOT

log() {
  printf '[ci] %s\n' "$*"
}

require_file() {
  local path="$1"
  if [[ ! -f "$REPO_ROOT/$path" ]]; then
    echo "missing required file: $path" >&2
    exit 1
  fi
}
