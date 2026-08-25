#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

lane="${1:-all}"
case "$lane" in
  required) bash ops/ci/required.sh ;;
  fast)     bash ops/ci/fast.sh ;;
  family)   bash ops/ci/family.sh ;;
  contract) bash ops/ci/contract.sh ;;
  family-contract) bash ops/ci/family-contract.sh ;;
  security) bash ops/ci/security.sh ;;
  audit)    bash ops/ci/audit.sh ;;
  toolchain-pinned) bash ops/ci/toolchain-pinned.sh ;;
  gates|all) bash ops/ci/required.sh && bash ops/ci/contract.sh ;;
  *) echo "usage: $0 {required|fast|contract|family-contract|family|security|audit|toolchain-pinned|all}" >&2; exit 2 ;;
esac
