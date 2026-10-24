@echo off
REM ============================================================================
REM Design Master (DM) — Windows Batch Installer & Background Browser Launcher
REM ============================================================================
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" %*
