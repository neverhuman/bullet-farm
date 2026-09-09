#!/usr/bin/env bash
set -euo pipefail

HUB="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
if [[ "$#" == 0 ]]; then
  echo 'readme-live-render: MEDIA_GENERATION_UNQUALIFIED (historical media remains unchanged)' >&2
  exit 78
fi
[[ "$#" == 2 && "$1" == --from-normalized ]] || {
  echo 'usage: readme-live-render.sh --from-normalized ABSOLUTE_DIRECTORY' >&2
  exit 2
}
bash "$HUB/scripts/readme-live-check.sh" --normalized-stage "$2" >/dev/null
# Admission of text data is deliberately a different boundary from rendering.
# The next packet must qualify literal drawtext expansion=none, complete output
# reconstruction, Portal provenance and publication custody before enabling it.
echo 'readme-live-render: MEDIA_GENERATION_UNQUALIFIED (normalized input admitted; renderer not qualified)' >&2
exit 78
