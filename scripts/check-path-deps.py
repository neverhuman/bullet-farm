#!/usr/bin/env python3
"""Fail if any member Cargo.toml uses a sibling checkout path dependency."""

from __future__ import annotations

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
FAMILY = ROOT.parent
FORBIDDEN = re.compile(r"""path\s*=\s*["']\.\./""")
SKIP_DIRS = {".git", "target", "node_modules", ".fusion"}


def iter_cargo_tomls(root: Path) -> list[Path]:
    found: list[Path] = []
    stack = [root]
    while stack:
        current = stack.pop()
        if not current.is_dir():
            continue
        if current.name in SKIP_DIRS:
            continue
        cargo = current / "Cargo.toml"
        if cargo.is_file():
            found.append(cargo)
        try:
            children = list(current.iterdir())
        except OSError:
            continue
        stack.extend(child for child in children if child.is_dir())
    return found


def main() -> None:
    scan_root = FAMILY if (FAMILY / "repos.manifest.toml").is_file() else ROOT
    hits: list[str] = []
    for cargo in iter_cargo_tomls(scan_root):
        text = cargo.read_text(encoding="utf-8")
        for lineno, line in enumerate(text.splitlines(), start=1):
            if FORBIDDEN.search(line):
                hits.append(f"{cargo}:{lineno}:{line.strip()}")
    if hits:
        print("path-deps: forbidden sibling path dependency:", file=sys.stderr)
        for hit in hits:
            print(f"  {hit}", file=sys.stderr)
        sys.exit(1)
    print(f"path-deps: ok ({scan_root})")


if __name__ == "__main__":
    main()
