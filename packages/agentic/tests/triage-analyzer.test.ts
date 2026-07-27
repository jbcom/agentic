import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { CIStatus, FeedbackItem } from '../src/triage/types.js';

const {
  generateObjectMock,
  generateTextMock,
  getOrLoadProviderMock,
  resolveProviderOptionsMock,
} = vi.hoisted(() => ({
  generateObjectMock: vi.fn(),
  generateTextMock: vi.fn(),
  getOrLoadProviderMock: vi.fn(),
  resolveProviderOptionsMock: vi.fn(),
}));

vi.mock('ai', () => ({
  generateObject: generateObjectMock,
  generateText: generateTextMock,
}));

vi.mock('../src/core/providers.js', () => ({
  getOrLoadProvider: getOrLoadProviderMock,
  resolveProviderOptions: resolveProviderOptionsMock,
}));

vi.mock('../src/core/config.js', () => ({
  getConfig: vi.fn(() => ({ defaultRepository: undefined })),
  log: {
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
    debug: vi.fn(),
  },
}));

import { Analyzer } from '../src/triage/analyzer.js';

function makeFeedback(overrides: Partial<FeedbackItem> = {}): FeedbackItem {
  return {
    id: 'comment-1',
    author: 'reviewer',
    body: 'Please address this.',
    path: 'src/example.ts',
    line: 10,
    severity: 'high',
    status: 'unaddressed',
    createdAt: '2026-04-15T00:00:00.000Z',
    url: 'https://github.com/acme/widgets/pull/42#discussion_r1',
    isAutoResolvable: false,
    suggestedAction: null,
    resolution: null,
    ...overrides,
  };
}

function makeCI(overrides: Partial<CIStatus> = {}): CIStatus {
  return {
    allPassing: true,
    anyPending: false,
    checks: [],
    failures: [],
    ...overrides,
  };
}

function makeGitHub(feedback: FeedbackItem[], ci: CIStatus = makeCI()) {
  return {
    getPR: vi.fn().mockResolvedValue({
      html_url: 'https://github.com/acme/widgets/pull/42',
      title: 'Improve triage workflow',
      merged: false,
      state: 'open',
      mergeable: true,
      mergeable_state: 'clean',
    }),
    getCIStatus: vi.fn().mockResolvedValue(ci),
    collectFeedback: vi.fn().mockResolvedValue(feedback),
    getPRFiles: vi.fn().mockResolvedValue([{ filename: 'src/example.ts' }]),
  };
}

describe('Analyzer PR triage', () => {
  beforeEach(() => {
    generateObjectMock.mockReset();
    generateTextMock.mockReset();
    getOrLoadProviderMock.mockReset();
    resolveProviderOptionsMock.mockReset();

    resolveProviderOptionsMock.mockReturnValue({
      providerName: 'anthropic',
      model: 'test-model',
      apiKey: 'test-key',
    });
    getOrLoadProviderMock.mockResolvedValue(() => ({ provider: 'mock-model' }));
    generateTextMock.mockResolvedValue({ text: 'Summary of current PR state.' });
  });

  it('marks mixed high-severity review feedback as blocked when any item still requires a human', async () => {
    const feedback = [
      makeFeedback({
        id: 'comment-1',
        severity: 'high',
        isAutoResolvable: true,
        suggestedAction: 'const enabled = true;',
      }),
      makeFeedback({
        id: 'comment-2',
        severity: 'critical',
        isAutoResolvable: false,
        suggestedAction: null,
      }),
    ];

    generateObjectMock.mockResolvedValue({
      object: {
        items: [
          {
            id: 'comment-1',
            status: 'unaddressed',
            isAutoResolvable: true,
            suggestedAction: 'const enabled = true;',
          },
          {
            id: 'comment-2',
            status: 'unaddressed',
            isAutoResolvable: false,
            suggestedAction: null,
          },
        ],
      },
    });

    const analyzer = new Analyzer();
    const result = await analyzer.analyzePR(makeGitHub(feedback), 42);

    expect(result.blockers).toContainEqual(
      expect.objectContaining({
        type: 'review_feedback',
        isAutoResolvable: false,
      })
    );
    expect(result.status).toBe('blocked');
    expect(result.nextActions).toEqual([]);
  });

  it('keeps review feedback auto-resolvable only when every critical item can be automated', async () => {
    const feedback = [
      makeFeedback({
        id: 'comment-1',
        severity: 'high',
        isAutoResolvable: true,
        suggestedAction: 'const enabled = true;',
      }),
      makeFeedback({
        id: 'comment-2',
        severity: 'critical',
        isAutoResolvable: true,
        suggestedAction: 'const retries = 3;',
      }),
    ];

    generateObjectMock.mockResolvedValue({
      object: {
        items: [
          {
            id: 'comment-1',
            status: 'unaddressed',
            isAutoResolvable: true,
            suggestedAction: 'const enabled = true;',
          },
          {
            id: 'comment-2',
            status: 'unaddressed',
            isAutoResolvable: true,
            suggestedAction: 'const retries = 3;',
          },
        ],
      },
    });

    const analyzer = new Analyzer();
    const result = await analyzer.analyzePR(makeGitHub(feedback), 42);

    expect(result.blockers).toContainEqual(
      expect.objectContaining({
        type: 'review_feedback',
        isAutoResolvable: true,
      })
    );
    expect(result.status).toBe('needs_work');
    expect(result.nextActions).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          automated: true,
          priority: 'high',
        }),
        expect.objectContaining({
          automated: true,
          priority: 'critical',
        }),
      ])
    );
  });

  it('marks a clean PR as ready to merge and suggests merging next', async () => {
    const analyzer = new Analyzer();
    const result = await analyzer.analyzePR(makeGitHub([]), 42);

    expect(result.blockers).toEqual([]);
    expect(result.status).toBe('ready_to_merge');
    expect(result.nextActions).toEqual([
      {
        action: 'Merge PR',
        priority: 'high',
        automated: false,
        reason: 'All checks pass and feedback addressed',
      },
    ]);
    expect(result.summary).toBe('Summary of current PR state.');
  });
});
