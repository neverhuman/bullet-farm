#!/usr/bin/env bash
set -euo pipefail
HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAMILY="$(cd "$HUB/.." && pwd)"
cd "$FAMILY/bullet-portal"
export VITE_BULLET_API="${VITE_BULLET_API:-http://127.0.0.1:7420}"
exec npm run dev -- --host 127.0.0.1 --port 5173
