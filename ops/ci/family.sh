#!/usr/bin/env bash
# Hub family lock: onboarding surface + no sibling path deps + member SHAs.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"

log "family: hub metadata"
python3 scripts/check-hub.py
python3 scripts/check-path-deps.py

FAMILY_ROOT="$(cd "$REPO_ROOT/.." && pwd)"
if [[ -f "$FAMILY_ROOT/repos.manifest.toml" ]]; then
  log "family members:"
  for member in bullet-farm bullet-kernel bullet-git bullet-portal; do
    dir="$FAMILY_ROOT/$member"
    if [[ -d "$dir/.git" ]]; then
      sha="$(git -C "$dir" rev-parse --short HEAD)"
      branch="$(git -C "$dir" rev-parse --abbrev-ref HEAD)"
      dirty=""
      if [[ -n "$(git -C "$dir" status --porcelain)" ]]; then
        dirty=" dirty"
      fi
      log "  $member $branch $sha$dirty"
    else
      log "  $member (not a checkout)"
    fi
  done
else
  log "no family container beside hub; skip member SHA report"
fi
log "family lane passed"
