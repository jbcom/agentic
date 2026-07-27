/**
 * Vitest reporter for Agentic Triage
 *
 * Generates structured test reports for AI analysis.
 *
 * Usage in vitest.config.ts:
 * ```ts
 * import { StrataReporter } from '@jbcom/agentic-triage/reporters/vitest';
 *
 * export default defineConfig({
 *   test: {
 *     reporters: ['default', new StrataReporter()],
 *   },
 * });
 * ```
 */

import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import type { Reporter, SerializedError, TestCase, TestModule, TestRunEndReason, Vitest } from 'vitest/node';
import type { CoverageData, TestError, TestFile, TestReport, TestResult as StrataTestResult } from '../test-results.js';

export interface StrataReporterOptions {
    /** Output file path */
    outputFile?: string;
    /** Test type */
    type?: 'unit' | 'integration' | 'e2e';
    /** Include coverage data */
    includeCoverage?: boolean;
}

export class StrataReporter implements Reporter {
    private ctx!: Vitest;
    private options: Required<StrataReporterOptions>;
    private startTime = 0;

    constructor(options: StrataReporterOptions = {}) {
        this.options = {
            outputFile: options.outputFile ?? './test-results/strata-report.json',
            type: options.type ?? 'unit',
            includeCoverage: options.includeCoverage ?? true,
        };
    }

    onInit(ctx: Vitest): void {
        this.ctx = ctx;
        this.startTime = Date.now();
    }

    async onTestRunEnd(
        testModules: ReadonlyArray<TestModule>,
        _unhandledErrors: ReadonlyArray<SerializedError>,
        _reason: TestRunEndReason
    ): Promise<void> {
        if (!testModules.length) return;

        const report = this.buildReport(testModules);

        // Write report
        const outputPath = resolve(this.options.outputFile);
        mkdirSync(dirname(outputPath), { recursive: true });
        writeFileSync(outputPath, JSON.stringify(report, null, 2));

        console.log(`\n📊 Agentic triage test report: ${outputPath}`);
    }

    private buildReport(testModules: ReadonlyArray<TestModule>): TestReport {
        const testFiles = testModules.map((m) => this.processModule(m));
        const allTests = testFiles.flatMap((f) => f.tests);

        const summary = {
            total: allTests.length,
            passed: allTests.filter((t) => t.status === 'passed').length,
            failed: allTests.filter((t) => t.status === 'failed').length,
            skipped: allTests.filter((t) => t.status === 'skipped').length,
            duration: Date.now() - this.startTime,
        };

        const report: TestReport = {
            version: '1.0',
            timestamp: new Date().toISOString(),
            runner: 'vitest',
            type: this.options.type,
            summary,
            files: testFiles,
            git: this.getGitContext(),
            ci: this.getCIContext(),
        };

        // Add coverage if available
        if (this.options.includeCoverage && this.ctx.config.coverage?.enabled) {
            report.coverage = this.getCoverageData();
        }

        return report;
    }

    private processModule(testModule: TestModule): TestFile {
        const tests = [...testModule.children.allTests()].map((testCase) => this.processTestCase(testCase));
        const diagnostic = testModule.diagnostic();

        // A module-level (collection) error surfaces as a failing state with
        // no test cases at all — vitest 4 no longer exposes a `result.errors`
        // shortcut for this, so fall back to the module state.
        const setupError =
            tests.length === 0 && testModule.state() === 'failed' ? { message: `Failed to collect tests in ${testModule.moduleId}` } : undefined;

        return {
            path: testModule.moduleId,
            tests,
            duration: diagnostic.duration,
            setupError,
        };
    }

    private processTestCase(testCase: TestCase): StrataTestResult {
        const result = testCase.result();
        const diagnostic = testCase.diagnostic();
        const error = result.state === 'failed' ? result.errors[0] : undefined;

        return {
            id: testCase.id,
            name: testCase.name,
            fullName: testCase.fullName,
            file: testCase.module.moduleId,
            line: testCase.location?.line,
            status: this.mapStatus(result.state),
            duration: diagnostic?.duration ?? 0,
            error: error ? this.processError(error) : undefined,
            retry: diagnostic?.retryCount,
        };
    }

    private mapStatus(state: 'pending' | 'passed' | 'failed' | 'skipped'): StrataTestResult['status'] {
        switch (state) {
            case 'passed':
                return 'passed';
            case 'failed':
                return 'failed';
            case 'skipped':
                return 'skipped';
            default:
                return 'skipped';
        }
    }

    private processError(error: unknown): TestError {
        if (error instanceof Error) {
            return {
                message: error.message,
                stack: error.stack,
                // Vitest-specific properties
                expected: (error as { expected?: unknown }).expected,
                actual: (error as { actual?: unknown }).actual,
                diff: (error as { diff?: string }).diff,
                codeFrame: (error as { codeFrame?: string }).codeFrame,
            };
        }
        return { message: String(error) };
    }

    private getGitContext(): TestReport['git'] | undefined {
        try {
            const branch = execFileSync('git', ['rev-parse', '--abbrev-ref', 'HEAD'], { encoding: 'utf-8' }).trim();
            const commit = execFileSync('git', ['rev-parse', '--short', 'HEAD'], { encoding: 'utf-8' }).trim();
            const message = execFileSync('git', ['log', '-1', '--pretty=%s'], { encoding: 'utf-8' }).trim();
            const author = execFileSync('git', ['log', '-1', '--pretty=%an'], { encoding: 'utf-8' }).trim();

            return { branch, commit, message, author };
        } catch {
            return undefined;
        }
    }

    private getCIContext(): TestReport['ci'] | undefined {
        // GitHub Actions
        if (process.env.GITHUB_ACTIONS) {
            const prNumber = process.env.GITHUB_REF?.match(/refs\/pull\/(\d+)/)?.[1];
            return {
                provider: 'github-actions',
                runId: process.env.GITHUB_RUN_ID ?? '',
                runUrl: `${process.env.GITHUB_SERVER_URL}/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}`,
                prNumber: prNumber ? parseInt(prNumber, 10) : undefined,
            };
        }

        return undefined;
    }

    private getCoverageData(): CoverageData | undefined {
        for (const candidate of this.getCoverageCandidates()) {
            if (!existsSync(candidate)) {
                continue;
            }

            try {
                const parsed = JSON.parse(readFileSync(candidate, 'utf-8')) as Record<string, unknown>;
                if (candidate.endsWith('coverage-summary.json')) {
                    return this.parseCoverageSummary(parsed);
                }
                if (candidate.endsWith('coverage-final.json')) {
                    return this.parseCoverageFinal(parsed);
                }
            } catch {
                // Ignore malformed coverage files and keep searching.
            }
        }

        return undefined;
    }

    private getCoverageCandidates(): string[] {
        const outputDir = dirname(resolve(this.options.outputFile));
        return [
            resolve(process.cwd(), 'coverage/coverage-summary.json'),
            resolve(process.cwd(), 'coverage/coverage-final.json'),
            resolve(outputDir, 'coverage/coverage-summary.json'),
            resolve(outputDir, 'coverage/coverage-final.json'),
        ];
    }

    private parseCoverageSummary(data: Record<string, unknown>): CoverageData | undefined {
        const totals = this.asCoverageMetricRecord(data.total);
        if (!totals) {
            return undefined;
        }

        const files = Object.entries(data)
            .filter(([key]) => key !== 'total')
            .map(([path, metrics]) => {
                const coverage = this.asCoverageMetricRecord(metrics);
                if (!coverage) {
                    return undefined;
                }

                return {
                    path,
                    lines: this.metricFromSummary(coverage.lines),
                    uncoveredLines: [],
                    functions: this.metricFromSummary(coverage.functions),
                    uncoveredFunctions: [],
                };
            })
            .filter((file): file is NonNullable<typeof file> => Boolean(file));

        return {
            lines: this.metricFromSummary(totals.lines),
            functions: this.metricFromSummary(totals.functions),
            branches: this.metricFromSummary(totals.branches),
            statements: this.metricFromSummary(totals.statements),
            files,
        };
    }

    private parseCoverageFinal(data: Record<string, unknown>): CoverageData | undefined {
        const files: CoverageData['files'] = [];
        const totals = {
            lines: { total: 0, covered: 0, percentage: 0 },
            functions: { total: 0, covered: 0, percentage: 0 },
            branches: { total: 0, covered: 0, percentage: 0 },
            statements: { total: 0, covered: 0, percentage: 0 },
        };

        for (const [path, rawCoverage] of Object.entries(data)) {
            const coverage = this.asFinalCoverageRecord(rawCoverage);
            if (!coverage) {
                continue;
            }

            const statementHits = Object.values(coverage.s) as number[];
            const functionHits = Object.values(coverage.f) as number[];
            const branchHits = Object.values(coverage.b).flat() as number[];
            const uncoveredLines = Object.entries(coverage.statementMap)
                .filter(([statementId]) => (coverage.s[statementId] ?? 0) === 0)
                .map(([, location]) => location.start.line);

            const uncoveredFunctions = Object.entries(coverage.fnMap)
                .filter(([fnId]) => (coverage.f[fnId] ?? 0) === 0)
                .map(([, fn]) => fn.name);

            const fileCoverage = {
                path,
                lines: this.metricFromHits(statementHits),
                uncoveredLines,
                functions: this.metricFromHits(functionHits),
                uncoveredFunctions,
            };

            files.push(fileCoverage);
            totals.lines.total += fileCoverage.lines.total;
            totals.lines.covered += fileCoverage.lines.covered;
            totals.functions.total += fileCoverage.functions.total;
            totals.functions.covered += fileCoverage.functions.covered;
            totals.branches.total += branchHits.length;
            totals.branches.covered += branchHits.filter((hit) => hit > 0).length;
            totals.statements.total += statementHits.length;
            totals.statements.covered += statementHits.filter((hit) => hit > 0).length;
        }

        if (files.length === 0) {
            return undefined;
        }

        totals.lines.percentage = this.toPercentage(totals.lines.covered, totals.lines.total);
        totals.functions.percentage = this.toPercentage(totals.functions.covered, totals.functions.total);
        totals.branches.percentage = this.toPercentage(totals.branches.covered, totals.branches.total);
        totals.statements.percentage = this.toPercentage(totals.statements.covered, totals.statements.total);

        return { ...totals, files };
    }

    private metricFromSummary(metric: { total: number; covered?: number; pct: number; skipped?: number }) {
        return {
            total: metric.total,
            covered: metric.covered ?? metric.total - (metric.skipped ?? 0),
            percentage: metric.pct,
        };
    }

    private metricFromHits(hits: number[]) {
        const total = hits.length;
        const covered = hits.filter((hit) => hit > 0).length;
        return {
            total,
            covered,
            percentage: this.toPercentage(covered, total),
        };
    }

    private toPercentage(covered: number, total: number): number {
        return total === 0 ? 100 : Number(((covered / total) * 100).toFixed(2));
    }

    private asCoverageMetricRecord(
        value: unknown
    ): { lines: { total: number; covered?: number; pct: number; skipped?: number }; functions: { total: number; covered?: number; pct: number; skipped?: number }; branches: { total: number; covered?: number; pct: number; skipped?: number }; statements: { total: number; covered?: number; pct: number; skipped?: number } } | undefined {
        if (!value || typeof value !== 'object') {
            return undefined;
        }
        const metrics = value as Record<string, { total: number; covered?: number; pct: number; skipped?: number }>;
        if (!metrics.lines || !metrics.functions || !metrics.branches || !metrics.statements) {
            return undefined;
        }
        return {
            lines: metrics.lines,
            functions: metrics.functions,
            branches: metrics.branches,
            statements: metrics.statements,
        };
    }

    private asFinalCoverageRecord(
        value: unknown
    ): { s: Record<string, number>; f: Record<string, number>; b: Record<string, number[]>; statementMap: Record<string, { start: { line: number } }>; fnMap: Record<string, { name: string }> } | undefined {
        if (!value || typeof value !== 'object') {
            return undefined;
        }
        const coverage = value as Record<string, unknown>;
        if (!coverage.s || !coverage.f || !coverage.b || !coverage.statementMap || !coverage.fnMap) {
            return undefined;
        }
        return coverage as {
            s: Record<string, number>;
            f: Record<string, number>;
            b: Record<string, number[]>;
            statementMap: Record<string, { start: { line: number } }>;
            fnMap: Record<string, { name: string }>;
        };
    }
}

export default StrataReporter;
