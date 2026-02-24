import fs from "node:fs";
import path from "node:path";
import { expect, test } from "@playwright/test";

const fixtureDir = path.resolve(process.cwd(), "e2e/fixtures");
const modelPath = path.join(fixtureDir, "model.glb");

test.beforeAll(() => {
  fs.mkdirSync(fixtureDir, { recursive: true });
  if (!fs.existsSync(modelPath)) {
    fs.writeFileSync(modelPath, "glTF");
  }
});

test("preview page renders model-viewer", async ({ page }) => {
  const url = `/preview?assetDir=${encodeURIComponent("e2e/fixtures")}&file=model.glb`;
  await page.goto(url);
  await expect(page.locator("model-viewer")).toBeVisible();
});
