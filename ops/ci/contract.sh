#!/usr/bin/env bash
# Hub-only canonical contract. No sibling checkout or live process is required.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"
log "contract lane: canonical v1alpha1 bundle and hostile fixtures"
cargo run --locked --quiet -p bullet-wire --bin bullet-contract -- check --root "$REPO_ROOT"
cargo test --locked -p bullet-wire
log "contract lane: exactly two pinned formal models"
bash formal/model-check.sh
log "contract lane passed"
