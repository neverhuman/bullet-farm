#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=ops/ci/family-custody.sh
source "$REPO_ROOT/ops/ci/family-custody.sh"

fixture="$(mktemp -d)"
outside="$(mktemp -d)"
cleanup() { rm -rf -- "$fixture" "$outside"; }
trap cleanup EXIT

for repository in bullet-farm bullet-git bullet-kernel bullet-portal; do
  mkdir -p "$fixture/$repository/.git"
  printf '%s\n' 'ref: refs/heads/main' >"$fixture/$repository/.git/HEAD"
done

expect_status_75() {
  local status
  set +e
  "$@" >"$fixture/refusal.out" 2>&1
  status=$?
  set -e
  [[ "$status" -eq 75 && "$(<"$fixture/refusal.out")" == *CI_PROOF_LOCKED_OR_STALE* ]]
}

farm="$fixture/bullet-farm"
standalone_record=''
(
  umask 000
  ci_proof_acquire "$farm" bullet-farm standalone fast standalone_record
  [[ "$CI_PROOF_RECORD_PID" == "$$" && "$CI_PROOF_RECORD_LANE" == fast ]]
  [[ "$(find "$farm/.git/bullet-ci.lock.d" -maxdepth 0 -type d -uid "$(id -u)" -perm 0700 -print)" \
      == "$farm/.git/bullet-ci.lock.d" ]]
  [[ "$(find "$farm/.git/bullet-ci.lock.d/owner" -maxdepth 0 -type f -uid "$(id -u)" -perm 0600 -print)" \
      == "$farm/.git/bullet-ci.lock.d/owner" ]]
  ci_proof_release "$farm" bullet-farm "$standalone_record" standalone
)
[[ ! -e "$farm/.git/bullet-ci.lock.d" ]]

ci_proof_acquire "$farm" bullet-farm standalone fast standalone_record
printf preserve >"$fixture/protected-report"
expect_status_75 ci_proof_acquire "$farm" bullet-farm standalone fast replay_record
[[ "$(<"$fixture/protected-report")" == preserve ]]
ci_proof_release "$farm" bullet-farm "$standalone_record" standalone
expect_status_75 ci_proof_verify "$farm" bullet-farm "$standalone_record" standalone

ci_proof_acquire "$farm" bullet-farm standalone fast standalone_record
printf '\n' >>"$farm/.git/bullet-ci.lock.d/owner"
expect_status_75 ci_proof_verify "$farm" bullet-farm "$standalone_record" standalone
printf '%s\n' "$standalone_record" >"$farm/.git/bullet-ci.lock.d/owner"
printf '\0' >>"$farm/.git/bullet-ci.lock.d/owner"
expect_status_75 ci_proof_verify "$farm" bullet-farm "$standalone_record" standalone
printf '%s\n' "$standalone_record" >"$farm/.git/bullet-ci.lock.d/owner"
ci_proof_release "$farm" bullet-farm "$standalone_record" standalone

mkdir "$fixture/hostile-git"
printf outside >"$outside/sentinel"
ln -s "$outside" "$fixture/hostile-git/.git"
expect_status_75 ci_proof_acquire "$fixture/hostile-git" hostile-git standalone fast hostile_record
[[ "$(<"$outside/sentinel")" == outside ]]

family_custody_initialize "$fixture" family
kernel_record=''
ci_proof_acquire "$fixture/bullet-kernel" bullet-kernel standalone fast kernel_record
printf preserve >"$fixture/member-report"
expect_status_75 family_custody_acquire_all
[[ ! -e "$fixture/bullet-git/.git/bullet-ci.lock.d" \
  && -d "$fixture/bullet-kernel/.git/bullet-ci.lock.d" \
  && ! -e "$fixture/bullet-portal/.git/bullet-ci.lock.d" \
  && "$(<"$fixture/member-report")" == preserve ]]
ci_proof_release "$fixture/bullet-kernel" bullet-kernel "$kernel_record" standalone

family_custody_initialize "$fixture" family
family_custody_acquire_all
family_custody_verify_all
for repository in bullet-git bullet-kernel bullet-portal; do
  record="$(family_custody_record "$repository")"
  [[ "$record" =~ ^schema=2\ repository=$repository\ scope=family\ pid=$$\ lane=family\ nonce=[0-9]+-[0-9]+-[0-9]+-[0-9]+$ ]]
  expect_status_75 ci_proof_acquire "$fixture/$repository" "$repository" standalone fast replay_record
done
portal_record="$(family_custody_record bullet-portal)"
printf '%s\n' 'schema=2 repository=bullet-portal scope=family pid=1 lane=family nonce=1-2-3-4' \
  >"$fixture/bullet-portal/.git/bullet-ci.lock.d/owner"
expect_status_75 family_custody_verify_all
printf '%s\n' "$portal_record" >"$fixture/bullet-portal/.git/bullet-ci.lock.d/owner"
family_custody_release_all
for repository in bullet-git bullet-kernel bullet-portal; do
  [[ ! -e "$fixture/$repository/.git/bullet-ci.lock.d" ]]
done

if rg -n 'BULLET_CI_INHERITED_OWNER' \
  "$REPO_ROOT/scripts/ci-local.sh" "$REPO_ROOT/ops/ci/family.sh" \
  "$REPO_ROOT/ops/ci/family-custody.sh"; then
  echo '[ci] CI_PROOF_CUSTODY_LEGACY_VARIABLE_PRESENT' >&2
  exit 1
fi

family_source="$REPO_ROOT/ops/ci/family.sh"
acquire_line="$(grep -nF 'family_custody_acquire_all || exit $?' "$family_source" | head -n 1 | cut -d: -f1)"
# Match exact production source text.
# shellcheck disable=SC2016
reset_line="$(grep -nF 'rm -f -- "$(member_root "$report_member")/$relative"' \
  "$family_source" | head -n 1 | cut -d: -f1)"
publication_line="$(grep -nF 'assert_family_subjects after-observation-publication' \
  "$family_source" | tail -n 1 | cut -d: -f1)"
verify_line="$(grep -nF 'family_custody_verify_all || exit $?' \
  "$family_source" | tail -n 1 | cut -d: -f1)"
[[ "$acquire_line" =~ ^[0-9]+$ && "$reset_line" =~ ^[0-9]+$ \
  && "$publication_line" =~ ^[0-9]+$ && "$verify_line" =~ ^[0-9]+$ \
  && "$acquire_line" -lt "$reset_line" && "$publication_line" -lt "$verify_line" ]]

expected_reports=(
  'bullet-git|.ci-artifacts/reports/fast.junit.xml'
  'bullet-git|.ci-artifacts/reports/contract.junit.xml'
  'bullet-kernel|.ci-artifacts/junit/fast.xml'
  'bullet-kernel|.ci-artifacts/junit/contract.xml'
  'bullet-kernel|.ci-artifacts/junit/family.xml'
  'bullet-portal|.ci-artifacts/reports/vitest.json'
  'bullet-portal|.ci-artifacts/reports/playwright.xml'
  'bullet-portal|.ci-artifacts/reports/real-farmd.xml'
  'bullet-farm|.ci-artifacts/junit/contract.xml'
  'bullet-farm|.ci-artifacts/formal/contract.json'
  'bullet-farm|.ci-artifacts/formal/contract.log'
)
report_specs_source="$(sed -n '/^report_specs=(/,/^)/p' "$family_source")"
for report in "${expected_reports[@]}"; do
  grep -Fq "$report|" <<<"$report_specs_source"
done
[[ "$(grep -cE "^[[:space:]]+[\"']bullet-(farm|git|kernel|portal)\\|" \
  <<<"$report_specs_source")" \
  -eq "${#expected_reports[@]}" ]]
grep -Fxq 'exec bash ops/ci/family.sh' "$REPO_ROOT/ops/ci/family-contract.sh"
grep -Fq "trap 'family_custody_uncertain=1; exit 129' HUP" "$family_source"
grep -Fq "trap 'family_custody_uncertain=1; exit 130' INT" "$family_source"
grep -Fq "trap 'family_custody_uncertain=1; exit 143' TERM" "$family_source"

echo '[ci] family proof custody fixture passed'
