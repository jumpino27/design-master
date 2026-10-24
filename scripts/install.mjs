#!/usr/bin/env node
/**
 * Cross-platform installer & background daemon manager for Design Gen.
 * Supports macOS (LaunchAgent + ~/Applications/Design Gen.app with .icns),
 * Windows (VBScript background daemon + Start Menu/Desktop/Startup .lnk shortcuts with .ico),
 * and Linux (systemd --user service + ~/.local/share/applications/design-gen.desktop with .png/.svg).
 */
import { spawn, spawnSync, execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, "..");
const platform = process.platform;
const isWin = platform === "win32";
const isMac = platform === "darwin";
const isLinux = platform === "linux";

const pnpmCmd = isWin ? "pnpm.cmd" : "pnpm";
const cargoCmd = isWin ? "cargo.exe" : "cargo";
const HOST = process.env.DESIGN_GEN_HOST || "127.0.0.1";
const PORT = process.env.DESIGN_GEN_PORT || "4318";
const APP_URL = `http://${HOST}:${PORT}`;

function getBaseDataDir() {
  if (process.env.DESIGN_GEN_DATA_DIR) {
    return path.resolve(process.env.DESIGN_GEN_DATA_DIR);
  }
  return path.join(rootDir, "data");
}

function getPaths() {
  const dataDir = getBaseDataDir();
  const appRuntimeDir = path.join(dataDir, "app");
  const binDir = path.join(appRuntimeDir, "bin");
  const staticDir = path.join(appRuntimeDir, "static");
  const promptsDir = path.join(appRuntimeDir, "prompts");
  const iconsDir = path.join(appRuntimeDir, "icons");
  const logsDir = isMac
    ? path.join(os.homedir(), "Library", "Logs", "DesignGen")
    : path.join(dataDir, "logs");
  const pidFile = path.join(dataDir, "design-gen-server.pid");
  const binName = isWin ? "design-gen-server.exe" : "design-gen-server";
  const installedBin = path.join(binDir, binName);

  return {
    dataDir,
    appRuntimeDir,
    binDir,
    staticDir,
    promptsDir,
    iconsDir,
    logsDir,
    pidFile,
    binName,
    installedBin,
  };
}

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
      else
        reject(
          new Error(`${cmd} ${args.join(" ")} exited with code ${code}`)
        );
    });
  });
}

async function isServerHealthy(timeoutMs = 1200) {
  try {
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), timeoutMs);
    const res = await fetch(`${APP_URL}/api/health`, {
      signal: controller.signal,
    });
    clearTimeout(timer);
    return res.ok;
  } catch {
    return false;
  }
}

async function waitForServerReady(maxWaitMs = 15000) {
  const start = Date.now();
  while (Date.now() - start < maxWaitMs) {
    if (await isServerHealthy(1000)) {
      return true;
    }
    await new Promise((r) => setTimeout(r, 350));
  }
  return false;
}

function openBrowser(url) {
  try {
    if (isMac) {
      spawn("open", [url], { detached: true, stdio: "ignore" }).unref();
    } else if (isWin) {
      spawn("cmd.exe", ["/c", "start", "", url], {
        detached: true,
        stdio: "ignore",
      }).unref();
    } else {
      spawn("xdg-open", [url], { detached: true, stdio: "ignore" }).unref();
    }
  } catch {
    console.log(`==> Open ${url} in your browser.`);
  }
}

function stopDetachedProcess(paths) {
  // 1. Stop by PID file if present
  if (fs.existsSync(paths.pidFile)) {
    try {
      const pid = parseInt(fs.readFileSync(paths.pidFile, "utf8").trim(), 10);
      if (pid > 0) {
        process.kill(pid, "SIGTERM");
      }
    } catch {
      // process already stopped
    }
    try {
      fs.unlinkSync(paths.pidFile);
    } catch {
      // ignore
    }
  }

  // 2. Also stop any running design-gen-server process by name
  try {
    if (isWin) {
      spawnSync("taskkill", ["/F", "/IM", "design-gen-server.exe"], {
        stdio: "ignore",
      });
    } else {
      spawnSync("pkill", ["-f", "design-gen-server"], { stdio: "ignore" });
    }
  } catch {
    // ignore
  }
}

function spawnDetachedDaemon(paths) {
  fs.mkdirSync(paths.logsDir, { recursive: true });
  const outLog = fs.openSync(path.join(paths.logsDir, "server.log"), "a");
  const errLog = fs.openSync(path.join(paths.logsDir, "server.log"), "a");

  const binToRun = fs.existsSync(paths.installedBin)
    ? paths.installedBin
    : path.join(rootDir, "target", "release", paths.binName);

  const staticToUse = fs.existsSync(path.join(paths.staticDir, "index.html"))
    ? paths.staticDir
    : path.join(rootDir, "web", "out");

  const promptsToUse = fs.existsSync(paths.promptsDir)
    ? paths.promptsDir
    : path.join(rootDir, "prompts", "v1");

  const child = spawn(binToRun, [], {
    cwd: paths.dataDir,
    detached: true,
    stdio: ["ignore", outLog, errLog],
    env: {
      ...process.env,
      DESIGN_GEN_HOST: HOST,
      DESIGN_GEN_PORT: PORT,
      DESIGN_GEN_DATA_DIR: paths.dataDir,
      DESIGN_GEN_STATIC_DIR: staticToUse,
      DESIGN_GEN_PROMPTS_DIR: promptsToUse,
    },
  });

  if (child.pid) {
    fs.mkdirSync(path.dirname(paths.pidFile), { recursive: true });
    fs.writeFileSync(paths.pidFile, String(child.pid), "utf8");
  }
  child.unref();
}

function setupMacOS(paths) {
  const home = os.homedir();
  const launchAgentsDir = path.join(home, "Library", "LaunchAgents");
  const plistPath = path.join(
    launchAgentsDir,
    "com.designgen.workbench.plist"
  );
  fs.mkdirSync(launchAgentsDir, { recursive: true });
  fs.mkdirSync(paths.logsDir, { recursive: true });

  const logFile = path.join(paths.logsDir, "server.log");
  const plistXml = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.designgen.workbench</string>
  <key>ProgramArguments</key>
  <array>
    <string>${paths.installedBin}</string>
  </array>
  <key>WorkingDirectory</key>
  <string>${paths.dataDir}</string>
  <key>EnvironmentVariables</key>
  <dict>
    <key>DESIGN_GEN_HOST</key>
    <string>${HOST}</string>
    <key>DESIGN_GEN_PORT</key>
    <string>${PORT}</string>
    <key>DESIGN_GEN_DATA_DIR</key>
    <string>${paths.dataDir}</string>
    <key>DESIGN_GEN_STATIC_DIR</key>
    <string>${paths.staticDir}</string>
    <key>DESIGN_GEN_PROMPTS_DIR</key>
    <string>${paths.promptsDir}</string>
  </dict>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
  <key>StandardOutPath</key>
  <string>${logFile}</string>
  <key>StandardErrorPath</key>
  <string>${logFile}</string>
</dict>
</plist>
`;
  fs.writeFileSync(plistPath, plistXml, "utf8");

  // Create native macOS ~/Applications/Design Gen.app bundle with custom .icns icon
  const userAppsDir = path.join(home, "Applications");
  const appBundleDir = path.join(userAppsDir, "Design Gen.app");
  const contentsDir = path.join(appBundleDir, "Contents");
  const macosDir = path.join(contentsDir, "MacOS");
  const resourcesDir = path.join(contentsDir, "Resources");

  fs.mkdirSync(macosDir, { recursive: true });
  fs.mkdirSync(resourcesDir, { recursive: true });

  const icnsSrc = path.join(paths.iconsDir, "design-gen.icns");
  if (fs.existsSync(icnsSrc)) {
    fs.copyFileSync(icnsSrc, path.join(resourcesDir, "AppIcon.icns"));
  }

  const infoPlist = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>design-gen-launcher</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon.icns</string>
  <key>CFBundleIdentifier</key>
  <string>com.designgen.workbench.app</string>
  <key>CFBundleName</key>
  <string>Design Gen</string>
  <key>CFBundleDisplayName</key>
  <string>Design Gen</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>
  <key>LSUIElement</key>
  <true/>
</dict>
</plist>
`;
  fs.writeFileSync(path.join(contentsDir, "Info.plist"), infoPlist, "utf8");

  const launcherScript = `#!/usr/bin/env bash
set -e
if ! curl -fsS "${APP_URL}/api/health" >/dev/null 2>&1; then
  launchctl load -w "${plistPath}" >/dev/null 2>&1 || true
  if ! curl -fsS "${APP_URL}/api/health" >/dev/null 2>&1; then
    DESIGN_GEN_HOST="${HOST}" \\
    DESIGN_GEN_PORT="${PORT}" \\
    DESIGN_GEN_DATA_DIR="${paths.dataDir}" \\
    DESIGN_GEN_STATIC_DIR="${paths.staticDir}" \\
    DESIGN_GEN_PROMPTS_DIR="${paths.promptsDir}" \\
    nohup "${paths.installedBin}" >> "${logFile}" 2>&1 &
    echo $! > "${paths.pidFile}"
  fi
  for i in 1 2 3 4 5 6 7 8 9 10; do
    if curl -fsS "${APP_URL}/api/health" >/dev/null 2>&1; then
      break
    fi
    sleep 0.3
  done
fi
open "${APP_URL}"
`;
  const launcherBin = path.join(macosDir, "design-gen-launcher");
  fs.writeFileSync(launcherBin, launcherScript, { mode: 0o755 });
  fs.chmodSync(launcherBin, 0o755);

  try {
    execFileSync("touch", [appBundleDir]);
  } catch {
    // ignore
  }

  // Try loading LaunchAgent
  spawnSync("launchctl", ["unload", plistPath], { stdio: "ignore" });
  spawnSync("launchctl", ["load", "-w", plistPath], { stdio: "ignore" });

  console.log(`==> [macOS] Installed LaunchAgent: ${plistPath}`);
  console.log(`==> [macOS] Installed Desktop App Bundle: ${appBundleDir}`);
}

function setupWindows(paths) {
  const icoPath = path.join(paths.iconsDir, "design-gen.ico");
  const vbsLauncher = path.join(paths.appRuntimeDir, "launch-design-gen.vbs");
  const vbsBgOnly = path.join(
    paths.appRuntimeDir,
    "start-background-server.vbs"
  );
  const cmdStart = path.join(paths.appRuntimeDir, "start-server.cmd");

  const cmdContent = `@echo off
set DESIGN_GEN_HOST=${HOST}
set DESIGN_GEN_PORT=${PORT}
set DESIGN_GEN_DATA_DIR=${paths.dataDir}
set DESIGN_GEN_STATIC_DIR=${paths.staticDir}
set DESIGN_GEN_PROMPTS_DIR=${paths.promptsDir}
"${paths.installedBin}" >> "${path.join(paths.logsDir, "server.log")}" 2>&1
`;
  fs.mkdirSync(paths.logsDir, { recursive: true });
  fs.writeFileSync(cmdStart, cmdContent, "utf8");

  const vbsBgContent = `Set WshShell = CreateObject("WScript.Shell")
WshShell.Run """${cmdStart}""", 0, False
`;
  fs.writeFileSync(vbsBgOnly, vbsBgContent, "utf8");

  const vbsLauncherContent = `Set WshShell = CreateObject("WScript.Shell")
On Error Resume Next
Set http = CreateObject("MSXML2.XMLHTTP")
http.Open "GET", "${APP_URL}/api/health", False
http.Send
If Err.Number <> 0 Or http.Status <> 200 Then
  WshShell.Run """${cmdStart}""", 0, False
  WScript.Sleep 1200
End If
WshShell.Run "${APP_URL}"
`;
  fs.writeFileSync(vbsLauncher, vbsLauncherContent, "utf8");

  // Create Desktop, Start Menu, and Startup folder shortcuts with custom .ico via PowerShell
  const psScript = `
$WshShell = New-Object -ComObject WScript.Shell
$DesktopDir = [Environment]::GetFolderPath("Desktop")
$StartMenuDir = [Environment]::GetFolderPath("Programs")
$StartupDir = [Environment]::GetFolderPath("Startup")

foreach ($targetDir in @($DesktopDir, $StartMenuDir)) {
  $ShortcutPath = Join-Path $targetDir "Design Gen.lnk"
  $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
  $Shortcut.TargetPath = "wscript.exe"
  $Shortcut.Arguments = """${vbsLauncher}"""
  $Shortcut.WorkingDirectory = "${paths.appRuntimeDir}"
  $Shortcut.IconLocation = "${icoPath},0"
  $Shortcut.Description = "Launch Design Gen Workbench (${APP_URL})"
  $Shortcut.Save()
}

$BgShortcutPath = Join-Path $StartupDir "Design Gen Background Server.lnk"
$BgShortcut = $WshShell.CreateShortcut($BgShortcutPath)
$BgShortcut.TargetPath = "wscript.exe"
$BgShortcut.Arguments = """${vbsBgOnly}"""
$BgShortcut.WorkingDirectory = "${paths.appRuntimeDir}"
$BgShortcut.IconLocation = "${icoPath},0"
$BgShortcut.Description = "Design Gen Local Background Server"
$BgShortcut.Save()
`;

  spawnSync(
    "powershell.exe",
    ["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", psScript],
    { stdio: "inherit" }
  );
  console.log(
    "==> [Windows] Created Desktop, Start Menu, and Startup shortcuts with Design Gen icon."
  );
}

function setupLinux(paths) {
  const home = os.homedir();
  const systemdDir = path.join(home, ".config", "systemd", "user");
  const serviceFile = path.join(systemdDir, "design-gen.service");
  const appsDir = path.join(home, ".local", "share", "applications");
  const icon512Dir = path.join(
    home,
    ".local",
    "share",
    "icons",
    "hicolor",
    "512x512",
    "apps"
  );
  const iconSvgDir = path.join(
    home,
    ".local",
    "share",
    "icons",
    "hicolor",
    "scalable",
    "apps"
  );

  fs.mkdirSync(systemdDir, { recursive: true });
  fs.mkdirSync(appsDir, { recursive: true });
  fs.mkdirSync(icon512Dir, { recursive: true });
  fs.mkdirSync(iconSvgDir, { recursive: true });
  fs.mkdirSync(paths.logsDir, { recursive: true });

  const pngSrc = path.join(paths.iconsDir, "design-gen.png");
  const svgSrc = path.join(paths.iconsDir, "design-gen.svg");
  if (fs.existsSync(pngSrc)) {
    fs.copyFileSync(pngSrc, path.join(icon512Dir, "design-gen.png"));
  }
  if (fs.existsSync(svgSrc)) {
    fs.copyFileSync(svgSrc, path.join(iconSvgDir, "design-gen.svg"));
  }

  const serviceContent = `[Unit]
Description=Design Gen Local Workbench Background Server
After=network.target

[Service]
Type=simple
WorkingDirectory=${paths.dataDir}
Environment="DESIGN_GEN_HOST=${HOST}"
Environment="DESIGN_GEN_PORT=${PORT}"
Environment="DESIGN_GEN_DATA_DIR=${paths.dataDir}"
Environment="DESIGN_GEN_STATIC_DIR=${paths.staticDir}"
Environment="DESIGN_GEN_PROMPTS_DIR=${paths.promptsDir}"
ExecStart=${paths.installedBin}
Restart=on-failure
RestartSec=2

[Install]
WantedBy=default.target
`;
  fs.writeFileSync(serviceFile, serviceContent, "utf8");

  const launcherSh = path.join(paths.appRuntimeDir, "launch-design-gen.sh");
  const logFile = path.join(paths.logsDir, "server.log");
  const launcherContent = `#!/usr/bin/env bash
set -e
if ! curl -fsS "${APP_URL}/api/health" >/dev/null 2>&1; then
  systemctl --user start design-gen.service >/dev/null 2>&1 || true
  if ! curl -fsS "${APP_URL}/api/health" >/dev/null 2>&1; then
    DESIGN_GEN_HOST="${HOST}" \\
    DESIGN_GEN_PORT="${PORT}" \\
    DESIGN_GEN_DATA_DIR="${paths.dataDir}" \\
    DESIGN_GEN_STATIC_DIR="${paths.staticDir}" \\
    DESIGN_GEN_PROMPTS_DIR="${paths.promptsDir}" \\
    nohup "${paths.installedBin}" >> "${logFile}" 2>&1 &
    echo $! > "${paths.pidFile}"
  fi
  for i in 1 2 3 4 5 6 7 8 9 10; do
    if curl -fsS "${APP_URL}/api/health" >/dev/null 2>&1; then
      break
    fi
    sleep 0.3
  done
fi
xdg-open "${APP_URL}"
`;
  fs.writeFileSync(launcherSh, launcherContent, { mode: 0o755 });
  fs.chmodSync(launcherSh, 0o755);

  const desktopEntry = path.join(appsDir, "design-gen.desktop");
  const desktopContent = `[Desktop Entry]
Version=1.0
Type=Application
Name=Design Gen
Comment=Local UI, Full-Page & Brand Asset Workbench
Exec=${launcherSh}
Icon=${ path.join(icon512Dir, "design-gen.png") }
Terminal=false
Categories=Development;Graphics;Design;
StartupNotify=true
`;
  fs.writeFileSync(desktopEntry, desktopContent, { mode: 0o755 });

  spawnSync("systemctl", ["--user", "daemon-reload"], { stdio: "ignore" });
  spawnSync("systemctl", ["--user", "enable", "--now", "design-gen.service"], {
    stdio: "ignore",
  });

  console.log(`==> [Linux] Installed systemd user service: ${serviceFile}`);
  console.log(`==> [Linux] Installed desktop entry & icon: ${desktopEntry}`);
}

async function handleInstall(noOpen) {
  const paths = getPaths();
  console.log("==============================================================");
  console.log("  Design Gen — Cross-Platform Installer & Background Launcher");
  console.log("==============================================================");

  // 1. Generate favicons & OS icons
  console.log("==> [1/5] Generating favicons, PWA manifest, and OS icons...");
  await runStep(process.execPath, [
    path.join(rootDir, "scripts", "generate-icons.mjs"),
  ]);

  // 2. Build Next.js static export
  console.log("==> [2/5] Building Next.js production static export (web/out)...");
  await runStep(pnpmCmd, ["--dir", "web", "build"]);

  // 3. Build Rust server release binary
  console.log("==> [3/5] Compiling release Rust server binary...");
  await runStep(cargoCmd, [
    "build",
    "--release",
    "--bin",
    "design-gen-server",
  ]);

  // 4. Stop any older background daemon before replacing binary
  stopDetachedProcess(paths);

  // 5. Copy binary, static export, prompts, and icons to runtime dir
  console.log(
    `==> [4/5] Installing runtime assets to ${paths.appRuntimeDir}...`
  );
  fs.mkdirSync(paths.binDir, { recursive: true });
  fs.rmSync(paths.staticDir, { recursive: true, force: true });
  fs.rmSync(paths.promptsDir, { recursive: true, force: true });
  fs.rmSync(paths.iconsDir, { recursive: true, force: true });

  const builtBin = path.join(rootDir, "target", "release", paths.binName);
  fs.copyFileSync(builtBin, paths.installedBin);
  if (!isWin) {
    fs.chmodSync(paths.installedBin, 0o755);
  }

  fs.cpSync(path.join(rootDir, "web", "out"), paths.staticDir, {
    recursive: true,
  });
  fs.cpSync(path.join(rootDir, "prompts", "v1"), paths.promptsDir, {
    recursive: true,
  });
  fs.cpSync(path.join(rootDir, "assets", "icons"), paths.iconsDir, {
    recursive: true,
  });

  // 6. Configure OS-specific desktop shortcut & background service
  console.log("==> [5/5] Configuring OS desktop shortcut & background daemon...");
  if (isMac) {
    setupMacOS(paths);
  } else if (isWin) {
    setupWindows(paths);
  } else if (isLinux) {
    setupLinux(paths);
  }

  // Ensure background daemon is running right now (wait up to 3.5s for launchd/systemd first)
  if (!(await waitForServerReady(3500))) {
    spawnDetachedDaemon(paths);
  }

  const ready = await waitForServerReady(15000);
  if (!ready) {
    throw new Error(
      `Background server did not respond at ${APP_URL}/api/health within 15s. Check logs in ${paths.logsDir}`
    );
  }

  console.log("");
  console.log("==============================================================");
  console.log(`  SUCCESS! Design Gen is running in the background:`);
  console.log(`  URL:       ${APP_URL}`);
  console.log(`  Data Dir:  ${paths.dataDir}`);
  console.log(`  Logs:      ${path.join(paths.logsDir, "server.log")}`);
  console.log("==============================================================");

  if (!noOpen) {
    console.log(`==> Opening ${APP_URL} in your browser...`);
    openBrowser(APP_URL);
  }
}

async function handleStartBg(noOpen) {
  const paths = getPaths();
  if (await isServerHealthy(1000)) {
    console.log(`==> Design Gen is already running in the background at ${APP_URL}`);
    if (!noOpen) openBrowser(APP_URL);
    return;
  }

  console.log(`==> Starting Design Gen in the background on ${APP_URL}...`);
  spawnDetachedDaemon(paths);
  const ok = await waitForServerReady(12000);
  if (!ok) {
    console.error(
      `==> Failed to verify server at ${APP_URL}. Run 'pnpm install:app' first if not yet built.`
    );
    process.exit(1);
  }
  console.log(`==> Design Gen is live in the background at ${APP_URL}`);
  if (!noOpen) openBrowser(APP_URL);
}

async function handleStopBg() {
  const paths = getPaths();
  if (isMac) {
    const plistPath = path.join(
      os.homedir(),
      "Library",
      "LaunchAgents",
      "com.designgen.workbench.plist"
    );
    if (fs.existsSync(plistPath)) {
      spawnSync("launchctl", ["unload", plistPath], { stdio: "ignore" });
    }
  } else if (isLinux) {
    spawnSync("systemctl", ["--user", "stop", "design-gen.service"], {
      stdio: "ignore",
    });
  }
  stopDetachedProcess(paths);
  console.log("==> Stopped Design Gen background server.");
}

async function handleStatusBg() {
  const paths = getPaths();
  const healthy = await isServerHealthy(1500);
  if (healthy) {
    console.log(`==> ONLINE: Design Gen background server is running at ${APP_URL}`);
    console.log(`==> Logs:   ${path.join(paths.logsDir, "server.log")}`);
  } else {
    console.log(`==> OFFLINE: Design Gen background server is not running on ${APP_URL}`);
    process.exit(1);
  }
}

async function handleUninstall() {
  const paths = getPaths();
  await handleStopBg();
  const home = os.homedir();

  if (isMac) {
    const plistPath = path.join(
      home,
      "Library",
      "LaunchAgents",
      "com.designgen.workbench.plist"
    );
    const appBundleDir = path.join(home, "Applications", "Design Gen.app");
    fs.rmSync(plistPath, { force: true });
    fs.rmSync(appBundleDir, { recursive: true, force: true });
  } else if (isLinux) {
    const serviceFile = path.join(
      home,
      ".config",
      "systemd",
      "user",
      "design-gen.service"
    );
    const desktopEntry = path.join(
      home,
      ".local",
      "share",
      "applications",
      "design-gen.desktop"
    );
    spawnSync("systemctl", ["--user", "disable", "design-gen.service"], {
      stdio: "ignore",
    });
    fs.rmSync(serviceFile, { force: true });
    fs.rmSync(desktopEntry, { force: true });
  } else if (isWin) {
    const psRemove = `
$DesktopDir = [Environment]::GetFolderPath("Desktop")
$StartMenuDir = [Environment]::GetFolderPath("Programs")
$StartupDir = [Environment]::GetFolderPath("Startup")
Remove-Item -Path (Join-Path $DesktopDir "Design Gen.lnk") -ErrorAction SilentlyContinue
Remove-Item -Path (Join-Path $StartMenuDir "Design Gen.lnk") -ErrorAction SilentlyContinue
Remove-Item -Path (Join-Path $StartupDir "Design Gen Background Server.lnk") -ErrorAction SilentlyContinue
`;
    spawnSync(
      "powershell.exe",
      ["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", psRemove],
      { stdio: "ignore" }
    );
  }

  fs.rmSync(paths.appRuntimeDir, { recursive: true, force: true });
  console.log(
    `==> Uninstalled Design Gen background service & shortcuts (preserved user projects in ${paths.dataDir}).`
  );
}

async function main() {
  const cmd = process.argv[2] || "install";
  const noOpen = process.argv.includes("--no-open");

  if (cmd === "install") {
    await handleInstall(noOpen);
  } else if (cmd === "start") {
    await handleStartBg(noOpen);
  } else if (cmd === "stop") {
    await handleStopBg();
  } else if (cmd === "status") {
    await handleStatusBg();
  } else if (cmd === "uninstall") {
    await handleUninstall();
  } else {
    console.error(
      `Unknown command: ${cmd}. Expected install, start, stop, status, or uninstall.`
    );
    process.exit(1);
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
