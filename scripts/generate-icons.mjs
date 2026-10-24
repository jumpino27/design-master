#!/usr/bin/env node
/**
 * Deterministically generates all Design Gen favicons, PWA icons, and native OS
 * desktop icons (.ico, .icns, .png, .svg, manifest.webmanifest).
 */
import fs from "node:fs";
import path from "node:path";
import zlib from "node:zlib";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, "..");
const webPublicDir = path.join(rootDir, "web", "public");
const assetsIconsDir = path.join(rootDir, "assets", "icons");
const tauriIconsDir = path.join(rootDir, "src-tauri", "icons");

fs.mkdirSync(webPublicDir, { recursive: true });
fs.mkdirSync(assetsIconsDir, { recursive: true });
fs.mkdirSync(tauriIconsDir, { recursive: true });

// ---------------------------------------------------------------------------
// 1. Vector SVG Icon (`icon.svg` and `design-gen.svg`) — Design Master ("DM") Dark Mode Emblem
// ---------------------------------------------------------------------------
const SVG_ICON = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="512" height="512">
  <defs>
    <linearGradient id="bgGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#0D0F18" />
      <stop offset="100%" stop-color="#07080D" />
    </linearGradient>
    <linearGradient id="accentGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7C3AED" />
      <stop offset="55%" stop-color="#4F46E5" />
      <stop offset="100%" stop-color="#06B6D4" />
    </linearGradient>
    <linearGradient id="borderGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#8B5CF6" stop-opacity="0.75" />
      <stop offset="100%" stop-color="#06B6D4" stop-opacity="0.55" />
    </linearGradient>
  </defs>
  <!-- Outer deep obsidian squircle -->
  <rect x="16" y="16" width="480" height="480" rx="112" fill="url(#bgGrad)" stroke="url(#borderGrad)" stroke-width="8" />
  <!-- Inner electric violet-cyan architectural tile -->
  <rect x="64" y="64" width="384" height="384" rx="84" fill="url(#accentGrad)" opacity="0.16" stroke="url(#borderGrad)" stroke-width="3" />
  <!-- Bold geometric 'D' monogram (left) -->
  <path d="M98 164 H168 C218 164 252 198 252 256 C252 314 218 348 168 348 H98 Z M142 204 V308 H165 C192 308 208 289 208 256 C208 223 192 204 165 204 Z" fill="url(#accentGrad)" />
  <!-- Bold geometric 'M' monogram (right) -->
  <path d="M268 348 V164 H308 L346 238 L384 164 H424 V348 H384 V236 L346 308 L308 236 V348 Z" fill="#F8FAFC" />
  <!-- Luminous cyan-gold precision spark node -->
  <circle cx="412" cy="116" r="14" fill="#22D3EE" />
</svg>
`;

fs.writeFileSync(path.join(webPublicDir, "icon.svg"), SVG_ICON, "utf8");
fs.writeFileSync(path.join(assetsIconsDir, "design-gen.svg"), SVG_ICON, "utf8");

// ---------------------------------------------------------------------------
// 2. Pure Node PNG & ICO Renderer (zero external native dependencies required)
// ---------------------------------------------------------------------------
function crc32(buf) {
  let c = 0xffffffff;
  for (let i = 0; i < buf.length; i++) {
    c ^= buf[i];
    for (let k = 0; k < 8; k++) {
      c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    }
  }
  return (c ^ 0xffffffff) >>> 0;
}

function makePngChunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length, 0);
  const typeBuf = Buffer.from(type, "ascii");
  const crcInput = Buffer.concat([typeBuf, data]);
  const crcBuf = Buffer.alloc(4);
  crcBuf.writeUInt32BE(crc32(crcInput), 0);
  return Buffer.concat([len, typeBuf, data, crcBuf]);
}

// Signed distance to a rounded rectangle centered at (cx, cy) with half-size (hx, hy) and radius r
function sdRoundedBox(px, py, cx, cy, hx, hy, r) {
  const qx = Math.abs(px - cx) - hx + r;
  const qy = Math.abs(py - cy) - hy + r;
  const outDist = Math.hypot(Math.max(qx, 0), Math.max(qy, 0));
  const inDist = Math.min(Math.max(qx, qy), 0);
  return outDist + inDist - r;
}

// Point-to-segment distance for crisp 'M' diagonals
function distToSegment(px, py, x1, y1, x2, y2) {
  const dx = x2 - x1;
  const dy = y2 - y1;
  const l2 = dx * dx + dy * dy;
  if (l2 === 0) return Math.hypot(px - x1, py - y1);
  const t = Math.max(0, Math.min(1, ((px - x1) * dx + (py - y1) * dy) / l2));
  return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
}

// Signed distance to the 'D' monogram in normalized 512x512 coordinates
function isInsideMonogramD(nx, ny) {
  if (ny < 164 || ny > 348 || nx < 98 || nx > 252) return false;
  const inOuterRect = nx >= 98 && nx <= 165 && ny >= 164 && ny <= 348;
  const distOuterArc = Math.hypot(nx - 165, ny - 256);
  const inOuter = inOuterRect || (nx > 165 && distOuterArc <= 92);
  if (!inOuter) return false;

  const inInnerRect = nx >= 140 && nx <= 164 && ny >= 204 && ny <= 308;
  const distInnerArc = Math.hypot(nx - 164, ny - 256);
  const inInner = inInnerRect || (nx > 164 && distInnerArc <= 52);
  return !inInner;
}

// Signed distance to the 'M' monogram in normalized 512x512 coordinates
function isInsideMonogramM(nx, ny) {
  if (ny < 164 || ny > 348 || nx < 268 || nx > 424) return false;
  const inLeftStem = nx >= 268 && nx <= 306 && ny >= 164 && ny <= 348;
  const inRightStem = nx >= 386 && nx <= 424 && ny >= 164 && ny <= 348;
  const inLeftDiag = distToSegment(nx, ny, 288, 178, 346, 292) <= 18.5;
  const inRightDiag = distToSegment(nx, ny, 404, 178, 346, 292) <= 18.5;
  return inLeftStem || inRightStem || inLeftDiag || inRightDiag;
}

function renderIconPng(size) {
  const raw = Buffer.alloc(size * (1 + size * 4));
  const scale = 512 / size;
  const aaWidth = 0.85 * scale;

  for (let y = 0; y < size; y++) {
    const rowOffset = y * (1 + size * 4);
    raw[rowOffset] = 0; // PNG filter: None

    for (let x = 0; x < size; x++) {
      const nx = (x + 0.5) * scale;
      const ny = (y + 0.5) * scale;
      const t = (nx + ny) / 1024;

      // 1. Outer obsidian rounded square (x: 16..496, r=112)
      const dOuter = sdRoundedBox(nx, ny, 256, 256, 240, 240, 112);
      if (dOuter > aaWidth) {
        const px = rowOffset + 1 + x * 4;
        raw[px] = 0;
        raw[px + 1] = 0;
        raw[px + 2] = 0;
        raw[px + 3] = 0;
        continue;
      }

      const outerAlpha =
        dOuter < -aaWidth
          ? 1.0
          : Math.max(0, Math.min(1, 0.5 - dOuter / (2 * aaWidth)));

      // Deep Obsidian gradient (#0D0F18 -> #07080D)
      let r = Math.round(13 * (1 - t) + 7 * t);
      let g = Math.round(15 * (1 - t) + 8 * t);
      let b = Math.round(24 * (1 - t) + 13 * t);

      // Luminous violet-cyan border around dOuter in [-9..0]
      if (dOuter > -9 && dOuter <= 0) {
        r = Math.round(124 * (1 - t) + 6 * t);
        g = Math.round(58 * (1 - t) + 182 * t);
        b = Math.round(237 * (1 - t) + 212 * t);
      }

      // 2. Inner dark glass tile (center 256,256, half-size 192, r=84)
      const dInner = sdRoundedBox(nx, ny, 256, 256, 192, 192, 84);
      if (dInner < aaWidth) {
        const innerBlend =
          dInner < -aaWidth
            ? 1.0
            : Math.max(0, Math.min(1, 0.5 - dInner / (2 * aaWidth)));
        const br = Math.round(24 * (1 - t) + 14 * t);
        const bg = Math.round(20 * (1 - t) + 28 * t);
        const bb = Math.round(44 * (1 - t) + 46 * t);
        r = Math.round(r * (1 - innerBlend) + br * innerBlend);
        g = Math.round(g * (1 - innerBlend) + bg * innerBlend);
        b = Math.round(b * (1 - innerBlend) + bb * innerBlend);
      }

      // 3. Monogram 'D' (Electric Violet-Cyan) & 'M' (Crisp Titanium White) with 4x supersampling
      let dSamples = 0;
      let mSamples = 0;
      let dotSamples = 0;
      const offsets = [-0.25, 0.25];
      for (const ox of offsets) {
        for (const oy of offsets) {
          const sx = (x + 0.5 + ox) * scale;
          const sy = (y + 0.5 + oy) * scale;
          if (isInsideMonogramD(sx, sy)) dSamples++;
          if (isInsideMonogramM(sx, sy)) mSamples++;
          if (Math.hypot(sx - 412, sy - 116) <= 14) dotSamples++;
        }
      }
      if (dSamples > 0) {
        const blend = dSamples / 4.0;
        const dr = Math.round(139 * (1 - t) + 34 * t);
        const dg = Math.round(92 * (1 - t) + 211 * t);
        const db = Math.round(246 * (1 - t) + 238 * t);
        r = Math.round(r * (1 - blend) + dr * blend);
        g = Math.round(g * (1 - blend) + dg * blend);
        b = Math.round(b * (1 - blend) + db * blend);
      }
      if (mSamples > 0) {
        const blend = mSamples / 4.0;
        r = Math.round(r * (1 - blend) + 248 * blend);
        g = Math.round(g * (1 - blend) + 250 * blend);
        b = Math.round(b * (1 - blend) + 252 * blend);
      }
      if (dotSamples > 0) {
        const blend = dotSamples / 4.0;
        r = Math.round(r * (1 - blend) + 34 * blend);
        g = Math.round(g * (1 - blend) + 211 * blend);
        b = Math.round(b * (1 - blend) + 238 * blend);
      }

      const px = rowOffset + 1 + x * 4;
      raw[px] = r;
      raw[px + 1] = g;
      raw[px + 2] = b;
      raw[px + 3] = Math.round(outerAlpha * 255);
    }
  }

  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(size, 0);
  ihdr.writeUInt32BE(size, 4);
  ihdr[8] = 8; // 8-bit
  ihdr[9] = 6; // RGBA
  ihdr[10] = 0;
  ihdr[11] = 0;
  ihdr[12] = 0;

  const idat = zlib.deflateSync(raw, { level: 9 });
  return Buffer.concat([
    signature,
    makePngChunk("IHDR", ihdr),
    makePngChunk("IDAT", idat),
    makePngChunk("IEND", Buffer.alloc(0)),
  ]);
}

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

const sizes = [16, 32, 48, 64, 128, 180, 192, 256, 512, 1024];
const pngBySize = new Map();
for (const sz of sizes) {
  pngBySize.set(sz, renderIconPng(sz));
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
  name: "Design Master — AI UI, Game HUD & Brand Workbench",
  short_name: "Design Master",
  description:
    "Dark-mode AI design workbench for generating brand-consistent multi-page webapps, landing pages, Game UI/HUD components, vector SVGs, and interactive 3D object references with pure human-language coding-agent handoffs.",
  start_url: "/",
  display: "standalone",
  background_color: "#090A0F",
  theme_color: "#090A0F",
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
