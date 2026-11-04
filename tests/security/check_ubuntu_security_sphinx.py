#!/usr/bin/env python3
"""Prepare and check a pinned Ubuntu Security documentation Sphinx build.

This script is solely a documentation integration test, never a production
security policy or claim of acceptance by Canonical.
SPDX-License-Identifier: MIT
"""
from __future__ import annotations

import argparse
from pathlib import Path
import sys


REL = Path("common-mistakes/ai-coding-agents.rst")
INDEX = Path("common-mistakes/index.rst")
HEADING = (
    "\nAI coding agents\n"
    "================\n"
    ".. toctree::\n"
    "   :maxdepth: 1\n\n"
    "   ai-coding-agents\n"
)


def prepare(repo: Path, upstream: Path) -> None:
    source = repo / "integrations/ubuntu-security-documentation" / REL
    destination = upstream / REL
    index = upstream / INDEX
    if not source.is_file() or not index.is_file():
        raise RuntimeError("Missing proposal or pinned upstream index file")
    if destination.exists():
        raise RuntimeError("Upstream already has this document. Reconcile rather than overwrite.")
    original = index.read_text(encoding="utf-8")
    if "   ai-coding-agents" in original:
        raise RuntimeError("Toctree entry already exists. Reconcile upstream layout.")
    # The checkout is deliberately temporary. Do not change Canonical's repository.
    destination.write_text(source.read_text(encoding="utf-8"), encoding="utf-8")
    index.write_text(original.rstrip() + "\n" + HEADING, encoding="utf-8")
    print("Prepared proposal inside pinned temporary upstream checkout")


def relevant_warnings(path: Path) -> set[str]:
    if not path.is_file():
        raise RuntimeError("Sphinx warning record is missing: " + str(path))
    return {
        line.strip() for line in path.read_text(encoding="utf-8").splitlines()
        if "WARNING:" in line or "ERROR:" in line
    }


def verify(upstream: Path, html: Path, warnings: Path, baseline_warnings: Path) -> None:
    page = html / "common-mistakes" / "ai-coding-agents" / "index.html"
    if not page.is_file():
        raise RuntimeError("Expected generated page not found: " + str(page))
    content = page.read_text(encoding="utf-8")
    for required in (
        "AI coding agents and confinement assumptions",
        "Ubuntu 22.04 LTS considerations",
        "Snap confinement is not universal",
    ):
        if required not in content:
            raise RuntimeError("Generated HTML is missing: " + required)
    navigation = html / "common-mistakes" / "index.html"
    if not navigation.is_file() or "ai-coding-agents" not in navigation.read_text(encoding="utf-8"):
        raise RuntimeError("Proposed document is missing from generated common-mistakes navigation")
    if not (upstream / REL).is_file():
        raise RuntimeError("Source document absent after build")
    original = relevant_warnings(baseline_warnings)
    modified = relevant_warnings(warnings)
    introduced = modified - original
    if introduced:
        raise RuntimeError(
            "New Sphinx warnings after adding the proposal:\n" +
            "\n".join(sorted(introduced))
        )
    print(
        "PASS: proposed page built and linked, "
        f"no new Sphinx warnings (upstream baseline {len(original)}, proposal {len(modified)})"
    )
    if original:
        print("NOTE: existing upstream warnings are preserved as baseline evidence", file=sys.stderr)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=("prepare", "verify"))
    parser.add_argument("--repo", required=True, type=Path)
    parser.add_argument("--upstream", required=True, type=Path)
    parser.add_argument("--html", type=Path)
    parser.add_argument("--warnings", type=Path)
    parser.add_argument("--baseline-warnings", type=Path)
    args = parser.parse_args()
    if args.action == "prepare":
        prepare(args.repo.resolve(), args.upstream.resolve())
    else:
        if args.html is None or args.warnings is None or args.baseline_warnings is None:
            parser.error("--html, --warnings and --baseline-warnings required for verify")
        verify(
            args.upstream.resolve(), args.html.resolve(),
            args.warnings.resolve(), args.baseline_warnings.resolve()
        )


if __name__ == "__main__":
    main()
