#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "required lane"
bash ops/ci/fast.sh
log "manifest cross-checks"
container_manifest="$REPO_ROOT/../repos.manifest.toml"
require_file "repos.manifest.toml"
require_file "family.lock"
for member in bullet-farm bullet-kernel bullet-git bullet-portal; do
  grep -q "\"$member\"" repos.manifest.toml || { echo "hub manifest missing member: $member" >&2; exit 1; }
  grep -q "name = \"$member\"" family.lock || { echo "family.lock missing member: $member" >&2; exit 1; }
  if [[ -f "$container_manifest" ]]; then
    grep -q "name = \"$member\"" "$container_manifest" || { echo "container manifest missing member: $member" >&2; exit 1; }
  fi
done
log "required lane passed"
