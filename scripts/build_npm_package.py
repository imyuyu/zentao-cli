#!/usr/bin/env python3
"""Stage and optionally package the zentao-cli npm modules."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
ROOT_NPM_NAME = "@imyuyu/zentao-cli"
ROOT_PACKAGE_DIR = REPO_ROOT

PLATFORM_PACKAGES: dict[str, dict[str, str]] = {
    "zentao-cli-linux-x64": {
        "npm_name": "@imyuyu/zentao-cli-linux-x64",
        "npm_tag": "linux-x64",
        "target_triple": "x86_64-unknown-linux-gnu",
        "os": "linux",
        "cpu": "x64",
    },
    "zentao-cli-linux-arm64": {
        "npm_name": "@imyuyu/zentao-cli-linux-arm64",
        "npm_tag": "linux-arm64",
        "target_triple": "aarch64-unknown-linux-gnu",
        "os": "linux",
        "cpu": "arm64",
    },
    "zentao-cli-darwin-x64": {
        "npm_name": "@imyuyu/zentao-cli-darwin-x64",
        "npm_tag": "darwin-x64",
        "target_triple": "x86_64-apple-darwin",
        "os": "darwin",
        "cpu": "x64",
    },
    "zentao-cli-darwin-arm64": {
        "npm_name": "@imyuyu/zentao-cli-darwin-arm64",
        "npm_tag": "darwin-arm64",
        "target_triple": "aarch64-apple-darwin",
        "os": "darwin",
        "cpu": "arm64",
    },
    "zentao-cli-win32-x64": {
        "npm_name": "@imyuyu/zentao-cli-win32-x64",
        "npm_tag": "win32-x64",
        "target_triple": "x86_64-pc-windows-msvc",
        "os": "win32",
        "cpu": "x64",
        "binary_name": "zentao-cli.exe",
    },
    "zentao-cli-win32-arm64": {
        "npm_name": "@imyuyu/zentao-cli-win32-arm64",
        "npm_tag": "win32-arm64",
        "target_triple": "aarch64-pc-windows-msvc",
        "os": "win32",
        "cpu": "arm64",
        "binary_name": "zentao-cli.exe",
    },
}

PACKAGE_EXPANSIONS: dict[str, list[str]] = {
    "zentao-cli": ["zentao-cli", *PLATFORM_PACKAGES],
}

PACKAGE_NATIVE_COMPONENTS: dict[str, list[str]] = {
    "zentao-cli": [],
    **{package_name: ["zentao-cli"] for package_name in PLATFORM_PACKAGES},
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--package",
        choices=tuple(PACKAGE_NATIVE_COMPONENTS),
        default="zentao-cli",
        help="Which npm package to stage (default: zentao-cli).",
    )
    parser.add_argument(
        "--version",
        help="Version number to write to package.json inside the staged package.",
    )
    parser.add_argument(
        "--release-version",
        help="Version to stage for npm release.",
    )
    parser.add_argument(
        "--staging-dir",
        type=Path,
        help="Directory to stage the package contents. Defaults to a temporary directory.",
    )
    parser.add_argument(
        "--pack-output",
        type=Path,
        help="Path where the generated npm tarball should be written.",
    )
    parser.add_argument(
        "--vendor-src",
        type=Path,
        help="Directory containing pre-installed binaries to bundle (vendor root).",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    version = args.version or args.release_version
    if not version:
        raise RuntimeError("Must specify --version or --release-version.")

    staging_dir, created_temp = prepare_staging_dir(args.staging_dir)
    try:
        stage_sources(staging_dir, version, args.package)

        native_components = PACKAGE_NATIVE_COMPONENTS.get(args.package, [])
        if native_components:
            if args.vendor_src is None:
                raise RuntimeError(
                    f"Package '{args.package}' requires --vendor-src with prebuilt binaries."
                )
            copy_native_binaries(args.vendor_src, staging_dir, args.package)

        if args.pack_output is not None:
            output_path = run_npm_pack(staging_dir, args.pack_output)
            print(f"npm pack output written to {output_path}")
        else:
            print(f"Staged package in {staging_dir}")
    finally:
        if created_temp:
            pass

    return 0


def prepare_staging_dir(staging_dir: Path | None) -> tuple[Path, bool]:
    if staging_dir is not None:
        resolved = staging_dir.resolve()
        resolved.mkdir(parents=True, exist_ok=True)
        if any(resolved.iterdir()):
            raise RuntimeError(f"Staging directory {resolved} is not empty.")
        return resolved, False

    return Path(tempfile.mkdtemp(prefix="zentao-cli-npm-stage-")), True


def stage_sources(staging_dir: Path, version: str, package: str) -> None:
    with open(ROOT_PACKAGE_DIR / "package.json", "r", encoding="utf-8") as fh:
        root_package_json = json.load(fh)

    if package == "zentao-cli":
        bin_dir = staging_dir / "bin"
        bin_dir.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT_PACKAGE_DIR / "bin" / "zentao-cli.js", bin_dir / "zentao-cli.js")

        for doc_name in ("README.md", "README.en.md"):
            doc_path = REPO_ROOT / doc_name
            if doc_path.exists():
                shutil.copy2(doc_path, staging_dir / doc_name)

        skills_dir = REPO_ROOT / "skills"
        if skills_dir.exists():
            shutil.copytree(skills_dir, staging_dir / "skills")

        package_json = dict(root_package_json)
        package_json["version"] = version
        package_json["files"] = ["bin", "skills", "README.md", "README.en.md"]
        package_json["optionalDependencies"] = {
            platform_config["npm_name"]: (
                f"npm:{ROOT_NPM_NAME}@"
                f"{compute_platform_package_version(version, platform_config['npm_tag'])}"
            )
            for platform_config in PLATFORM_PACKAGES.values()
        }
    elif package in PLATFORM_PACKAGES:
        platform_package = PLATFORM_PACKAGES[package]
        platform_version = compute_platform_package_version(version, platform_package["npm_tag"])

        readme_src = REPO_ROOT / "README.md"
        if readme_src.exists():
            shutil.copy2(readme_src, staging_dir / "README.md")

        package_json = {
            "name": platform_package["npm_name"],
            "version": platform_version,
            "license": root_package_json.get("license", "MIT"),
            "os": [platform_package["os"]],
            "cpu": [platform_package["cpu"]],
            "files": ["vendor"],
            "repository": root_package_json.get("repository"),
            "engines": root_package_json.get("engines", {}),
        }
    else:
        raise RuntimeError(f"Unknown package '{package}'.")

    with open(staging_dir / "package.json", "w", encoding="utf-8") as out:
        json.dump(package_json, out, indent=2)
        out.write("\n")


def compute_platform_package_version(version: str, platform_tag: str) -> str:
    return f"{version}-{platform_tag}"


def copy_native_binaries(vendor_src: Path, staging_dir: Path, package: str) -> None:
    platform_package = PLATFORM_PACKAGES[package]
    target_triple = platform_package["target_triple"]
    binary_name = platform_package.get("binary_name", "zentao-cli")
    vendor_src = vendor_src.resolve()

    # Try the exact path: vendor_src/target_triple/zentao-cli/binary_name
    binary_src = vendor_src / target_triple / "zentao-cli" / binary_name
    if not binary_src.exists():
        # Fallback: maybe binary is directly in target_triple/zentao-cli/
        alt_path = vendor_src / target_triple / "zentao-cli"
        if alt_path.exists() and alt_path.is_dir():
            # Find any file in that directory
            files = list(alt_path.iterdir())
            if files:
                binary_src = files[0]
            else:
                raise RuntimeError(f"No files found in {alt_path}")
        else:
            raise RuntimeError(f"Vendor binary not found: {binary_src}")

    vendor_dir = staging_dir / "vendor"
    vendor_dir.mkdir(parents=True, exist_ok=True)
    final_binary_name = binary_name if binary_name else binary_src.name
    shutil.copy2(binary_src, vendor_dir / final_binary_name)
    print(f"Copied {binary_src} -> {vendor_dir / final_binary_name}")


def run_npm_pack(staging_dir: Path, output_path: Path) -> Path:
    output_path = output_path.resolve()
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix="zentao-cli-npm-pack-") as pack_dir_str:
        pack_dir = Path(pack_dir_str)
        npm_command = resolve_npm_command()
        stdout = subprocess.check_output(
            [npm_command, "pack", "--json", "--pack-destination", str(pack_dir)],
            cwd=staging_dir,
            text=True,
        )
        pack_output = json.loads(stdout)
        if not pack_output:
            raise RuntimeError("npm pack did not produce an output tarball.")

        tarball_name = pack_output[0].get("filename") or pack_output[0].get("name")
        if not tarball_name:
            raise RuntimeError("Unable to determine npm pack output filename.")

        tarball_path = pack_dir / tarball_name
        if not tarball_path.exists():
            raise RuntimeError(f"Expected npm pack output not found: {tarball_path}")

        shutil.move(str(tarball_path), output_path)

    return output_path


def resolve_npm_command() -> str:
    candidates = ["npm.cmd", "npm"] if os.name == "nt" else ["npm"]
    for candidate in candidates:
        resolved = shutil.which(candidate)
        if resolved:
            return resolved
    raise RuntimeError("Unable to locate npm in PATH.")


if __name__ == "__main__":
    raise SystemExit(main())
