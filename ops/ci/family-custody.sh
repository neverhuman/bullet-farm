#!/usr/bin/env bash
# Source-only exact custody for fixed CI artifact namespaces.

CI_PROOF_RECORD_REPOSITORY=
CI_PROOF_RECORD_SCOPE=
CI_PROOF_RECORD_PID=
CI_PROOF_RECORD_LANE=

ci_proof_refusal() {
  printf '%s\n' \
    "ci-local: CI_PROOF_LOCKED_OR_STALE: $1/.git/bullet-ci.lock.d is occupied or cannot be trusted" \
    "ci-local: verify the exact owner, then explicitly reconcile only $1/.git/bullet-ci.lock.d" >&2
  return 75
}

ci_proof_exact_directory() {
  local path="$1" mode="$2" uid
  uid="$(id -u)"
  [[ -d "$path" && ! -L "$path" \
    && "$(find "$path" -maxdepth 0 -type d -uid "$uid" -perm "$mode" -print)" == "$path" ]]
}

ci_proof_exact_file() {
  local path="$1" mode="$2" uid
  uid="$(id -u)"
  [[ -f "$path" && ! -L "$path" \
    && "$(find "$path" -maxdepth 0 -type f -uid "$uid" -perm "$mode" -print)" == "$path" ]]
}

ci_proof_parse() {
  local record="$1" repository="$2" scope="$3"
  [[ "$record" =~ ^schema=2\ repository=([a-z0-9-]+)\ scope=(standalone|family)\ pid=([1-9][0-9]*)\ lane=([a-z0-9-]+)\ nonce=([0-9]+-[0-9]+-[0-9]+-[0-9]+)$ ]] || return 1
  CI_PROOF_RECORD_REPOSITORY="${BASH_REMATCH[1]}"
  CI_PROOF_RECORD_SCOPE="${BASH_REMATCH[2]}"
  CI_PROOF_RECORD_PID="${BASH_REMATCH[3]}"
  CI_PROOF_RECORD_LANE="${BASH_REMATCH[4]}"
  [[ "$CI_PROOF_RECORD_REPOSITORY" == "$repository" \
    && "$CI_PROOF_RECORD_SCOPE" == "$scope" ]]
}

ci_proof_owner_matches() {
  local owner="$1" record="$2" first='' extra='' second_status descriptor byte_count
  read -r byte_count < <(LC_ALL=C wc -c <"$owner") || return 1
  [[ "$byte_count" =~ ^[0-9]+$ && "$byte_count" -eq $((${#record} + 1)) ]] || return 1
  exec {descriptor}<"$owner" || return 1
  IFS= read -r -u "$descriptor" first || {
    exec {descriptor}<&-
    return 1
  }
  if IFS= read -r -u "$descriptor" extra; then
    second_status=0
  else
    second_status=$?
  fi
  exec {descriptor}<&-
  [[ "$first" == "$record" && "$second_status" -ne 0 && -z "$extra" ]]
}

ci_proof_verify() {
  local root="$1" repository="$2" record="$3" scope="$4"
  local lock_dir owner
  lock_dir="$root/.git/bullet-ci.lock.d"
  owner="$lock_dir/owner"
  if [[ ! -d "$root/.git" || -L "$root/.git" \
    || ! -f "$root/.git/HEAD" || -L "$root/.git/HEAD" ]] \
    || ! ci_proof_exact_directory "$lock_dir" 0700 \
    || ! ci_proof_exact_file "$owner" 0600 \
    || ! ci_proof_parse "$record" "$repository" "$scope" \
    || ! ci_proof_owner_matches "$owner" "$record"; then
    ci_proof_refusal "$root"
    return 75
  fi
}

ci_proof_acquire() {
  local root="$1" repository="$2" scope="$3" lane="$4" output_name="$5"
  local lock_dir owner acquired_record
  lock_dir="$root/.git/bullet-ci.lock.d"
  owner="$lock_dir/owner"
  [[ "$repository" =~ ^[a-z0-9-]+$ && "$scope" =~ ^(standalone|family)$ \
    && "$lane" =~ ^[a-z0-9-]+$ && -d "$root/.git" && ! -L "$root/.git" \
    && -f "$root/.git/HEAD" && ! -L "$root/.git/HEAD" ]] || {
    ci_proof_refusal "$root"
    return 75
  }
  if ! (umask 077; mkdir -- "$lock_dir") 2>/dev/null; then
    ci_proof_refusal "$root"
    return 75
  fi
  acquired_record="schema=2 repository=$repository scope=$scope pid=$$ lane=$lane nonce=$$-${BASHPID:-$$}-$RANDOM-$RANDOM"
  if ! (umask 077; set -o noclobber; printf '%s\n' "$acquired_record" >"$owner") 2>/dev/null; then
    ci_proof_refusal "$root"
    return 75
  fi
  ci_proof_verify "$root" "$repository" "$acquired_record" "$scope" || return $?
  printf -v "$output_name" '%s' "$acquired_record"
}

ci_proof_release() {
  local root="$1" repository="$2" record="$3" scope="$4"
  local lock_dir="$root/.git/bullet-ci.lock.d"
  ci_proof_verify "$root" "$repository" "$record" "$scope" || return $?
  rm -- "$lock_dir/owner" || {
    ci_proof_refusal "$root"
    return 75
  }
  rmdir -- "$lock_dir" || {
    ci_proof_refusal "$root"
    return 75
  }
}

FAMILY_CUSTODY_ACTIVE=0
FAMILY_CUSTODY_LANE=
declare -ag FAMILY_CUSTODY_ACQUIRED=()
declare -Ag FAMILY_CUSTODY_ROOTS=()
declare -Ag FAMILY_CUSTODY_RECORDS=()

family_custody_initialize() {
  local family_root="$1" lane="$2"
  [[ "$lane" =~ ^family(-contract)?$ ]] || return 75
  FAMILY_CUSTODY_ROOTS=(
    [bullet-git]="$family_root/bullet-git"
    [bullet-kernel]="$family_root/bullet-kernel"
    [bullet-portal]="$family_root/bullet-portal"
  )
  FAMILY_CUSTODY_RECORDS=()
  FAMILY_CUSTODY_ACQUIRED=()
  FAMILY_CUSTODY_ACTIVE=0
  FAMILY_CUSTODY_LANE="$lane"
}

family_custody_verify_hub() {
  local root="$1" record="$2" lane="$3"
  ci_proof_verify "$root" bullet-farm "$record" family || return $?
  [[ "$CI_PROOF_RECORD_PID" == "$PPID" \
    && "$CI_PROOF_RECORD_LANE" == "$lane" ]] || {
    ci_proof_refusal "$root"
    return 75
  }
}

family_custody_verify_member() {
  local member="$1"
  ci_proof_verify "${FAMILY_CUSTODY_ROOTS[$member]}" "$member" \
    "${FAMILY_CUSTODY_RECORDS[$member]}" family || return $?
  [[ "$CI_PROOF_RECORD_PID" == "$$" \
    && "$CI_PROOF_RECORD_LANE" == "$FAMILY_CUSTODY_LANE" ]] || {
    ci_proof_refusal "${FAMILY_CUSTODY_ROOTS[$member]}"
    return 75
  }
}

family_custody_release_member() {
  local member="$1"
  family_custody_verify_member "$member" || return $?
  ci_proof_release "${FAMILY_CUSTODY_ROOTS[$member]}" "$member" \
    "${FAMILY_CUSTODY_RECORDS[$member]}" family || return $?
  unset 'FAMILY_CUSTODY_RECORDS[$member]'
}

family_custody_release_all() {
  local index member status=0
  for ((index=${#FAMILY_CUSTODY_ACQUIRED[@]} - 1; index >= 0; index--)); do
    member="${FAMILY_CUSTODY_ACQUIRED[$index]}"
    [[ -v "FAMILY_CUSTODY_RECORDS[$member]" ]] || continue
    family_custody_release_member "$member" || status=$?
  done
  FAMILY_CUSTODY_ACQUIRED=()
  FAMILY_CUSTODY_ACTIVE=0
  return "$status"
}

family_custody_acquire_member() {
  local member="$1" record
  ci_proof_acquire "${FAMILY_CUSTODY_ROOTS[$member]}" "$member" family \
    "$FAMILY_CUSTODY_LANE" record \
    || return $?
  FAMILY_CUSTODY_RECORDS[$member]="$record"
  FAMILY_CUSTODY_ACQUIRED+=("$member")
  family_custody_verify_member "$member"
}

family_custody_acquire_all() {
  local member status
  for member in bullet-git bullet-kernel bullet-portal; do
    if family_custody_acquire_member "$member"; then
      continue
    else
      status=$?
      family_custody_release_all || true
      return "$status"
    fi
  done
  FAMILY_CUSTODY_ACTIVE=1
}

family_custody_verify_all() {
  local member
  [[ "$FAMILY_CUSTODY_ACTIVE" -eq 1 ]] || return 75
  for member in bullet-git bullet-kernel bullet-portal; do
    family_custody_verify_member "$member" || return $?
  done
}

family_custody_record() {
  local member="$1"
  family_custody_verify_all || return $?
  [[ -v "FAMILY_CUSTODY_RECORDS[$member]" ]] || return 75
  printf '%s\n' "${FAMILY_CUSTODY_RECORDS[$member]}"
}
