Design Master is a local Tauri workbench for generating UI mockups, game HUDs, transparent assets, and implementation handoffs from a project's brand rules.

Downloads include macOS Apple Silicon DMG and app archive, Windows x64 NSIS installer and portable ZIP, and Linux x64 DEB, AppImage, and shell installer. The Rust backend runs inside the desktop application.

The Windows installer checks for WebView2 and downloads the runtime when needed. Linux requires WebKit2GTK 4.1 and GTK3. The DEB declares these dependencies. Install it with `sudo apt install ./design-master_1.0.0_amd64.deb`.

The macOS app uses an ad hoc signature and is not notarized. Windows packages are unsigned. Operating systems may show a security prompt. SHA256SUMS.txt contains checksums for every download.

The updater checks this repository's releases, chooses a package for the current OS and architecture, and backs up the SQLite database before installing. Windows updates launch the installer and may require closing the running app.

Generation requires an OpenRouter API key and incurs provider charges. Keys and local project data are excluded from the source repository.
