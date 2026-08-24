#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
bash scripts/ci-doctor.sh security
log "security lane"
gitleaks detect --source . --no-git --redact --no-banner
cargo deny check bans
zizmor .
log "security lane passed"
