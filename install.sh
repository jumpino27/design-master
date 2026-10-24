#!/usr/bin/env bash
# ==============================================================================
# Design Master (DM) — macOS & Linux One-Step Installer & Background Launcher
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if ! command -v node >/dev/null 2>&1; then
  echo "Error: Node.js (v20+) is required. Install it from https://nodejs.org"
  exit 1
fi

if ! command -v pnpm >/dev/null 2>&1; then
  echo "==> Enabling pnpm via corepack..."
  corepack enable >/dev/null 2>&1 || npm install -g pnpm
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: Rust / Cargo is required. Install it from https://rustup.rs"
  exit 1
fi

echo "==> Installing workspace dependencies..."
pnpm install

echo "==> Running Design Master (DM) installer & launching background server at http://127.0.0.1:4318..."
node scripts/install.mjs install "$@"
