/**
 * Vitest Reporter for Strata Triage
 *
 * Generates test reports in Strata's custom format for AI analysis.
 *
 * Usage in vitest.config.ts:
 * ```ts
 * import { StrataReporter } from '@strata/triage/reporters/vitest';
 *
 * export default defineConfig({
 *   test: {
 *     reporters: ['default', new StrataReporter()],
 *   },
 * });
 * ```
 */

import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import type { File, Reporter, Task, Vitest } from 'vitest';
import type { CoverageData, TestError, TestFile, TestReport, TestResult } from '../test-results.js';

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

    async onFinished(files?: File[], _errors?: unknown[]): Promise<void> {
        if (!files) return;

        const report = this.buildReport(files);

        // Write report
        const outputPath = resolve(this.options.outputFile);
        mkdirSync(dirname(outputPath), { recursive: true });
        writeFileSync(outputPath, JSON.stringify(report, null, 2));

        console.log(`\n📊 Strata test report: ${outputPath}`);
    }

    private buildReport(files: File[]): TestReport {
        const testFiles = files.map((f) => this.processFile(f));
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

    private processFile(file: File): TestFile {
        const tests = this.collectTests(file.tasks, file.filepath);

        return {
            path: file.filepath,
            tests,
            duration: file.result?.duration ?? 0,
            setupError: file.result?.errors?.[0] ? this.processError(file.result.errors[0]) : undefined,
        };
    }

    private collectTests(tasks: Task[], filepath: string, prefix = ''): TestResult[] {
        const results: TestResult[] = [];

        for (const task of tasks) {
            const fullName = prefix ? `${prefix} > ${task.name}` : task.name;

            if (task.type === 'test') {
                results.push({
                    id: task.id,
                    name: task.name,
                    fullName,
                    file: filepath,
                    line: task.location?.line,
                    status: this.mapStatus(task.result?.state),
                    duration: task.result?.duration ?? 0,
                    error: task.result?.errors?.[0] ? this.processError(task.result.errors[0]) : undefined,
                    retry: task.result?.retryCount,
                });
            } else if (task.type === 'suite' && task.tasks) {
                results.push(...this.collectTests(task.tasks, filepath, fullName));
            }
        }

        return results;
    }

    private mapStatus(state?: string): TestResult['status'] {
        switch (state) {
            case 'pass':
                return 'passed';
            case 'fail':
                return 'failed';
            case 'skip':
                return 'skipped';
            case 'todo':
                return 'todo';
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
        // Coverage data would be read from the coverage output
        // This is a placeholder - actual implementation would parse c8/istanbul output
        return undefined;
    }
}

export default StrataReporter;
