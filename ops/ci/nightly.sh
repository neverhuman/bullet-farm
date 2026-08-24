#!/usr/bin/env bash
# Hub has no live harnesses. Skip so nightly never blocks.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "nightly lane: hub has no live adapters; skip"
exit 0
