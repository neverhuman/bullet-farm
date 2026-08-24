#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "security lane"
for tool in gitleaks cargo-deny; do
  command -v "$tool" >/dev/null 2>&1 || {
    echo "[ci] missing required tool: $tool" >&2
    exit 1
  }
done
gitleaks detect --source . --no-git --redact --no-banner
cargo deny check bans
log "security lane passed"
