#!/usr/bin/env bash
# Rapid component smoke for xbabe2 engineering.
# Runs each member's existing scripts/ci-local.sh fast lane in family order.
# Not required, not family Evidence, not hosted CI, not dogfood PASS, not release.
set -euo pipefail

HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  printf '%s\n' \
    "dogfood-rapid: existing member fast lanes only" \
    "set absolute checkouts:" \
    "  BULLET_GIT_ROOT" \
    "  BULLET_KERNEL_ROOT" \
    "  BULLET_PORTAL_ROOT" \
    "optional: DOGFOOD_RAPID_HUB_ROOT (defaults to this hub)" \
    "refuses inherited CARGO_TARGET_DIR / BULLET_CI_PROOF_CUSTODY" >&2
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ ${CARGO_TARGET_DIR+x} || ${BULLET_CI_CARGO_TARGET_DIR+x} \
    || ${BULLET_CI_PROOF_CUSTODY+x} ]]; then
  printf 'dogfood-rapid: refuse inherited CARGO_TARGET_DIR or BULLET_CI_PROOF_CUSTODY\n' >&2
  exit 75
fi

: "${BULLET_GIT_ROOT:?set absolute bullet-git checkout}"
: "${BULLET_KERNEL_ROOT:?set absolute bullet-kernel checkout}"
: "${BULLET_PORTAL_ROOT:?set absolute bullet-portal checkout}"
HUB_ROOT="${DOGFOOD_RAPID_HUB_ROOT:-$HUB}"

require_repo() {
  local name="$1" root="$2"
  [[ "$root" == /* && -d "$root/.git" && -f "$root/scripts/ci-local.sh" ]] || {
    printf 'dogfood-rapid: %s is not an absolute git checkout with scripts/ci-local.sh: %s\n' \
      "$name" "$root" >&2
    exit 1
  }
}

require_repo bullet-git "$BULLET_GIT_ROOT"
require_repo bullet-kernel "$BULLET_KERNEL_ROOT"
require_repo bullet-portal "$BULLET_PORTAL_ROOT"
require_repo bullet-farm "$HUB_ROOT"

run_fast() {
  local name="$1" root="$2"
  printf 'dogfood-rapid: %s fast in %s\n' "$name" "$root"
  (
    cd "$root"
    unset CARGO_HOME CARGO_TARGET_DIR BULLET_CI_CARGO_TARGET_DIR BULLET_CI_PROOF_CUSTODY
    bash scripts/ci-local.sh fast
  )
}

run_fast bullet-git "$BULLET_GIT_ROOT"
run_fast bullet-kernel "$BULLET_KERNEL_ROOT"
run_fast bullet-portal "$BULLET_PORTAL_ROOT"
run_fast bullet-farm "$HUB_ROOT"
printf 'dogfood-rapid: four member fast lanes returned 0 (component smoke only)\n'
