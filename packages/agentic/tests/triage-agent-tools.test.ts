import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const { execFileSyncMock, execSyncMock } = vi.hoisted(() => ({
  execFileSyncMock: vi.fn(),
  execSyncMock: vi.fn(),
}));

vi.mock('node:child_process', () => ({
  execFileSync: execFileSyncMock,
  execSync: execSyncMock,
}));

vi.mock('@ai-sdk/anthropic', () => {
  const anthropic = Object.assign(
    vi.fn((model: string) => ({ model })),
    {
      tools: {
        bash_20250124: vi.fn((config: Record<string, unknown>) => config),
        textEditor_20250124: vi.fn((config: Record<string, unknown>) => config),
        webSearch_20250305: vi.fn((config: Record<string, unknown>) => config),
      },
    }
  );

  return { anthropic };
});

vi.mock('ai', () => ({
  generateObject: vi.fn(),
  generateText: vi.fn(),
  stepCountIs: vi.fn((count: number) => ({ count })),
  streamText: vi.fn(),
  tool: vi.fn((config: Record<string, unknown>) => config),
}));

vi.mock('../src/triage/mcp-clients.js', () => ({
  closeMCPClients: vi.fn(),
  getMCPTools: vi.fn().mockResolvedValue({}),
  initializeMCPClients: vi.fn().mockResolvedValue({}),
}));

import { Agent } from '../src/triage/agent.js';

interface ExecutableTool<Input> {
  execute: (input: Input) => Promise<string> | string;
}

interface ToolMap {
  bash: ExecutableTool<{ command?: string; restart?: boolean }>;
  delete_file: ExecutableTool<{ path: string }>;
  git_diff: ExecutableTool<{ staged?: boolean; file?: string }>;
  str_replace_editor: ExecutableTool<{
    command: string;
    path: string;
    file_text?: string;
    insert_line?: number;
    new_str?: string;
    old_str?: string;
    view_range?: [number, number];
  }>;
}

type AgentWithPrivateTools = Agent & {
  buildToolSet: (recordStep: (...args: [string, unknown, string, boolean?]) => void) => Promise<ToolMap>;
};

describe('Agent tool security and approval handling', () => {
  let workdir: string;

  beforeEach(async () => {
    execFileSyncMock.mockReset();
    execSyncMock.mockReset();
    workdir = await mkdtemp(join(tmpdir(), 'agentic-agent-tools-'));
  });

  afterEach(async () => {
    await rm(workdir, { recursive: true, force: true });
  });

  it('requires approval for dangerous bash commands even without explicit requireApproval config', async () => {
    const approvalRequest = vi.fn().mockResolvedValue(false);
    const agent = new Agent({
      workingDirectory: workdir,
      approval: {
        onApprovalRequest: approvalRequest,
      },
    });

    const tools = await (agent as unknown as AgentWithPrivateTools).buildToolSet(vi.fn());
    const result = await tools.bash.execute({
      command: 'rm -rf dist',
    });

    expect(result).toBe('Command rejected by approval policy');
    expect(approvalRequest).toHaveBeenCalledWith('bash', {
      command: 'rm -rf dist',
      risks: ['Recursive deletion detected'],
    });
    expect(execSyncMock).not.toHaveBeenCalled();
  });

  it('always consults approval for file deletion', async () => {
    const approvalRequest = vi.fn().mockResolvedValue(false);
    const filePath = join(workdir, 'delete-me.txt');
    await writeFile(filePath, 'temporary', 'utf-8');

    const agent = new Agent({
      workingDirectory: workdir,
      approval: {
        onApprovalRequest: approvalRequest,
      },
    });

    const tools = await (agent as unknown as AgentWithPrivateTools).buildToolSet(vi.fn());
    const result = await tools.delete_file.execute({
      path: 'delete-me.txt',
    });

    expect(result).toBe('Delete operation rejected by approval policy');
    expect(approvalRequest).toHaveBeenCalledWith('delete_file', {
      path: 'delete-me.txt',
    });
  });

  it('uses execFileSync for git diff so validated file paths are not interpolated into a shell string', async () => {
    execFileSyncMock.mockReturnValue('diff output');
    const fileName = 'unsafe name;still.ts';
    await writeFile(join(workdir, fileName), 'const value = 1;\n', 'utf-8');

    const agent = new Agent({
      workingDirectory: workdir,
    });

    const tools = await (agent as unknown as AgentWithPrivateTools).buildToolSet(vi.fn());
    const result = await tools.git_diff.execute({
      staged: true,
      file: fileName,
    });

    expect(result).toBe('diff output');
    expect(execFileSyncMock).toHaveBeenCalledWith(
      'git',
      ['diff', '--cached', '--', fileName],
      expect.objectContaining({
        cwd: workdir,
        encoding: 'utf-8',
      })
    );
    expect(execSyncMock).not.toHaveBeenCalled();
  });

  it('surfaces security errors for editor path traversal attempts', async () => {
    const recordStep = vi.fn();
    const agent = new Agent({
      workingDirectory: workdir,
    });

    const tools = await (agent as unknown as AgentWithPrivateTools).buildToolSet(recordStep);
    const result = await tools.str_replace_editor.execute({
      command: 'view',
      path: '../outside.txt',
    });

    expect(result).toContain('Security Error:');
    expect(recordStep).toHaveBeenCalledWith(
      'str_replace_editor',
      { command: 'view', path: '../outside.txt' },
      expect.stringContaining('Security Error:')
    );
  });
});
