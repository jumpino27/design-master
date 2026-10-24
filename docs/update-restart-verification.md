# Update and restart verification

Version 1.1.2, checked on 25 October 2026.

The previous restart endpoint launched a new instance with `open -n`, chose `~/Applications/Design Master.app` regardless of which bundle had been updated, and exited the process serving the request. A desktop could also reuse another process's backend. In that case, restarting the backend did not close the window that requested the restart. Installation returned a restart button without scheduling a restart.

The desktop now supplies the backend with its own restart handler. Successful in-place installation schedules that handler after returning the installation result. Tauri receives `request_restart`, closes the old window, releases the single-instance guard, and relaunches the original installation path. Duplicate requests share one pending restart. A desktop always starts its own backend, and the development frontend is used only when its health endpoint identifies that same process.

Restart support is checked before scheduling. Tauri rejects macOS launch paths containing symbolic links. Such launches retain the manual quit-and-reopen path instead of silently closing without relaunching. Browser-only servers return a visible error and never open an unrelated installed desktop app. Windows still hands installation to its external installer.

Verification passed:

- Twelve Rust unit tests and eleven integration tests, including automatic restart scheduling, duplicate restart requests, and browser-only error handling.
- Thirteen browser checks, including automatic restart status without another click and a visible, retryable restart error.
- Web typecheck, Rust formatting, workspace compilation, and the production macOS build.
- Native macOS bundle replacement in an isolated workspace. The previous test process exited, one new process opened the replaced bundle, its version and process ID matched its own health endpoint, and the existing project remained present.
- The native test used the default port while the owner's older app occupied it. The updated app started a separate backend and did not reuse the older process. A second launch focused the existing test app both before and after restart.

The running owner's app and creator projects were left intact. Windows and Linux installers require their native CI builds; this Mac test does not establish interactive update behavior on those systems.

Upgrading from 1.1.1 or earlier still uses the old version's updater until it exits. One final manual quit may be needed for that transition. Automatic restart applies to installations performed by the corrected updater.
