/**
 * Tests for the Station-to-Station Handoff Protocol
 *
 * Focuses on validation logic, branch name checking, and protocol behavior
 * without requiring live API connections.
 */

import { beforeEach, describe, expect, it, vi } from 'vitest';

const {
  spawnSyncMock,
  getRepoMock,
  getEnvForRepoMock,
  existsSyncMock,
  mkdirSyncMock,
  writeFileSyncMock,
  cursorApiGetAgentConversationMock,
  cursorApiLaunchAgentMock,
  cursorApiGetAgentStatusMock,
  cursorApiAddFollowupMock,
} = vi.hoisted(() => ({
  spawnSyncMock: vi.fn(),
  getRepoMock: vi.fn(),
  getEnvForRepoMock: vi.fn(() => ({})),
  existsSyncMock: vi.fn(),
  mkdirSyncMock: vi.fn(),
  writeFileSyncMock: vi.fn(),
  cursorApiGetAgentConversationMock: vi.fn(),
  cursorApiLaunchAgentMock: vi.fn(),
  cursorApiGetAgentStatusMock: vi.fn(),
  cursorApiAddFollowupMock: vi.fn(),
}));

// Mock config first (before any imports that use it)
vi.mock('../src/core/config.js', () => ({
  log: {
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
    debug: vi.fn(),
  },
  getConfig: vi.fn().mockReturnValue({ defaultRepository: undefined }),
}));

vi.mock('node:child_process', () => ({
  spawnSync: spawnSyncMock,
}));

vi.mock('node:fs', () => ({
  existsSync: existsSyncMock,
  mkdirSync: mkdirSyncMock,
  writeFileSync: writeFileSyncMock,
}));

vi.mock('../src/core/tokens.js', () => ({
  getEnvForRepo: getEnvForRepoMock,
}));

vi.mock('../src/github/client.js', () => ({
  GitHubClient: {
    getRepo: getRepoMock,
  },
}));

vi.mock('../src/fleet/cursor-api.js', () => ({
  CursorAPI: class MockCursorAPI {
    constructor(options: { apiKey?: string } = {}) {
      if (!(options.apiKey ?? process.env.CURSOR_API_KEY)) {
        throw new Error('CURSOR_API_KEY is required');
      }
    }

    getAgentConversation = cursorApiGetAgentConversationMock;
    launchAgent = cursorApiLaunchAgentMock;
    getAgentStatus = cursorApiGetAgentStatusMock;
    addFollowup = cursorApiAddFollowupMock;
  },
}));

describe('Handoff Protocol', () => {
  beforeEach(() => {
    spawnSyncMock.mockReset();
    spawnSyncMock.mockImplementation((command: string, args: string[]) => {
      if (command === 'git' && args[0] === 'status') {
        return { status: 0, stdout: '', stderr: '' };
      }

      if (command === 'git' && args[0] === 'rev-parse') {
        return { status: 1, stdout: '', stderr: '' };
      }

      return { status: 0, stdout: '', stderr: '' };
    });
    getRepoMock.mockReset();
    getRepoMock.mockResolvedValue({
      success: true,
      data: {
        defaultBranch: 'main',
      },
    });
    getEnvForRepoMock.mockClear();
    existsSyncMock.mockReset();
    existsSyncMock.mockReturnValue(false);
    mkdirSyncMock.mockReset();
    writeFileSyncMock.mockReset();
    cursorApiGetAgentConversationMock.mockReset();
    cursorApiGetAgentConversationMock.mockImplementation(async (agentId: string) => ({
      success: true,
      data: {
        agentId,
        messages: agentId === 'bc-succ' ? [{ type: 'assistant_message', text: 'HANDOFF CONFIRMED' }] : [],
        totalMessages: agentId === 'bc-succ' ? 1 : 0,
      },
    }));
    cursorApiLaunchAgentMock.mockReset();
    cursorApiLaunchAgentMock.mockResolvedValue({
      success: true,
      data: { id: 'bc-succ' },
    });
    cursorApiGetAgentStatusMock.mockReset();
    cursorApiGetAgentStatusMock.mockResolvedValue({
      success: true,
      data: { status: 'RUNNING' },
    });
    cursorApiAddFollowupMock.mockReset();
  });

  describe('Branch Name Validation', () => {
    // The validation logic from manager.ts
    const isValidBranchName = (branch: string): boolean =>
      /^[a-zA-Z0-9._/-]+$/.test(branch) && branch.length <= 200;

    it('should accept valid branch names', () => {
      const validNames = [
        'main',
        'feature/my-branch',
        'fix/issue-123',
        'release/v1.0.0',
        'successor/continue-work-20240101',
        'user.name/branch',
        'a'.repeat(200),
      ];

      for (const name of validNames) {
        expect(isValidBranchName(name), `Expected "${name}" to be valid`).toBe(true);
      }
    });

    it('should reject invalid branch names', () => {
      const invalidNames = [
        'branch with spaces',
        'branch;injection',
        'branch$(command)',
        'branch`cmd`',
        'branch|pipe',
        'branch&&chain',
        '',
        'a'.repeat(201),
      ];

      for (const name of invalidNames) {
        expect(isValidBranchName(name), `Expected "${name}" to be invalid`).toBe(false);
      }
    });
  });

  describe('HandoffManager Construction', () => {
    it('should create without any arguments', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      // This will have null API since no CURSOR_API_KEY
      const manager = new HandoffManager();
      expect(manager).toBeDefined();
    });

    it('should allow setting repo after construction', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager();
      manager.setRepo('owner/repo');
      expect(manager).toBeDefined();
    });
  });

  describe('Takeover Validation (without API)', () => {
    it('should reject when repository is not set', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager();

      const result = await manager.takeover('bc-pred', 42, 'valid-branch');

      expect(result.success).toBe(false);
      expect(result.error).toContain('Repository is required');
    });

    it('should reject invalid branch names in takeover', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const result = await manager.takeover(
        'bc-pred',
        42,
        'branch with spaces'
      );

      expect(result.success).toBe(false);
      expect(result.error).toContain('Invalid branch name');
    });

    it('should reject when admin and auto are both set', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const result = await manager.takeover('bc-pred', 42, 'valid-branch', {
        admin: true,
        auto: true,
      });

      expect(result.success).toBe(false);
      expect(result.error).toContain('Cannot use --admin and --auto simultaneously');
    });

    it('uses the repository default branch instead of assuming main', async () => {
      getRepoMock.mockResolvedValue({
        success: true,
        data: {
          defaultBranch: 'master',
        },
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const result = await manager.takeover('bc-pred', 42, 'successor/continue-work');

      expect(result.success).toBe(true);
      expect(getRepoMock).toHaveBeenCalledWith('owner', 'repo');
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        1,
        'git',
        ['status', '--porcelain'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        2,
        'git',
        ['rev-parse', '--verify', '--quiet', 'refs/heads/successor/continue-work'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        3,
        'gh',
        ['pr', 'merge', '42', '--squash', '--repo', 'owner/repo', '--delete-branch'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        4,
        'git',
        ['checkout', 'master'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        5,
        'git',
        ['pull', '--ff-only', 'origin', 'master'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        6,
        'git',
        ['checkout', '-b', 'successor/continue-work'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
    });

    it('fails before merging when the default branch cannot be resolved', async () => {
      getRepoMock.mockResolvedValue({
        success: false,
        error: 'No token available for this repository',
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const result = await manager.takeover('bc-pred', 42, 'successor/continue-work');

      expect(result.success).toBe(false);
      expect(result.error).toContain('Failed to resolve default branch');
      expect(spawnSyncMock).not.toHaveBeenCalled();
    });

    it('fails before merging when the working tree is dirty', async () => {
      spawnSyncMock.mockImplementation((command: string, args: string[]) => {
        if (command === 'git' && args[0] === 'status') {
          return { status: 0, stdout: ' M packages/agentic/src/handoff/manager.ts\n', stderr: '' };
        }

        return { status: 0, stdout: '', stderr: '' };
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const result = await manager.takeover('bc-pred', 42, 'successor/continue-work');

      expect(result.success).toBe(false);
      expect(result.error).toContain('Working tree must be clean');
      expect(getRepoMock).toHaveBeenCalledWith('owner', 'repo');
      expect(spawnSyncMock).toHaveBeenCalledTimes(1);
      expect(spawnSyncMock).toHaveBeenCalledWith(
        'git',
        ['status', '--porcelain'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
    });

    it('fails before merging when the successor branch already exists locally', async () => {
      spawnSyncMock.mockImplementation((command: string, args: string[]) => {
        if (command === 'git' && args[0] === 'status') {
          return { status: 0, stdout: '', stderr: '' };
        }

        if (command === 'git' && args[0] === 'rev-parse') {
          return { status: 0, stdout: 'refs/heads/successor/continue-work\n', stderr: '' };
        }

        return { status: 0, stdout: '', stderr: '' };
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const result = await manager.takeover('bc-pred', 42, 'successor/continue-work');

      expect(result.success).toBe(false);
      expect(result.error).toContain('Branch already exists locally');
      expect(spawnSyncMock).toHaveBeenCalledTimes(2);
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        1,
        'git',
        ['status', '--porcelain'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
      expect(spawnSyncMock).toHaveBeenNthCalledWith(
        2,
        'git',
        ['rev-parse', '--verify', '--quiet', 'refs/heads/successor/continue-work'],
        expect.objectContaining({ encoding: 'utf-8' })
      );
    });

    it('should reject branch names with shell injection characters', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ repo: 'owner/repo' });

      const injectionAttempts = [
        'branch;rm -rf /',
        'branch$(whoami)',
        'branch`id`',
        'branch|cat /etc/passwd',
      ];

      for (const branch of injectionAttempts) {
        const result = await manager.takeover('bc-pred', 42, branch);
        expect(result.success).toBe(false);
        expect(result.error).toContain('Invalid branch name');
      }
    });
  });

  describe('Handoff Initiation (without API)', () => {
    it('should fail gracefully when Cursor API is unavailable', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      // No API key set, so API will be null
      const manager = new HandoffManager();

      const result = await manager.initiateHandoff('bc-pred', {
        repository: 'owner/repo',
        currentPr: 1,
        currentBranch: 'main',
      });

      expect(result.success).toBe(false);
      expect(result.error).toContain('Cursor API not available');
    });

    it('uses the repository default branch for successor spawn when ref is omitted', async () => {
      getRepoMock.mockResolvedValue({
        success: true,
        data: {
          defaultBranch: 'master',
        },
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ cursorApiKey: 'cursor-key' });

      const result = await manager.initiateHandoff('bc-pred', {
        repository: 'owner/repo',
        currentPr: 1,
        currentBranch: 'feature/current-work',
      });

      expect(result).toEqual({
        success: true,
        successorId: 'bc-succ',
        successorHealthy: true,
      });
      expect(getRepoMock).toHaveBeenCalledWith('owner', 'repo');
      expect(cursorApiLaunchAgentMock).toHaveBeenCalledWith(
        expect.objectContaining({
          source: {
            repository: 'owner/repo',
            ref: 'master',
          },
        })
      );
      expect(writeFileSyncMock).toHaveBeenCalled();
    });

    it('honors an explicit ref without resolving the repository default branch', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ cursorApiKey: 'cursor-key' });

      const result = await manager.initiateHandoff('bc-pred', {
        repository: 'owner/repo',
        ref: 'release/2026-04',
        currentPr: 1,
        currentBranch: 'feature/current-work',
      });

      expect(result).toEqual({
        success: true,
        successorId: 'bc-succ',
        successorHealthy: true,
      });
      expect(getRepoMock).not.toHaveBeenCalled();
      expect(cursorApiLaunchAgentMock).toHaveBeenCalledWith(
        expect.objectContaining({
          source: {
            repository: 'owner/repo',
            ref: 'release/2026-04',
          },
        })
      );
    });

    it('fails before writing handoff state when the default branch cannot be resolved', async () => {
      getRepoMock.mockResolvedValue({
        success: false,
        error: 'No token available for this repository',
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ cursorApiKey: 'cursor-key' });

      const result = await manager.initiateHandoff('bc-pred', {
        repository: 'owner/repo',
        currentPr: 1,
        currentBranch: 'feature/current-work',
      });

      expect(result.success).toBe(false);
      expect(result.error).toContain('Failed to resolve default branch');
      expect(cursorApiLaunchAgentMock).not.toHaveBeenCalled();
      expect(writeFileSyncMock).not.toHaveBeenCalled();
    });

    it('marks the successor healthy when it confirms health and then completes quickly', async () => {
      cursorApiGetAgentStatusMock.mockResolvedValue({
        success: true,
        data: { status: 'COMPLETED' },
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ cursorApiKey: 'cursor-key' });

      const result = await manager.initiateHandoff('bc-pred', {
        repository: 'owner/repo',
        ref: 'feature/current-work',
        currentPr: 1,
        currentBranch: 'feature/current-work',
      });

      expect(result).toEqual({
        success: true,
        successorId: 'bc-succ',
        successorHealthy: true,
      });
      expect(cursorApiGetAgentConversationMock).toHaveBeenCalledWith('bc-succ');
    });

    it('marks the successor unhealthy immediately when it is cancelled before confirming health', async () => {
      cursorApiGetAgentStatusMock.mockResolvedValue({
        success: true,
        data: { status: 'CANCELLED' },
      });
      cursorApiGetAgentConversationMock.mockResolvedValue({
        success: true,
        data: {
          agentId: 'bc-succ',
          messages: [],
          totalMessages: 0,
        },
      });

      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager({ cursorApiKey: 'cursor-key' });

      const result = await manager.initiateHandoff('bc-pred', {
        repository: 'owner/repo',
        ref: 'feature/current-work',
        currentPr: 1,
        currentBranch: 'feature/current-work',
      });

      expect(result).toEqual({
        success: true,
        successorId: 'bc-succ',
        successorHealthy: false,
      });
      expect(cursorApiGetAgentConversationMock).not.toHaveBeenCalledWith('bc-succ');
    });
  });

  describe('Health Confirmation (without API)', () => {
    it('should throw when Cursor API is unavailable for health confirmation', async () => {
      const { HandoffManager } = await import('../src/handoff/manager.js');
      const manager = new HandoffManager();

      await expect(
        manager.confirmHealthAndBegin('bc-succ', 'bc-pred')
      ).rejects.toThrow('Cursor API not available');
    });
  });

  describe('Handoff Context Structure', () => {
    it('should have required fields defined in HandoffContext type', () => {
      // Validate the structure matches the interface
      const context = {
        predecessorId: 'bc-123',
        predecessorPr: 42,
        predecessorBranch: 'feat/work',
        handoffTime: new Date().toISOString(),
        completedWork: [
          {
            id: 'task-1',
            title: 'Fix tests',
            description: 'Fixed failing tests',
            priority: 'high' as const,
            category: 'bug' as const,
            status: 'completed' as const,
          },
        ],
        outstandingTasks: [
          {
            id: 'task-2',
            title: 'Add docs',
            description: 'Write documentation',
            priority: 'medium' as const,
            category: 'documentation' as const,
            status: 'pending' as const,
          },
        ],
        decisions: ['Use TypeScript for all new code'],
      };

      expect(context.predecessorId).toBe('bc-123');
      expect(context.predecessorPr).toBe(42);
      expect(context.handoffTime).toBeDefined();
      expect(context.completedWork).toHaveLength(1);
      expect(context.outstandingTasks).toHaveLength(1);
      expect(context.decisions).toHaveLength(1);
    });
  });

  describe('Merge Method Options', () => {
    it('should support all merge methods', () => {
      const validMethods: Array<'merge' | 'squash' | 'rebase'> = ['merge', 'squash', 'rebase'];
      expect(validMethods).toHaveLength(3);
      expect(validMethods).toContain('merge');
      expect(validMethods).toContain('squash');
      expect(validMethods).toContain('rebase');
    });
  });
});
