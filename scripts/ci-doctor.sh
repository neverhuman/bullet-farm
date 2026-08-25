#!/usr/bin/env bash
set -euo pipefail
lane="${1:-all}"
baseline=(awk bash dirname git head jq mkdir mv realpath rm sed sort tr)
case "$lane" in
  source-scan) tools=("${baseline[@]}" cat gitleaks xargs) ;;
  fast) tools=("${baseline[@]}" cargo cargo-nextest chmod grep mktemp rmdir rustc) ;;
  lint) tools=("${baseline[@]}" actionlint cargo cargo-clippy cargo-nextest cmp comm cp find ln rg rustc rustfmt shellcheck wc) ;;
  contract) tools=("${baseline[@]}" cargo cargo-nextest curl find grep java mktemp rustc sha1sum tee) ;;
  security) tools=("${baseline[@]}" cargo cargo-deny date gitleaks mktemp rustc zizmor) ;;
  docs) tools=("${baseline[@]}" cargo cmp docker file find grep id mktemp realpath rg rustc stat) ;;
  required) tools=("${baseline[@]}" actionlint cargo cargo-clippy cargo-deny cargo-nextest chmod cmp comm cp curl date docker file find gitleaks grep id java ln mktemp realpath rg rmdir rustc rustfmt sha1sum shellcheck stat tee wc zizmor) ;;
  family|family-contract) tools=("${baseline[@]}" actionlint cargo cargo-clippy cargo-deny cargo-nextest chmod cmp comm cp curl date docker file find gitleaks grep id java ln mktemp node npm rg rmdir rustc rustfmt sha1sum shellcheck stat tee uname wc zizmor) ;;
  history) tools=("${baseline[@]}" cat gitleaks xargs) ;;
  links) tools=("${baseline[@]}" lychee rg) ;;
  advisory) tools=("${baseline[@]}" cargo cargo-deny date rustc) ;;
  coverage) tools=("${baseline[@]}" cargo cargo-llvm-cov cargo-nextest cmp comm grep ln mktemp rustc wc) ;;
  platform) tools=("${baseline[@]}" cargo grep rustc uname) ;;
  audit) tools=("${baseline[@]}" jankurai) ;;
  toolchain-pinned) tools=("${baseline[@]}" b3sum cargo date grep rustc rustup tee wc) ;;
  all) tools=("${baseline[@]}" actionlint b3sum cargo cargo-clippy cargo-deny cargo-llvm-cov cargo-nextest cat chmod cmp comm cp curl date docker file find gitleaks grep id jankurai java ln lychee mktemp node npm realpath rg rmdir rustc rustfmt rustup sha1sum shellcheck stat tee uname wc xargs zizmor) ;;
  *)
    echo "ci-doctor: expected source-scan|fast|lint|contract|security|docs|required|family|family-contract|history|links|advisory|coverage|platform|audit|toolchain-pinned|all" >&2
    exit 2
    ;;
esac

missing=0
for tool in "${tools[@]}"; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    printf 'ci-doctor: missing %s for %s\n' "$tool" "$lane" >&2
    missing=1
  fi
done
if ! command -v sha256sum >/dev/null 2>&1 && ! command -v shasum >/dev/null 2>&1; then
  printf 'ci-doctor: missing sha256sum or shasum for %s\n' "$lane" >&2
  missing=1
fi
[[ "$missing" -eq 0 ]] || exit 1

if [[ "$lane" =~ ^(fast|lint|contract|security|docs|required|family|family-contract|advisory|coverage|platform|toolchain-pinned|all)$ ]]; then
  rust_version="$(rustc --version)"
  [[ "$rust_version" == "rustc 1.95.0 "* ]] || {
    printf 'ci-doctor: expected rustc 1.95.0, found %s\n' "$rust_version" >&2
    exit 1
  }
fi
if [[ "$lane" =~ ^(fast|lint|contract|required|family|family-contract|coverage|all)$ ]]; then
  nextest_version="$(cargo-nextest --version | head -n 1)"
  [[ "$nextest_version" == "cargo-nextest 0.9.137 "* ]] || {
    printf 'ci-doctor: expected cargo-nextest 0.9.137, found %s\n' "$nextest_version" >&2
    exit 1
  }
fi
if [[ "$lane" =~ ^(source-scan|security|required|family|family-contract|history|all)$ ]]; then
  [[ "$(gitleaks version)" == 8.21.2 ]] || { echo "ci-doctor: expected gitleaks 8.21.2" >&2; exit 1; }
fi
if [[ "$lane" =~ ^(security|required|family|family-contract|advisory|all)$ ]]; then
  [[ "$(cargo-deny --version)" == "cargo-deny 0.19.8" ]] || { echo "ci-doctor: expected cargo-deny 0.19.8" >&2; exit 1; }
fi
if [[ "$lane" =~ ^(lint|required|family|family-contract|all)$ ]]; then
  [[ "$(actionlint -version | head -n 1)" == 1.7.8 ]] || { echo "ci-doctor: expected actionlint 1.7.8" >&2; exit 1; }
  [[ "$(shellcheck --version | awk '/^version:/{print $2}')" == 0.10.0 ]] || { echo "ci-doctor: expected ShellCheck 0.10.0" >&2; exit 1; }
fi
if [[ "$lane" =~ ^(security|required|family|family-contract|all)$ ]]; then
  [[ "$(zizmor --version)" == "zizmor 1.25.2" ]] || { echo "ci-doctor: expected zizmor 1.25.2" >&2; exit 1; }
fi
if [[ "$lane" =~ ^(contract|required|family|family-contract|all)$ ]]; then
  java_version="$(java -version 2>&1 | head -n 1)"
  [[ "$java_version" == *'"21.'* ]] || { printf 'ci-doctor: expected Java 21, found %s\n' "$java_version" >&2; exit 1; }
fi
if [[ "$lane" == links || "$lane" == all ]]; then
  [[ "$(lychee --version)" == "lychee 0.24.0" ]] || { echo "ci-doctor: expected lychee 0.24.0" >&2; exit 1; }
fi
if [[ "$lane" == coverage || "$lane" == all ]]; then
  [[ "$(cargo llvm-cov --version)" == "cargo-llvm-cov 0.8.7" ]] || { echo "ci-doctor: expected cargo-llvm-cov 0.8.7" >&2; exit 1; }
fi
if [[ "$lane" == family || "$lane" == family-contract || "$lane" == all ]]; then
  [[ "$(node --version)" == "v22.23.2" ]] \
    || { printf 'ci-doctor: expected Node v22.23.2, found %s\n' "$(node --version)" >&2; exit 1; }
  [[ "$(npm --version)" == "10.9.8" ]] \
    || { printf 'ci-doctor: expected npm 10.9.8, found %s\n' "$(npm --version)" >&2; exit 1; }
fi
if [[ "$lane" == audit || "$lane" == all ]]; then
  [[ "$(jankurai --version)" == "jankurai 1.6.11" ]] || { echo "ci-doctor: expected jankurai 1.6.11" >&2; exit 1; }
fi
if [[ "$lane" == toolchain-pinned || "$lane" == all ]]; then
  export RUSTUP_AUTO_INSTALL=0
  rustup toolchain list | grep -q '^1\.97\.1-' || { echo "ci-doctor: Rust 1.97.1 toolchain is missing" >&2; exit 1; }
  [[ "$(rustup run 1.97.1 rustc --version)" == "rustc 1.97.1 "* ]] || { echo "ci-doctor: invalid Rust 1.97.1 toolchain" >&2; exit 1; }
  [[ "$(b3sum --version)" == "b3sum 1.8.2" ]] || { echo "ci-doctor: expected b3sum 1.8.2" >&2; exit 1; }
fi
printf 'ci-doctor: %s lane tools present and pinned\n' "$lane"
