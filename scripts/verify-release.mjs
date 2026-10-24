import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

const { version } = JSON.parse(fs.readFileSync("package.json", "utf8"));
const expected = {
  macos: [`Design-Master-${version}-macos-arm64.dmg`, `Design-Master-${version}-macos-arm64.app.tar.gz`],
  windows: [`Design-Master-Setup-${version}-x64.exe`, `Design-Master-${version}-windows-x64.zip`],
  linux: [`design-master_${version}_amd64.deb`, `Design-Master-${version}-linux-x86_64.AppImage`, `Design-Master-${version}-linux-x64-installer.run`],
};
const checksums = [];
for (const [platform, names] of Object.entries(expected)) {
  const folder = path.join("dist", "installers", platform);
  const manifest = fs.readFileSync(path.join(folder, "SHA256SUMS.txt"), "utf8");
  for (const name of names) {
    const bytes = fs.readFileSync(path.join(folder, name));
    assert(bytes.length > 1024, `Empty or incomplete installer: ${name}`);
    const checksum = createHash("sha256").update(bytes).digest("hex");
    assert(manifest.includes(`${checksum}  ${name}\n`), `Checksum mismatch: ${name}`);
    if (name.endsWith(".AppImage")) {
      assert(bytes.subarray(0, 4).equals(Buffer.from([0x7f, 0x45, 0x4c, 0x46])), "AppImage must be an ELF executable");
    }
    if (name.endsWith(".exe")) assert.equal(bytes.subarray(0, 2).toString(), "MZ");
    checksums.push(`${checksum}  ${name}`);
    console.log(`Verified ${name}: ${bytes.length} bytes`);
  }
}
fs.writeFileSync("dist/installers/SHA256SUMS.txt", `${checksums.join("\n")}\n`);
