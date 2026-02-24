/**
 * Tests for sandbox execution - SandboxExecutor, ContainerManager, and runtime adapters
 */

import { describe, expect, it, vi } from 'vitest';
import { ContainerManager } from '../src/sandbox/container.js';
import { SandboxExecutor } from '../src/sandbox/executor.js';
import { ClaudeRuntime } from '../src/sandbox/runtime/claude.js';
import { CursorRuntime } from '../src/sandbox/runtime/cursor.js';

// Mock the subprocess module since container operations need Docker
vi.mock('../src/core/subprocess.js', () => ({
  safeDockerCommand: vi.fn().mockReturnValue({
    success: true,
    stdout: 'mock-container-id',
    stderr: '',
    code: 0,
  }),
  safeSpawn: vi.fn().mockResolvedValue({
    success: true,
    stdout: '{"result": "done"}',
    stderr: '',
    code: 0,
  }),
  validateCommandArgs: vi.fn(),
}));

describe('Runtime Adapters', () => {
  describe('ClaudeRuntime', () => {
    const runtime = new ClaudeRuntime();

    it('should have correct name and image', () => {
      expect(runtime.name).toBe('claude');
      expect(runtime.image).toBe('jbcom/agentic-control:latest');
    });

    it('should prepare command with prompt', () => {
      const cmd = runtime.prepareCommand('Fix the bug', {});
      expect(cmd).toContain('npx');
      expect(cmd).toContain('@anthropic-ai/claude-agent-sdk');
      expect(cmd).toContain('query');
      expect(cmd).toContain('Fix the bug');
    });

    it('should include timeout in command when specified', () => {
      const cmd = runtime.prepareCommand('test', { timeout: 30000 });
      expect(cmd).toContain('--timeout');
      expect(cmd).toContain('30000');
    });

    it('should not include timeout when not specified', () => {
      const cmd = runtime.prepareCommand('test', {});
      expect(cmd).not.toContain('--timeout');
    });

    it('should parse JSON output correctly', () => {
      const output = runtime.parseOutput(
        JSON.stringify({ result: 'success', files: ['a.ts', 'b.ts'] }),
        ''
      );
      expect(output.result).toBe('success');
      expect(output.files).toEqual(['a.ts', 'b.ts']);
    });

    it('should parse plain text output as fallback', () => {
      const output = runtime.parseOutput('plain text result', '');
      expect(output.result).toBe('plain text result');
      expect(output.files).toEqual([]);
    });

    it('should include stderr in error field', () => {
      const output = runtime.parseOutput('output', 'some warning');
      expect(output.error).toBe('some warning');
    });

    it('should validate environment based on ANTHROPIC_API_KEY', async () => {
      process.env.ANTHROPIC_API_KEY = 'test-key';
      expect(await runtime.validateEnvironment()).toBe(true);

      const original = process.env.ANTHROPIC_API_KEY;
      delete process.env.ANTHROPIC_API_KEY;
      expect(await runtime.validateEnvironment()).toBe(false);
      process.env.ANTHROPIC_API_KEY = original;
    });
  });

  describe('CursorRuntime', () => {
    const runtime = new CursorRuntime();

    it('should have correct name and image', () => {
      expect(runtime.name).toBe('cursor');
      expect(runtime.image).toBe('jbcom/agentic-control:latest');
    });

    it('should prepare command with prompt', () => {
      const cmd = runtime.prepareCommand('Implement feature', {});
      expect(cmd).toContain('cursor-agent');
      expect(cmd).toContain('run');
      expect(cmd).toContain('--task');
      expect(cmd).toContain('Implement feature');
    });

    it('should include timeout in command when specified', () => {
      const cmd = runtime.prepareCommand('test', { timeout: 60000 });
      expect(cmd).toContain('--timeout');
      expect(cmd).toContain('60000');
    });

    it('should parse JSON output correctly', () => {
      const output = runtime.parseOutput(
        JSON.stringify({ result: 'completed', files: ['file.ts'] }),
        ''
      );
      expect(output.result).toBe('completed');
      expect(output.files).toEqual(['file.ts']);
    });

    it('should validate environment based on CURSOR_API_KEY', async () => {
      process.env.CURSOR_API_KEY = 'test-key';
      expect(await runtime.validateEnvironment()).toBe(true);

      const original = process.env.CURSOR_API_KEY;
      delete process.env.CURSOR_API_KEY;
      expect(await runtime.validateEnvironment()).toBe(false);
      process.env.CURSOR_API_KEY = original;
    });
  });
});

describe('SandboxExecutor', () => {
  it('should initialize with default runtimes', () => {
    const executor = new SandboxExecutor();
    expect(executor).toBeDefined();
  });

  it('should throw for unknown runtime', async () => {
    const executor = new SandboxExecutor();
    await expect(
      executor.execute({
        runtime: 'unknown' as 'claude',
        workspace: '/tmp/workspace',
        outputDir: '/tmp/output',
        prompt: 'test prompt',
      })
    ).rejects.toThrow('Unknown runtime: unknown');
  });
});

describe('ContainerManager', () => {
  it('should create a container with the correct configuration', async () => {
    const manager = new ContainerManager();
    const { safeDockerCommand } = await import('../src/core/subprocess.js');

    const containerId = await manager.create({
      runtime: 'claude',
      workspace: '/test/workspace',
      outputDir: '/test/output',
    });

    expect(containerId).toMatch(/^agentic-sandbox-/);
    expect(safeDockerCommand).toHaveBeenCalled();
  });

  it('should include memory limit in docker args when specified', async () => {
    const manager = new ContainerManager();
    const { safeDockerCommand } = await import('../src/core/subprocess.js');

    await manager.create({
      runtime: 'claude',
      workspace: '/test/workspace',
      outputDir: '/test/output',
      memory: 512,
    });

    const callArgs = (safeDockerCommand as ReturnType<typeof vi.fn>).mock.calls;
    const lastCall = callArgs[callArgs.length - 1][0];
    expect(lastCall).toContain('-m');
    expect(lastCall).toContain('512m');
  });

  it('should include environment variables in docker args', async () => {
    const manager = new ContainerManager();
    const { safeDockerCommand } = await import('../src/core/subprocess.js');

    await manager.create({
      runtime: 'cursor',
      workspace: '/test/workspace',
      outputDir: '/test/output',
      env: { MY_VAR: 'my_value' },
    });

    const callArgs = (safeDockerCommand as ReturnType<typeof vi.fn>).mock.calls;
    const lastCall = callArgs[callArgs.length - 1][0];
    expect(lastCall).toContain('-e');
    expect(lastCall).toContain('MY_VAR=my_value');
  });
});
