#!/usr/bin/env bash
# Real Git and pinned scanner fixtures; never invokes Cargo or a remote forge.
set -euo pipefail
wrapper="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/ci.sh"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 GIT_NO_REPLACE_OBJECTS=1
export GIT_TERMINAL_PROMPT=0
repo="$scratch/aggregate"
mkdir "$repo"
git -C "$repo" init -q
git -C "$repo" config user.name 'Publication fixture'
git -C "$repo" config user.email fixture@bullet.invalid
printf 'public fixture\n' >"$repo/README.md"
mkdir -p "$repo/bullet-farm/publication"
printf '[extend]\nuseDefault = true\n' >"$repo/bullet-farm/publication/gitleaks.toml"
git -C "$repo" add README.md bullet-farm/publication/gitleaks.toml
git -C "$repo" commit -qm fixture
sha="$(git -C "$repo" rev-parse HEAD)"
completed="$scratch/completed-wrapper-tests.log"
: >"$completed"

pass_case() {
  printf 'publication wrapper case: %s ... ok\n' "$1" | tee -a "$completed"
}

require_refusal() {
  local code="$1"
  shift
  if "$@" >"$scratch/refusal.log" 2>&1; then
    printf 'expected refusal: %s\n' "$code" >&2
    cat "$scratch/refusal.log" >&2
    exit 1
  fi
  grep -Fq -- "$code" "$scratch/refusal.log" || {
    printf 'missing refusal: %s\n' "$code" >&2
    cat "$scratch/refusal.log" >&2
    exit 1
  }
}

expect_refusal() {
  local identity="$1"
  shift
  require_refusal "$@"
  pass_case "$identity"
}

invoke() {
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" GITHUB_SHA="$sha" \
    RUNNER_TEMP="$scratch/runner" bash "$wrapper" "$@"
}

# Reviewed nextest publication subset, retained as an explicit fixture so removing
# a real test cannot silently rewrite the expected successful run in this test.
cat >"$scratch/identities" <<'IDENTITIES'
publication::observation::tests::hosted_artifact_inventory_refuses_extra_missing_empty_or_symbolic_bytes
publication::observation::tests::hosted_observation_preserves_event_and_member_identities_without_release_authority
publication::pull_request::tests::creation_receipt_reopens_same_pr_after_restart_closed_and_merged
publication::pull_request::tests::existing_human_changed_head_marker_and_duplicate_prs_refuse_creation
publication::pull_request::tests::github_api_child_has_private_configuration_and_no_inherited_credentials
publication::pull_request::tests::github_executable_refuses_relative_symlink_and_wrong_digest_subjects
publication::pull_request::tests::preexisting_closed_bot_pr_is_adopted_but_author_readback_drift_refuses
publication::pull_request::tests::receipt_substitution_and_deleted_review_ref_do_not_create_another_pr
publication::pull_request::tests::remote_drift_and_changed_intent_refuse_before_creation
publication::pull_request::tests::response_loss_reconciles_one_attempt_and_never_reposts_unknown_outcome
publication::scan::tests::actual_pinned_scanner_detects_binary_and_chunk_boundary_canaries
publication::scan::tests::actual_scan_receipt_survives_restart_and_rejects_report_tampering
publication::scan::tests::cat_file_frames_preserve_nul_newline_and_binary_bytes
publication::scan::tests::object_and_total_limits_refuse_without_truncation
publication::scan::tests::scanner_config_requires_pinned_defaults_and_refuses_external_extends
publication::scan::tests::scanner_pin_refuses_substituted_executable
publication::scan::tests::scanner_report_requires_empty_structural_array
publication::scan::tests::synthetic_root_retains_deleted_history_and_commit_metadata
publication::tests::deterministic_publication_preserves_all_exact_source_trees_and_templates
publication::tests::durable_requests_refuse_changed_inputs_and_same_tree_wrong_parent
publication::tests::publication_manifest_refuses_duplicates_unknowns_paths_and_noncanonical_bytes
publication::tests::publication_refuses_dirty_hidden_flags_and_symlinked_checkouts
publication::tests::publication_refuses_history_and_endpoint_substitution
publication::tests::publication_rejects_template_and_member_subtree_drift
publication::transport_tests::actual_prepare_recovers_interruption_after_refs_and_rejects_changed_request
publication::transport_tests::atomic_source_publication_reconstructs_real_checkouts_and_reconciles_response_loss
publication::transport_tests::conflicting_or_stale_remote_ref_cannot_partially_publish
publication::transport_tests::git_basic_auth_encoding_and_unrelated_child_custody_are_exact
publication::transport_tests::reconstructed_source_ref_drift_is_refused_before_checkout
IDENTITIES
sed 's/^/test /; s/$/ ... ok/' "$scratch/identities" >"$scratch/valid-tests.log"
summary='test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 300 filtered out; finished in 1.23s'
printf '%s\n' "$summary" >>"$scratch/valid-tests.log"
bash "$wrapper" test-inventory "$scratch/valid-tests.log"
pass_case rust_inventory_exact
tac "$scratch/valid-tests.log" | sed 's/300 filtered out/0 filtered out/; s/1.23s$/20.001s/' \
  >"$scratch/reordered-tests.log"
bash "$wrapper" test-inventory "$scratch/reordered-tests.log"
pass_case rust_inventory_reordered

for spec in \
  'missing_test|1d' \
  'renamed_test|1s/hosted_artifact_inventory/renamed_inventory/' \
  'failed_test|1s/ ... ok$/ ... FAILED/' \
  'ignored_test|1s/ ... ok$/ ... ignored/' \
  'skipped_test|1s/ ... ok$/ ... skipped/' \
  'malformed_test|1s/ ... ok$/ malformed/' \
  'foreign_test|1s/publication::/unrelated::/' \
  'missing_summary|/^test result/d' \
  'wrong_pass_count|/^test result/s/29 passed/28 passed/' \
  'zero_pass_count|/^test result/s/29 passed/0 passed/' \
  'ignored_summary|/^test result/s/0 ignored/1 ignored/' \
  'malformed_duration|/^test result/s/1.23s/1..23s/' \
  'failed_summary|/^test result/s/ok\./FAILED./'; do
  identity="${spec%%|*}"
  change="${spec#*|}"
  sed "$change" "$scratch/valid-tests.log" >"$scratch/invalid-tests.log"
  expect_refusal "rust_inventory_$identity" PUBLICATION_TEST_INVENTORY_INVALID \
    bash "$wrapper" test-inventory "$scratch/invalid-tests.log"
done
awk 'NR == 1 {first = $0} NR == 2 {$0 = first} {print}' "$scratch/valid-tests.log" \
  >"$scratch/duplicate-tests.log"
expect_refusal rust_inventory_duplicate_test PUBLICATION_TEST_INVENTORY_INVALID \
  bash "$wrapper" test-inventory "$scratch/duplicate-tests.log"
for spec in \
  "duplicate_summary|$summary" \
  'extra_test|test publication::unexpected ... ok' \
  'extra_failed_test|test unrelated::unexpected ... FAILED' \
  'extra_malformed_test|test malformed'; do
  identity="${spec%%|*}"
  extra="${spec#*|}"
  cat "$scratch/valid-tests.log" >"$scratch/extra-tests.log"
  printf '%s\n' "$extra" >>"$scratch/extra-tests.log"
  expect_refusal "rust_inventory_$identity" PUBLICATION_TEST_INVENTORY_INVALID \
    bash "$wrapper" test-inventory "$scratch/extra-tests.log"
done
ln -s "$scratch/valid-tests.log" "$scratch/symlink-tests.log"
: >"$scratch/empty-tests.log"
for spec in "symlink_log|$scratch/symlink-tests.log" "empty_log|$scratch/empty-tests.log" \
  "missing_log|$scratch/missing-tests.log" "directory_log|$scratch"; do
  identity="${spec%%|*}"
  invalid="${spec#*|}"
  expect_refusal "rust_inventory_$identity" PUBLICATION_TEST_INVENTORY_INVALID \
    bash "$wrapper" test-inventory "$invalid"
done
expect_refusal rust_inventory_missing_argument PUBLICATION_CI_USAGE bash "$wrapper" test-inventory
expect_refusal rust_inventory_extra_argument PUBLICATION_CI_USAGE \
  bash "$wrapper" test-inventory "$scratch/valid-tests.log" extra

mkdir "$scratch/runner"
expect_refusal usage_missing_command PUBLICATION_CI_USAGE bash "$wrapper"
expect_refusal usage_unknown_command PUBLICATION_CI_USAGE bash "$wrapper" arbitrary
expect_refusal usage_source_scan_extra_argument PUBLICATION_CI_USAGE bash "$wrapper" source-scan extra
expect_refusal runner_not_github_actions PUBLICATION_DISPOSABLE_CI_REQUIRED \
  env GITHUB_ACTIONS=false bash "$wrapper" source-scan
expect_refusal runner_event_sha_mismatch PUBLICATION_EVENT_SHA_MISMATCH \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" \
  GITHUB_SHA=0000000000000000000000000000000000000000 \
  RUNNER_TEMP="$scratch/runner" bash "$wrapper" source-scan
expect_refusal runner_event_sha_invalid PUBLICATION_EVENT_SHA_INVALID \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" GITHUB_SHA=main \
  RUNNER_TEMP="$scratch/runner" bash "$wrapper" source-scan
printf 'dirty\n' >"$repo/untracked.txt"
expect_refusal runner_dirty_checkout PUBLICATION_CHECKOUT_DIRTY invoke source-scan
rm "$repo/untracked.txt"
ln -s "$scratch/runner" "$scratch/runner-link"
expect_refusal runner_temp_symlink PUBLICATION_RUNNER_TEMP_INVALID \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" GITHUB_SHA="$sha" \
  RUNNER_TEMP="$scratch/runner-link" bash "$wrapper" source-scan
ln -s "$repo" "$scratch/aggregate-link"
expect_refusal runner_checkout_symlink PUBLICATION_CHECKOUT_INVALID \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$scratch/aggregate-link" GITHUB_SHA="$sha" \
  RUNNER_TEMP="$scratch/runner" bash "$wrapper" source-scan
expect_refusal source_scan_receipt_missing PUBLICATION_SOURCE_SCAN_REQUIRED invoke prove
mkdir "$scratch/runner/bullet-publication-report" "$scratch/runner/bullet-publication-private"
printf '{"success":true}\n' >"$scratch/runner/bullet-publication-report/source-scan.json"
expect_refusal source_scan_receipt_malformed PUBLICATION_SOURCE_SCAN_REQUIRED invoke prove
rm -r "$scratch/runner/bullet-publication-report" "$scratch/runner/bullet-publication-private"
invoke source-scan
[[ "$(cat "$scratch/runner/bullet-publication-report/source-scan.json")" == '[]' ]]
[[ ! -e "$scratch/runner/bullet-publication-cargo" ]]
pass_case source_scan_clean
expect_refusal source_scan_repeated 'File exists' invoke source-scan
mkdir "$scratch/canary-runner"
printf 'aws_access_key_id = %s%s\n' 'AKIA' '6QWERTYUIOPASDFG' >"$repo/canary.txt"
git -C "$repo" add canary.txt
git -C "$repo" commit -qm 'intentional scanner fixture'
require_refusal PUBLICATION_SOURCE_SCAN_FAILED \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" \
  GITHUB_SHA="$(git -C "$repo" rev-parse HEAD)" RUNNER_TEMP="$scratch/canary-runner" \
  bash "$wrapper" source-scan
[[ ! -e "$scratch/canary-runner/bullet-publication-report/source-scan.json" ]]
[[ "$(find "$scratch/canary-runner/bullet-publication-report" -type f | wc -l)" == 1 ]]
pass_case source_scan_canary

# Execute the actual always-running final check, without a duplicate implementation.
awk 'capture {sub(/^          /, ""); print} /^        run: \|$/ {capture = 1}' \
  "${wrapper%/*}/root/.github/workflows/publication.yml" >"$scratch/final.sh"
[[ -s "$scratch/final.sh" ]]
env INTEGRITY_RESULT=success bash "$scratch/final.sh"
pass_case final_success
for result in failure cancelled skipped neutral malformed ''; do
  expect_refusal "final_${result:-empty}" 'publication integrity did not succeed' \
    env INTEGRITY_RESULT="$result" bash "$scratch/final.sh"
done
printf 'publication wrapper fixtures: 47 passed; 0 failed; 0 skipped\n' >>"$completed"
bash "$wrapper" wrapper-inventory "$completed"
printf 'publication wrapper fixtures: 47 passed; 0 failed; 0 skipped\n'
