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
passed=0

expect_refusal() {
  local code="$1"
  shift
  if "$@" >"$scratch/refusal.log" 2>&1; then
    printf 'expected refusal: %s\n' "$code" >&2
    exit 1
  fi
  grep -Fq -- "$code" "$scratch/refusal.log" || {
    printf 'missing refusal: %s\n' "$code" >&2
    exit 1
  }
  passed=$((passed + 1))
}

invoke() {
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" GITHUB_SHA="$sha" \
    RUNNER_TEMP="$scratch/runner" bash "$wrapper" "$@"
}

mkdir "$scratch/runner"
expect_refusal PUBLICATION_CI_USAGE bash "$wrapper"
expect_refusal PUBLICATION_CI_USAGE bash "$wrapper" arbitrary
expect_refusal PUBLICATION_CI_USAGE bash "$wrapper" source-scan extra
expect_refusal PUBLICATION_DISPOSABLE_CI_REQUIRED \
  env GITHUB_ACTIONS=false bash "$wrapper" source-scan
expect_refusal PUBLICATION_EVENT_SHA_MISMATCH \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" \
  GITHUB_SHA=0000000000000000000000000000000000000000 \
  RUNNER_TEMP="$scratch/runner" bash "$wrapper" source-scan
expect_refusal PUBLICATION_EVENT_SHA_INVALID \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" GITHUB_SHA=main \
  RUNNER_TEMP="$scratch/runner" bash "$wrapper" source-scan
printf 'dirty\n' >"$repo/untracked.txt"
expect_refusal PUBLICATION_CHECKOUT_DIRTY invoke source-scan
rm "$repo/untracked.txt"
ln -s "$scratch/runner" "$scratch/runner-link"
expect_refusal PUBLICATION_RUNNER_TEMP_INVALID \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" GITHUB_SHA="$sha" \
  RUNNER_TEMP="$scratch/runner-link" bash "$wrapper" source-scan
ln -s "$repo" "$scratch/aggregate-link"
expect_refusal PUBLICATION_CHECKOUT_INVALID \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$scratch/aggregate-link" GITHUB_SHA="$sha" \
  RUNNER_TEMP="$scratch/runner" bash "$wrapper" source-scan
expect_refusal PUBLICATION_SOURCE_SCAN_REQUIRED invoke prove
mkdir "$scratch/runner/bullet-publication-report" "$scratch/runner/bullet-publication-private"
printf '{"success":true}\n' >"$scratch/runner/bullet-publication-report/source-scan.json"
expect_refusal PUBLICATION_SOURCE_SCAN_REQUIRED invoke prove
rm -r "$scratch/runner/bullet-publication-report" "$scratch/runner/bullet-publication-private"
invoke source-scan
[[ "$(cat "$scratch/runner/bullet-publication-report/source-scan.json")" == '[]' ]]
[[ ! -e "$scratch/runner/bullet-publication-cargo" ]]
passed=$((passed + 1))
expect_refusal 'File exists' invoke source-scan
mkdir "$scratch/canary-runner"
printf 'aws_access_key_id = %s%s\n' 'AKIA' '6QWERTYUIOPASDFG' >"$repo/canary.txt"
git -C "$repo" add canary.txt
git -C "$repo" commit -qm 'intentional scanner fixture'
expect_refusal PUBLICATION_SOURCE_SCAN_FAILED \
  env GITHUB_ACTIONS=true GITHUB_WORKSPACE="$repo" \
  GITHUB_SHA="$(git -C "$repo" rev-parse HEAD)" RUNNER_TEMP="$scratch/canary-runner" \
  bash "$wrapper" source-scan
[[ ! -e "$scratch/canary-runner/bullet-publication-report/source-scan.json" ]]
[[ "$(find "$scratch/canary-runner/bullet-publication-report" -type f | wc -l)" == 1 ]]

# Execute the actual always-running final check, without a duplicate implementation.
awk 'capture {sub(/^          /, ""); print} /^        run: \|$/ {capture = 1}' \
  "${wrapper%/*}/root/.github/workflows/publication.yml" >"$scratch/final.sh"
[[ -s "$scratch/final.sh" ]]
env INTEGRITY_RESULT=success bash "$scratch/final.sh"
passed=$((passed + 1))
for result in failure cancelled skipped neutral malformed ''; do
  expect_refusal 'publication integrity did not succeed' \
    env INTEGRITY_RESULT="$result" bash "$scratch/final.sh"
done
printf 'publication wrapper fixtures: %s passed\n' "$passed"
