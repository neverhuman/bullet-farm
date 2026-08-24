#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"

log "fast lane: hub metadata and onboarding checks"
require_file "README.md"
require_file "AGENTS.md"
require_file "repos.manifest.toml"
require_file "family.lock"
require_file "agent/owner-map.json"
require_file "agent/test-map.json"
python3 scripts/check-hub.py
log "fast lane passed"
