# ==============================================================================
# Design Master (DM) — Windows PowerShell Installer & Background Browser Launcher
# ==============================================================================
$ErrorActionPreference = "Stop"
Set-Location -Path $PSScriptRoot

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
  Write-Error "Node.js (v20+) is required. Install it from https://nodejs.org"
  exit 1
}

if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
  Write-Host "==> Enabling pnpm..."
  try { corepack enable } catch { npm install -g pnpm }
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  Write-Error "Rust / Cargo is required. Install it from https://rustup.rs"
  exit 1
}

Write-Host "==> Installing workspace dependencies..."
pnpm install

Write-Host "==> Running Design Master (DM) installer & launching background server at http://127.0.0.1:4318..."
node scripts/install.mjs install $args
