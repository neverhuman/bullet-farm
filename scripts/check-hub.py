#!/usr/bin/env python3
"""Fail-closed checks for the public hub onboarding surface."""

from pathlib import Path
import json
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_README = [
    "Many minds. One verified line to main.",
    "just setup",
    "just demo",
    "What we will not claim",
    "Gastown",
    "BulletGit",
    "family.lock",
]
REQUIRED_FILES = [
    "README.md",
    "AGENTS.md",
    "SPLIT.md",
    "Justfile",
    "repos.manifest.toml",
    "family.lock",
    "scripts/fuse.sh",
    "scripts/demo.sh",
    "scripts/check-path-deps.py",
    "ops/ci/family.sh",
    "docs/architecture/overview.md",
]


def fail(message: str) -> None:
    print(f"hub-check: {message}", file=sys.stderr)
    sys.exit(1)


def main() -> None:
    for rel in REQUIRED_FILES:
        if not (ROOT / rel).is_file():
            fail(f"missing {rel}")
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    for needle in REQUIRED_README:
        if needle not in readme:
            fail(f"README.md missing required phrase: {needle}")
    denied = readme.split("## What we will not claim", 1)
    if len(denied) != 2:
        fail("README.md missing 'What we will not claim'")
    claimed = denied[0]
    if re.search(r"100%\s+autonomy|zero regressions", claimed, re.I):
        fail("README.md must not claim 100% autonomy or zero regressions")
    owners = json.loads((ROOT / "agent/owner-map.json").read_text(encoding="utf-8"))
    if "README.md" not in owners.get("owners", {}):
        fail("owner-map.json missing README.md")
    print("hub-check: ok")


if __name__ == "__main__":
    main()
