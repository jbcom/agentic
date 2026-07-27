import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import type { TestCase, TestModule, Vitest } from 'vitest/node';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { StrataReporter } from '../src/reporters/vitest.ts';
import type { TestReport } from '../src/test-results.ts';

/**
 * Minimal stand-ins for vitest 4's reporter task graph. StrataReporter only
 * reads a handful of properties/methods off TestModule and TestCase, so we
 * mock exactly those rather than the entire real class hierarchy.
 */
function mockTestCase(overrides: {
    id: string;
    name: string;
    fullName: string;
    moduleId: string;
    state: 'passed' | 'failed' | 'skipped' | 'pending';
    duration?: number;
    retryCount?: number;
    error?: Error;
}): TestCase {
    const result =
        overrides.state === 'failed'
            ? { state: 'failed' as const, errors: overrides.error ? [overrides.error] : [] }
            : { state: overrides.state, errors: undefined };

    return {
        id: overrides.id,
        name: overrides.name,
        fullName: overrides.fullName,
        location: undefined,
        module: { moduleId: overrides.moduleId } as TestModule,
        result: () => result,
        diagnostic: () => ({
            slow: false,
            heap: undefined,
            duration: overrides.duration ?? 0,
            startTime: 0,
            retryCount: overrides.retryCount ?? 0,
            repeatCount: 0,
            flaky: false,
        }),
    } as unknown as TestCase;
}

function mockTestModule(moduleId: string, tests: TestCase[], state: 'passed' | 'failed' = 'passed'): TestModule {
    return {
        moduleId,
        state: () => state,
        diagnostic: () => ({
            environmentSetupDuration: 0,
            prepareDuration: 0,
            collectDuration: 0,
            setupDuration: 0,
            duration: tests.reduce((sum, t) => sum + (t.diagnostic()?.duration ?? 0), 0),
            heap: undefined,
            importDurations: {},
        }),
        children: {
            allTests: function* () {
                yield* tests;
            },
        },
    } as unknown as TestModule;
}

describe('StrataReporter (vitest 4)', () => {
    let outputDir: string;
    let outputFile: string;

    beforeEach(() => {
        outputDir = mkdtempSync(join(tmpdir(), 'strata-reporter-test-'));
        outputFile = join(outputDir, 'report.json');
    });

    afterEach(() => {
        rmSync(outputDir, { recursive: true, force: true });
    });

    it('writes a report summarizing passed, failed, and skipped tests', async () => {
        const reporter = new StrataReporter({ outputFile, includeCoverage: false });
        reporter.onInit({ config: { coverage: { enabled: false } } } as unknown as Vitest);

        const passing = mockTestCase({
            id: 't1',
            name: 'adds numbers',
            fullName: 'math > adds numbers',
            moduleId: '/repo/tests/math.test.ts',
            state: 'passed',
            duration: 5,
        });
        const failing = mockTestCase({
            id: 't2',
            name: 'divides by zero',
            fullName: 'math > divides by zero',
            moduleId: '/repo/tests/math.test.ts',
            state: 'failed',
            duration: 3,
            error: new Error('division by zero'),
        });
        const skipped = mockTestCase({
            id: 't3',
            name: 'not implemented yet',
            fullName: 'math > not implemented yet',
            moduleId: '/repo/tests/math.test.ts',
            state: 'skipped',
        });

        const testModule = mockTestModule('/repo/tests/math.test.ts', [passing, failing, skipped]);

        await reporter.onTestRunEnd([testModule], [], 'passed');

        expect(existsSync(outputFile)).toBe(true);
        const report = JSON.parse(readFileSync(outputFile, 'utf-8')) as TestReport;

        expect(report.runner).toBe('vitest');
        expect(report.summary).toMatchObject({ total: 3, passed: 1, failed: 1, skipped: 1 });
        expect(report.files).toHaveLength(1);
        expect(report.files[0]?.path).toBe('/repo/tests/math.test.ts');

        const failedTest = report.files[0]?.tests.find((t) => t.id === 't2');
        expect(failedTest?.status).toBe('failed');
        expect(failedTest?.error?.message).toBe('division by zero');

        const passedTest = report.files[0]?.tests.find((t) => t.id === 't1');
        expect(passedTest?.status).toBe('passed');
        expect(passedTest?.duration).toBe(5);
    });

    it('does nothing when no test modules ran', async () => {
        const reporter = new StrataReporter({ outputFile });
        reporter.onInit({ config: { coverage: { enabled: false } } } as unknown as Vitest);

        await reporter.onTestRunEnd([], [], 'passed');

        expect(existsSync(outputFile)).toBe(false);
    });

    it('marks a module with no tests as a setup error when the module failed', async () => {
        const reporter = new StrataReporter({ outputFile, includeCoverage: false });
        reporter.onInit({ config: { coverage: { enabled: false } } } as unknown as Vitest);

        const brokenModule = mockTestModule('/repo/tests/broken.test.ts', [], 'failed');

        await reporter.onTestRunEnd([brokenModule], [], 'failed');

        const report = JSON.parse(readFileSync(outputFile, 'utf-8')) as TestReport;
        expect(report.files[0]?.setupError?.message).toContain('/repo/tests/broken.test.ts');
    });
});
