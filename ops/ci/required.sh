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
# exactly the signed hub manifest even when readiness remains BLOCKED. It now
# reports that verdict in its exit status too: 0 READY, 3 BLOCKED (the family's
# "diagnosed, not usable" code, the same one `check` and `coord` use). Both are
# valid here, so this lane asserts the stronger property: the exit status and
# the JSON body must agree, and any other exit status fails the lane.
doctor_report="$(mktemp)"
doctor_exit=0
cargo run --locked --quiet --bin bullet-family -- doctor --json >"$doctor_report" || doctor_exit=$?
doctor_status="$(grep -o '"status": "[A-Z]*"' "$doctor_report" | head -n 1 | cut -d'"' -f4)"
case "$doctor_exit:$doctor_status" in
  0:READY|3:BLOCKED) log "doctor $doctor_status (exit $doctor_exit)" ;;
  *)
    echo "doctor exit $doctor_exit does not match reported status ${doctor_status:-<none>}" >&2
    cat "$doctor_report" >&2
    rm -f "$doctor_report"
    exit 1
    ;;
esac
rm -f "$doctor_report"
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
  schema_smoke="$(mktemp)"
  if cargo run --locked --quiet --bin bullet-family -- setup \
    --root "${REPO_ROOT%/*}" --source jeryu --offline >"$schema_smoke" 2>&1
  then
    echo "legacy lock unexpectedly authorized direct setup" >&2
    rm -f "$schema_smoke"
    exit 1
  fi
  grep -q 'UNSUPPORTED_SCHEMA' "$schema_smoke" || {
    cat "$schema_smoke" >&2
    rm -f "$schema_smoke"
    exit 1
  }
  rm -f "$schema_smoke"

  setup_smoke="$(mktemp)"
  setup_shim="$(mktemp -d)"
  setup_marker="$setup_shim/ambient-cargo-executed"
  printf '%s\n' '#!/bin/sh' ": > \"\${BULLET_SETUP_CARGO_MARKER:?}\"" 'exit 99' \
    >"$setup_shim/cargo"
  chmod 700 "$setup_shim/cargo"
  if (cd /tmp && PATH="$setup_shim:$PATH" BULLET_SETUP_CARGO_MARKER="$setup_marker" \
    "$REPO_ROOT/scripts/setup.sh" --offline) >"$setup_smoke" 2>&1
  then
    echo "source wrapper unexpectedly ran without operator-pre-admitted bootstrap" >&2
    rm -f "$setup_smoke"
    rm -f "$setup_shim/cargo" "$setup_marker"
    rmdir "$setup_shim"
    exit 1
  fi
  grep -q 'operator-pre-admitted bootstrap unavailable' "$setup_smoke" || {
    cat "$setup_smoke" >&2
    rm -f "$setup_smoke"
    rm -f "$setup_shim/cargo" "$setup_marker"
    rmdir "$setup_shim"
    exit 1
  }
  if [[ -e "$setup_marker" ]]; then
    echo "source wrapper executed ambient Cargo before bootstrap admission" >&2
    rm -f "$setup_smoke"
    rm -f "$setup_shim/cargo" "$setup_marker"
    rmdir "$setup_shim"
    exit 1
  fi
  rm -f "$setup_smoke"
  rm -f "$setup_shim/cargo"
  rmdir "$setup_shim"
fi
log "required lane passed"
