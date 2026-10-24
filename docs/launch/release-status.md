# Release and launch status

Checked on 25 October 2026.

[Design Master v1.1.2](https://github.com/jumpino27/design-master/releases/tag/v1.1.2) is the latest public release. Its [tagged build](https://github.com/jumpino27/design-master/actions/runs/40138955655) passed on macOS ARM64, Windows x64, and Linux x64. All seven published installer digests match the release checksum manifest. The two macOS artifacts were also downloaded into `dist/verification/v1.1.2-ci/`, checked against their published sizes and checksums, and verified for bundle version and ARM64 architecture. The downloaded DMG passed `hdiutil verify`.

Version 1.1.2 fixes automatic restart after an in-place update, prevents duplicate desktop instances, and gives each desktop its own backend. Native macOS tests confirmed that replacing a bundle and requesting restart closes the old process, opens one new window from the replaced bundle, and preserves the existing project. Local checks passed twelve Rust unit tests, eleven integration tests, and thirteen browser checks. See [update and restart verification](../update-restart-verification.md). The owner's running app and creator projects were preserved. An upgrade from an older updater may need one final manual quit before the corrected restart flow takes over.

The earlier [Design Master v1.1.1](https://github.com/jumpino27/design-master/releases/tag/v1.1.1) introduced the visual refresh and four showcases. Its [tagged build](https://github.com/jumpino27/design-master/actions/runs/40131447731) passed on macOS ARM64, Windows x64, and Linux x64. All seven published installers were downloaded into `dist/verification/v1.1.1/` and matched the published file sizes and SHA-256 checksums.

The downloaded macOS DMG passed `hdiutil verify`, and its app archive contains the native executable. The Windows portable ZIP passed its archive integrity check and contains the PE executable and static frontend. Native installer headers were checked for the Windows setup, Linux DEB, and Linux AppImage. Windows and Linux packages were built and checked on their native CI runners. They were not interactively installed on this Mac.

The macOS application was launched and inspected through native Computer Use. Its four built-in showcases are Nexora, Elevate Studio, MetricFlow, and VYBE, with 23 embedded images. Seeding preserves existing projects and edits. Local verification passed ten Rust unit tests, nine integration tests, eleven browser UI checks, the web typecheck, and the production build.

The new crimson logo supplies the application, installer, browser, README, Product Hunt, and Ko-fi branding. GitHub's custom social preview shows the refreshed project library. Seven screenshots were captured from the running application in Aside and replace the former purple interface screenshots.

The [Product Hunt launch](https://www.producthunt.com/products/design-master?launch=design-master) has the new logo and seven gallery images. It remains scheduled for 25 October 2026 at 00:01 Pacific time, or 08:01 Europe/Rome.

[Ko-fi](https://ko-fi.com/designmasterdm) has the new profile logo, a screenshot cover, and a pinned seven-image showcase gallery with download, source, and sponsor links. Its owner page still reports that payment setup needs attention before it can accept support. The page should not be advertised as accepting payments until that is resolved. [GitHub Sponsors](https://github.com/sponsors/jumpino27) remains the active funding link in `.github/FUNDING.yml`.

Hacker News was not posted. Its guidelines prohibit generated text and automated posting. The owner should write and submit their own announcement. No grant application has been submitted.
