import { expect, test } from "@playwright/test";

const installedUpdate = {
  ok: true,
  previous_version: "1.1.1",
  installed_version: "1.1.2",
  backup_sqlite_path: "/workspace/backups/pre-update.sqlite",
  updated_target_path: "/Applications/Design Master.app",
  restart_required: true,
  restart_scheduled: true,
  message: "Installed Design Master v1.1.2. Restarting automatically…",
};

test.beforeEach(async ({ page }) => {
  await page.route("**/api/system/update/check", (route) => route.fulfill({
    json: {
      current_version: "1.1.1",
      latest_version: "1.1.2",
      update_available: true,
      github_repo: "jumpino27/design-master",
      auto_check_enabled: true,
      auto_install_enabled: false,
      status_message: "Update available",
      safety_guarantees: [],
    },
  }));
});

test("a completed update restarts without asking for another click", async ({ page }) => {
  await page.route("**/api/system/update/install", (route) => route.fulfill({
    json: installedUpdate,
  }));
  await page.goto("/projects");
  await page.getByRole("button", { name: "Check for updates", exact: true }).click();
  await page.getByRole("button", { name: "Install v1.1.2 Update Now", exact: true }).click();
  await expect(page.getByRole("status")).toHaveText("Opening the updated app…");
  await expect(page.getByRole("button", { name: "Restart Design Master Now", exact: true })).toHaveCount(0);
  await expect(page.getByRole("button", { name: "Restarting…", exact: true })).toBeDisabled();
});

test("a restart failure stays visible instead of reloading the old app", async ({ page }) => {
  await page.route("**/api/system/update/install", (route) => route.fulfill({
    json: { ...installedUpdate, restart_scheduled: false, message: "Update installed" },
  }));
  await page.route("**/api/system/update/restart", (route) => route.fulfill({
    status: 400,
    json: { error: { message: "Restart this server from your terminal." } },
  }));
  await page.goto("/projects");
  await page.getByRole("button", { name: "Check for updates", exact: true }).click();
  await page.getByRole("button", { name: "Install v1.1.2 Update Now", exact: true }).click();
  await page.getByRole("button", { name: "Restart Design Master Now", exact: true }).click();
  await expect(page.getByText("Restart this server from your terminal.", { exact: true })).toBeVisible();
  await expect(page.getByRole("button", { name: "Restart Design Master Now", exact: true })).toBeEnabled();
});
