import { defineConfig, devices } from "@playwright/test";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const dataDir = fs.mkdtempSync(path.join(os.tmpdir(), "design-master-ui-"));

export default defineConfig({
  testDir: "./tests",
  testMatch: ["ui-layout.spec.ts", "chat-flow.spec.ts", "updater.spec.ts"],
  workers: 1,
  timeout: 30_000,
  reporter: "list",
  use: { baseURL: "http://127.0.0.1:4322", trace: "retain-on-failure" },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"], channel: "chrome" } }],
  webServer: {
    command: "./target/debug/design-gen-server",
    url: "http://127.0.0.1:4322/api/health",
    reuseExistingServer: false,
    env: {
      DESIGN_GEN_BIND_ADDR: "127.0.0.1:4322",
      DESIGN_GEN_DATA_DIR: dataDir,
      DESIGN_GEN_STATIC_DIR: path.resolve("web", process.env.DESIGN_MASTER_NEXT_DIST_DIR || "out"),
      DESIGN_GEN_SEED_EXAMPLES: "1",
      DESIGN_GEN_TEST_ADAPTER: "0",
      DESIGN_GEN_DISABLE_KEYRING: "1",
      OPENROUTER_API_KEY: "",
    },
  },
});
