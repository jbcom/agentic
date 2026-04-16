/**
 * Tests for Fleet management - spawn, routing, status, summary, and coordination
 */

import { afterEach, beforeEach, describe, expect, it, type Mock, vi } from 'vitest';
import { Fleet } from '../src/fleet/fleet.js';
import { GitHubClient } from '../src/github/client.js';

type TestableFleet = Fleet & {
  pollCoordinationComments(
    owner: string,
    repo: string,
    config: { repo: string; coordinationPr: number },
    agentIds: Set<string>,
    processedIds: Set<number>
  ): Promise<void>;
};

describe('Fleet Management', () => {
  let fleet: Fleet;
  const originalFetch = global.fetch;

  beforeEach(() => {
    process.env.CURSOR_API_KEY = 'test-key';
    fleet = new Fleet({
      apiKey: 'test-key',
      retryDelay: 1,
    });
    global.fetch = vi.fn();
  });

  afterEach(() => {
    global.fetch = originalFetch;
    delete process.env.CURSOR_API_KEY;
    vi.restoreAllMocks();
  });

  describe('API Availability', () => {
    it('should report API as available when key is provided', () => {
      expect(fleet.isApiAvailable()).toBe(true);
    });

    it('should report API as unavailable when no key is provided', () => {
      delete process.env.CURSOR_API_KEY;
      const noKeyFleet = new Fleet({ apiKey: undefined });
      expect(noKeyFleet.isApiAvailable()).toBe(false);
    });

    it('should return error result when API is unavailable for list', async () => {
      delete process.env.CURSOR_API_KEY;
      const noKeyFleet = new Fleet({ apiKey: undefined });
      const result = await noKeyFleet.list();
      expect(result.success).toBe(false);
      expect(result.error).toContain('Cursor API not available');
    });

    it('should return error result when API is unavailable for spawn', async () => {
      delete process.env.CURSOR_API_KEY;
      const noKeyFleet = new Fleet({ apiKey: undefined });
      const result = await noKeyFleet.spawn({ repository: 'owner/repo', task: 'test' });
      expect(result.success).toBe(false);
      expect(result.error).toContain('Cursor API not available');
    });
  });

  describe('Agent Listing', () => {
    it('should list all agents', async () => {
      const mockAgents = [
        { id: 'bc-1', status: 'RUNNING', source: { repository: 'owner/repo' } },
        { id: 'bc-2', status: 'COMPLETED', source: { repository: 'owner/repo2' } },
      ];

      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: mockAgents })),
      });

      const result = await fleet.list();
      expect(result.success).toBe(true);
      expect(result.data).toHaveLength(2);
    });

    it('should filter agents by status', async () => {
      const mockAgents = [
        { id: 'bc-1', status: 'RUNNING', source: { repository: 'owner/repo' } },
        { id: 'bc-2', status: 'COMPLETED', source: { repository: 'owner/repo2' } },
        { id: 'bc-3', status: 'RUNNING', source: { repository: 'owner/repo3' } },
      ];

      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: mockAgents })),
      });

      const result = await fleet.listByStatus('RUNNING');
      expect(result.success).toBe(true);
      expect(result.data).toHaveLength(2);
      expect(result.data?.every((a) => a.status === 'RUNNING')).toBe(true);
    });

    it('should return only running agents via running() shortcut', async () => {
      const mockAgents = [
        { id: 'bc-1', status: 'RUNNING', source: { repository: 'owner/repo' } },
        { id: 'bc-2', status: 'FAILED', source: { repository: 'owner/repo2' } },
      ];

      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: mockAgents })),
      });

      const result = await fleet.running();
      expect(result.success).toBe(true);
      expect(result.data).toHaveLength(1);
      expect(result.data?.[0]?.id).toBe('bc-1');
    });
  });

  describe('Agent Discovery', () => {
    it('should find an agent by ID', async () => {
      const mockAgents = [
        { id: 'bc-123', status: 'RUNNING', source: { repository: 'owner/repo' } },
        { id: 'bc-456', status: 'COMPLETED', source: { repository: 'owner/repo2' } },
      ];

      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: mockAgents })),
      });

      const result = await fleet.find('bc-123');
      expect(result.success).toBe(true);
      expect(result.data?.id).toBe('bc-123');
    });

    it('should return undefined when agent is not found', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: [] })),
      });

      const result = await fleet.find('nonexistent');
      expect(result.success).toBe(true);
      expect(result.data).toBeUndefined();
    });
  });

  describe('Agent Spawning', () => {
    it('should spawn an agent with basic options', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () =>
          Promise.resolve(
            JSON.stringify({ id: 'bc-new', status: 'RUNNING' })
          ),
      });

      const result = await fleet.spawn({
        repository: 'owner/repo',
        task: 'Fix the bug',
      });

      expect(result.success).toBe(true);
      expect(result.data?.id).toBe('bc-new');
    });

    it('should include coordination context in spawn task', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () =>
          Promise.resolve(
            JSON.stringify({ id: 'bc-ctx', status: 'RUNNING' })
          ),
      });

      await fleet.spawn({
        repository: 'owner/repo',
        task: 'Test task',
        context: {
          controlManagerId: 'cm-1',
          controlCenter: 'https://github.com/owner/control/pull/42',
          relatedAgents: ['bc-a', 'bc-b'],
          metadata: { pattern: 'diamond' },
        },
      });

      const fetchCall = (global.fetch as Mock).mock.calls[0];
      const body = JSON.parse(fetchCall[1].body);
      const promptText = body.prompt.text;

      expect(promptText).toContain('Test task');
      expect(promptText).toContain('COORDINATION CONTEXT');
      expect(promptText).toContain('cm-1');
      expect(promptText).toContain('bc-a, bc-b');
      expect(promptText).toContain('diamond');
    });

    it('should default ref to main when not specified', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () =>
          Promise.resolve(
            JSON.stringify({ id: 'bc-ref', status: 'RUNNING' })
          ),
      });

      await fleet.spawn({
        repository: 'owner/repo',
        task: 'Test task',
      });

      const fetchCall = (global.fetch as Mock).mock.calls[0];
      const body = JSON.parse(fetchCall[1].body);
      expect(body.source.ref).toBe('main');
    });
  });

  describe('Agent Communication', () => {
    it('should send a followup message', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve('{}'),
      });

      const result = await fleet.followup('bc-123', 'Continue working');
      expect(result.success).toBe(true);
    });

    it('should broadcast a message to multiple agents', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve('{}'),
      });

      const results = await fleet.broadcast(
        ['bc-1', 'bc-2', 'bc-3'],
        'Status update please'
      );

      expect(results.size).toBe(3);
      for (const [_id, result] of results) {
        expect(result.success).toBe(true);
      }
    });

    it('should return error for followup when API unavailable', async () => {
      delete process.env.CURSOR_API_KEY;
      const noKeyFleet = new Fleet({ apiKey: undefined });
      const result = await noKeyFleet.followup('bc-123', 'test');
      expect(result.success).toBe(false);
      expect(result.error).toContain('Cursor API not available');
    });
  });

  describe('Fleet Summary', () => {
    it('should return a fleet summary with correct counts', async () => {
      const mockAgents = [
        { id: 'bc-1', status: 'RUNNING', source: { repository: 'owner/repo' } },
        { id: 'bc-2', status: 'COMPLETED', source: { repository: 'owner/repo' } },
        { id: 'bc-3', status: 'FAILED', source: { repository: 'owner/repo' } },
        { id: 'bc-4', status: 'RUNNING', source: { repository: 'owner/repo' } },
        { id: 'bc-5', status: 'FINISHED', source: { repository: 'owner/repo' } },
      ];

      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: mockAgents })),
      });

      const result = await fleet.summary();
      expect(result.success).toBe(true);
      expect(result.data?.total).toBe(5);
      expect(result.data?.running).toBe(2);
      expect(result.data?.completed).toBe(2); // COMPLETED + FINISHED
      expect(result.data?.failed).toBe(1);
    });

    it('should handle empty fleet summary', async () => {
      (global.fetch as Mock).mockResolvedValue({
        ok: true,
        status: 200,
        headers: { get: () => 'application/json' },
        text: () => Promise.resolve(JSON.stringify({ agents: [] })),
      });

      const result = await fleet.summary();
      expect(result.success).toBe(true);
      expect(result.data?.total).toBe(0);
      expect(result.data?.running).toBe(0);
    });
  });

  describe('GitHub Coordination', () => {
    it('processes DONE coordination comments even without an @cursor mention', async () => {
      const listPRCommentsSpy = vi
        .spyOn(GitHubClient, 'listPRComments')
        .mockResolvedValue({
          success: true,
          data: [
            {
              id: 101,
              body: '✅ DONE: bc-123456789abc shipped the fix',
              author: 'agent-1',
              createdAt: '2026-04-15T23:00:00Z',
              updatedAt: '2026-04-15T23:00:10Z',
            },
          ],
        });
      const postPRCommentSpy = vi
        .spyOn(GitHubClient, 'postPRComment')
        .mockResolvedValue({ success: true });

      const agentIds = new Set(['bc-123456789abc']);
      const processedIds = new Set<number>();

      await (fleet as TestableFleet).pollCoordinationComments(
        'owner',
        'repo',
        { repo: 'owner/repo', coordinationPr: 42 },
        agentIds,
        processedIds
      );

      expect(listPRCommentsSpy).toHaveBeenCalledWith('owner', 'repo', 42);
      expect(agentIds.has('bc-123456789abc')).toBe(false);
      expect(processedIds.has(101)).toBe(true);
      expect(postPRCommentSpy).toHaveBeenCalledWith(
        'owner',
        'repo',
        42,
        '✅ Acknowledged completion from bc-123456789. Summary: shipped the fix'
      );
    });

    it('processes BLOCKED coordination comments even without an @cursor mention', async () => {
      vi.spyOn(GitHubClient, 'listPRComments').mockResolvedValue({
        success: true,
        data: [
          {
            id: 102,
            body: '⚠️ BLOCKED: bc-blocked01 waiting on maintainer review',
            author: 'agent-2',
            createdAt: '2026-04-15T23:10:00Z',
            updatedAt: '2026-04-15T23:10:10Z',
          },
        ],
      });
      const postPRCommentSpy = vi
        .spyOn(GitHubClient, 'postPRComment')
        .mockResolvedValue({ success: true });

      const agentIds = new Set(['bc-blocked01']);
      const processedIds = new Set<number>();

      await (fleet as TestableFleet).pollCoordinationComments(
        'owner',
        'repo',
        { repo: 'owner/repo', coordinationPr: 42 },
        agentIds,
        processedIds
      );

      expect(agentIds.has('bc-blocked01')).toBe(true);
      expect(processedIds.has(102)).toBe(true);
      expect(postPRCommentSpy).toHaveBeenCalledWith(
        'owner',
        'repo',
        42,
        '⚠️ Agent bc-blocked01 blocked: waiting on maintainer review\n\nManual intervention may be required.'
      );
    });

    it('ignores unrelated comments while still marking them as processed', async () => {
      vi.spyOn(GitHubClient, 'listPRComments').mockResolvedValue({
        success: true,
        data: [
          {
            id: 103,
            body: 'Regular coordination chatter with no control signal.',
            author: 'maintainer-1',
            createdAt: '2026-04-15T23:20:00Z',
            updatedAt: '2026-04-15T23:20:10Z',
          },
        ],
      });
      const postPRCommentSpy = vi
        .spyOn(GitHubClient, 'postPRComment')
        .mockResolvedValue({ success: true });

      const agentIds = new Set(['bc-unchanged']);
      const processedIds = new Set<number>();

      await (fleet as TestableFleet).pollCoordinationComments(
        'owner',
        'repo',
        { repo: 'owner/repo', coordinationPr: 42 },
        agentIds,
        processedIds
      );

      expect(agentIds.has('bc-unchanged')).toBe(true);
      expect(processedIds.has(103)).toBe(true);
      expect(postPRCommentSpy).not.toHaveBeenCalled();
    });
  });
});
