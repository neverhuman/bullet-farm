#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
empty_path="$(mktemp -d)"
trap 'rm -rf -- "$empty_path"' EXIT
for tool in b3sum curl node npm realpath rustup lychee cargo-llvm-cov; do
  set +e
  output="$(PATH="$empty_path" /bin/bash "$REPO_ROOT/scripts/ci-doctor.sh" all 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 && "$output" == *"missing $tool for all"* ]] || {
    printf '[ci] DOCTOR_ALL_INVENTORY_MISSING: %s status=%s\n' "$tool" "$status" >&2
    exit 1
  }
done
for lane_tool in \
  'lint cp' 'lint ln' \
  'coverage cmp' 'coverage comm' 'coverage ln' 'coverage wc' \
  'platform grep' \
  'toolchain-pinned date' 'toolchain-pinned grep' 'toolchain-pinned tee' \
  'toolchain-pinned wc' 'family-contract node'; do
  read -r lane tool <<<"$lane_tool"
  set +e
  output="$(PATH="$empty_path" /bin/bash "$REPO_ROOT/scripts/ci-doctor.sh" "$lane" 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 && "$output" == *"missing $tool for $lane"* \
    && "$output" == *"missing realpath for $lane"* ]] || {
    printf '[ci] DOCTOR_LANE_INVENTORY_MISSING: %s/%s status=%s\n' \
      "$lane" "$tool" "$status" >&2
    exit 1
  }
done
source_text="$(<"$REPO_ROOT/scripts/ci-doctor.sh")"
local_source="$(<"$REPO_ROOT/scripts/ci-local.sh")"
dollar='$'
[[ "$source_text" == *"\"${dollar}lane\" == links || \"${dollar}lane\" == all"* \
  && "$source_text" == *"\"${dollar}lane\" == coverage || \"${dollar}lane\" == all"* \
  && "$source_text" == *"\"${dollar}lane\" == family || \"${dollar}lane\" == family-contract || \"${dollar}lane\" == all"* \
  && "$source_text" == *"\"${dollar}lane\" == toolchain-pinned || \"${dollar}lane\" == all"* ]] || {
  echo '[ci] DOCTOR_ALL_VERSION_UNION_MISSING' >&2
  exit 1
}
[[ "$local_source" == *'family-contract) run_observed family-contract ops/ci/family-contract.sh'* ]] || {
  echo '[ci] FAMILY_CONTRACT_OBSERVATION_IDENTITY_DRIFT' >&2
  exit 1
}
printf '[ci] ci-doctor all-lane union guards passed\n'
