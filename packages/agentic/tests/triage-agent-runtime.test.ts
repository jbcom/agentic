import { beforeEach, describe, expect, it, vi } from 'vitest';

const { closeMCPClientsMock, initializeMCPClientsMock } = vi.hoisted(() => ({
  closeMCPClientsMock: vi.fn(),
  initializeMCPClientsMock: vi.fn(),
}));

vi.mock('../src/triage/mcp-clients.js', () => ({
  closeMCPClients: closeMCPClientsMock,
  getMCPTools: vi.fn().mockResolvedValue({}),
  initializeMCPClients: initializeMCPClientsMock,
}));

import { Agent } from '../src/triage/agent.js';

describe('Agent lifecycle', () => {
  beforeEach(() => {
    closeMCPClientsMock.mockReset();
    initializeMCPClientsMock.mockReset();
  });

  it('reinitializes MCP clients after close', async () => {
    const firstClients = { github: { close: vi.fn() } };
    const secondClients = { github: { close: vi.fn() } };
    initializeMCPClientsMock.mockResolvedValueOnce(firstClients).mockResolvedValueOnce(secondClients);

    const agent = new Agent({ mcp: {} });

    await agent.initialize();
    await agent.close();
    await agent.initialize();

    expect(initializeMCPClientsMock).toHaveBeenCalledTimes(2);
    expect(closeMCPClientsMock).toHaveBeenCalledTimes(1);
    expect(closeMCPClientsMock).toHaveBeenCalledWith(firstClients);
  });

  it('shares a single initialization across concurrent callers', async () => {
    let resolveInitialization: ((value: { github: { close: ReturnType<typeof vi.fn> } }) => void) | undefined;
    const pendingInitialization = new Promise<{ github: { close: ReturnType<typeof vi.fn> } }>(
      (resolve) => {
        resolveInitialization = resolve;
      }
    );
    initializeMCPClientsMock.mockReturnValue(pendingInitialization);

    const agent = new Agent({ mcp: {} });
    const firstCall = agent.initialize();
    const secondCall = agent.initialize();

    expect(initializeMCPClientsMock).toHaveBeenCalledTimes(1);

    resolveInitialization?.({ github: { close: vi.fn() } });
    await Promise.all([firstCall, secondCall]);

    expect(initializeMCPClientsMock).toHaveBeenCalledTimes(1);
  });
});
