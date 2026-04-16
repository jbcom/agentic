import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const { createMCPClientMock, getConfigMock, transportCtorMock } = vi.hoisted(() => ({
  createMCPClientMock: vi.fn(),
  getConfigMock: vi.fn(),
  transportCtorMock: vi.fn(function MockStdioTransport(
    config: Record<string, unknown>
  ) {
    return { kind: 'stdio', ...config };
  }),
}));

vi.mock('@ai-sdk/mcp', () => ({
  experimental_createMCPClient: createMCPClientMock,
}));

vi.mock('@ai-sdk/mcp/mcp-stdio', () => ({
  Experimental_StdioMCPTransport: transportCtorMock,
}));

vi.mock('../src/core/config.js', () => ({
  getConfig: getConfigMock,
}));

import {
  closeMCPClients,
  getMCPTools,
  initializeMCPClients,
} from '../src/triage/mcp-clients.js';

const ORIGINAL_ENV = { ...process.env };

function disableDefaultServers() {
  return {
    cursor: { enabled: false },
    github: { enabled: false },
    context7: { enabled: false },
    '21st-magic': { enabled: false },
    'vendor-connectors': { enabled: false },
  };
}

describe('MCP client integration', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    createMCPClientMock.mockReset();
    getConfigMock.mockReset();
    transportCtorMock.mockClear();
    process.env = { ...ORIGINAL_ENV };
  });

  afterEach(() => {
    process.env = { ...ORIGINAL_ENV };
  });

  it('respects per-call enabled overrides for default servers', async () => {
    process.env.GITHUB_TOKEN = 'github-token';
    getConfigMock.mockReturnValue({
      mcp: {
        ...disableDefaultServers(),
        github: {
          enabled: true,
          tokenEnvVar: 'GITHUB_TOKEN',
          mode: 'stdio',
          command: 'npx',
          args: ['-y', '@modelcontextprotocol/server-github'],
        },
      },
    });

    const clients = await initializeMCPClients({
      github: { enabled: false },
    });

    expect(createMCPClientMock).not.toHaveBeenCalled();
    expect(clients).toEqual({});
  });

  it('initializes a custom stdio server that does not require a token', async () => {
    const localfsClient = { close: vi.fn(), tools: vi.fn() };
    createMCPClientMock.mockResolvedValue(localfsClient);
    getConfigMock.mockReturnValue({
      mcp: {
        ...disableDefaultServers(),
        localfs: {
          enabled: true,
          mode: 'stdio',
          command: 'uvx',
          args: ['localfs-mcp'],
        },
      },
    });

    const clients = await initializeMCPClients({} as never);

    expect(createMCPClientMock).toHaveBeenCalledTimes(1);
    expect(transportCtorMock).toHaveBeenCalledWith(
      expect.objectContaining({
        command: 'uvx',
        args: ['localfs-mcp'],
      })
    );
    expect(clients.localfs).toBe(localfsClient);
  });

  it('uses override tokens when initializing a default server', async () => {
    const githubClient = { close: vi.fn(), tools: vi.fn() };
    createMCPClientMock.mockResolvedValue(githubClient);
    getConfigMock.mockReturnValue({
      mcp: {
        ...disableDefaultServers(),
        github: {
          enabled: true,
          tokenEnvVar: 'GITHUB_TOKEN',
          mode: 'stdio',
          command: 'npx',
          args: ['-y', '@modelcontextprotocol/server-github'],
        },
      },
    });

    const clients = await initializeMCPClients({
      github: { token: 'override-github-token' },
    });

    expect(createMCPClientMock).toHaveBeenCalledTimes(1);
    expect(transportCtorMock).toHaveBeenCalledWith(
      expect.objectContaining({
        command: 'npx',
        args: ['-y', '@modelcontextprotocol/server-github'],
        env: expect.objectContaining({
          GITHUB_TOKEN: 'override-github-token',
        }),
      })
    );
    expect(clients.github).toBe(githubClient);
  });

  it('namespaces tools and ignores per-client tool failures', async () => {
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {
      // Intentionally suppress warning noise in this test.
    });
    const tools = await getMCPTools({
      github: {
        tools: vi.fn().mockResolvedValue({
          tools: {
            list_prs: { description: 'List PRs' },
          },
        }),
      },
      broken: {
        tools: vi.fn().mockRejectedValue(new Error('tool failure')),
      },
    } as never);

    expect(tools).toEqual({
      github_list_prs: { description: 'List PRs' },
    });
    expect(warnSpy).toHaveBeenCalledWith(
      'Failed to get tools from broken:',
      expect.any(Error)
    );
  });

  it('closes all clients and continues past close failures', async () => {
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {
      // Intentionally suppress warning noise in this test.
    });
    const githubClose = vi.fn().mockResolvedValue(undefined);
    const brokenClose = vi.fn().mockRejectedValue(new Error('close failure'));

    await closeMCPClients({
      github: {
        close: githubClose,
      },
      broken: {
        close: brokenClose,
      },
    } as never);

    expect(githubClose).toHaveBeenCalledOnce();
    expect(brokenClose).toHaveBeenCalledOnce();
    expect(warnSpy).toHaveBeenCalledWith(
      'Failed to close broken MCP client:',
      expect.any(Error)
    );
  });
});
