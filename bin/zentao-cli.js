#!/usr/bin/env node

const { spawn } = require("child_process");
const { existsSync } = require("fs");
const path = require("path");

const PLATFORM_PACKAGE_BY_TARGET = {
  "x86_64-unknown-linux-gnu": "@imyuyu/zentao-cli-linux-x64",
  "aarch64-unknown-linux-gnu": "@imyuyu/zentao-cli-linux-arm64",
  "x86_64-apple-darwin": "@imyuyu/zentao-cli-darwin-x64",
  "aarch64-apple-darwin": "@imyuyu/zentao-cli-darwin-arm64",
  "x86_64-pc-windows-msvc": "@imyuyu/zentao-cli-win32-x64",
  "aarch64-pc-windows-msvc": "@imyuyu/zentao-cli-win32-arm64",
};

function detectTargetTriple() {
  switch (process.platform) {
    case "linux":
      switch (process.arch) {
        case "x64":
          return "x86_64-unknown-linux-gnu";
        case "arm64":
          return "aarch64-unknown-linux-gnu";
        default:
          return null;
      }
    case "darwin":
      switch (process.arch) {
        case "x64":
          return "x86_64-apple-darwin";
        case "arm64":
          return "aarch64-apple-darwin";
        default:
          return null;
      }
    case "win32":
      switch (process.arch) {
        case "x64":
          return "x86_64-pc-windows-msvc";
        case "arm64":
          return "aarch64-pc-windows-msvc";
        default:
          return null;
      }
    default:
      return null;
  }
}

function detectPackageManager() {
  const userAgent = process.env.npm_config_user_agent || "";
  if (/\bbun\//.test(userAgent)) {
    return "bun";
  }

  const execPath = process.env.npm_execpath || "";
  if (execPath.includes("bun")) {
    return "bun";
  }

  return userAgent ? "npm" : null;
}

const targetTriple = detectTargetTriple();
if (!targetTriple) {
  throw new Error(`Unsupported platform: ${process.platform} (${process.arch})`);
}

const platformPackage = PLATFORM_PACKAGE_BY_TARGET[targetTriple];
if (!platformPackage) {
  throw new Error(`Unsupported target triple: ${targetTriple}`);
}

const binaryName = process.platform === "win32" ? "zentao-cli.exe" : "zentao-cli";
const localVendorRoot = path.join(__dirname, "..", "vendor");
const localBinaryPath = path.join(localVendorRoot, targetTriple, "zentao-cli", binaryName);

let vendorRoot;
try {
  const packageJsonPath = require.resolve(`${platformPackage}/package.json`);
  vendorRoot = path.join(path.dirname(packageJsonPath), "vendor");
} catch (error) {
  if (existsSync(localBinaryPath)) {
    vendorRoot = localVendorRoot;
  } else {
    const packageManager = detectPackageManager();
    const reinstallCommand =
      packageManager === "bun"
        ? "bun install -g @imyuyu/zentao-cli@latest"
        : "npm install -g @imyuyu/zentao-cli@latest";
    throw new Error(
      `Missing optional dependency ${platformPackage}. Reinstall zentao-cli: ${reinstallCommand}`,
    );
  }
}

const binaryPath = path.join(vendorRoot, targetTriple, "zentao-cli", binaryName);
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  env: process.env,
});

child.on("error", (error) => {
  console.error(error.message);
  process.exit(1);
});

const forwardSignal = (signal) => {
  if (!child.killed) {
    try {
      child.kill(signal);
    } catch {
      // Ignore signal forwarding failures during shutdown.
    }
  }
};

["SIGINT", "SIGTERM", "SIGHUP"].forEach((signal) => {
  process.on(signal, () => forwardSignal(signal));
});

child.on("exit", (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 1);
});
