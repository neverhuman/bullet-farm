#!/usr/bin/env bash
# Validate one dependency-ordered family report and emit a normalized summary
# with no test names, paths, timestamps, or captured output.
set -euo pipefail
# shellcheck source=ops/ci/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

kind="${1:-}"
report="${2:-}"
[[ -n "$kind" && -f "$report" && ! -L "$report" && -s "$report" ]] \
  || { refuse FAMILY_REPORT_MISSING "${report:-missing}"; exit 1; }

validate_json_report() {
  bash "$REPO_ROOT/ops/ci/strict-json.sh" "$1" >/dev/null 2>&1 \
    || { refuse FAMILY_REPORT_JSON_INVALID "$1"; return 1; }
}

xml_attribute() {
  local line="$1" attribute="$2" matches count value
  matches="$(grep -oE "[[:space:]]${attribute}=\"[0-9]+\"" <<<"$line" || true)"
  count="$(grep -c . <<<"$matches")"
  [[ "$count" -eq 1 ]] \
    || { refuse FAMILY_REPORT_INVALID "expected one $attribute attribute"; return 1; }
  value="${matches#*=\"}"
  printf '%s\n' "${value%\"}"
}

case "$kind" in
  vitest-source-pair)
    [[ "$#" -eq 3 && -f "$3" && ! -L "$3" && -s "$3" ]] \
      || { refuse FAMILY_VITEST_SOURCE_MISSING "fast/coverage source"; exit 1; }
    source_count() {
      local source="$1" report_name="$2"
      local -a values=()
      # The declaration being parsed contains the literal shell variable name `$reports`.
      # shellcheck disable=SC2016
      mapfile -t values < <(sed -nE \
        's#^node ops/ci/assert-report\.mjs vitest "\$reports/'"$report_name"'" ([0-9]+)$#\1#p' \
        "$source")
      [[ "${#values[@]}" -eq 1 && "${values[0]}" =~ ^[1-9][0-9]*$ ]] \
        || { refuse FAMILY_VITEST_SOURCE_INVALID "$source:$report_name"; return 1; }
      printf '%s\n' "${values[0]}"
    }
    fast_count="$(source_count "$2" vitest.json)" || exit 1
    coverage_count="$(source_count "$3" coverage-tests.json)" || exit 1
    [[ "$fast_count" -eq "$coverage_count" ]] \
      || { refuse FAMILY_VITEST_SOURCE_DRIFT "fast=$fast_count coverage=$coverage_count"; exit 1; }
    printf '%s\n' "$fast_count"
    ;;
  junit)
    [[ "$#" -eq 4 ]] || { refuse FAMILY_REPORT_USAGE "junit FILE TESTS SKIPPED"; exit 2; }
    expected_tests="$3"
    expected_skipped="$4"
    [[ "$expected_tests" =~ ^[1-9][0-9]*$ && "$expected_skipped" =~ ^[0-9]+$ ]] \
      || { refuse FAMILY_REPORT_EXPECTATION_INVALID "$expected_tests/$expected_skipped"; exit 1; }
    mapfile -t roots < <(grep '^<testsuites ' "$report" || true)
    [[ "${#roots[@]}" -eq 1 ]] \
      || { refuse FAMILY_REPORT_INVALID "expected one testsuites root"; exit 1; }
    tests="$(xml_attribute "${roots[0]}" tests)"
    failures="$(xml_attribute "${roots[0]}" failures)"
    errors="$(xml_attribute "${roots[0]}" errors)"
    if grep -qE '[[:space:]]skipped="[0-9]+"' <<<"${roots[0]}"; then
      skipped="$(xml_attribute "${roots[0]}" skipped)"
    else
      mapfile -t suites < <(grep '^[[:space:]]*<testsuite ' "$report" || true)
      [[ "${#suites[@]}" -gt 0 ]] || { refuse FAMILY_REPORT_INVALID "no testsuite counters"; exit 1; }
      skipped=0
      for suite in "${suites[@]}"; do
        skipped=$((skipped + $(xml_attribute "$suite" disabled)))
      done
    fi
    [[ "$tests" -eq "$expected_tests" && "$tests" -gt 0 && "$failures" -eq 0 \
      && "$errors" -eq 0 && "$skipped" -eq "$expected_skipped" ]] \
      || { refuse FAMILY_REPORT_OUTCOME_INVALID "tests=$tests/$expected_tests failures=$failures errors=$errors skipped=$skipped/$expected_skipped"; exit 1; }
    executed=$((tests - skipped))
    jq -cn --argjson tests "$tests" --argjson executed "$executed" --argjson skipped "$skipped" \
      '{kind:"junit",tests:$tests,executed:$executed,failures:0,errors:0,skipped:$skipped}'
    ;;
  vitest)
    [[ "$#" -eq 3 && "$3" =~ ^[1-9][0-9]*$ ]] \
      || { refuse FAMILY_REPORT_USAGE "vitest FILE TESTS"; exit 2; }
    validate_json_report "$report" || exit 1
    jq -ce --argjson expected "$3" '
      select(.success == true and .numTotalTests == $expected and .numTotalTests > 0 and
        .numPassedTests == $expected and .numFailedTests == 0 and
        .numPendingTests == 0 and .numTodoTests == 0) |
      {kind:"vitest",tests:.numTotalTests,passed:.numPassedTests,failed:0,pending:0,todo:0}
    ' "$report" || { refuse FAMILY_REPORT_OUTCOME_INVALID "$report"; exit 1; }
    ;;
  formal-json)
    [[ "$#" -eq 2 ]] || { refuse FAMILY_REPORT_USAGE "formal-json FILE"; exit 2; }
    validate_json_report "$report" || exit 1
    jq -ce '
      select(. == {schema_version:"bullet.formal-summary.v1",models:2,completed_models:2,
        pinned_summary_present:true,status:"PASS",exit_code:0,signed:false,
        evidence_class:"DIAGNOSTIC_ONLY"}) |
      {kind:"formal",models:2,completed_models:2,pinned_summary_present:true,status:"PASS"}
    ' "$report" || { refuse FAMILY_REPORT_OUTCOME_INVALID "$report"; exit 1; }
    ;;
  formal-log)
    [[ "$#" -eq 2 ]] || { refuse FAMILY_REPORT_USAGE "formal-log FILE"; exit 2; }
    expected="$(printf '%s\n' \
      'schema=bullet.formal-log.v1' 'models=2' 'completed_without_error=2' \
      'pinned_summary_present=1' 'exit_code=0' 'classification=DIAGNOSTIC_ONLY')"
    [[ "$(<"$report")" == "$expected" ]] \
      || { refuse FAMILY_REPORT_OUTCOME_INVALID "$report"; exit 1; }
    jq -cn '{kind:"formal-log",models:2,completed_without_error:2,pinned_summary_present:true,exit_code:0}'
    ;;
  *) refuse FAMILY_REPORT_USAGE "expected vitest-source-pair|junit|vitest|formal-json|formal-log"; exit 2 ;;
esac
