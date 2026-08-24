#!/usr/bin/env bash
# Write a local fusion workspace. This is the only place sibling path patches
# may appear. The generated tree is gitignored.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

SOURCE="${1:-local}"
if [[ "${1:-}" == "--source" ]]; then
  SOURCE="${2:-local}"
fi

ROOT="$(cd .. && pwd)"
OUT=".fusion"
rm -rf "$OUT"
mkdir -p "$OUT"

cat > "$OUT/dev.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
cmd="\${1:-help}"
case "\$cmd" in
  build)
    (cd "$ROOT/bullet-kernel" && cargo test --workspace --offline --locked || cargo test --workspace)
    (cd "$ROOT/bullet-git" && cargo test --workspace --offline --locked || cargo test --workspace)
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
