#!/usr/bin/env bash
# Canonical dependency-ordered component family proof. This is an unsigned
# observation over clean ordinary checkouts, not Family release Evidence.
set -euo pipefail
# shellcheck source=ops/ci/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
# shellcheck source=ops/ci/toolchain-pins.sh
source "$(dirname "${BASH_SOURCE[0]}")/toolchain-pins.sh"
cd "$REPO_ROOT"
[[ "$(uname -s)" == Linux ]] \
  || { refuse FAMILY_MUTATION_LINUX_ONLY "family process tests require Linux containment"; exit 1; }
[[ "$(node --version)" == "v$PINNED_NODE_VERSION" ]] \
  || { refuse FAMILY_NODE_VERSION_INVALID "$(node --version)"; exit 1; }
[[ "$(npm --version)" == "$PINNED_NPM_VERSION" ]] \
  || { refuse FAMILY_NPM_VERSION_INVALID "$(npm --version)"; exit 1; }
FAMILY_ROOT="$(cd "$REPO_ROOT/.." && pwd)"
GIT_ROOT="$FAMILY_ROOT/bullet-git"
KERNEL_ROOT="$FAMILY_ROOT/bullet-kernel"
PORTAL_ROOT="$FAMILY_ROOT/bullet-portal"

read_inventory_constant() {
  local file="$1" name="$2" value
  value="$(sed -nE "s/^(readonly[[:space:]]+)?${name}=([0-9]+)$/\\2/p" "$file")"
  [[ "$value" =~ ^[1-9][0-9]*$ ]] \
    || { refuse FAMILY_INVENTORY_INVALID "$file:$name"; return 1; }
  printf '%s\n' "$value"
}
git_fast_expected="$(read_inventory_constant "$GIT_ROOT/ops/ci/lib.sh" FAST_EXPECTED_TESTS)"
git_contract_expected="$(read_inventory_constant "$GIT_ROOT/ops/ci/lib.sh" CONTRACT_EXPECTED_TESTS)"
kernel_fast_expected="$(read_inventory_constant "$KERNEL_ROOT/ops/ci/inventory.sh" EXPECTED_STANDALONE_TESTS)"
kernel_contract_expected="$(read_inventory_constant "$KERNEL_ROOT/ops/ci/inventory.sh" EXPECTED_CONTRACT_TESTS)"
kernel_family_expected="$(read_inventory_constant "$KERNEL_ROOT/ops/ci/inventory.sh" EXPECTED_FAMILY_TESTS)"
portal_vitest_expected="$(bash ops/ci/family-report-check.sh vitest-source-pair \
  "$PORTAL_ROOT/ops/ci/fast.sh" "$PORTAL_ROOT/ops/ci/coverage.sh")"
portal_real_farmd_expected="$(grep -Ec '^[[:space:]]*test\(' "$PORTAL_ROOT/e2e/real-farmd.spec.ts")"
[[ "$portal_real_farmd_expected" =~ ^[1-9][0-9]*$ ]] \
  || { refuse FAMILY_INVENTORY_INVALID "bullet-portal:e2e/real-farmd.spec.ts"; exit 1; }

report_specs=(
  "bullet-git|.ci-artifacts/reports/fast.junit.xml|junit|$git_fast_expected|0"
  "bullet-git|.ci-artifacts/reports/contract.junit.xml|junit|$git_contract_expected|0"
  "bullet-kernel|.ci-artifacts/junit/fast.xml|junit|$kernel_fast_expected|0"
  "bullet-kernel|.ci-artifacts/junit/contract.xml|junit|$kernel_contract_expected|0"
  "bullet-kernel|.ci-artifacts/junit/family.xml|junit|$kernel_family_expected|0"
  "bullet-portal|.ci-artifacts/reports/vitest.json|vitest|$portal_vitest_expected|0"
  'bullet-portal|.ci-artifacts/reports/playwright.xml|junit|10|0'
  "bullet-portal|.ci-artifacts/reports/real-farmd.xml|junit|$portal_real_farmd_expected|0"
  "bullet-farm|.ci-artifacts/junit/contract.xml|junit|$WIRE_EXPECTED_TESTS|0"
  'bullet-farm|.ci-artifacts/formal/contract.json|formal-json|0|0'
  'bullet-farm|.ci-artifacts/formal/contract.log|formal-log|0|0'
)

member_root() {
  case "$1" in
    bullet-farm) printf '%s\n' "$REPO_ROOT" ;;
    bullet-git) printf '%s\n' "$GIT_ROOT" ;;
    bullet-kernel) printf '%s\n' "$KERNEL_ROOT" ;;
    bullet-portal) printf '%s\n' "$PORTAL_ROOT" ;;
    *) refuse FAMILY_MEMBER_UNKNOWN "$1"; return 1 ;;
  esac
}

members=(bullet-farm bullet-kernel bullet-git bullet-portal)
declare -A family_commits family_trees
for member in "${members[@]}"; do
  root="$FAMILY_ROOT/$member"
  [[ -d "$root/.git" && ! -L "$root/.git" && -f "$root/.git/HEAD" ]] \
    || { refuse FAMILY_MEMBER_NOT_PRIMARY_CHECKOUT "$root"; exit 1; }
  family_commits[$member]="$(git -C "$root" rev-parse --verify HEAD)"
  family_trees[$member]="$(git -C "$root" rev-parse --verify 'HEAD^{tree}')"
  [[ -z "$(git -C "$root" status --porcelain=v1 --untracked-files=all)" ]] \
    || { refuse DIRTY_FAMILY_SUBJECT "$member"; exit 1; }
  [[ "$(git -C "$root" rev-parse --verify HEAD)" == "${family_commits[$member]}" \
    && "$(git -C "$root" rev-parse --verify 'HEAD^{tree}')" == "${family_trees[$member]}" ]] \
    || { refuse FAMILY_SUBJECT_CHANGED_DURING_CAPTURE "$member"; exit 1; }
done

# A passing stage must produce every report during this invocation; ignored
# leftovers from an earlier run cannot satisfy the family observation.
for spec in "${report_specs[@]}"; do
  IFS='|' read -r report_member relative _ <<<"$spec"
  report_root="$(member_root "$report_member")"
  prepare_ci_directory "$report_root" "${relative%/*}" \
    || { refuse FAMILY_ARTIFACT_ROOT_INVALID "$report_member/${relative%/*}"; exit 1; }
  rm -f -- "$(member_root "$report_member")/$relative"
done

family_tmp="$(mktemp -d)"
cleanup() { rm -rf -- "$family_tmp"; }
trap cleanup EXIT

assert_family_subjects() {
  local phase="$1" member root current_commit current_tree
  for member in "${members[@]}"; do
    root="$FAMILY_ROOT/$member"
    [[ -d "$root/.git" && ! -L "$root/.git" && -f "$root/.git/HEAD" ]] \
      || { refuse FAMILY_MEMBER_NOT_PRIMARY_CHECKOUT "$phase:$member"; return 1; }
    current_commit="$(git -C "$root" rev-parse --verify HEAD)"
    current_tree="$(git -C "$root" rev-parse --verify 'HEAD^{tree}')"
    [[ "$current_commit" == "${family_commits[$member]}" \
      && "$current_tree" == "${family_trees[$member]}" ]] \
      || { refuse FAMILY_SUBJECT_CHANGED_DURING_PROOF "$phase:$member"; return 1; }
    [[ -z "$(git -C "$root" status --porcelain=v1 --untracked-files=all)" ]] \
      || { refuse DIRTY_FAMILY_SUBJECT "$phase:$member"; return 1; }
    [[ "$(git -C "$root" rev-parse --verify HEAD)" == "$current_commit" \
      && "$(git -C "$root" rev-parse --verify 'HEAD^{tree}')" == "$current_tree" ]] \
      || { refuse FAMILY_SUBJECT_CHANGED_DURING_PROOF "$phase:$member"; return 1; }
  done
}

assert_family_subjects before-stage-1
log "1/7 BulletGit standalone required"
(cd "$GIT_ROOT" && bash scripts/ci-local.sh required)
assert_family_subjects after-stage-1
assert_family_subjects before-stage-2
log "2/7 build the sole-writer daemon from the admitted BulletGit subject"
(cd "$GIT_ROOT" && CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$family_tmp/gitd-target" \
  cargo build --locked -p bullet-gitd --bin bullet-gitd)
gitd_expected="$family_tmp/gitd-target/debug/bullet-gitd"
gitd_bin="$(realpath -e -- "$gitd_expected")" \
  || { refuse BULLET_GITD_BIN_MISSING "$gitd_expected"; exit 1; }
[[ "$gitd_bin" == "$gitd_expected" && -f "$gitd_bin" && -x "$gitd_bin" && ! -L "$gitd_bin" ]] \
  || { refuse BULLET_GITD_BIN_WRONG_SUBJECT "$gitd_bin"; exit 1; }
gitd_sha256="$(sha256_file "$gitd_bin")"
assert_family_subjects after-stage-2

assert_family_subjects before-stage-3
log "3/7 Kernel standalone required"
(cd "$KERNEL_ROOT" && bash scripts/ci-local.sh required)
assert_family_subjects after-stage-3
assert_family_subjects before-stage-4
log "4/7 Kernel family inventory with exact absolute bullet-gitd"
[[ "$(sha256_file "$gitd_bin")" == "$gitd_sha256" ]] \
  || { refuse BULLET_GITD_BIN_CHANGED before-kernel-family; exit 1; }
(cd "$KERNEL_ROOT" && BULLET_GITD_BIN="$gitd_bin" BULLET_GITD_SHA256="$gitd_sha256" \
  bash scripts/ci-local.sh family)
[[ "$(sha256_file "$gitd_bin")" == "$gitd_sha256" ]] \
  || { refuse BULLET_GITD_BIN_CHANGED after-kernel-family; exit 1; }
assert_family_subjects after-stage-4

assert_family_subjects before-stage-5
log "5/7 Portal standalone required"
(cd "$PORTAL_ROOT" && bash scripts/ci-local.sh required)
assert_family_subjects after-stage-5
assert_family_subjects before-stage-6
log "6/7 Portal real-farmd browser proof"
(cd "$PORTAL_ROOT" && bash scripts/ci-local.sh family)
assert_family_subjects after-stage-6

assert_family_subjects before-stage-7
log "7/7 cross-family drift plus Hub contracts and pinned models"
bash scripts/sync-family-contracts.sh check
bash ops/ci/contract.sh
assert_family_subjects after-stage-7

prepare_ci_directory "$REPO_ROOT" .ci-artifacts/family \
  || { refuse FAMILY_ARTIFACT_ROOT_INVALID .ci-artifacts/family; exit 1; }
subjects='{}'
for member in "${members[@]}"; do
  subjects="$(jq -c --arg member "$member" --arg commit "${family_commits[$member]}" \
    --arg tree "${family_trees[$member]}" \
    '. + {($member): {commit_oid:$commit,tree_oid:$tree,clean:true}}' <<<"$subjects")"
done
reports='[]'
for spec in "${report_specs[@]}"; do
  IFS='|' read -r report_member relative kind expected_tests expected_skipped <<<"$spec"
  report="$(member_root "$report_member")/$relative"
  prepare_ci_directory "$(member_root "$report_member")" "${relative%/*}" \
    || { refuse FAMILY_ARTIFACT_ROOT_INVALID "$report_member/${relative%/*}"; exit 1; }
  case "$kind" in
    junit) summary="$(bash ops/ci/family-report-check.sh junit "$report" "$expected_tests" "$expected_skipped")" ;;
    vitest) summary="$(bash ops/ci/family-report-check.sh vitest "$report" "$expected_tests")" ;;
    formal-json|formal-log) summary="$(bash ops/ci/family-report-check.sh "$kind" "$report")" ;;
    *) refuse FAMILY_REPORT_KIND_INVALID "$kind"; exit 1 ;;
  esac
  digest="$(sha256_file "$report")"
  label="$report_member/$relative"
  reports="$(jq -c --arg repository "$report_member" --arg path "$label" \
    --arg sha256 "$digest" --argjson summary "$summary" \
    '. + [{repository:$repository,path:$path,sha256:$sha256,summary:$summary}]' <<<"$reports")"
done
[[ "$(jq '[.[].path] | unique | length' <<<"$reports")" -eq "${#report_specs[@]}" ]] \
  || { refuse FAMILY_REPORT_LABEL_DUPLICATE "report paths"; exit 1; }
jq -n --argjson subjects "$subjects" --argjson reports "$reports" \
  --arg gitd_sha256 "$gitd_sha256" --arg gitd_commit "${family_commits[bullet-git]}" \
  --arg node "$(node --version)" --arg npm "$(npm --version)" \
  '{schema_version:"bullet.family-ci-observation.v1",subjects:$subjects,reports:$reports,
   sole_writer_daemon:{repository:"bullet-git",commit_oid:$gitd_commit,sha256:$gitd_sha256,
     build:{cargo_locked:true,incremental:false,fresh_target:true}},
   tool_versions:{node:$node,npm:$npm},signed:false,evidence_class:"DIAGNOSTIC_ONLY",
   release_authority:false}' \
  >.ci-artifacts/family/subjects.json

jq -e --argjson report_count "${#report_specs[@]}" \
  --arg node "v$PINNED_NODE_VERSION" --arg npm "$PINNED_NPM_VERSION" '
  .schema_version == "bullet.family-ci-observation.v1" and
  (.subjects | keys | sort) == ["bullet-farm","bullet-git","bullet-kernel","bullet-portal"] and
  all(.subjects[]; .clean == true and (.commit_oid | test("^[0-9a-f]{40}$")) and
    (.tree_oid | test("^[0-9a-f]{40}$"))) and
  (.reports | length) == $report_count and
  ([.reports[].path] | unique | length) == $report_count and
  all(.reports[]; (.repository | type == "string") and
    (.path | test("^bullet-(farm|git|kernel|portal)/\\.ci-artifacts/")) and
    (.sha256 | test("^[0-9a-f]{64}$")) and (.summary | type == "object")) and
  .sole_writer_daemon.repository == "bullet-git" and
  .sole_writer_daemon.commit_oid == .subjects["bullet-git"].commit_oid and
  (.sole_writer_daemon.sha256 | test("^[0-9a-f]{64}$")) and
  .sole_writer_daemon.build == {cargo_locked:true,incremental:false,fresh_target:true} and
  .tool_versions == {node:$node,npm:$npm} and
  .signed == false and .evidence_class == "DIAGNOSTIC_ONLY" and
  .release_authority == false
' .ci-artifacts/family/subjects.json >/dev/null \
  || { refuse FAMILY_OBSERVATION_INVALID ".ci-artifacts/family/subjects.json"; exit 1; }

assert_family_subjects before-observation-publication
while IFS=$'\t' read -r label expected_hash; do
  report="$FAMILY_ROOT/$label"
  [[ -f "$report" && ! -L "$report" && "$(sha256_file "$report")" == "$expected_hash" ]] \
    || { refuse FAMILY_REPORT_CHANGED_AFTER_HASH "$label"; exit 1; }
done < <(jq -r '.reports[] | [.path,.sha256] | @tsv' .ci-artifacts/family/subjects.json)
[[ "$(sha256_file "$gitd_bin")" == "$(jq -r '.sole_writer_daemon.sha256' .ci-artifacts/family/subjects.json)" ]] \
  || { refuse BULLET_GITD_BIN_CHANGED after-observation; exit 1; }
log "family lane passed (unsigned clean component observation only)"
