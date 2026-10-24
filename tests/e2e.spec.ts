import { test, expect } from "@playwright/test";
import zlib from "node:zlib";

function crc32(buf: Buffer): number {
  let c = 0xffffffff;
  for (let i = 0; i < buf.length; i++) {
    c ^= buf[i];
    for (let k = 0; k < 8; k++) {
      c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    }
  }
  return (c ^ 0xffffffff) >>> 0;
}

function makeChunk(type: string, data: Buffer): Buffer {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length, 0);
  const typeBuf = Buffer.from(type, "ascii");
  const crcInput = Buffer.concat([typeBuf, data]);
  const crcBuf = Buffer.alloc(4);
  crcBuf.writeUInt32BE(crc32(crcInput), 0);
  return Buffer.concat([len, typeBuf, data, crcBuf]);
}

function createValidPngBuffer(width = 32, height = 24): Buffer {
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; // bit depth 8
  ihdr[9] = 6; // color type RGBA
  ihdr[10] = 0;
  ihdr[11] = 0;
  ihdr[12] = 0;

  const rawScanlines = Buffer.alloc(height * (1 + width * 4));
  for (let y = 0; y < height; y++) {
    const rowStart = y * (1 + width * 4);
    rawScanlines[rowStart] = 0; // filter type None
    for (let x = 0; x < width; x++) {
      const px = rowStart + 1 + x * 4;
      rawScanlines[px] = 213; // R (#D56A3A)
      rawScanlines[px + 1] = 106; // G
      rawScanlines[px + 2] = 58; // B
      rawScanlines[px + 3] = 255; // A
    }
  }
  const idatData = zlib.deflateSync(rawScanlines);

  return Buffer.concat([
    signature,
    makeChunk("IHDR", ihdr),
    makeChunk("IDAT", idatData),
    makeChunk("IEND", Buffer.alloc(0)),
  ]);
}

test.describe("Design Gen End-to-End Workbench Verification", () => {
  test("1. Missing /api and /media paths return JSON 404 (never HTML fallback)", async ({
    request,
  }) => {
    const apiRes = await request.get("/api/does-not-exist");
    expect(apiRes.status()).toBe(404);
    expect(apiRes.headers()["content-type"]).toContain("application/json");
    const apiBody = await apiRes.json();
    expect(apiBody.error.code).toBe("NOT_FOUND");

    const mediaRes = await request.get("/media/output/missing-id/original");
    expect(mediaRes.status()).toBe(404);
    expect(mediaRes.headers()["content-type"]).toContain("application/json");

    // Verify favicons, icons, and webmanifest are served with 200 OK
    const favRes = await request.get("/favicon.ico");
    expect(favRes.status()).toBe(200);
    expect(favRes.headers()["content-type"]).toContain("image/x-icon");

    const svgIconRes = await request.get("/icon.svg");
    expect(svgIconRes.status()).toBe(200);
    expect(svgIconRes.headers()["content-type"]).toContain("image/svg+xml");

    const png512Res = await request.get("/icon-512.png");
    expect(png512Res.status()).toBe(200);
    expect(png512Res.headers()["content-type"]).toContain("image/png");

    const manifestRes = await request.get("/manifest.webmanifest");
    expect(manifestRes.status()).toBe(200);
    expect(manifestRes.headers()["content-type"]).toContain(
      "application/manifest+json"
    );
  });

  test("2. Full user flow: Setup -> Create Project -> Upload Screenshot -> Component 4x -> Copy for Agent -> Full Page Plan & Set -> Brand Snapshot Immutability -> Deep Links", async ({
    page,
  }) => {
    // Step A: First-run 3-step setup wizard
    await page.goto("/");
    await expect(page).toHaveURL(/\/setup/);
    await expect(
      page.getByRole("heading", { name: "Welcome to Design Gen" })
    ).toBeVisible();
    await expect(
      page.getByRole("heading", {
        name: /Step 1: Connect Your OpenRouter API Key/i,
      })
    ).toBeVisible();

    // Advance from Step 1 -> Step 2 (AI Models & Test Fixture Mode)
    await page.getByTestId("setup-next-step-1").click();
    await expect(
      page.getByRole("heading", {
        name: /Step 2: Pick Your Text & Image Models/i,
      })
    ).toBeVisible();
    await expect(
      page.getByText(/What is this & why enable it\?/i)
    ).toBeVisible();

    // Test the "Test Selected Models" button
    await page.getByTestId("test-models-button").click();
    await expect(
      page.getByText(/Multimodal vision \+ text input supported!/i)
    ).toBeVisible();

    // Advance from Step 2 -> Step 3 (Storage Folder Picker)
    await page.getByTestId("setup-next-step-2").click();
    await expect(
      page.getByRole("heading", {
        name: /Step 3: Where Should Your Projects & Images Be Saved\?/i,
      })
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Choose Folder \(Finder \/ Explorer\)/i })
    ).toBeVisible();

    await page
      .getByRole("button", { name: /Finish setup & open project library/i })
      .click();
    await expect(page).toHaveURL(/\/projects/);

    // Step B: Create Project ("Forma")
    await expect(
      page.getByRole("heading", { name: "Local Projects", exact: true })
    ).toBeVisible();
    await page
      .getByRole("button", { name: /Create first project/i })
      .click();
    await expect(
      page.getByRole("heading", { name: "Create Project & Brand Kit" })
    ).toBeVisible();

    await page.locator("#proj-name").fill("Forma");
    await page.getByTestId("choose-style-btn").click();
    await expect(
      page.getByRole("heading", {
        name: /Choose Your Project’s Web Design Style/i,
      })
    ).toBeVisible();
    await page.locator("#style-keyword-search-input").fill("neobrutalism");
    await page.getByTestId("style-search-btn").click();
    await expect(page.getByTestId("style-card-21")).toBeVisible();
    await page.getByTestId("use-style-btn-21").click();
    await expect(page.getByTestId("selected-style-preview-card")).toBeVisible();

    await page
      .getByRole("button", { name: /Create project & open workbench/i })
      .click();

    await expect(page).toHaveURL(/\/workspace\?/);
    await expect(
      page.getByRole("heading", { name: /Pricing card refresh/i })
    ).toBeVisible();

    // Step C: Upload Current Screenshot in the right composer
    const screenshotInput = page.locator('input[type="file"]').first();
    await screenshotInput.setInputFiles({
      name: "pricing-current.png",
      mimeType: "image/png",
      buffer: createValidPngBuffer(64, 48),
    });

    await expect(page.getByText("pricing-current.png").first()).toBeVisible();
    await expect(
      page.getByText("Brand kit + current request + 1 image")
    ).toBeVisible();

    // Step D: Generate 4 variations in Component mode
    const generateBtn = page.getByTestId("generate-cta-button");
    await expect(generateBtn).toHaveText("Generate 4 versions");
    await generateBtn.click();

    // Wait for all 4 output cards to complete
    for (let i = 1; i <= 4; i++) {
      const card = page.getByTestId(`output-card-${i}`);
      await expect(card).toBeVisible({ timeout: 20_000 });
      await expect(card.locator("img")).toBeVisible({ timeout: 20_000 });
    }
    await expect(page.getByText("Completed", { exact: true })).toBeVisible();

    // Step E: Click `Copy for agent` on variation 01 and verify clipboard + real files on disk
    const copyBtn1 = page.getByTestId("copy-for-agent-1");
    await copyBtn1.click();
    await expect(copyBtn1).toContainText("Copied");

    const clipboardText = await page.evaluate(() =>
      navigator.clipboard.readText()
    );
    expect(clipboardText).toContain("PRIMARY GENERATED DESIGN REFERENCE");
    expect(clipboardText).toContain("original.png");
    expect(clipboardText).toContain("BRAND AND SOURCE ASSETS");
    expect(clipboardText).toContain(
      "CHOSEN WEB DESIGN STYLE & DESIGN SYSTEM RULES"
    );
    expect(clipboardText).toContain("Neobrutalism");

    // Verify Copy entire set opens HandoffDialog
    await page.getByRole("button", { name: /Copy entire set/i }).click();
    await expect(
      page.getByRole("heading", {
        name: /Copy Entire Set for Coding Agent/i,
      })
    ).toBeVisible();

    // Close HandoffDialog
    await page.getByRole("button", { name: "Close dialog" }).click();

    // Step F: Switch to Full-Page mode, review plan, and generate coordinated set
    await page.getByRole("button", { name: "Full page", exact: true }).click();
    await expect(generateBtn).toHaveText("Plan full page set");
    await generateBtn.click();

    // Expect interactive plan review card
    await expect(
      page.getByText("Full-Page Coordinated Set Plan Ready for Review")
    ).toBeVisible({ timeout: 15_000 });
    await page
      .getByRole("button", { name: /Approve & Generate Set/i })
      .click();

    // Wait for full-page set completion
    await expect(page.getByTestId("output-card-5").locator("img")).toBeVisible({
      timeout: 25_000,
    });

    // Step G: Edit Brand Kit and verify prior runs keep frozen brand_version v1
    await page
      .locator("header")
      .getByRole("button", { name: "Brand kit" })
      .click();
    await expect(
      page.getByRole("heading", { name: "Brand Kit & Project Brief" })
    ).toBeVisible();

    await page
      .getByRole("button", { name: /Save Brand v2/i })
      .click();
    await expect(page.getByText("Active Snapshot v2")).toBeVisible();

    // Switch back to Generations view and verify Run #1 still shows (v1)
    await page
      .locator("aside")
      .getByRole("button", { name: "Generations" })
      .click();
    await expect(page.getByText(/Run #1 · component \(v1\)/i)).toBeVisible();

    // Step H: Verify direct deep-link navigation to /settings and /projects
    await page.goto("/settings");
    await expect(page.getByText("Design Gen Settings")).toBeVisible();

    await page.goto("/projects");
    await expect(
      page.getByRole("heading", { name: "Local Projects", exact: true })
    ).toBeVisible();
    await expect(page.getByText("Forma").first()).toBeVisible();
  });
});
