#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "family-contract lane: every member plus the hub canonical contract"
bash ops/ci/family.sh
bash ops/ci/contract.sh
log "family-contract lane passed"
