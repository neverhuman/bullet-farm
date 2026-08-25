#!/usr/bin/env bash
set -euo pipefail
# shellcheck source=ops/ci/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
test_root="$(mktemp -d)"
trap 'rm -rf -- "$test_root"' EXIT
mkdir -p .ci-artifacts/test
printf 'sanitized\n' >.ci-artifacts/test/artifact.txt
CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 0 \
  'bash ops/ci/observation-test.sh' .ci-artifacts/test/artifact.txt >/dev/null
observation=.ci-artifacts/observations/observation-test.json
jq -e '
  .schema_version == "bullet.ci-observation.v1" and .repository == "bullet-farm" and
  (.commit_oid | test("^[0-9a-f]{40}$")) and (.tree_oid | test("^[0-9a-f]{40}$")) and
  (.clean | type == "boolean") and .commands == ["bash ops/ci/observation-test.sh"] and
  .outcomes == [{"lane":"observation-test","status":"PASS","exit_code":0}] and
  (.artifact_hashes | length == 1) and
  .artifact_hashes[0].path == ".ci-artifacts/test/artifact.txt" and
  (.artifact_hashes[0].sha256 | test("^[0-9a-f]{64}$")) and
  .signed == false and .evidence_class == "DIAGNOSTIC_ONLY"
' "$observation" >/dev/null
bash ops/ci/artifact-check.sh observation-test >/dev/null
valid_observation="$test_root/valid-observation.json"
cp "$observation" "$valid_observation"
for mutation in \
  '.outcomes[0].raw_detail="forbidden"' \
  '.artifact_hashes[0].raw_detail="forbidden"' \
  '.tool_versions.fixture={raw_detail:"forbidden"}'; do
  jq "$mutation" "$valid_observation" >"$observation"
  if output="$(bash ops/ci/artifact-check.sh observation-test 2>&1)" \
    || [[ "$output" != *CI_OBSERVATION_INVALID* ]]; then
    refuse OBSERVATION_NESTED_SCHEMA_GUARD_FAILED "$mutation: $output"; exit 1
  fi
done
cp "$valid_observation" "$observation"
schema_pattern="$(jq -r '.properties.artifact_hashes.items.properties.path.pattern' \
  docs/schemas/bullet.ci-observation.v1.schema.json)"
jq -ne --arg pattern "$schema_pattern" --arg path '.ci-artifacts/report.xml' \
  '$path | test($pattern)' >/dev/null
for invalid in report.xml .ci-artifacts/../escape .ci-artifacts/a//b '.ci-artifacts/a\b'; do
  if jq -ne --arg pattern "$schema_pattern" --arg path "$invalid" '$path | test($pattern)' >/dev/null; then
    refuse OBSERVATION_SCHEMA_PATH_GUARD_FAILED "$invalid"; exit 1
  fi
done
if CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 0 valid ../escape >/dev/null 2>&1; then
  refuse OBSERVATION_PATH_GUARD_FAILED "parent traversal accepted"; exit 1
fi
if CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 0 valid .ci-artifacts/../escape >/dev/null 2>&1; then
  refuse OBSERVATION_PATH_GUARD_FAILED "root traversal accepted"; exit 1
fi
if CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 0 '' >/dev/null 2>&1; then
  refuse OBSERVATION_COMMAND_GUARD_FAILED "empty command accepted"; exit 1
fi
if CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 256 valid >/dev/null 2>&1; then
  refuse OBSERVATION_EXIT_GUARD_FAILED "exit 256 accepted"; exit 1
fi
if CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh ../escape 1 valid >/dev/null 2>&1; then
  refuse OBSERVATION_LANE_GUARD_FAILED "path-shaped lane accepted"; exit 1
fi

printf 'secret-shaped raw diagnostic\n' >.ci-artifacts/test/raw.log
CI_COMMAND_COUNT=2 bash scripts/ci-observation.sh lint 1 \
  'bash scripts/ci-doctor.sh lint' 'bash ops/ci/lint.sh' .ci-artifacts/test/raw.log >/dev/null
if output="$(bash ops/ci/artifact-check.sh lint 2>&1)" \
  || [[ "$output" != *CI_ARTIFACT_INVENTORY_INVALID* ]]; then
  refuse OBSERVATION_FAIL_ARTIFACT_GUARD_FAILED "FAIL observation accepted raw artifact: $output"; exit 1
fi
rm -f .ci-artifacts/test/raw.log .ci-artifacts/observations/lint.json

CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 1 \
  'bash ops/ci/observation-test.sh' .ci-artifacts/test/artifact.txt >/dev/null
if output="$(bash ops/ci/artifact-check.sh observation-test 2>&1)" \
  || [[ "$output" != *CI_ARTIFACT_INVENTORY_INVALID* ]]; then
  refuse OBSERVATION_FAIL_ALLOWLIST_GUARD_FAILED \
    "FAIL observation accepted an allow-listed but unvalidated artifact: $output"; exit 1
fi
CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-test 0 \
  'bash ops/ci/observation-test.sh' .ci-artifacts/test/artifact.txt >/dev/null

CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh history 0 \
  'bash ops/ci/not-history.sh' >/dev/null
if output="$(bash ops/ci/artifact-check.sh history 2>&1)" \
  || [[ "$output" != *CI_OBSERVATION_COMMAND_INVALID* ]]; then
  refuse OBSERVATION_SCHEDULED_COMMAND_GUARD_FAILED "$output"; exit 1
fi
CI_COMMAND_COUNT=2 bash scripts/ci-observation.sh history 0 \
  'bash scripts/ci-doctor.sh history' 'bash ops/ci/history.sh' >/dev/null
jq 'del(.tool_versions.gitleaks)' .ci-artifacts/observations/history.json >"$test_root/history.json"
mv "$test_root/history.json" .ci-artifacts/observations/history.json
if output="$(bash ops/ci/artifact-check.sh history 2>&1)" \
  || [[ "$output" != *CI_TOOL_VERSION_MISSING* ]]; then
  refuse OBSERVATION_SCHEDULED_TOOL_GUARD_FAILED "$output"; exit 1
fi
rm -f .ci-artifacts/observations/history.json

mkdir -p .ci-artifacts/family
printf '{"schema_version":"bullet.family-ci-observation.v1"}\n' \
  >.ci-artifacts/family/subjects.json
CI_COMMAND_COUNT=2 bash scripts/ci-observation.sh family-contract 0 \
  'bash scripts/ci-doctor.sh family-contract' 'bash ops/ci/family-contract.sh' \
  .ci-artifacts/family/subjects.json >/dev/null
jq '.tool_versions += {
  git:"git version 2.43.0",rustc:"rustc 1.95.0 fixture",cargo:"cargo 1.95.0 fixture",
  cargo_nextest:"cargo-nextest 0.9.137 fixture",node:"v22.23.2",npm:"10.9.8"
}' .ci-artifacts/observations/family-contract.json >"$test_root/family-contract.json"
mv "$test_root/family-contract.json" .ci-artifacts/observations/family-contract.json
bash ops/ci/artifact-check.sh family-contract >/dev/null
rm -rf .ci-artifacts/family
rm -f .ci-artifacts/observations/family-contract.json

guard_root="$test_root/artifact-root"
guard_outside="$test_root/outside"
mkdir -p "$guard_root" "$guard_outside"
ln -s "$guard_outside" "$guard_root/.ci-artifacts"
if prepare_ci_directory "$guard_root" .ci-artifacts; then
  refuse OBSERVATION_ROOT_SYMLINK_GUARD_FAILED "root symlink accepted"; exit 1
fi
rm "$guard_root/.ci-artifacts"
mkdir "$guard_root/.ci-artifacts"
ln -s "$guard_outside" "$guard_root/.ci-artifacts/junit"
if prepare_ci_directory "$guard_root" .ci-artifacts/junit; then
  refuse OBSERVATION_NESTED_SYMLINK_GUARD_FAILED "nested symlink accepted"; exit 1
fi

saved_observation="$observation.saved"
cp "$observation" "$saved_observation"
jq '.artifact_hashes[0].path=".ci-artifacts/../escape"' "$saved_observation" >"$observation"
if output="$(bash ops/ci/artifact-check.sh observation-test 2>&1)" \
  || [[ "$output" != *CI_ARTIFACT_PATH_INVALID* ]]; then
  refuse OBSERVATION_VALIDATOR_GUARD_FAILED "validator accepted traversal: $output"; exit 1
fi
mv "$saved_observation" "$observation"

outside="$REPO_ROOT/.ci-artifact-outside.$$"
mv .ci-artifacts/test/artifact.txt "$outside"
ln -s "../../${outside##*/}" .ci-artifacts/test/artifact.txt
if output="$(bash ops/ci/artifact-check.sh observation-test 2>&1)" \
  || [[ "$output" != *CI_ARTIFACT_PATH_INVALID* ]]; then
  refuse OBSERVATION_SYMLINK_GUARD_FAILED "validator accepted symlink: $output"; exit 1
fi
if CI_COMMAND_COUNT=1 bash scripts/ci-observation.sh observation-symlink-test 0 valid \
  .ci-artifacts/test/artifact.txt >/dev/null 2>&1; then
  refuse OBSERVATION_SYMLINK_GUARD_FAILED "producer accepted symlink"; exit 1
fi
rm .ci-artifacts/test/artifact.txt
mv "$outside" .ci-artifacts/test/artifact.txt
rm -rf .ci-artifacts/test
rm -f "$observation"
log "CI observation guards passed"
