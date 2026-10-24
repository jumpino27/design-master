#!/usr/bin/env node
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const webPublicDir = path.join(rootDir, "web", "public");
const assetsIconsDir = path.join(rootDir, "assets", "icons");
const tauriIconsDir = path.join(rootDir, "src-tauri", "icons");
const logoPath = path.join(assetsIconsDir, "dm-logo.png");
const sizes = [16, 32, 48, 64, 128, 180, 192, 256, 512, 1024];
const pngBySize = new Map();
const resizedDir = fs.mkdtempSync(path.join(os.tmpdir(), "design-master-icons-"));

// Resize the generated source logo with Tauri's existing cross-platform image tool.
try {
  execFileSync(process.execPath, [
    path.join(rootDir, "node_modules", "@tauri-apps", "cli", "tauri.js"),
    "icon", logoPath, "--output", resizedDir,
    ...sizes.flatMap((size) => ["--png", String(size)]),
  ], { stdio: "inherit" });
  for (const size of sizes) {
    pngBySize.set(size, fs.readFileSync(path.join(resizedDir, `${size}x${size}.png`)));
  }
} finally {
  fs.rmSync(resizedDir, { recursive: true, force: true });
}
for (const dir of [webPublicDir, assetsIconsDir, tauriIconsDir]) {
  fs.mkdirSync(dir, { recursive: true });
}
fs.copyFileSync(logoPath, path.join(webPublicDir, "dm-logo.png"));
// Keep legacy SVG URLs consistent with the same raster mark.
const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><image width="1024" height="1024" href="data:image/png;base64,${pngBySize.get(1024).toString("base64")}"/></svg>`;
fs.writeFileSync(path.join(webPublicDir, "icon.svg"), svg);
fs.writeFileSync(path.join(assetsIconsDir, "design-gen.svg"), svg);

function buildIcoFile(pngEntries) {
  // ICONDIR header (6 bytes) + ICONDIRENTRY (16 bytes * count)
  const count = pngEntries.length;
  const header = Buffer.alloc(6 + count * 16);
  header.writeUInt16LE(0, 0); // Reserved
  header.writeUInt16LE(1, 2); // Type: 1 = ICO
  header.writeUInt16LE(count, 4);

  let dataOffset = 6 + count * 16;
  const buffers = [header];

  pngEntries.forEach(({ size, png }, idx) => {
    const entryOffset = 6 + idx * 16;
    header[entryOffset] = size >= 256 ? 0 : size;
    header[entryOffset + 1] = size >= 256 ? 0 : size;
    header[entryOffset + 2] = 0; // palette
    header[entryOffset + 3] = 0; // reserved
    header.writeUInt16LE(1, entryOffset + 4); // color planes
    header.writeUInt16LE(32, entryOffset + 6); // bits per pixel
    header.writeUInt32LE(png.length, entryOffset + 8);
    header.writeUInt32LE(dataOffset, entryOffset + 12);
    dataOffset += png.length;
    buffers.push(png);
  });

  return Buffer.concat(buffers);
}

function buildIcnsFile(pngBySize) {
  // Standard Apple ICNS container with PNG payloads (ic07=128, ic08=256, ic09=512, ic10=1024)
  const types = [
    { ostype: "icp4", size: 16 },
    { ostype: "icp5", size: 32 },
    { ostype: "icp6", size: 64 },
    { ostype: "ic07", size: 128 },
    { ostype: "ic08", size: 256 },
    { ostype: "ic09", size: 512 },
    { ostype: "ic10", size: 1024 },
  ];

  const chunks = [];
  let totalLen = 8;
  for (const t of types) {
    const png = pngBySize.get(t.size);
    if (!png) continue;
    const chunkHeader = Buffer.alloc(8);
    chunkHeader.write(t.ostype, 0, 4, "ascii");
    chunkHeader.writeUInt32BE(png.length + 8, 4);
    chunks.push(chunkHeader, png);
    totalLen += png.length + 8;
  }

  const fileHeader = Buffer.alloc(8);
  fileHeader.write("icns", 0, 4, "ascii");
  fileHeader.writeUInt32BE(totalLen, 4);
  return Buffer.concat([fileHeader, ...chunks]);
}

// Write web/public icons
fs.writeFileSync(
  path.join(webPublicDir, "favicon.ico"),
  buildIcoFile([
    { size: 16, png: pngBySize.get(16) },
    { size: 32, png: pngBySize.get(32) },
    { size: 48, png: pngBySize.get(48) },
  ])
);
fs.writeFileSync(
  path.join(webPublicDir, "apple-touch-icon.png"),
  pngBySize.get(180)
);
fs.writeFileSync(path.join(webPublicDir, "icon-192.png"), pngBySize.get(192));
fs.writeFileSync(path.join(webPublicDir, "icon-512.png"), pngBySize.get(512));

// Write web/public/manifest.webmanifest
const webManifest = {
  name: "Design Master",
  short_name: "Design Master",
  description:
    "Local UI mockups, reusable assets, and coding-agent handoffs.",
  start_url: "/",
  display: "standalone",
  background_color: "#0E0E10",
  theme_color: "#0E0E10",
  icons: [
    {
      src: "/icon-192.png",
      sizes: "192x192",
      type: "image/png",
      purpose: "any maskable",
    },
    {
      src: "/icon-512.png",
      sizes: "512x512",
      type: "image/png",
      purpose: "any maskable",
    },
    {
      src: "/icon.svg",
      sizes: "any",
      type: "image/svg+xml",
    },
  ],
};
fs.writeFileSync(
  path.join(webPublicDir, "manifest.webmanifest"),
  JSON.stringify(webManifest, null, 2) + "\n",
  "utf8"
);

// Write native OS icons in assets/icons/
fs.writeFileSync(
  path.join(assetsIconsDir, "design-gen.png"),
  pngBySize.get(512)
);
fs.writeFileSync(
  path.join(assetsIconsDir, "design-gen.ico"),
  buildIcoFile([
    { size: 16, png: pngBySize.get(16) },
    { size: 32, png: pngBySize.get(32) },
    { size: 48, png: pngBySize.get(48) },
    { size: 64, png: pngBySize.get(64) },
    { size: 128, png: pngBySize.get(128) },
    { size: 256, png: pngBySize.get(256) },
  ])
);
fs.writeFileSync(
  path.join(assetsIconsDir, "design-gen.icns"),
  buildIcnsFile(pngBySize)
);

// On macOS, if `iconutil` is available, also verify/build via `.iconset`
if (process.platform === "darwin") {
  try {
    const iconsetDir = path.join(assetsIconsDir, "DesignGen.iconset");
    fs.mkdirSync(iconsetDir, { recursive: true });
    const mappings = [
      ["icon_16x16.png", 16],
      ["icon_16x16@2x.png", 32],
      ["icon_32x32.png", 32],
      ["icon_32x32@2x.png", 64],
      ["icon_128x128.png", 128],
      ["icon_128x128@2x.png", 256],
      ["icon_256x256.png", 256],
      ["icon_256x256@2x.png", 512],
      ["icon_512x512.png", 512],
      ["icon_512x512@2x.png", 1024],
    ];
    for (const [name, sz] of mappings) {
      fs.writeFileSync(path.join(iconsetDir, name), pngBySize.get(sz));
    }
    execFileSync("iconutil", [
      "-c",
      "icns",
      iconsetDir,
      "-o",
      path.join(assetsIconsDir, "design-gen.icns"),
    ]);
    fs.rmSync(iconsetDir, { recursive: true, force: true });
  } catch {
    // Fallback ICNS already written by buildIcnsFile()
  }
}

// Write Tauri v2 bundle icons in src-tauri/icons/
fs.writeFileSync(path.join(tauriIconsDir, "32x32.png"), pngBySize.get(32));
fs.writeFileSync(path.join(tauriIconsDir, "128x128.png"), pngBySize.get(128));
fs.writeFileSync(path.join(tauriIconsDir, "128x128@2x.png"), pngBySize.get(256));
fs.writeFileSync(path.join(tauriIconsDir, "icon.png"), pngBySize.get(512));
fs.copyFileSync(
  path.join(assetsIconsDir, "design-gen.ico"),
  path.join(tauriIconsDir, "icon.ico")
);
fs.copyFileSync(
  path.join(assetsIconsDir, "design-gen.icns"),
  path.join(tauriIconsDir, "icon.icns")
);

console.log(
  "==> Generated favicons, PWA manifest, Tauri icons (src-tauri/icons), and native OS icons (.ico, .icns, .png, .svg)."
);
