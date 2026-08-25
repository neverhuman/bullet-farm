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
# Doctor strictly decodes the active lock schema and proves its member set is
# exactly the signed hub manifest even when readiness remains BLOCKED.
cargo run --locked --quiet --bin bullet-family -- doctor --json >/dev/null
# The committed release-truth page is a generated zone rendered from the static
# gate inventory and hub-only inputs; a stale copy fails here, never silently.
log "release truth drift"
bash scripts/release-truth.sh check
for member in bullet-farm bullet-kernel bullet-git bullet-portal; do
  grep -q "\"$member\"" repos.manifest.toml || { echo "hub manifest missing member: $member" >&2; exit 1; }
  if [[ -f "$container_manifest" ]]; then
    grep -q "name = \"$member\"" "$container_manifest" || { echo "container manifest missing member: $member" >&2; exit 1; }
  fi
done
if grep -qx 'schema_version = "2"' family.lock; then
  setup_smoke="$(mktemp)"
  if (cd /tmp && "$REPO_ROOT/scripts/setup.sh" --offline) >"$setup_smoke" 2>&1; then
    echo "legacy lock unexpectedly authorized setup" >&2
    rm -f "$setup_smoke"
    exit 1
  fi
  grep -q 'UNSUPPORTED_SCHEMA' "$setup_smoke" || {
    cat "$setup_smoke" >&2
    rm -f "$setup_smoke"
    exit 1
  }
  rm -f "$setup_smoke"
fi
log "required lane passed"
