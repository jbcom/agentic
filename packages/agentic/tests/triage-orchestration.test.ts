import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { ActionResult, TriageResult } from '../src/triage/types.js';

const { analyzerInstance, githubInstance, resolverInstance } = vi.hoisted(() => ({
  analyzerInstance: {
    analyzePR: vi.fn(),
  },
  githubInstance: {
    getCIStatus: vi.fn(),
    postComment: vi.fn(),
  },
  resolverInstance: {
    resolveBlockers: vi.fn(),
    resolveFeedback: vi.fn(),
    commitAndPush: vi.fn(),
  },
}));

vi.mock('../src/github/client.js', () => ({
  GitHubClient: vi.fn(function MockGitHubClient() {
    return githubInstance;
  }),
}));

vi.mock('../src/triage/analyzer.js', () => ({
  Analyzer: vi.fn(function MockAnalyzer() {
    return analyzerInstance;
  }),
}));

vi.mock('../src/triage/resolver.js', () => ({
  Resolver: vi.fn(function MockResolver() {
    return resolverInstance;
  }),
}));

import { Triage } from '../src/triage/triage.js';

function makeTriageResult(overrides: Partial<TriageResult> = {}): TriageResult {
  return {
    prNumber: 42,
    prUrl: 'https://github.com/acme/widgets/pull/42',
    prTitle: 'Improve triage workflow',
    status: 'needs_work',
    ci: {
      allPassing: false,
      anyPending: false,
      checks: [],
      failures: [],
    },
    feedback: {
      total: 0,
      unaddressed: 0,
      items: [],
    },
    blockers: [],
    nextActions: [],
    summary: 'PR needs additional work.',
    timestamp: '2026-04-15T00:00:00.000Z',
    ...overrides,
  };
}

function makeActionResult(overrides: Partial<ActionResult> = {}): ActionResult {
  return {
    success: true,
    action: 'Apply fix',
    description: 'Applied an automated fix.',
    error: null,
    changes: null,
    commitSha: null,
    ...overrides,
  };
}

describe('Triage orchestration', () => {
  beforeEach(() => {
    vi.useRealTimers();
    vi.clearAllMocks();

    analyzerInstance.analyzePR.mockReset();
    githubInstance.getCIStatus.mockReset();
    githubInstance.postComment.mockReset();
    resolverInstance.resolveBlockers.mockReset();
    resolverInstance.resolveFeedback.mockReset();
    resolverInstance.commitAndPush.mockReset();

    githubInstance.getCIStatus.mockResolvedValue({
      allPassing: true,
      anyPending: false,
      checks: [],
      failures: [],
    });
    githubInstance.postComment.mockResolvedValue(undefined);
    resolverInstance.resolveBlockers.mockResolvedValue([]);
    resolverInstance.resolveFeedback.mockResolvedValue([]);
    resolverInstance.commitAndPush.mockResolvedValue(undefined);
  });

  it('creates coherent plan dependencies for CI-only remediation', async () => {
    analyzerInstance.analyzePR.mockResolvedValue(
      makeTriageResult({
        blockers: [
          {
            type: 'ci_failure',
            description: 'CI check "unit" failed',
            isAutoResolvable: true,
            suggestedFix: 'Fix the failing unit test',
            url: 'https://ci.example/unit',
            resolved: false,
          },
        ],
      })
    );

    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });

    const plan = await triage.plan(42);

    expect(plan.steps).toEqual([
      {
        order: 1,
        action: 'Fix CI failure',
        description: 'CI check "unit" failed',
        automated: true,
        estimatedDuration: '5-10 minutes',
        dependencies: [],
      },
      {
        order: 2,
        action: 'Request re-review',
        description: 'Request AI reviewers to re-review changes',
        automated: true,
        estimatedDuration: '1-5 minutes',
        dependencies: [1],
      },
      {
        order: 3,
        action: 'Wait for CI',
        description: 'Wait for all CI checks to complete',
        automated: true,
        estimatedDuration: '5-15 minutes',
        dependencies: [1, 2],
      },
      {
        order: 4,
        action: 'Merge PR',
        description: 'Merge the PR once all checks pass',
        automated: false,
        estimatedDuration: '1 minute',
        dependencies: [3],
      },
    ]);
  });

  it('returns a merge-only plan when the PR is already ready', async () => {
    analyzerInstance.analyzePR.mockResolvedValue(
      makeTriageResult({
        status: 'ready_to_merge',
        ci: {
          allPassing: true,
          anyPending: false,
          checks: [],
          failures: [],
        },
        summary: 'Ready to merge.',
      })
    );

    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });

    const plan = await triage.plan(42);

    expect(plan.steps).toEqual([
      {
        order: 1,
        action: 'Merge PR',
        description: 'Merge the PR once all checks pass',
        automated: false,
        estimatedDuration: '1 minute',
        dependencies: [],
      },
    ]);
  });

  it('returns no plan steps for merged PRs', async () => {
    analyzerInstance.analyzePR.mockResolvedValue(
      makeTriageResult({
        status: 'merged',
        ci: {
          allPassing: true,
          anyPending: false,
          checks: [],
          failures: [],
        },
      })
    );

    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });

    const plan = await triage.plan(42);

    expect(plan.steps).toEqual([]);
    expect(plan.estimatedTotalDuration).toBe('0 minutes');
    expect(plan.requiresHumanIntervention).toBe(false);
    expect(plan.humanInterventionReason).toBeNull();
  });

  it('aggregates blocker and feedback actions during resolve', async () => {
    const current = makeTriageResult();
    const updated = makeTriageResult({
      status: 'ready_to_merge',
      ci: {
        allPassing: true,
        anyPending: false,
        checks: [],
        failures: [],
      },
      summary: 'All blockers resolved.',
    });

    analyzerInstance.analyzePR.mockResolvedValueOnce(current).mockResolvedValueOnce(updated);
    resolverInstance.resolveBlockers.mockResolvedValue([
      makeActionResult({ action: 'Fix CI failure' }),
    ]);
    resolverInstance.resolveFeedback.mockResolvedValue([
      makeActionResult({ action: 'Reply to reviewer' }),
    ]);

    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });

    const result = await triage.resolve(42);

    expect(result.triage).toEqual(updated);
    expect(result.actions).toEqual([
      makeActionResult({ action: 'Fix CI failure' }),
      makeActionResult({ action: 'Reply to reviewer' }),
    ]);
    expect(resolverInstance.resolveBlockers).toHaveBeenCalledOnce();
    expect(resolverInstance.resolveFeedback).toHaveBeenCalledOnce();
  });

  it('waits for CI and then succeeds once the PR is ready', async () => {
    analyzerInstance.analyzePR
      .mockResolvedValueOnce(makeTriageResult({ status: 'needs_ci' }))
      .mockResolvedValueOnce(
        makeTriageResult({
          status: 'ready_to_merge',
          ci: {
            allPassing: true,
            anyPending: false,
            checks: [],
            failures: [],
          },
        })
      );

    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });
    const waitSpy = vi.spyOn(triage as never, 'waitForCI').mockResolvedValue(undefined);

    const result = await triage.runUntilReady(42, { maxIterations: 2 });

    expect(waitSpy).toHaveBeenCalledWith(42);
    expect(resolverInstance.resolveBlockers).not.toHaveBeenCalled();
    expect(result.success).toBe(true);
    expect(result.iterations).toBe(2);
    expect(result.finalTriage.status).toBe('ready_to_merge');
  });

  it('commits changes produced during iterative resolution', async () => {
    vi.useFakeTimers();
    analyzerInstance.analyzePR
      .mockResolvedValueOnce(makeTriageResult({ status: 'needs_work' }))
      .mockResolvedValueOnce(
        makeTriageResult({
          status: 'ready_to_merge',
          ci: {
            allPassing: true,
            anyPending: false,
            checks: [],
            failures: [],
          },
        })
      );

    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });
    vi.spyOn(triage, 'resolve').mockResolvedValue({
      triage: makeTriageResult({
        status: 'ready_to_merge',
        ci: {
          allPassing: true,
          anyPending: false,
          checks: [],
          failures: [],
        },
      }),
      actions: [
        makeActionResult({
          changes: [{ file: 'src/triage.ts', type: 'modified' }],
        }),
      ],
    });

    const runPromise = triage.runUntilReady(42, { maxIterations: 2 });
    await vi.advanceTimersByTimeAsync(5000);
    const result = await runPromise;

    expect(resolverInstance.commitAndPush).toHaveBeenCalledWith('fix: address feedback (iteration 1)');
    expect(result.success).toBe(true);
    expect(result.iterations).toBe(2);
  });

  it('posts both AI review commands', async () => {
    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });

    await triage.requestReviews(42);

    expect(githubInstance.postComment).toHaveBeenNthCalledWith(1, 42, '/gemini review');
    expect(githubInstance.postComment).toHaveBeenNthCalledWith(2, 42, '/q review');
  });

  it('formats a triage report with CI, feedback, blockers, and summary details', () => {
    const triage = new Triage({
      github: {} as never,
      resolver: { workingDirectory: '/tmp' },
    });

    const report = triage.formatTriageReport(
      makeTriageResult({
        blockers: [
          {
            type: 'ci_failure',
            description: 'CI check "lint" failed',
            isAutoResolvable: true,
            suggestedFix: 'Run the formatter',
            url: 'https://ci.example/lint',
            resolved: false,
          },
        ],
        ci: {
          allPassing: false,
          anyPending: false,
          checks: [
            {
              name: 'lint',
              status: 'failure',
              conclusion: 'failure',
              url: 'https://ci.example/lint',
              startedAt: null,
              completedAt: null,
            },
          ],
          failures: [
            {
              name: 'lint',
              status: 'failure',
              conclusion: 'failure',
              url: 'https://ci.example/lint',
              startedAt: null,
              completedAt: null,
            },
          ],
        },
        feedback: {
          total: 1,
          unaddressed: 1,
          items: [
            {
              id: 'feedback-1',
              author: 'reviewer',
              body: 'Please update the failing lint rule.',
              path: 'src/triage.ts',
              line: 12,
              severity: 'high',
              status: 'unaddressed',
              createdAt: '2026-04-15T00:00:00.000Z',
              url: 'https://github.com/acme/widgets/pull/42#discussion_r1',
              isAutoResolvable: true,
              suggestedAction: 'Update the lint rule',
              resolution: null,
            },
          ],
        },
        nextActions: [
          {
            action: 'Fix lint failure',
            priority: 'critical',
            automated: true,
            reason: 'CI must pass before merge',
          },
        ],
      })
    );

    expect(report).toContain('Triage Report: PR #42');
    expect(report).toContain('❌ 1 failing checks:');
    expect(report).toContain('https://ci.example/lint');
    expect(report).toContain('### Unaddressed Items:');
    expect(report).toContain('✅ Auto-resolvable');
    expect(report).toContain('## Summary');
    expect(report).toContain('PR needs additional work.');
  });
});
