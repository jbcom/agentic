import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { PRAnalysis } from '../src/triage/pr-triage-agent.js';

const {
  agentInstances,
  anthropicMock,
  closeMCPClientsMock,
  generateObjectMock,
  generateTextMock,
  getMCPToolsMock,
  initializeMCPClientsMock,
  stepCountIsMock,
} = vi.hoisted(() => ({
  agentInstances: [] as Array<{
    initialize: ReturnType<typeof vi.fn>;
    close: ReturnType<typeof vi.fn>;
    execute: ReturnType<typeof vi.fn>;
  }>,
  anthropicMock: vi.fn((model: string) => ({ model })),
  closeMCPClientsMock: vi.fn(),
  generateObjectMock: vi.fn(),
  generateTextMock: vi.fn(),
  getMCPToolsMock: vi.fn(),
  initializeMCPClientsMock: vi.fn(),
  stepCountIsMock: vi.fn((count: number) => ({ count })),
}));

vi.mock('@ai-sdk/anthropic', () => ({
  anthropic: anthropicMock,
}));

vi.mock('ai', () => ({
  generateObject: generateObjectMock,
  generateText: generateTextMock,
  stepCountIs: stepCountIsMock,
}));

vi.mock('../src/triage/mcp-clients.js', () => ({
  closeMCPClients: closeMCPClientsMock,
  getMCPTools: getMCPToolsMock,
  initializeMCPClients: initializeMCPClientsMock,
}));

vi.mock('../src/triage/agent.js', () => ({
  Agent: vi.fn(function MockAgent() {
    const instance = {
      initialize: vi.fn().mockResolvedValue(undefined),
      close: vi.fn().mockResolvedValue(undefined),
      execute: vi.fn().mockResolvedValue({
        success: true,
        result: 'ok',
        steps: [],
      }),
    };
    agentInstances.push(instance);
    return instance;
  }),
}));

import { PRTriageAgent, triagePR } from '../src/triage/pr-triage-agent.js';

function makeAnalysis(overrides: Partial<PRAnalysis> = {}): PRAnalysis {
  return {
    prNumber: 42,
    prUrl: 'https://github.com/acme/widgets/pull/42',
    prTitle: 'Improve triage workflow',
    status: 'needs_work',
    ci: {
      status: 'failing',
      checks: [
        {
          name: 'unit',
          status: 'failure',
          url: 'https://ci.example/unit',
          summary: 'Unit tests failed',
        },
      ],
      failureReasons: ['Unit tests failed'],
    },
    feedback: {
      total: 2,
      unaddressed: 1,
      critical: 1,
      items: [
        {
          id: 'comment-1',
          source: 'gemini-code-assist',
          severity: 'critical',
          category: 'bug',
          file: 'src/example.ts',
          line: 10,
          content: 'This logic can throw when retries is undefined.',
          suggestion: 'Guard retries before decrementing.',
          addressed: false,
        },
      ],
    },
    blockers: [
      {
        type: 'ci_failure',
        description: 'Unit tests failed',
        autoResolvable: true,
        suggestedFix: 'Fix the failing test or implementation.',
      },
    ],
    summary: 'The PR still needs fixes before it can merge.',
    nextActions: [
      {
        action: 'Fix CI failure',
        priority: 'critical',
        automated: true,
        reason: 'CI must pass before merge',
      },
    ],
    ...overrides,
  };
}

describe('PRTriageAgent', () => {
  beforeEach(() => {
    agentInstances.length = 0;
    anthropicMock.mockClear();
    closeMCPClientsMock.mockReset();
    generateObjectMock.mockReset();
    generateTextMock.mockReset();
    getMCPToolsMock.mockReset();
    initializeMCPClientsMock.mockReset();
    stepCountIsMock.mockClear();
  });

  it('analyzes a PR using gathered MCP context', async () => {
    const mcpClients = { github: { close: vi.fn() } };
    initializeMCPClientsMock.mockResolvedValue(mcpClients);
    getMCPToolsMock.mockResolvedValue({
      github_get_pull_request: { description: 'mock tool' },
    });
    generateTextMock.mockResolvedValue({
      text: 'Gathered PR details, CI status, review comments, and recent commits.',
    });
    generateObjectMock.mockResolvedValue({
      object: makeAnalysis({ status: 'waiting_review' }),
    });

    const agent = new PRTriageAgent({
      repository: 'acme/widgets',
      mcp: {},
    });

    const result = await agent.analyze(42);

    expect(result.status).toBe('waiting_review');
    expect(agentInstances[0]?.initialize).toHaveBeenCalledOnce();
    expect(getMCPToolsMock).toHaveBeenCalledWith(mcpClients);
    expect(generateTextMock).toHaveBeenCalledWith(
      expect.objectContaining({
        tools: {
          github_get_pull_request: { description: 'mock tool' },
        },
      })
    );
    expect(generateObjectMock).toHaveBeenCalledWith(
      expect.objectContaining({
        prompt: expect.stringContaining('Gathered PR details'),
      })
    );
  });

  it('reinitializes MCP clients after close', async () => {
    const firstClients = { github: { close: vi.fn() } };
    const secondClients = { github: { close: vi.fn() } };
    initializeMCPClientsMock.mockResolvedValueOnce(firstClients).mockResolvedValueOnce(secondClients);

    const agent = new PRTriageAgent({
      repository: 'acme/widgets',
      mcp: {},
    });

    await agent.initialize();
    await agent.close();
    await agent.initialize();

    expect(initializeMCPClientsMock).toHaveBeenCalledTimes(2);
    expect(closeMCPClientsMock).toHaveBeenCalledOnce();
    expect(closeMCPClientsMock).toHaveBeenCalledWith(firstClients);
    expect(agentInstances[0]?.initialize).toHaveBeenCalledTimes(2);
    expect(agentInstances[0]?.close).toHaveBeenCalledOnce();
  });

  it('shares a single initialization across concurrent callers', async () => {
    let resolveInitialization:
      | ((value: { github: { close: ReturnType<typeof vi.fn> } }) => void)
      | undefined;
    initializeMCPClientsMock.mockReturnValue(
      new Promise<{ github: { close: ReturnType<typeof vi.fn> } }>((resolve) => {
        resolveInitialization = resolve;
      })
    );

    const agent = new PRTriageAgent({
      repository: 'acme/widgets',
      mcp: {},
    });

    const firstCall = agent.initialize();
    const secondCall = agent.initialize();
    await Promise.resolve();

    expect(initializeMCPClientsMock).toHaveBeenCalledTimes(1);
    expect(agentInstances[0]?.initialize).toHaveBeenCalledTimes(1);

    resolveInitialization?.({ github: { close: vi.fn() } });
    await Promise.all([firstCall, secondCall]);
  });

  it('runs the ready workflow and requests reviews before merging', async () => {
    const agent = new PRTriageAgent({
      repository: 'acme/widgets',
    });
    vi.spyOn(agent, 'analyze').mockResolvedValue(
      makeAnalysis({
        status: 'ready',
        ci: {
          status: 'passing',
          checks: [{ name: 'unit', status: 'success' }],
          failureReasons: [],
        },
        feedback: {
          total: 0,
          unaddressed: 0,
          critical: 0,
          items: [],
        },
        blockers: [],
        nextActions: [{ action: 'Merge PR', priority: 'high', automated: false, reason: 'Ready' }],
        summary: 'The PR is ready to merge.',
      })
    );

    const result = await agent.runUntilReady(42, {
      requestReviews: true,
      autoMerge: true,
    });

    expect(result).toMatchObject({
      success: true,
      finalStatus: 'ready',
      iterations: 1,
    });
    expect(agentInstances[0]?.execute).toHaveBeenNthCalledWith(
      1,
      'Request reviews for PR #42 if not already requested'
    );
    expect(agentInstances[0]?.execute).toHaveBeenNthCalledWith(
      2,
      'Merge PR #42 using squash merge and delete the branch'
    );
    expect(result.report).toContain('READY');
    expect(result.report).toContain('Merge PR');
  });

  it('stops when only non-auto-resolvable blockers remain', async () => {
    const agent = new PRTriageAgent({
      repository: 'acme/widgets',
    });
    vi.spyOn(agent, 'analyze').mockResolvedValue(
      makeAnalysis({
        status: 'blocked',
        ci: {
          status: 'unknown',
          checks: [],
          failureReasons: [],
        },
        blockers: [
          {
            type: 'merge_conflict',
            description: 'PR has merge conflicts',
            autoResolvable: false,
            suggestedFix: 'Rebase onto main and resolve conflicts manually.',
          },
        ],
        nextActions: [],
        summary: 'Manual conflict resolution is required.',
      })
    );
    const resolveSpy = vi.spyOn(agent, 'resolve');

    const result = await agent.runUntilReady(42);

    expect(result).toMatchObject({
      success: false,
      finalStatus: 'blocked',
      iterations: 1,
    });
    expect(result.report).toContain('Cannot Auto-Resolve');
    expect(result.report).toContain('PR has merge conflicts');
    expect(resolveSpy).not.toHaveBeenCalled();
  });

  it('always closes the agent in triagePR, even when analysis fails', async () => {
    const analyzeSpy = vi
      .spyOn(PRTriageAgent.prototype, 'analyze')
      .mockRejectedValueOnce(new Error('analysis failed'));
    const closeSpy = vi.spyOn(PRTriageAgent.prototype, 'close').mockResolvedValueOnce();

    await expect(triagePR('acme/widgets', 42)).rejects.toThrow('analysis failed');
    expect(closeSpy).toHaveBeenCalledOnce();

    analyzeSpy.mockRestore();
    closeSpy.mockRestore();
  });
});
