#!/usr/bin/env node
import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, "..");
const isWin = process.platform === "win32";
const pnpmCmd = isWin ? "pnpm.cmd" : "pnpm";
const cargoCmd = isWin ? "cargo.exe" : "cargo";

function runStep(cmd, args, cwd = rootDir) {
  return new Promise((resolve, reject) => {
    const child = spawn(cmd, args, {
      cwd,
      stdio: "inherit",
      shell: isWin,
      env: process.env,
    });
    child.on("error", reject);
    child.on("exit", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`${cmd} ${args.join(" ")} exited with code ${code}`));
    });
  });
}

async function waitForUrl(url, timeoutMs = 20000) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const r = await fetch(url);
      if (r.ok || r.status < 500) return true;
    } catch {
      // retry
    }
    await new Promise((r) => setTimeout(r, 350));
  }
  return false;
}

async function main() {
  const mode = process.argv[2] || "dev";
  const webOnly = process.argv.includes("--web");

  if (mode === "build") {
    console.log("==> Generating Tauri & OS icons...");
    await runStep("node", ["scripts/generate-icons.mjs"]);
    console.log("==> Building Next.js static export in web/out...");
    await runStep(pnpmCmd, ["--dir", "web", "build"]);
    console.log("==> Building Tauri Desktop Application & OS Bundles...");
    const bundles = { darwin: "app", linux: "deb,appimage", win32: "nsis" }[process.platform];
    if (!bundles) throw new Error(`Unsupported build host: ${process.platform}`);
    await runStep(pnpmCmd, ["tauri", "build", "--bundles", bundles]);
    console.log("==> Packaging native installers for this build host...");
    await runStep("node", ["scripts/build-installers.mjs"]);
    console.log("==> Build complete! Run `pnpm start` to launch the Design Master Tauri desktop app.");
    return;
  }

  if (mode === "start") {
    const tauriBinName = isWin ? "design-master.exe" : "design-master";
    const releaseTauriBin = path.join(rootDir, "target", "release", tauriBinName);
    const debugTauriBin = path.join(rootDir, "target", "debug", tauriBinName);
    const staticDir = path.join(rootDir, "web", "out");

    const env = {
      ...process.env,
      DESIGN_GEN_STATIC_DIR: process.env.DESIGN_GEN_STATIC_DIR || staticDir,
    };

    if (!webOnly && (fs.existsSync(releaseTauriBin) || fs.existsSync(debugTauriBin))) {
      const bin = fs.existsSync(releaseTauriBin) ? releaseTauriBin : debugTauriBin;
      console.log("==> Launching Design Master (DM) native Tauri desktop app...");
      const appProc = spawn(bin, [], {
        cwd: rootDir,
        stdio: "inherit",
        shell: false,
        env,
      });
      process.on("SIGINT", () => appProc.kill("SIGINT"));
      process.on("SIGTERM", () => appProc.kill("SIGTERM"));
      return;
    }

    if (!webOnly) {
      console.log("==> Starting Design Master (DM) Tauri desktop app...");
      const appProc = spawn(cargoCmd, ["run", "--release", "--bin", "design-master"], {
        cwd: rootDir,
        stdio: "inherit",
        shell: isWin,
        env,
      });
      process.on("SIGINT", () => appProc.kill("SIGINT"));
      process.on("SIGTERM", () => appProc.kill("SIGTERM"));
      return;
    }

    const binName = isWin ? "design-gen-server.exe" : "design-gen-server";
    const releaseBin = path.join(rootDir, "target", "release", binName);
    const cmd = fs.existsSync(releaseBin) ? releaseBin : cargoCmd;
    const args = fs.existsSync(releaseBin)
      ? []
      : ["run", "--release", "--bin", "design-gen-server"];
    const server = spawn(cmd, args, {
      cwd: rootDir,
      stdio: "inherit",
      shell: false,
      env,
    });
    process.on("SIGINT", () => server.kill("SIGINT"));
    process.on("SIGTERM", () => server.kill("SIGTERM"));
    return;
  }

  if (mode === "dev") {
    console.log("==> Starting Design Master (DM) Next.js dev server + Native Tauri Desktop App (with embedded Rust backend)...");
    const webProc = spawn(pnpmCmd, ["--dir", "web", "dev"], {
      cwd: rootDir,
      stdio: "inherit",
      shell: isWin,
      env: process.env,
    });

    await waitForUrl("http://localhost:3000", 15000);

    const tauriBinName = isWin ? "design-master.exe" : "design-master";
    const releaseTauriBin = path.join(rootDir, "target", "release", tauriBinName);
    const tauriCmd = fs.existsSync(releaseTauriBin) ? releaseTauriBin : cargoCmd;
    const tauriArgs = fs.existsSync(releaseTauriBin)
      ? []
      : ["run", "--bin", "design-master"];

    const tauriProc = spawn(tauriCmd, tauriArgs, {
      cwd: rootDir,
      stdio: "inherit",
      shell: isWin && tauriCmd === cargoCmd,
      env: {
        ...process.env,
        DESIGN_MASTER_USE_DEV_SERVER: "1",
      },
    });

    const shutdown = () => {
      tauriProc.kill("SIGINT");
      webProc.kill("SIGINT");
      process.exit(0);
    };

    tauriProc.on("exit", () => {
      webProc.kill("SIGINT");
      process.exit(0);
    });

    process.on("SIGINT", shutdown);
    process.on("SIGTERM", shutdown);
    return;
  }

  console.error(`Unknown mode: ${mode}. Expected dev, build, or start.`);
  process.exit(1);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
