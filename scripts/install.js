#!/usr/bin/env node
// ZenTao CLI Installer
// SPDX-License-Identifier: MIT

const fs = require("fs");
const path = require("path");
const { execFileSync } = require("child_process");
const os = require("os");
const crypto = require("crypto");

const REPO = "zentao-cli/cli";
const NAME = "zentao";

const ALLOWED_HOSTS = [
  "github.com",
  "objects.githubusercontent.com",
];

const PLATFORM_MAP = {
  darwin: "darwin",
  linux: "linux",
  win32: "windows",
};

const ARCH_MAP = {
  x64: "x86_64",
  arm64: "aarch64",
};

const platform = PLATFORM_MAP[process.platform];
const arch = ARCH_MAP[process.arch];
const isWindows = process.platform === "win32";

function getVersion() {
  const packageJson = path.join(__dirname, "..", "package.json");
  if (fs.existsSync(packageJson)) {
    const pkg = JSON.parse(fs.readFileSync(packageJson, "utf8"));
    return pkg.version;
  }
  // Fallback: fetch from GitHub API
  const { execFileSync: _exec } = require("child_process");
  const result = execFileSync("curl", [
    "-sL", `https://api.github.com/repos/${REPO}/releases/latest`
  ], { encoding: "utf8" });
  const data = JSON.parse(result);
  return data.tag_name.replace(/^v/, "");
}

const VERSION = process.env.ZENTAO_CLI_VERSION || getVersion();
const ext = isWindows ? ".exe" : "";
const archiveName = `${NAME}-${platform}-${arch}${isWindows ? "" : ".tar.gz"}`;
const GITHUB_URL = `https://github.com/${REPO}/releases/download/v${VERSION}/${archiveName}`;

const binDir = path.join(__dirname, "..", "bin");
const dest = path.join(binDir, NAME + (isWindows ? ".exe" : ""));

function assertAllowedHost(url) {
  const { hostname } = new URL(url);
  if (!ALLOWED_HOSTS.includes(hostname)) {
    throw new Error(`Download host not allowed: ${hostname}`);
  }
}

function download(url, destPath) {
  assertAllowedHost(url);
  const args = [
    "--fail", "--location", "--silent", "--show-error",
    "--connect-timeout", "10", "--max-time", "120",
    "--max-redirs", "3",
    "--output", destPath,
  ];
  if (isWindows) args.unshift("--ssl-revoke-best-effort");
  args.push(url);
  execFileSync("curl", args, { stdio: ["ignore", "ignore", "pipe"] });
}

function getExpectedChecksum(archiveName) {
  const checksumsPath = path.join(__dirname, "..", "checksums.txt");

  if (!fs.existsSync(checksumsPath)) {
    console.warn("[WARN] checksums.txt not found, skipping checksum verification");
    return null;
  }

  const content = fs.readFileSync(checksumsPath, "utf8");
  for (const line of content.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const idx = trimmed.indexOf("  ");
    if (idx === -1) continue;
    const hash = trimmed.slice(0, idx);
    const name = trimmed.slice(idx + 2);
    if (name === archiveName) return hash;
  }

  throw new Error(`Checksum entry not found for ${archiveName}`);
}

function verifyChecksum(archivePath, expectedHash) {
  if (expectedHash === null) return;

  const hash = crypto.createHash("sha256");
  const fd = fs.openSync(archivePath, "r");
  try {
    const buf = Buffer.alloc(64 * 1024);
    let bytesRead;
    while ((bytesRead = fs.readSync(fd, buf, 0, buf.length, null)) > 0) {
      hash.update(buf.subarray(0, bytesRead));
    }
  } finally {
    fs.closeSync(fd);
  }
  const actual = hash.digest("hex");

  if (actual.toLowerCase() !== expectedHash.toLowerCase()) {
    throw new Error(`Checksum mismatch for ${archivePath}`);
  }
}

function install() {
  fs.mkdirSync(binDir, { recursive: true });

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "zentao-cli-"));
  const archivePath = path.join(tmpDir, archiveName);

  try {
    console.log(`Downloading ${NAME} v${VERSION} for ${platform}-${arch}...`);
    download(GITHUB_URL, archivePath);

    const expectedHash = getExpectedChecksum(archiveName);
    verifyChecksum(archivePath, expectedHash);

    if (isWindows) {
      // For Windows, assume the release is a zip
      execFileSync("powershell", [
        "-Command",
        `Expand-Archive -Path '${archivePath}' -DestinationPath '${tmpDir}'`,
      ], { stdio: "ignore" });
    } else {
      execFileSync("tar", ["-xzf", archivePath, "-C", tmpDir], {
        stdio: "ignore",
      });
    }

    const binaryName = NAME + (isWindows ? ".exe" : "");
    const extractedBinary = path.join(tmpDir, binaryName);

    if (!fs.existsSync(extractedBinary)) {
      // Try alternative location
      const altPath = path.join(tmpDir, NAME, binaryName);
      if (fs.existsSync(altPath)) {
        fs.copyFileSync(altPath, dest);
      } else {
        throw new Error(`Binary not found in archive: ${binaryName}`);
      }
    } else {
      fs.copyFileSync(extractedBinary, dest);
    }
    fs.chmodSync(dest, 0o755);
    console.log(`${NAME} v${VERSION} installed successfully to ${dest}`);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

// Run install
try {
  install();
} catch (err) {
  console.error(`Installation failed: ${err.message}`);
  process.exit(1);
}
