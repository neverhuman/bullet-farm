#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "family lane: hub required plus every member's required lane"
bash ops/ci/required.sh
FAMILY="$(cd "$REPO_ROOT/.." && pwd)"
for member in bullet-kernel bullet-git bullet-portal; do
  log "member: $member"
  (cd "$FAMILY/$member" && bash scripts/ci-local.sh required)
done
log "family lane passed"
