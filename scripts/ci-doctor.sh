#!/usr/bin/env bash
set -euo pipefail

lane="${1:-all}"
case "$lane" in
  fast|required) tools=(bash cargo git grep mktemp rustc) ;;
  contract) tools=(awk bash cargo curl find grep java jq rustc sed sha1sum sha256sum sort tee) ;;
  security) tools=(bash cargo-deny gitleaks zizmor) ;;
  audit) tools=(bash jankurai) ;;
  all) tools=(awk bash cargo cargo-deny curl find git gitleaks grep java jankurai jq mktemp rustc sed sha1sum sha256sum sort tee zizmor) ;;
  *) echo "ci-doctor: expected fast|required|contract|security|audit|all" >&2; exit 2 ;;
esac

missing=0
for tool in "${tools[@]}"; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    printf 'ci-doctor: missing %s for %s\n' "$tool" "$lane" >&2
    missing=1
  fi
done
[[ "$missing" -eq 0 ]] || exit 1

rust_version="$(rustc --version)"
[[ "$rust_version" == "rustc 1.95.0 "* ]] || {
  printf 'ci-doctor: expected rustc 1.95.0, found %s\n' "$rust_version" >&2
  exit 1
}
if [[ "$lane" == audit || "$lane" == all ]]; then
  jankurai_version="$(jankurai --version)"
  [[ "$jankurai_version" == "jankurai 1.6.11" ]] || {
    printf 'ci-doctor: expected jankurai 1.6.11, found %s\n' "$jankurai_version" >&2
    exit 1
  }
fi
if [[ "$lane" == security || "$lane" == all ]]; then
  [[ "$(gitleaks version)" == "8.21.2" ]] || {
    echo "ci-doctor: expected gitleaks 8.21.2" >&2
    exit 1
  }
  [[ "$(cargo-deny --version)" == "cargo-deny 0.19.8" ]] || {
    echo "ci-doctor: expected cargo-deny 0.19.8" >&2
    exit 1
  }
  [[ "$(zizmor --version)" == "zizmor 1.25.2" ]] || {
    echo "ci-doctor: expected zizmor 1.25.2" >&2
    exit 1
  }
fi
printf 'ci-doctor: %s lane tools present; %s\n' "$lane" "$rust_version"
