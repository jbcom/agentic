import { defineConfig } from 'tsup';

/**
 * tsup configuration for @agentic-dev-library/triage
 *
 * This configuration ensures the package works correctly in:
 * - Node.js ESM (with proper .js extensions)
 * - Bundlers (Vite, Webpack, esbuild)
 * - TypeScript projects with any moduleResolution setting
 */
export default defineConfig({
    // Entry points matching package.json exports
    entry: {
        // Main entry
        index: 'src/index.ts',
        // CLI entry
        cli: 'src/cli.ts',
        // Subpath exports
        'schemas/index': 'src/schemas/index.ts',
        'tools/index': 'src/tools/index.ts',
        'handlers/index': 'src/handlers/index.ts',
        'reporters/playwright': 'src/reporters/playwright.ts',
        'reporters/vitest': 'src/reporters/vitest.ts',
    },

    // Output format - ESM only (the package is "type": "module")
    format: ['esm'],

    // Generate TypeScript declaration files
    dts: true,

    // Clean output directory before each build
    clean: true,

    // Generate source maps for debugging
    sourcemap: true,

    // Don't split chunks - each entry point is independent
    splitting: false,

    // Target ES2022 (matches tsconfig)
    target: 'ES2022',

    // External packages (don't bundle dependencies)
    external: [
        '@actions/core',
        '@ai-sdk/mcp',
        '@linear/sdk',
        '@modelcontextprotocol/sdk',
        '@playwright/mcp',
        '@playwright/test',
        'ai',
        'ai-sdk-ollama',
        'commander',
        'glob',
        'octokit',
        'ollama',
        'picocolors',
        'zod',
        'vitest',
    ],

    // Ensure proper ESM output
    treeshake: true,

    // Add banner for module compatibility
    banner: {
        js: '/* @agentic-dev-library/triage - ESM Build */',
    },

    // Minification disabled for library (consumers can minify)
    minify: false,

    // Keep names for better debugging
    keepNames: true,
});
