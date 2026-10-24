Design Master 1.1.2 fixes the desktop update and restart flow. After an in-place update finishes, the desktop closes its window and restarts the updated application automatically. Restart uses the current installation instead of opening another copy from a different folder. A single-instance guard focuses the existing window when the app is opened again.

The desktop now owns its backend and cannot reuse an older version still listening on the default port. Repeated restart requests schedule one restart, and restart errors remain visible. Pre-update SQLite backups and package verification still run before installation.

When upgrading from 1.1.1 or earlier, the old updater may still show its restart button. Quit the old window once if it remains open. The corrected automatic restart applies to updates installed by 1.1.2 and later. On Windows, finish the external installer when prompted.

The crimson interface, sticky project controls, sequential chat questions, and four finished showcases from 1.1.1 are included. Nexora, Elevate Studio, MetricFlow, and VYBE contain 23 rendered screens and assets. Existing projects and edits remain intact when upgrading.

Downloads include macOS Apple Silicon DMG and app archive, Windows x64 setup and portable ZIP, and Linux x64 DEB, AppImage, and shell installer. Check files against SHA256SUMS.txt. The macOS app is ad hoc signed and is not notarized. Windows packages are unsigned. Linux requires WebKit2GTK 4.1 and GTK3. Install the DEB with `sudo apt install ./design-master_1.1.2_amd64.deb`.

Generation requires your own OpenRouter key and incurs provider charges. Your API keys, workspace settings, and local usage history are not included in the showcases.
