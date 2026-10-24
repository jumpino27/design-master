# Release and launch status

Checked on 24 October 2026.

[Design Master v1.0.0](https://github.com/jumpino27/design-master/releases/tag/v1.0.0) is public. The [tagged build](https://github.com/jumpino27/design-master/actions/runs/40087532938) passed on macOS ARM64, Windows x64, and Linux x64. All seven published installers were downloaded into `dist/installers/` and matched the release checksums.

The macOS DMG passed `hdiutil verify`. The Windows portable app contains a PE32+ x64 executable; its setup program uses the standard NSIS bootstrapper. The Linux AppImage is an ELF x64 executable, and the `.run` archive contains the x64 app, static frontend, and prompts. A clean macOS app launch seeded all seven showcase images, and its updater selected the published ARM64 app archive and SHA-256 digest from `jumpino27/design-master`.

Local verification passed nine Rust unit tests, six integration tests, the web typecheck, and the Rust format check. Windows and Linux packages were built and checked on their native CI runners. They were not interactively installed on this Mac.

[GitHub Sponsors](https://github.com/sponsors/jumpino27) is approved and public. Repository Sponsorships is enabled, and `.github/FUNDING.yml` lists the verified GitHub account.

[Ko-fi](https://ko-fi.com/designmasterdm) is created with the project logo, description, repository link, euro currency, and monthly tips enabled. Its separate Stripe account is named Design Master. Stripe accepted the identity submission and lists it as in review, with payments and payouts paused. Stripe estimates 2–3 days. Ko-fi still shows Action required, so its link has not been added to `.github/FUNDING.yml` or advertised as accepting payments. A [public release announcement](https://ko-fi.com/post/Design-Master-1-0-0-is-available-E5P329DVAE) links to the downloads and active GitHub Sponsors page while verification is pending.

The [Product Hunt launch](https://www.producthunt.com/posts/new/SSMQFR4B5S3MYPM6CBYOLX5MOOPF7MQCE6) was successfully scheduled for 25 October 2026 at 00:01 Pacific time, or 08:01 Europe/Rome. It has a logo, three gallery screenshots, free pricing, a maker introduction, and Design Tools, Developer Tools, and Artificial Intelligence tags.

Hacker News was not posted. Its current guidelines prohibit generated text and automated posting. The owner should write and submit their own announcement. Grant eligibility and required applicant budgets remain unresolved; no grant application has been submitted.
