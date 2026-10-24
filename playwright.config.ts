import { defineConfig, devices } from "@playwright/test";
import path from "node:path";
import os from "node:os";
import fs from "node:fs";

const e2eDataDir = fs.mkdtempSync(path.join(os.tmpdir(), "design-gen-e2e-"));

export default defineConfig({
  testDir: "./tests",
  testMatch: "e2e.spec.ts",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: 0,
  workers: 1,
  reporter: "list",
  timeout: 60_000,
  use: {
    baseURL: "http://127.0.0.1:4318",
    trace: "on-first-retry",
    permissions: ["clipboard-read", "clipboard-write"],
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"], channel: "chrome" },
    },
  ],
  webServer: {
    command: "./target/debug/design-gen-server",
    url: "http://127.0.0.1:4318/api/health",
    reuseExistingServer: false,
    timeout: 30_000,
    env: {
      DESIGN_GEN_HOST: "127.0.0.1",
      DESIGN_GEN_PORT: "4318",
      DESIGN_GEN_DATA_DIR: e2eDataDir,
      DESIGN_GEN_STATIC_DIR: path.resolve(__dirname, "web/out"),
      DESIGN_GEN_PROMPTS_DIR: path.resolve(__dirname, "prompts/v1"),
      DESIGN_GEN_TEST_ADAPTER: "1",
    },
  },
});
