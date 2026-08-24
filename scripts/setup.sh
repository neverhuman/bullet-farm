#!/usr/bin/env bash
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
cd "$HUB"

exec cargo run --locked --quiet --manifest-path "$HUB/Cargo.toml" --bin bullet-family -- \
  setup --root "$FAMILY" --source jeryu "$@"
