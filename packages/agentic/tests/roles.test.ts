import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const { generateTextMock, initializeMCPClientsMock, closeMCPClientsMock } = vi.hoisted(() => ({
  generateTextMock: vi.fn(),
  initializeMCPClientsMock: vi.fn(),
  closeMCPClientsMock: vi.fn(),
}));

vi.mock('ai', async (importOriginal) => ({
  ...(await importOriginal<typeof import('ai')>()),
  generateText: generateTextMock,
}));
vi.mock('../src/triage/mcp-clients.js', () => ({
  initializeMCPClients: initializeMCPClientsMock,
  closeMCPClients: closeMCPClientsMock,
  getMCPTools: vi.fn().mockResolvedValue({}),
}));

import {
  getConfig,
  initConfig,
  loadConfigFromPath,
  resetConfig,
  setConfig,
} from '../src/core/config.js';
import { validateConfig } from '../src/core/validation.js';
import {
  DEFAULT_ROLES,
  executeRole,
  findRoleByTrigger,
  getEffectiveRole,
  listRoles,
} from '../src/roles/index.js';
import type { RolesConfig } from '../src/roles/types.js';

let directory: string;
beforeEach(() => {
  directory = mkdtempSync(join(tmpdir(), 'agentic-roles-'));
  resetConfig();
  initConfig();
  vi.clearAllMocks();
  generateTextMock.mockResolvedValue({ text: 'Reviewed successfully', usage: {} });
  initializeMCPClientsMock.mockResolvedValue({ github: {} });
  closeMCPClientsMock.mockResolvedValue(undefined);
});
afterEach(() => {
  resetConfig();
  rmSync(directory, { recursive: true, force: true });
});

describe('role configuration and routing', () => {
  it('loads role overrides from JSON and merges individual fields', () => {
    const configPath = join(directory, 'agentic.config.json');
    writeFileSync(
      configPath,
      JSON.stringify({
        roles: { reviewer: { model: 'custom-review-model' }, fixer: { enabled: false } },
      })
    );
    loadConfigFromPath(configPath);
    setConfig({ roles: { reviewer: { enabled: false } } });
    expect(getConfig().roles?.reviewer).toEqual({ model: 'custom-review-model', enabled: false });
    expect(getEffectiveRole('fixer')?.enabled).toBe(false);
    expect(getEffectiveRole('reviewer')?.model).toBe('custom-review-model');
  });

  it.each([
    { reviewer: { enabled: 'false' } },
    { reviewer: { model: ' ' } },
    { reviewer: { maxSteps: 0 } },
    { reviewer: { systemPrompt: '' } },
    { unknown: { enabled: true } },
  ])('rejects invalid role settings: %j', (roles) => {
    expect(() => validateConfig({ roles })).toThrow();
    expect(() => setConfig({ roles: roles as RolesConfig })).toThrow();
    expect(getConfig().roles).toBeUndefined();
  });

  it('returns independent snapshots without mutating built-in definitions', () => {
    const first = getEffectiveRole('reviewer', { reviewer: { systemPrompt: 'Custom persona' } });
    expect(first?.systemPrompt).toBe('Custom persona');
    const trigger = first?.triggers[0];
    if (trigger) trigger.value = 'changed';
    expect(getEffectiveRole('reviewer')?.triggers[0]?.value).toBe('pull_request');
    expect(DEFAULT_ROLES.reviewer.systemPrompt).toContain('Act as Reviewer');
    expect(Object.isFrozen(DEFAULT_ROLES.reviewer)).toBe(true);
    expect(listRoles()).toHaveLength(5);
    expect(getEffectiveRole('unknown')).toBeUndefined();
  });

  it('matches command tokens without matching prefixes or prose', () => {
    expect(findRoleByTrigger('/review src/index.ts')?.name).toBe('reviewer');
    expect(findRoleByTrigger('/reviewer')).toBeUndefined();
    expect(findRoleByTrigger('please /review')).toBeUndefined();
    expect(findRoleByTrigger('/cursor')?.name).toBe('delegator');
    expect(findRoleByTrigger('/jules')?.name).toBe('delegator');
    expect(findRoleByTrigger('/review', { reviewer: { enabled: false } })).toBeUndefined();
  });

  it('keeps schedule and event kinds distinct', () => {
    expect(findRoleByTrigger({ type: 'event', value: 'pull_request' })?.name).toBe('reviewer');
    expect(findRoleByTrigger({ type: 'schedule', value: 'pull_request' })).toBeUndefined();
    expect(findRoleByTrigger('*/15 * * * *')?.name).toBe('harvester');
    expect(findRoleByTrigger('0 2 * * *')?.name).toBe('curator');
    expect(findRoleByTrigger('workflow_run:failure')?.name).toBe('fixer');
  });
});

describe('role execution through the existing Agent', () => {
  it('does not send a non-Anthropic global model to the Anthropic execution engine', async () => {
    setConfig({ triage: { provider: 'openai', model: 'demo-openai-model' } });
    await executeRole('reviewer', 'Review the current changes');
    expect(generateTextMock.mock.calls[0]?.[0].model.modelId).toBe('claude-sonnet-4-20250514');
  });

  it('rejects missing approval callbacks before creating clients', async () => {
    await expect(
      executeRole('fixer', 'Repair CI', {
        agent: { mcp: {}, approval: { requireApproval: ['bash'] } },
      })
    ).rejects.toThrow('onApprovalRequest');
    expect(initializeMCPClientsMock).not.toHaveBeenCalled();
    expect(generateTextMock).not.toHaveBeenCalled();
  });
  it('passes persona/model overrides and preserves approvals and cleanup', async () => {
    const approval = vi.fn().mockResolvedValue(true);
    const result = await executeRole('reviewer', 'Review the current changes', {
      roles: {
        reviewer: {
          model: 'custom-review-model',
          systemPrompt: 'Review for data loss',
          maxSteps: 3,
        },
      },
      agent: {
        mcp: {},
        systemPrompt: 'Keep output concise',
        approval: { requireApproval: ['bash'], onApprovalRequest: approval },
      },
    });
    expect(result.success).toBe(true);
    expect(generateTextMock).toHaveBeenCalledOnce();
    const request = generateTextMock.mock.calls[0]?.[0];
    expect(request.model.modelId).toBe('custom-review-model');
    expect(request.system).toContain('Review for data loss');
    expect(request.system).toContain('Keep output concise');
    expect(request.system).toContain('Respect approval policies');
    expect(request.prompt).toBe('Review the current changes');
    expect(initializeMCPClientsMock).toHaveBeenCalledOnce();
    expect(closeMCPClientsMock).toHaveBeenCalledOnce();
  });

  it('closes clients after model failures and preserves the Agent failure result', async () => {
    generateTextMock.mockRejectedValueOnce(new Error('model unavailable'));
    const result = await executeRole('fixer', 'Repair the failing job', { agent: { mcp: {} } });
    expect(result).toMatchObject({ success: false, result: 'model unavailable' });
    expect(closeMCPClientsMock).toHaveBeenCalledOnce();
  });

  it('propagates initialization failures without invoking the model', async () => {
    initializeMCPClientsMock.mockRejectedValueOnce(new Error('transport unavailable'));
    await expect(
      executeRole('curator', 'Assess the open issues', { agent: { mcp: {} } })
    ).rejects.toThrow('transport unavailable');
    expect(generateTextMock).not.toHaveBeenCalled();
  });

  it('rejects disabled, unknown, and empty tasks before creating clients', async () => {
    setConfig({ roles: { reviewer: { enabled: false } } });
    await expect(
      executeRole('reviewer', 'Review changes', { roles: { reviewer: { model: 'custom' } } })
    ).rejects.toThrow('disabled');
    await expect(
      executeRole('delegator', 'Route this issue', {
        roles: { delegator: { enabled: false } },
        agent: { mcp: {} },
      })
    ).rejects.toThrow('disabled');
    await expect(executeRole('unknown', 'Route this issue')).rejects.toThrow('Unknown');
    await expect(
      executeRole('reviewer', ' ', { roles: { reviewer: { enabled: true } } })
    ).rejects.toThrow('empty');
    expect(initializeMCPClientsMock).not.toHaveBeenCalled();
    expect(generateTextMock).not.toHaveBeenCalled();
  });
});
