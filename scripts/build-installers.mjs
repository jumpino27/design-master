#!/usr/bin/env node
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const { version } = JSON.parse(fs.readFileSync(path.join(rootDir, "package.json"), "utf8"));
const releaseDir = path.join(rootDir, "target", "release");
const bundleDir = path.join(releaseDir, "bundle");
const platform = { darwin: "macos", win32: "windows", linux: "linux" }[process.platform];
if (!platform) throw new Error(`Unsupported build host: ${process.platform}`);
if (process.platform !== "darwin" && process.arch !== "x64") {
  throw new Error("Windows and Linux release packages require an x64 build host.");
}
const distDir = path.join(rootDir, "dist", "installers", platform);
fs.mkdirSync(distDir, { recursive: true });

function requireFile(file) {
  if (!fs.statSync(file).isFile() || fs.statSync(file).size === 0) {
    throw new Error(`Missing or empty build artifact: ${file}`);
  }
  return file;
}

function copyBundle(folder, extension, name) {
  const matches = fs.readdirSync(path.join(bundleDir, folder)).filter(file => file.endsWith(extension));
  if (matches.length !== 1) throw new Error(`Expected one ${extension} bundle, found ${matches.length}`);
  fs.copyFileSync(requireFile(path.join(bundleDir, folder, matches[0])), path.join(distDir, name));
}

function packageMacOS() {
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  const app = path.join(bundleDir, "macos", "Design Master.app");
  const binary = requireFile(path.join(app, "Contents", "MacOS", "design-master"));
  execFileSync("file", [binary], { stdio: "inherit" });
  execFileSync("codesign", ["--force", "--deep", "--sign", "-", app], { stdio: "inherit" });
  execFileSync("codesign", ["--verify", "--deep", "--strict", app], { stdio: "inherit" });
  const archive = path.join(distDir, `Design-Master-${version}-macos-${arch}.app.tar.gz`);
  execFileSync("tar", ["-czf", archive, "-C", path.dirname(app), path.basename(app)]);
  const stage = fs.mkdtempSync(path.join(os.tmpdir(), "dm-dmg-"));
  try {
    fs.cpSync(app, path.join(stage, "Design Master.app"), { recursive: true });
    fs.symlinkSync("/Applications", path.join(stage, "Applications"));
    const dmg = path.join(distDir, `Design-Master-${version}-macos-${arch}.dmg`);
    execFileSync("hdiutil", ["create", "-volname", "Design Master", "-srcfolder", stage, "-ov", "-format", "UDZO", dmg], { stdio: "inherit" });
    execFileSync("hdiutil", ["verify", dmg], { stdio: "inherit" });
  } finally {
    fs.rmSync(stage, { recursive: true, force: true });
  }
}

function packageWindows() {
  copyBundle("nsis", ".exe", `Design-Master-Setup-${version}-x64.exe`);
  const stage = fs.mkdtempSync(path.join(os.tmpdir(), "dm-windows-"));
  try {
    // Tauri embeds the Rust backend in the desktop executable.
    fs.copyFileSync(requireFile(path.join(releaseDir, "design-master.exe")), path.join(stage, "Design-Master.exe"));
    fs.cpSync(path.join(rootDir, "web", "out"), path.join(stage, "web-out"), { recursive: true });
    fs.cpSync(path.join(rootDir, "prompts", "v1"), path.join(stage, "prompts", "v1"), { recursive: true });
    const archive = path.join(distDir, `Design-Master-${version}-windows-x64.zip`);
    execFileSync("tar", ["-a", "-c", "-f", archive, "-C", stage, "."], { stdio: "inherit" });
    execFileSync("tar", ["-t", "-f", archive], { stdio: "ignore" });
  } finally {
    fs.rmSync(stage, { recursive: true, force: true });
  }
}

function packageLinux() {
  copyBundle("deb", ".deb", `design-master_${version}_amd64.deb`);
  copyBundle("appimage", ".AppImage", `Design-Master-${version}-linux-x86_64.AppImage`);
  const appImage = path.join(distDir, `Design-Master-${version}-linux-x86_64.AppImage`);
  if (!fs.readFileSync(appImage).subarray(0, 4).equals(Buffer.from([0x7f, 0x45, 0x4c, 0x46]))) {
    throw new Error("Tauri AppImage does not have an ELF executable header.");
  }
  fs.chmodSync(appImage, 0o755);
  execFileSync("dpkg-deb", ["--info", path.join(distDir, `design-master_${version}_amd64.deb`)], { stdio: "inherit" });
  const stage = fs.mkdtempSync(path.join(os.tmpdir(), "dm-linux-"));
  try {
    fs.copyFileSync(requireFile(path.join(releaseDir, "design-master")), path.join(stage, "design-master"));
    fs.cpSync(path.join(rootDir, "web", "out"), path.join(stage, "web-out"), { recursive: true });
    fs.cpSync(path.join(rootDir, "prompts", "v1"), path.join(stage, "prompts", "v1"), { recursive: true });
    fs.copyFileSync(path.join(rootDir, "src-tauri", "icons", "icon.png"), path.join(stage, "icon.png"));
    const archive = path.join(os.tmpdir(), `dm-linux-${process.pid}.tar.gz`);
    try {
      execFileSync("tar", ["-czf", archive, "-C", stage, "."]);
      const header = `#!/usr/bin/env bash
set -euo pipefail
INSTALL_DIR="$HOME/.local/lib/design-master"
mkdir -p "$HOME/.local/lib"
STAGING_DIR=$(mktemp -d "$HOME/.local/lib/design-master-stage-XXXXXX")
trap 'rm -rf "$STAGING_DIR"' EXIT
mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications" "$HOME/.local/share/icons/hicolor/512x512/apps"
ARCHIVE_LINE=$(awk '/^__ARCHIVE_BELOW__$/ {print NR + 1; exit}' "$0")
tail -n +"$ARCHIVE_LINE" "$0" | tar -xz -C "$STAGING_DIR"
test -f "$STAGING_DIR/design-master"
if [ -d "$INSTALL_DIR" ]; then mv "$INSTALL_DIR" "$STAGING_DIR/previous"; fi
if ! mv "$STAGING_DIR" "$INSTALL_DIR"; then
  if [ -d "$STAGING_DIR/previous" ]; then mv "$STAGING_DIR/previous" "$INSTALL_DIR"; fi
  exit 1
fi
rm -rf "$INSTALL_DIR/previous"
chmod +x "$INSTALL_DIR/design-master"
ln -sfn "$INSTALL_DIR/design-master" "$HOME/.local/bin/design-master"
cp "$INSTALL_DIR/icon.png" "$HOME/.local/share/icons/hicolor/512x512/apps/design-master.png"
cat > "$HOME/.local/share/applications/design-master.desktop" <<EOF
[Desktop Entry]
Name=Design Master
Exec="$INSTALL_DIR/design-master"
Icon=design-master
Terminal=false
Type=Application
Categories=Development;Graphics;
EOF
echo "Installed Design Master. Requires WebKit2GTK 4.1 and GTK3. Launch with $HOME/.local/bin/design-master"
exit 0
__ARCHIVE_BELOW__
`;
      fs.writeFileSync(path.join(distDir, `Design-Master-${version}-linux-x64-installer.run`), Buffer.concat([Buffer.from(header), fs.readFileSync(archive)]), { mode: 0o755 });
    } finally {
      fs.rmSync(archive, { force: true });
    }
  } finally {
    fs.rmSync(stage, { recursive: true, force: true });
  }
}

({ macos: packageMacOS, windows: packageWindows, linux: packageLinux })[platform]();
const checksums = fs.readdirSync(distDir)
  .filter(file => file !== "SHA256SUMS.txt")
  .sort()
  .map(file => `${createHash("sha256").update(fs.readFileSync(requireFile(path.join(distDir, file)))).digest("hex")}  ${file}`);
fs.writeFileSync(path.join(distDir, "SHA256SUMS.txt"), `${checksums.join("\n")}\n`);
console.log(`Native ${platform} packages ready in ${distDir}`);
