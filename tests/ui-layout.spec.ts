import { expect, test, type Page } from "@playwright/test";

async function expectPageToFit(page: Page) {
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - window.innerWidth,
  );
  expect(overflow).toBeLessThanOrEqual(1);
}

for (const viewport of [
  { width: 1440, height: 900 },
  { width: 1024, height: 768 },
  { width: 800, height: 600 },
]) {
  test(`handoff stays readable and scrollable at ${viewport.width}x${viewport.height}`, async ({
    page,
  }) => {
    await page.setViewportSize(viewport);
    await page.goto(
      "/workspace?project=example-nexora&thread=example-thread-nexora",
    );
    await page
      .getByRole("button", { name: "Tab Handoff", exact: true })
      .click();
    const dialog = page.getByRole("dialog", { name: /Copy Prompt for Whole/ });
    await expect(dialog).toBeVisible();
    const bounds = await dialog.boundingBox();
    expect(bounds!.height).toBeGreaterThan(350);
    expect(bounds!.height).toBeLessThanOrEqual(viewport.height - 30);
    expect(bounds!.y).toBeGreaterThanOrEqual(0);
    const prompt = dialog.getByRole("textbox", {
      name: "Handoff implementation prompt text",
    });
    await expect(prompt).not.toHaveValue("");
    await prompt.scrollIntoViewIfNeeded();
    await expect(prompt).toBeInViewport();
    const promptBounds = await prompt.boundingBox();
    expect(promptBounds!.height).toBeGreaterThan(250);
    const copy = dialog.getByRole("button", {
      name: "Copy handoff prompt",
      exact: true,
    });
    await copy.scrollIntoViewIfNeeded();
    await expect(copy).toBeInViewport();
    await page.keyboard.press("Escape");
    await expect(dialog).not.toBeVisible();

    await page
      .getByRole("button", { name: "Run Details", exact: true })
      .click();
    const details = page.getByRole("dialog", {
      name: "Run Diagnostics & Technical Details",
    });
    await expect(details).toBeVisible();
    await expect(
      details.getByText("Models, Capabilities & Cost Accounting"),
    ).toBeInViewport();
    await details.getByRole("button", { name: "Close dialog" }).click();
    await expect(details).not.toBeVisible();
  });
}

test("project creation and style picker dismiss without trapping the wizard", async ({
  page,
}) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  await page.goto("/projects");
  await page.getByRole("button", { name: "New Project", exact: true }).click();
  const wizard = page.getByRole("dialog");
  await expect(wizard).toBeVisible();
  await page.getByRole("button", { name: /Create Manually/ }).click();
  await page.getByTestId("choose-style-btn").click();
  const picker = page.getByRole("dialog", { name: /Choose Your Project/ });
  await expect(picker).toBeVisible();
  await page
    .getByPlaceholder(/Search styles by design system keyword/)
    .fill("minimal");
  await page.keyboard.press("Escape");
  await expect(picker).not.toBeVisible();
  await expect(
    page.getByRole("dialog", { name: /Manual Project/ }),
  ).toBeVisible();
});

test("project library fits a narrow window and primary actions use readable crimson", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/projects");
  await expect(
    page.getByRole("heading", { name: "Local Projects", exact: true }),
  ).toBeVisible();
  await expectPageToFit(page);
  const colors = await page
    .getByRole("button", { name: "New Project", exact: true })
    .evaluate((button) => {
      const style = getComputedStyle(button);
      return { background: style.backgroundColor, foreground: style.color };
    });
  expect(colors.background).toBe("rgb(201, 39, 70)");
  expect(colors.foreground).toBe("rgb(255, 255, 255)");
});

test("four showcases fit the desktop library and its controls stay visible while scrolling", async ({
  page,
  request,
}) => {
  const response = await request.get("/api/projects");
  const { projects } = await response.json();
  const showcases = projects.filter(
    (project: { is_example: boolean }) => project.is_example,
  );
  expect(showcases).toHaveLength(4);
  await page.route("**/api/projects", (route) =>
    route.fulfill({ json: { projects: showcases } }),
  );
  for (const viewport of [
    { width: 1440, height: 900 },
    { width: 1204, height: 844 },
  ]) {
    await page.setViewportSize(viewport);
    await page.goto("/projects");
    await expect(page.getByTestId("project-card")).toHaveCount(4);
    for (const card of await page.getByTestId("project-card").all()) {
      const bounds = await card.boundingBox();
      expect(bounds!.y + bounds!.height).toBeLessThanOrEqual(viewport.height);
    }
    await expectPageToFit(page);
  }
  await page.setViewportSize({ width: 800, height: 600 });
  await page.mouse.move(600, 500);
  await page.mouse.wheel(0, 900);
  await expect
    .poll(
      async () =>
        (await page.getByTestId("project-library-toolbar").boundingBox())!.y,
    )
    .toBe(0);
  await expect(
    page.getByRole("heading", { name: "Local Projects", exact: true }),
  ).toBeInViewport();
  const search = page.getByRole("searchbox", {
    name: "Search projects",
    exact: true,
  });
  await expect(search).toBeInViewport();
  await search.fill("no project matches this");
  await expect(
    page.getByRole("heading", { name: "No matching projects" }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Clear filters", exact: true })
    .click();
  await expect(page.getByTestId("project-card")).toHaveCount(4);
});

test("workspace navigation, tab menu, and fullscreen viewer remain usable", async ({
  page,
}) => {
  await page.goto(
    "/workspace?project=example-nexora&thread=example-thread-nexora",
  );
  await page.getByTestId("canvas-tabs-dropdown-btn").click();
  await expect(
    page.getByRole("button", { name: /Full Landing Page/ }).last(),
  ).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("canvas-tabs-dropdown-menu")).not.toBeVisible();
  await page.getByTestId("sidebar-nav-design-md").click();
  const exportButton = page.getByTestId("download-design-md-btn");
  await expect(exportButton).toBeVisible();
  expect(
    await exportButton.evaluate(
      (button) => getComputedStyle(button).backgroundColor,
    ),
  ).toBe("rgb(201, 39, 70)");
  await page
    .getByRole("button", { name: "Brand Kit & Style", exact: true })
    .click();
  await expect(
    page.getByRole("heading", { name: "Brand Kit & Design System Studio" }),
  ).toBeVisible();
  await page.getByRole("button", { name: /Assets Library/ }).click();
  await expect(
    page
      .getByRole("heading", { name: "Project Assets & Reference Studio" })
      .first(),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Chat & Multi-Tab Canvas", exact: true })
    .click();
  await page
    .getByRole("button", { name: "View Larger", exact: true })
    .first()
    .click();
  const viewer = page.getByRole("dialog", { name: /Fullscreen viewer/ });
  await expect(viewer).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(viewer).not.toBeVisible();
});

for (const route of ["setup", "settings"]) {
  test(`${route} form fits a smaller desktop window`, async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    await page.goto(`/${route}`);
    await expect(page.getByRole("heading").first()).toBeVisible();
    await expectPageToFit(page);
    expect(errors).toEqual([]);
  });
}
