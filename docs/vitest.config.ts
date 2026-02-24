import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['tests/unit/**/*.test.ts'],
    globals: true,
    testTimeout: 30_000,
    coverage: {
      provider: 'v8',
      reportsDirectory: './coverage',
      include: ['src/**'],
      exclude: [
        'src/content/docs/api/control/**',
        'src/content/docs/api/triage/**',
        'src/content/docs/api/meshy/**',
        'src/content/docs/api/providers/**',
        'src/content/docs/api/vitest/**',
        'src/content/docs/api/_generated/**',
      ],
    },
  },
});
