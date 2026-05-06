#!/usr/bin/env python3
"""Stage zentao-cli npm packages for release."""

from __future__ import annotations

import argparse
import importlib.util
import shutil
import tempfile
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
BUILD_SCRIPT = REPO_ROOT / "scripts" / "build_npm_package.py"

SPEC = importlib.util.spec_from_file_location("zentao_build_npm_package", BUILD_SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"Unable to load module from {BUILD_SCRIPT}")
BUILD_MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BUILD_MODULE)

PACKAGE_NATIVE_COMPONENTS = getattr(BUILD_MODULE, "PACKAGE_NATIVE_COMPONENTS", {})
PACKAGE_EXPANSIONS = getattr(BUILD_MODULE, "PACKAGE_EXPANSIONS", {})
PLATFORM_PACKAGES = getattr(BUILD_MODULE, "PLATFORM_PACKAGES", {})


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--release-version",
        required=True,
        help="Version to stage (for example 0.1.0 or 0.1.0-beta.1).",
    )
    parser.add_argument(
        "--package",
        dest="packages",
        action="append",
        required=True,
        help="Package name to stage. May be provided multiple times.",
    )
    parser.add_argument(
        "--vendor-src",
        type=Path,
        required=True,
        help="Directory containing prebuilt vendor payloads by target triple.",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="Directory where npm tarballs should be written (default: dist/npm).",
    )
    parser.add_argument(
        "--keep-staging-dirs",
        action="store_true",
        help="Retain temporary staging directories instead of deleting them.",
    )
    return parser.parse_args()


def expand_packages(packages: list[str]) -> list[str]:
    expanded: list[str] = []
    for package in packages:
        for expanded_package in PACKAGE_EXPANSIONS.get(package, [package]):
            if expanded_package not in expanded:
                expanded.append(expanded_package)
    return expanded


def tarball_name_for_package(package: str, version: str) -> str:
    if package in PLATFORM_PACKAGES:
        platform = package.removeprefix("zentao-cli-")
        return f"zentao-cli-npm-{platform}-{version}.tgz"
    return f"{package}-npm-{version}.tgz"


def main() -> int:
    args = parse_args()
    output_dir = args.output_dir or (REPO_ROOT / "dist" / "npm")
    output_dir.mkdir(parents=True, exist_ok=True)

    packages = expand_packages(list(args.packages))
    final_messages = []

    for package in packages:
        staging_dir = Path(tempfile.mkdtemp(prefix=f"npm-stage-{package}-"))
        pack_output = output_dir / tarball_name_for_package(package, args.release_version)

        try:
            cmd = [
                str(BUILD_SCRIPT),
                "--package",
                package,
                "--release-version",
                args.release_version,
                "--staging-dir",
                str(staging_dir),
                "--pack-output",
                str(pack_output),
            ]
            if PACKAGE_NATIVE_COMPONENTS.get(package):
                cmd.extend(["--vendor-src", str(args.vendor_src)])

            run_command(cmd)
            final_messages.append(f"Staged {package} at {pack_output}")
        finally:
            if not args.keep_staging_dirs:
                shutil.rmtree(staging_dir, ignore_errors=True)

    for message in final_messages:
        print(message)

    return 0


def run_command(cmd: list[str]) -> None:
    import subprocess
    import sys

    print("+", " ".join(cmd))
    # Run via python interpreter to avoid needing execute permissions on scripts
    full_cmd = [sys.executable, *cmd]
    subprocess.run(full_cmd, cwd=REPO_ROOT, check=True)


if __name__ == "__main__":
    raise SystemExit(main())
