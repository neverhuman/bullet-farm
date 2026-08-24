#!/usr/bin/env bash
# Family contract. No Rust compile, no Playwright, no live models.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "contract lane: family lock"
bash ops/ci/family.sh
log "contract lane passed"
