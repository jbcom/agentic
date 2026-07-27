import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'lcov', 'html'],
      reportsDirectory: './coverage',
    },
    // Setup files for test fixtures from @jbcom/vitest-agentic
    // NOTE: Path is resolved relative to this vitest.config.ts file location
    setupFiles: ['./tests/setup.ts'],
  },
});
