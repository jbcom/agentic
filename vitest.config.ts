import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        coverage: {
            // Use v8 for Node.js coverage
            provider: 'v8',
            
            // Reporters - lcov.info is REQUIRED for SonarCloud
            reporter: ['text', 'lcov', 'json', 'html', 'clover'],
            
            // Output directory - SonarCloud looks for coverage/lcov.info
            reportsDirectory: './coverage',
            
            // What to include in coverage
            include: [
                'packages/*/src/**/*.ts',
            ],
            
            // What to exclude from coverage
            exclude: [
                '**/node_modules/**',
                '**/dist/**',
                '**/*.test.ts',
                '**/*.spec.ts',
                '**/*.config.ts',
                '**/*.config.js',
                '**/tests/**',
            ],
            
            // Thresholds - start low, increase over time
            thresholds: {
                lines: 50,
                functions: 50,
                branches: 50,
                statements: 50,
            },
            
            // Generate coverage even if tests fail
            reportOnFailure: true,
        },
    },
});
