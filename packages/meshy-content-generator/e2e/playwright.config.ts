import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  timeout: 60_000,
  use: {
    baseURL: "http://127.0.0.1:5177",
    browserName: "chromium",
  },
  webServer: {
    command: "pnpm dev",
    url: "http://127.0.0.1:5177/openapi.json",
    reuseExistingServer: !process.env.CI,
    env: {
      MESHY_API_KEY: process.env.MESHY_API_KEY ?? "test-key",
    },
  },
});
