import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { FeedbackItem, TriageResult } from '../src/triage/types.js';

const { anthropicMock, generateTextMock } = vi.hoisted(() => ({
  anthropicMock: vi.fn(() => 'mock-model'),
  generateTextMock: vi.fn(),
}));

vi.mock('@ai-sdk/anthropic', () => ({
  anthropic: anthropicMock,
}));

vi.mock('ai', () => ({
  generateText: generateTextMock,
}));

import { Resolver } from '../src/triage/resolver.js';

function makeFeedback(overrides: Partial<FeedbackItem> = {}): FeedbackItem {
  return {
    id: 'comment-101',
    author: 'reviewer',
    body: 'Please update this line.',
    path: 'src/example.ts',
    line: 2,
    severity: 'medium',
    status: 'unaddressed',
    createdAt: '2026-04-15T00:00:00.000Z',
    url: 'https://github.com/acme/widgets/pull/42#discussion_r101',
    isAutoResolvable: true,
    suggestedAction: null,
    resolution: null,
    ...overrides,
  };
}

function makeTriage(feedbackItems: FeedbackItem[]): TriageResult {
  return {
    prNumber: 42,
    prUrl: 'https://github.com/acme/widgets/pull/42',
    prTitle: 'Improve triage workflow',
    status: 'needs_work',
    ci: {
      allPassing: true,
      anyPending: false,
      checks: [],
      failures: [],
    },
    feedback: {
      total: feedbackItems.length,
      unaddressed: feedbackItems.filter((item) => item.status === 'unaddressed').length,
      items: feedbackItems,
    },
    blockers: [],
    nextActions: [],
    summary: 'Feedback needs to be addressed.',
    timestamp: '2026-04-15T00:00:00.000Z',
  };
}

describe('Resolver feedback application', () => {
  let tempDir: string;
  let github: {
    postComment: ReturnType<typeof vi.fn>;
    replyToComment: ReturnType<typeof vi.fn>;
  };

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), 'agentic-resolver-'));
    await mkdir(join(tempDir, 'src'), { recursive: true });
    await writeFile(
      join(tempDir, 'src/example.ts'),
      'const start = true;\nconst enabled = false;\nconst end = true;\n',
      'utf-8'
    );

    github = {
      postComment: vi.fn().mockResolvedValue(undefined),
      replyToComment: vi.fn().mockResolvedValue(undefined),
    };

    generateTextMock.mockReset();
    anthropicMock.mockClear();
  });

  afterEach(async () => {
    await rm(tempDir, { recursive: true, force: true });
  });

  it('applies GitHub suggestions to the targeted line without replacing the whole file', async () => {
    const resolver = new Resolver({ workingDirectory: tempDir });
    const feedback = makeFeedback({
      suggestedAction: 'const enabled = true;',
    });

    const results = await resolver.resolveFeedback(github as never, makeTriage([feedback]));

    expect(results).toHaveLength(1);
    expect(results[0]).toMatchObject({
      success: true,
      action: 'Apply suggestion',
      changes: [{ file: 'src/example.ts', type: 'modified' }],
    });
    await expect(readFile(join(tempDir, 'src/example.ts'), 'utf-8')).resolves.toBe(
      'const start = true;\nconst enabled = true;\nconst end = true;\n'
    );
    expect(github.postComment).not.toHaveBeenCalled();
    expect(github.replyToComment).not.toHaveBeenCalled();
  });

  it('refuses to apply a suggestion when no target line is available', async () => {
    const resolver = new Resolver({ workingDirectory: tempDir });
    const feedback = makeFeedback({
      line: null,
      suggestedAction: 'const enabled = true;',
    });

    const [result] = await resolver.resolveFeedback(github as never, makeTriage([feedback]));

    expect(result).toMatchObject({
      success: false,
      action: 'Apply suggestion',
      error: 'Cannot safely apply suggestion without a target line',
    });
    await expect(readFile(join(tempDir, 'src/example.ts'), 'utf-8')).resolves.toBe(
      'const start = true;\nconst enabled = false;\nconst end = true;\n'
    );
  });

  it('applies fenced AI fixes locally when a target line is available', async () => {
    generateTextMock.mockResolvedValue({
      text: 'TYPE: fix\nCONTENT: ```ts\nconst enabled = true;\n```',
    });

    const resolver = new Resolver({ workingDirectory: tempDir });
    const feedback = makeFeedback();

    const [result] = await resolver.resolveFeedback(github as never, makeTriage([feedback]));

    expect(result).toMatchObject({
      success: true,
      action: 'Apply fix',
      changes: [{ file: 'src/example.ts', type: 'modified' }],
    });
    await expect(readFile(join(tempDir, 'src/example.ts'), 'utf-8')).resolves.toBe(
      'const start = true;\nconst enabled = true;\nconst end = true;\n'
    );
    expect(github.postComment).not.toHaveBeenCalled();
  });

  it('falls back to posting a comment when an AI fix is not safe to apply locally', async () => {
    generateTextMock.mockResolvedValue({
      text: 'TYPE: fix\nCONTENT: Replace the deprecated helper with the new adapter.',
    });

    const resolver = new Resolver({ workingDirectory: tempDir });
    const feedback = makeFeedback();

    const [result] = await resolver.resolveFeedback(github as never, makeTriage([feedback]));

    expect(result).toMatchObject({
      success: true,
      action: 'Suggest fix',
      changes: null,
    });
    expect(github.postComment).toHaveBeenCalledOnce();
    await expect(readFile(join(tempDir, 'src/example.ts'), 'utf-8')).resolves.toBe(
      'const start = true;\nconst enabled = false;\nconst end = true;\n'
    );
  });

  it('replies directly to inline comments for justification responses', async () => {
    generateTextMock.mockResolvedValue({
      text: 'TYPE: justification\nCONTENT: This behavior is intentional because it preserves compatibility.',
    });

    const resolver = new Resolver({ workingDirectory: tempDir });
    const feedback = makeFeedback({
      id: 'comment-12345',
      path: null,
      line: null,
      suggestedAction: null,
    });

    const [result] = await resolver.resolveFeedback(github as never, makeTriage([feedback]));

    expect(result).toMatchObject({
      success: true,
      action: 'Post justification',
    });
    expect(github.replyToComment).toHaveBeenCalledWith(
      42,
      12345,
      'This behavior is intentional because it preserves compatibility.'
    );
    expect(github.postComment).not.toHaveBeenCalled();
  });
});
