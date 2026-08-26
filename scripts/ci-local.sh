#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
REPO_ROOT="$PWD"
# shellcheck source=ops/ci/artifact-path.sh
source "$REPO_ROOT/ops/ci/artifact-path.sh"

CI_PROOF_LOCK_DIR="$REPO_ROOT/.git/bullet-ci.lock.d"
CI_PROOF_LOCK_OWNER="$CI_PROOF_LOCK_DIR/owner"
CI_PROOF_LOCK_RECORD=""

proof_lock_refusal() {
  printf '%s\n' \
    "ci-local: CI_PROOF_LOCKED_OR_STALE: $CI_PROOF_LOCK_DIR is occupied or cannot be trusted" \
    "ci-local: verify that no scripts/ci-local.sh process is using this exact checkout; then inspect and explicitly reconcile only $CI_PROOF_LOCK_DIR" >&2
  return 75
}

verify_proof_lock() {
  [[ -d "$CI_PROOF_LOCK_DIR" && ! -L "$CI_PROOF_LOCK_DIR" \
    && -f "$CI_PROOF_LOCK_OWNER" && ! -L "$CI_PROOF_LOCK_OWNER" \
    && "$(<"$CI_PROOF_LOCK_OWNER")" == "$CI_PROOF_LOCK_RECORD" ]] || {
    proof_lock_refusal
    return 75
  }
}

acquire_proof_lock() {
  local lane="$1"
  [[ -d "$REPO_ROOT/.git" && ! -L "$REPO_ROOT/.git" ]] || {
    proof_lock_refusal
    return 75
  }
  if ! (umask 077; mkdir -- "$CI_PROOF_LOCK_DIR") 2>/dev/null; then
    proof_lock_refusal
    return 75
  fi
  [[ -d "$CI_PROOF_LOCK_DIR" && ! -L "$CI_PROOF_LOCK_DIR" ]] || {
    proof_lock_refusal
    return 75
  }
  CI_PROOF_LOCK_RECORD="schema=1 pid=$$ lane=$lane nonce=$$-${BASHPID:-$$}-$RANDOM-$RANDOM"
  if ! (umask 077; set -o noclobber; printf '%s\n' "$CI_PROOF_LOCK_RECORD" \
      >"$CI_PROOF_LOCK_OWNER") 2>/dev/null; then
    proof_lock_refusal
    return 75
  fi
  verify_proof_lock
}

release_proof_lock() {
  verify_proof_lock || return $?
  rm -- "$CI_PROOF_LOCK_OWNER" || {
    proof_lock_refusal
    return 75
  }
  rmdir -- "$CI_PROOF_LOCK_DIR" || {
    proof_lock_refusal
    return 75
  }
}

run_observed_locked() {
  local lane="$1" script="$2" status artifact command_count observation_status
  shift 2
  local -a produced=() commands=("bash scripts/ci-doctor.sh $lane")
  command -v realpath >/dev/null 2>&1 || {
    printf 'ci-local: missing required tool realpath before %s doctor\n' "$lane" >&2
    return 1
  }
  prepare_ci_directory "$REPO_ROOT" .ci-artifacts || {
    printf 'ci-local: unsafe .ci-artifacts root\n' >&2
    return 1
  }
  for artifact in "$@"; do
    [[ "$artifact" == .ci-artifacts/* ]] || {
      printf 'ci-local: unsafe artifact path %s\n' "$artifact" >&2
      return 1
    }
    prepare_ci_directory "$REPO_ROOT" "${artifact%/*}" || {
      printf 'ci-local: unsafe artifact parent %s\n' "${artifact%/*}" >&2
      return 1
    }
    if [[ -L "$artifact" || (-e "$artifact" && ! -f "$artifact") ]]; then
      printf 'ci-local: unsafe artifact subject %s\n' "$artifact" >&2
      return 1
    fi
    rm -f -- "$artifact" || {
      printf 'ci-local: cannot reset artifact %s\n' "$artifact" >&2
      return 1
    }
  done
  set +e
  bash scripts/ci-doctor.sh "$lane"
  status=$?
  if [[ "$status" -eq 0 ]]; then
    commands+=("bash $script")
    bash "$script"
    status=$?
  fi
  set -e
  if [[ "$status" -eq 0 ]]; then
    for artifact in "$@"; do
      [[ -f "$artifact" ]] && produced+=("$artifact")
    done
  fi
  command_count="${#commands[@]}"
  verify_proof_lock || return $?
  set +e
  CI_COMMAND_COUNT="$command_count" bash scripts/ci-observation.sh "$lane" "$status" \
    "${commands[@]}" "${produced[@]}"
  observation_status=$?
  set -e
  [[ "$observation_status" -eq 0 ]] || return "$observation_status"
  return "$status"
}

run_observed() {
  local lane="$1" status
  acquire_proof_lock "$lane" || return $?
  if run_observed_locked "$@"; then
    status=0
  else
    status=$?
  fi
  release_proof_lock || return $?
  return "$status"
}

lane="${1:-required}"
case "$lane" in
  source-scan) run_observed source-scan ops/ci/source-scan.sh ;;
  fast) run_observed fast ops/ci/fast.sh .ci-artifacts/junit/fast.xml ;;
  lint) run_observed lint ops/ci/lint.sh ;;
  contract) run_observed contract ops/ci/contract.sh \
    .ci-artifacts/junit/contract.xml .ci-artifacts/formal/contract.json \
    .ci-artifacts/formal/contract.log .ci-artifacts/contracts/bundle-manifest.json ;;
  security) run_observed security ops/ci/security.sh ;;
  docs) run_observed docs ops/ci/docs.sh ;;
  required) run_observed required ops/ci/required.sh \
    .ci-artifacts/junit/fast.xml .ci-artifacts/junit/contract.xml \
    .ci-artifacts/formal/contract.json .ci-artifacts/formal/contract.log \
    .ci-artifacts/contracts/bundle-manifest.json ;;
  family) run_observed family ops/ci/family.sh .ci-artifacts/family/subjects.json ;;
  family-contract) run_observed family-contract ops/ci/family-contract.sh .ci-artifacts/family/subjects.json ;;
  history) run_observed history ops/ci/history.sh ;;
  links) run_observed links ops/ci/external-links.sh ;;
  advisory) run_observed advisory ops/ci/advisory.sh ;;
  coverage) run_observed coverage ops/ci/coverage.sh .ci-artifacts/coverage/cobertura.xml ;;
  platform) run_observed platform ops/ci/platform-refusal.sh ;;
  audit) run_observed audit ops/ci/audit.sh ;;
  toolchain-pinned) run_observed toolchain-pinned ops/ci/toolchain-pinned.sh ;;
  all) run_observed required ops/ci/required.sh \
    .ci-artifacts/junit/fast.xml .ci-artifacts/junit/contract.xml \
    .ci-artifacts/formal/contract.json .ci-artifacts/formal/contract.log \
    .ci-artifacts/contracts/bundle-manifest.json ;;
  *)
    echo "usage: $0 {source-scan|fast|lint|contract|security|docs|required|family|family-contract|history|links|advisory|coverage|platform|audit|toolchain-pinned|all}" >&2
    exit 2
    ;;
esac
