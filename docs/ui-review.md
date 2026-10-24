# Interface verification

Checked on 24 October 2026 for version 1.1.1.

The interface uses charcoal surfaces and crimson controls. All app, browser, and desktop icons derive from the Imagegen source at `assets/icons/dm-logo.png`. Its generation prompt is recorded in `assets/icons/logo-source.md`. White on the primary crimson button has a 5.44:1 contrast ratio. Project artwork and project color specimens keep their own palettes.

The project library uses two columns with landscape cards on desktop and stacked previews on smaller screens. Project actions, filters, and search stay together in a sticky toolbar. Empty search results offer a clear-filters action. Shared dialogs preserve their content height, scroll within the viewport, and support Escape dismissal.

Starter buttons send immediately. Clarification asks one question with three choices and a custom-answer box, using the complete current conversation. Stale and duplicate answers are rejected. Failed model requests leave the current question retryable.

Verification passed ten Rust unit tests, nine integration tests, eleven browser checks, the web typecheck, and the production build. Browser checks cover handoff and diagnostics dialogs at 1440×900, 1024×768, and 800×600, style-picker dismissal, a 390px-wide library, four-card desktop layout, sticky controls, search recovery, workspace navigation, fullscreen images, and the sequential chat. Chat checks use an isolated deterministic adapter and make no paid generation calls. The interface and launch screenshots were also inspected through native Computer Use in Aside.

Four approved showcases contain 23 embedded original images. The seed test verifies all images and thumbnails exist, costs start at zero, and reopening preserves user edits without duplicating outputs. The export omits settings, credentials, usage records, and provider request identifiers. Seeding does not replace projects already in a workspace.

`pnpm dev:creator` continues to use the owner's persistent `data/creator-workspace/` directory with automatic examples disabled. The creator database was backed up before the backend update.
