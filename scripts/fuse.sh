#!/usr/bin/env bash
# Write a local fusion workspace. This is the only place sibling path patches
# may appear. The generated tree is gitignored.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

SOURCE="local"
ALL=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --source) SOURCE="${2:?--source requires a value}"; shift 2 ;;
    --all)    ALL=1; shift ;;
    --*)      echo "unknown flag: $1" >&2; exit 2 ;;
    *)        SOURCE="$1"; shift ;;
  esac
done
export ALL

ROOT="$(cd .. && pwd)"
cargo run --quiet --locked --bin bullet-family -- deps check
OUT=".fusion"
rm -rf "$OUT"
mkdir -p "$OUT"

cat > "$OUT/dev.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
cmd="\${1:-help}"
case "\$cmd" in
  build)
    (cd "$ROOT/bullet-kernel" && cargo test --workspace --offline --locked)
    (cd "$ROOT/bullet-git" && cargo test --workspace --offline --locked)
    ;;
  help|*)
    echo "usage: .fusion/dev.sh build"
    ;;
esac
EOF
chmod +x "$OUT/dev.sh"

cat > "$OUT/source" <<EOF
$SOURCE
EOF

printf 'fused %s workspace at %s\n' "$SOURCE" "$OUT"
