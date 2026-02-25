import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createCursorAgent, pollCursorAgent, type CursorConfig } from '../src/cursor.ts';

// ---------------------------------------------------------------------------
// Global fetch mock
// ---------------------------------------------------------------------------

const fetchMock = vi.fn<typeof globalThis.fetch>();

beforeEach(() => {
    vi.stubGlobal('fetch', fetchMock);
});

afterEach(() => {
    vi.restoreAllMocks();
    fetchMock.mockReset();
});

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function jsonResponse(body: unknown, status = 200): Response {
    return new Response(JSON.stringify(body), {
        status,
        headers: { 'Content-Type': 'application/json' },
    });
}

function textResponse(body: string, status: number): Response {
    return new Response(body, { status });
}

const defaultConfig: CursorConfig = {
    apiKey: 'test-cursor-api-key',
};

const sampleTask = {
    id: 'task-c1',
    description: 'Redesign the database schema for multi-tenancy',
    context: 'The application needs to support multiple tenants sharing the same DB.',
    complexityScore: 9,
    complexityTier: 'expert' as const,
    repo: 'org/big-app',
};

const sampleTaskNoRepo = {
    id: 'task-c2',
    description: 'Expert task',
    context: 'Context data',
    complexityScore: 8,
    complexityTier: 'expert' as const,
};

// ===========================================================================
// createCursorAgent - Definition shape
// ===========================================================================

describe('createCursorAgent', () => {
    describe('agent definition shape', () => {
        it('returns a valid AgentDefinition with required fields', () => {
            const agent = createCursorAgent('cursor-1', defaultConfig);

            expect(agent.id).toBe('cursor-1');
            expect(agent.name).toBe('Cursor Cloud Agent');
            expect(agent.cost).toBe(100);
            expect(agent.priority).toBe(100);
            expect(agent.enabled).toBe(true);
            expect(agent.requiresApproval).toBe(true);
            expect(typeof agent.execute).toBe('function');
        });

        it('applies custom options', () => {
            const agent = createCursorAgent('cursor-2', defaultConfig, {
                name: 'My Cursor',
                cost: 50,
                priority: 50,
                requiresApproval: false,
            });

            expect(agent.name).toBe('My Cursor');
            expect(agent.cost).toBe(50);
            expect(agent.priority).toBe(50);
            expect(agent.requiresApproval).toBe(false);
        });

        it('has correct default capabilities', () => {
            const agent = createCursorAgent('cursor-3', defaultConfig);

            expect(agent.capabilities.tiers).toEqual(['expert']);
            expect(agent.capabilities.maxContext).toBe(200000);
            expect(agent.capabilities.canCreatePR).toBe(true);
            expect(agent.capabilities.canExecute).toBe(true);
            expect(agent.capabilities.async).toBe(true);
        });

        it('merges custom capabilities', () => {
            const agent = createCursorAgent('cursor-4', defaultConfig, {
                capabilities: {
                    tiers: ['complex', 'expert'],
                    maxContext: 100000,
                },
            });

            expect(agent.capabilities.tiers).toEqual(['complex', 'expert']);
            expect(agent.capabilities.maxContext).toBe(100000);
            // Non-overridden defaults remain
            expect(agent.capabilities.canCreatePR).toBe(true);
            expect(agent.capabilities.canExecute).toBe(true);
        });

        it('defaults requiresApproval to true (expensive agent)', () => {
            const agent = createCursorAgent('cursor-approval', defaultConfig);
            expect(agent.requiresApproval).toBe(true);
        });

        it('has highest cost (100) by default - last resort agent', () => {
            const agent = createCursorAgent('cursor-cost', defaultConfig);
            expect(agent.cost).toBe(100);
            expect(agent.priority).toBe(100);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Successful agent creation
    // -----------------------------------------------------------------------
    describe('execute() - success', () => {
        it('creates an agent and returns CursorAgentResult', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({
                    id: 'agent-abc-123',
                    status: 'running',
                    messages: [{ role: 'assistant', content: 'Working on it...' }],
                })
            );

            const agent = createCursorAgent('cursor-ok', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(true);
            expect(result.data).toEqual({
                agentId: 'agent-abc-123',
                status: 'running',
                messages: [{ role: 'assistant', content: 'Working on it...' }],
            });
            expect(result.jobId).toBe('agent-abc-123');
            expect(result.cost).toBe(100);
        });

        it('sends correct request with Bearer auth', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-x', status: 'running' })
            );

            const agent = createCursorAgent('cursor-auth', {
                apiKey: 'my-cursor-secret',
                baseUrl: 'https://custom-cursor.example.com/v1',
            });
            await agent.execute(sampleTask);

            const [url, opts] = fetchMock.mock.calls[0];
            expect(url).toBe('https://custom-cursor.example.com/v1/agents');
            expect(opts?.method).toBe('POST');
            expect((opts?.headers as Record<string, string>).Authorization).toBe('Bearer my-cursor-secret');
            expect((opts?.headers as Record<string, string>)['Content-Type']).toBe('application/json');
        });

        it('sends correct body with messages and workspace', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-ws', status: 'running' })
            );

            const agent = createCursorAgent('cursor-ws', {
                apiKey: 'key',
                workspacePath: '/workspace/my-project',
            });
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.workspacePath).toBe('/workspace/my-project');
            expect(body.messages).toHaveLength(1);
            expect(body.messages[0].role).toBe('user');
            expect(body.messages[0].content).toContain('Redesign the database schema');
        });

        it('formats prompt with task details and repo', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-fmt', status: 'running' })
            );

            const agent = createCursorAgent('cursor-fmt', defaultConfig);
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            const content = body.messages[0].content;
            expect(content).toContain('TASK: Redesign the database schema');
            expect(content).toContain('REPOSITORY: org/big-app');
            expect(content).toContain('multi-tenancy');
            expect(content).toContain('escalated to you');
        });

        it('shows "Unknown" for repo when task has no repo', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-norepo', status: 'running' })
            );

            const agent = createCursorAgent('cursor-norepo', defaultConfig);
            await agent.execute(sampleTaskNoRepo);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.messages[0].content).toContain('REPOSITORY: Unknown');
        });

        it('uses agentId field when id is missing from response', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ agentId: 'fallback-id', status: 'pending' })
            );

            const agent = createCursorAgent('cursor-fallback', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.data?.agentId).toBe('fallback-id');
        });

        it('defaults status to "running" when not in response', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-nostat' })
            );

            const agent = createCursorAgent('cursor-nostat', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.data?.status).toBe('running');
        });
    });

    // -----------------------------------------------------------------------
    // execute() - HTTP error
    // -----------------------------------------------------------------------
    describe('execute() - HTTP errors', () => {
        it('returns error result on non-ok response', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Unauthorized', 401));

            const agent = createCursorAgent('cursor-err', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Cursor API error: 401 - Unauthorized');
            expect(result.escalate).toBe(false); // No more tiers to escalate to
            expect(result.cost).toBe(0); // Don't charge for failed requests
        });

        it('does not escalate on failure (last resort agent)', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Server Error', 500));

            const agent = createCursorAgent('cursor-noesc', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.escalate).toBe(false);
        });

        it('does not charge cost on failure', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Rate Limited', 429));

            const agent = createCursorAgent('cursor-nocharge', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.cost).toBe(0);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Network / exception errors
    // -----------------------------------------------------------------------
    describe('execute() - network errors', () => {
        it('catches fetch exceptions and returns error result', async () => {
            fetchMock.mockRejectedValueOnce(new Error('DNS resolution failed'));

            const agent = createCursorAgent('cursor-net', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('DNS resolution failed');
            expect(result.escalate).toBe(false);
            expect(result.cost).toBe(0);
        });

        it('handles non-Error exceptions gracefully', async () => {
            fetchMock.mockRejectedValueOnce(null);

            const agent = createCursorAgent('cursor-nonerr', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Unknown error');
        });
    });

    // -----------------------------------------------------------------------
    // Config defaults
    // -----------------------------------------------------------------------
    describe('config defaults', () => {
        it('uses default base URL https://api.cursor.com/v0', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-def', status: 'running' })
            );

            const agent = createCursorAgent('cursor-defurl', defaultConfig);
            await agent.execute(sampleTask);

            const [url] = fetchMock.mock.calls[0];
            expect(url).toBe('https://api.cursor.com/v0/agents');
        });

        it('uses custom base URL', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ id: 'agent-cust', status: 'running' })
            );

            const agent = createCursorAgent('cursor-custurl', {
                apiKey: 'key',
                baseUrl: 'https://my-cursor.internal/api',
            });
            await agent.execute(sampleTask);

            const [url] = fetchMock.mock.calls[0];
            expect(url).toBe('https://my-cursor.internal/api/agents');
        });
    });
});

// ===========================================================================
// pollCursorAgent
// ===========================================================================

describe('pollCursorAgent', () => {
    it('polls agent and returns CursorAgentResult', async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({
                id: 'agent-poll-1',
                status: 'completed',
                messages: [
                    { role: 'user', content: 'task' },
                    { role: 'assistant', content: 'done' },
                ],
            })
        );

        const result = await pollCursorAgent(defaultConfig, 'agent-poll-1');

        expect(result.agentId).toBe('agent-poll-1');
        expect(result.status).toBe('completed');
        expect(result.messages).toHaveLength(2);
    });

    it('sends Bearer auth header', async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ id: 'x', status: 'running' })
        );

        await pollCursorAgent({ apiKey: 'poll-secret' }, 'agent-x');

        const [, opts] = fetchMock.mock.calls[0];
        expect((opts?.headers as Record<string, string>).Authorization).toBe('Bearer poll-secret');
    });

    it('uses correct URL with agent ID', async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ id: 'y', status: 'running' })
        );

        await pollCursorAgent(
            { apiKey: 'key', baseUrl: 'https://custom.cursor/v2' },
            'agent-y'
        );

        const [url] = fetchMock.mock.calls[0];
        expect(url).toBe('https://custom.cursor/v2/agents/agent-y');
    });

    it('throws on non-ok HTTP response', async () => {
        fetchMock.mockResolvedValueOnce(textResponse('Not Found', 404));

        await expect(
            pollCursorAgent(defaultConfig, 'agent-missing')
        ).rejects.toThrow('Failed to poll agent: 404');
    });

    it('uses default base URL when not specified', async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ id: 'z', status: 'running' })
        );

        await pollCursorAgent({ apiKey: 'key' }, 'agent-z');

        const [url] = fetchMock.mock.calls[0];
        expect(url).toBe('https://api.cursor.com/v0/agents/agent-z');
    });
});
