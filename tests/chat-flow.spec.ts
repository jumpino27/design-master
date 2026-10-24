import { expect, test, type APIRequestContext } from "@playwright/test";

async function fixtureSettings(request: APIRequestContext, enabled: boolean) {
  const status = await (await request.get("/api/setup/status")).json();
  const response = await request.put("/api/settings", {
    headers: { "X-CSRF-Token": status.csrf_token },
    data: { test_adapter_enabled: enabled },
  });
  expect(response.ok()).toBeTruthy();
  return status.csrf_token as string;
}

test.beforeEach(async ({ request }) => { await fixtureSettings(request, true); });
test.afterEach(async ({ request }) => { await fixtureSettings(request, false); });

test("starter sends immediately and answers advance one question at a time", async ({ page, request }) => {
  const csrf = (await (await request.get("/api/setup/status")).json()).csrf_token;
  const created = await request.post("/api/projects", {
    headers: { "X-CSRF-Token": csrf }, data: { name: "Chat UI test", industry: "SaaS" },
  });
  const project = await created.json();
  const endpoint = `/api/threads/${project.default_thread_id}/chat`;
  await page.goto(`/workspace?project=${project.project_id}&thread=${project.default_thread_id}`);
  const starter = page.getByTestId("chat-starter").first();
  const starterText = await starter.innerText();
  const firstResponse = page.waitForResponse((response) => response.url().endsWith(endpoint) && response.request().method() === "POST");
  await starter.click();
  const first = await (await firstResponse).json();
  expect(first.user_message.content).toBe(starterText);
  expect(first.action).toBe("ask_clarification");
  await expect(page.getByTestId("chat-composer-input")).toHaveValue("");
  await expect(page.getByTestId("clarification-card")).toHaveCount(1);
  await expect(page.getByTestId("clarification-option")).toHaveCount(3);

  const nextResponse = page.waitForResponse((response) => response.url().endsWith(endpoint) && response.request().method() === "POST");
  await page.getByTestId("clarification-option").first().click();
  const second = await (await nextResponse).json();
  expect(second.action).toBe("ask_clarification");
  expect(second.run).toBeNull();
  expect(second.assistant_message.content).not.toBe(first.assistant_message.content);
  await expect(page.getByTestId("clarification-option")).toHaveCount(3);
  const custom = page.getByRole("textbox", { name: "Custom answer", exact: true });
  await custom.fill("A quiet editorial layout without a 3D object");
  const customResponse = page.waitForResponse((response) => response.url().endsWith(endpoint) && response.request().method() === "POST");
  await page.getByRole("button", { name: "Send answer", exact: true }).click();
  const third = await (await customResponse).json();
  expect(third.user_message.content).toBe("A quiet editorial layout without a 3D object");
  expect(third.action).toBe("ask_clarification");
  await page.reload();
  await expect(page.getByTestId("clarification-card")).toHaveCount(1);
  await expect(page.getByTestId("clarification-option")).toHaveCount(3);
  await expect(page.getByText("Confirm Selected", { exact: false })).toHaveCount(0);
});

test("rapid choices send one request and a failed custom answer stays retryable", async ({ page, request }) => {
  const csrf = (await (await request.get("/api/setup/status")).json()).csrf_token;
  const created = await request.post("/api/projects", {
    headers: { "X-CSRF-Token": csrf }, data: { name: "Retry UI test" },
  });
  const project = await created.json();
  const endpoint = `/api/threads/${project.default_thread_id}/chat`;
  await page.goto(`/workspace?project=${project.project_id}&thread=${project.default_thread_id}`);
  await page.getByTestId("chat-starter").first().click();
  await expect(page.getByTestId("clarification-option")).toHaveCount(3);
  let requests = 0;
  page.on("request", (request) => { if (request.url().endsWith(endpoint) && request.method() === "POST") requests++; });
  await page.getByTestId("clarification-option").first().evaluate((button) => {
    button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    button.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  });
  await expect(page.getByRole("group", { name: "What should be the main visual focal point?" })).toBeVisible();
  expect(requests).toBe(1);

  let failOnce = true;
  await page.route(`**${endpoint}`, async (route) => {
    if (failOnce) {
      failOnce = false;
      await route.fulfill({ status: 502, contentType: "application/json", body: JSON.stringify({ error: { message: "Temporary provider failure" } }) });
    } else await route.continue();
  });
  const custom = page.getByRole("textbox", { name: "Custom answer", exact: true });
  await custom.fill("Use a product preview with real metrics");
  await page.getByRole("button", { name: "Send answer", exact: true }).click();
  await expect(page.getByText("Temporary provider failure", { exact: false })).toBeVisible();
  await expect(custom).toHaveValue("Use a product preview with real metrics");
  await page.getByRole("button", { name: "Send answer", exact: true }).click();
  await expect(page.getByRole("group", { name: "Which viewport should the design target?" })).toBeVisible();
});
