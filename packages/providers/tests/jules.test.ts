import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createJulesAgent, pollJulesSession, sendJulesFollowUp, type JulesConfig } from '../src/jules.ts';

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

const defaultConfig: JulesConfig = {
    apiKey: 'test-jules-api-key',
};

const sampleTask = {
    id: 'task-j1',
    description: 'Refactor the authentication module',
    context: 'The auth module needs to support OAuth2 flows.',
    complexityScore: 6,
    complexityTier: 'moderate' as const,
    repo: 'org/my-repo',
};

const sampleTaskNoRepo = {
    id: 'task-j2',
    description: 'Fix a bug',
    context: 'Bug context here',
    complexityScore: 5,
    complexityTier: 'moderate' as const,
};

// ===========================================================================
// createJulesAgent - Definition shape
// ===========================================================================

describe('createJulesAgent', () => {
    describe('agent definition shape', () => {
        it('returns a valid AgentDefinition with required fields', () => {
            const agent = createJulesAgent('jules-1', defaultConfig);

            expect(agent.id).toBe('jules-1');
            expect(agent.name).toBe('Google Jules');
            expect(agent.cost).toBe(0);
            expect(agent.priority).toBe(10);
            expect(agent.enabled).toBe(true);
            expect(agent.requiresApproval).toBe(false);
            expect(typeof agent.execute).toBe('function');
        });

        it('applies custom options', () => {
            const agent = createJulesAgent('jules-2', defaultConfig, {
                name: 'My Jules',
                cost: 10,
                priority: 5,
            });

            expect(agent.name).toBe('My Jules');
            expect(agent.cost).toBe(10);
            expect(agent.priority).toBe(5);
        });

        it('has correct default capabilities', () => {
            const agent = createJulesAgent('jules-3', defaultConfig);

            expect(agent.capabilities.tiers).toEqual(['moderate', 'complex', 'expert']);
            expect(agent.capabilities.maxContext).toBe(100000);
            expect(agent.capabilities.canCreatePR).toBe(true);
            expect(agent.capabilities.canExecute).toBe(true);
            expect(agent.capabilities.async).toBe(true);
        });

        it('merges custom capabilities', () => {
            const agent = createJulesAgent('jules-4', defaultConfig, {
                capabilities: {
                    tiers: ['complex', 'expert'],
                    maxContext: 50000,
                },
            });

            expect(agent.capabilities.tiers).toEqual(['complex', 'expert']);
            expect(agent.capabilities.maxContext).toBe(50000);
            // Non-overridden defaults remain
            expect(agent.capabilities.canCreatePR).toBe(true);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Successful session creation
    // -----------------------------------------------------------------------
    describe('execute() - success', () => {
        it('creates a session and returns session result', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({
                    name: 'sessions/abc-123',
                    state: 'RUNNING',
                })
            );

            const agent = createJulesAgent('jules-ok', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(true);
            expect(result.data).toEqual({
                sessionId: 'abc-123',
                name: 'sessions/abc-123',
                state: 'RUNNING',
            });
            expect(result.jobId).toBe('sessions/abc-123');
            expect(result.cost).toBe(0);
        });

        it('sends correct request with API key header', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/x', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-auth', {
                apiKey: 'my-secret-key',
                baseUrl: 'https://custom-jules.example.com/v1',
            });
            await agent.execute(sampleTask);

            const [url, opts] = fetchMock.mock.calls[0];
            expect(url).toBe('https://custom-jules.example.com/v1/sessions');
            expect(opts?.method).toBe('POST');
            expect((opts?.headers as Record<string, string>)['X-Goog-Api-Key']).toBe('my-secret-key');
            expect((opts?.headers as Record<string, string>)['Content-Type']).toBe('application/json');
        });

        it('sends repo context when task has repo', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/r1', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-repo', defaultConfig);
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.sourceContext).toEqual({
                source: 'sources/github/org/my-repo',
                githubRepoContext: {
                    startingBranch: 'main',
                },
            });
        });

        it('omits sourceContext when no repo in task', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/nr', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-norepo', defaultConfig);
            await agent.execute(sampleTaskNoRepo);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.sourceContext).toBeUndefined();
        });

        it('sends automation mode in request', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/am', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-am', {
                apiKey: 'key',
                automationMode: 'MANUAL',
            });
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.automationMode).toBe('MANUAL');
        });

        it('uses default automation mode AUTO_CREATE_PR', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/def', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-defam', defaultConfig);
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.automationMode).toBe('AUTO_CREATE_PR');
        });

        it('extracts sessionId from the last segment of name', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'projects/123/sessions/sess-456', state: 'RUNNING' })
            );

            const agent = createJulesAgent('jules-sid', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.data?.sessionId).toBe('sess-456');
        });

        it('defaults state to PENDING when not provided by API', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/no-state' })
            );

            const agent = createJulesAgent('jules-nostate', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.data?.state).toBe('PENDING');
        });

        it('formats prompt correctly', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/fmt', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-fmt', defaultConfig);
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.prompt).toContain('Refactor the authentication module');
            expect(body.prompt).toContain('OAuth2 flows');
            expect(body.prompt).toContain('Create a PR with your changes');
        });
    });

    // -----------------------------------------------------------------------
    // execute() - HTTP error
    // -----------------------------------------------------------------------
    describe('execute() - HTTP errors', () => {
        it('returns error result on non-ok response', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Forbidden', 403));

            const agent = createJulesAgent('jules-err', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Jules API error: 403 - Forbidden');
            expect(result.escalate).toBe(true);
            expect(result.cost).toBe(0);
        });

        it('returns error result on 500', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Internal Server Error', 500));

            const agent = createJulesAgent('jules-500', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toContain('500');
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Network / exception errors
    // -----------------------------------------------------------------------
    describe('execute() - network errors', () => {
        it('catches fetch exceptions and returns error result', async () => {
            fetchMock.mockRejectedValueOnce(new Error('Network timeout'));

            const agent = createJulesAgent('jules-net', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Network timeout');
            expect(result.escalate).toBe(true);
            expect(result.cost).toBe(0);
        });

        it('handles non-Error exceptions gracefully', async () => {
            fetchMock.mockRejectedValueOnce(42);

            const agent = createJulesAgent('jules-nonerr', defaultConfig);
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Unknown error');
        });
    });

    // -----------------------------------------------------------------------
    // Config defaults
    // -----------------------------------------------------------------------
    describe('config defaults', () => {
        it('uses default base URL', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ name: 'sessions/def', state: 'PENDING' })
            );

            const agent = createJulesAgent('jules-defurl', defaultConfig);
            await agent.execute(sampleTask);

            const [url] = fetchMock.mock.calls[0];
            expect(url).toBe('https://jules.googleapis.com/v1alpha/sessions');
        });
    });
});

// ===========================================================================
// pollJulesSession
// ===========================================================================

describe('pollJulesSession', () => {
    it('polls session and returns state with PR URL', async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({
                state: 'COMPLETED',
                pullRequestUrl: 'https://github.com/org/repo/pull/42',
            })
        );

        const result = await pollJulesSession(defaultConfig, 'sessions/abc');

        expect(result.state).toBe('COMPLETED');
        expect(result.prUrl).toBe('https://github.com/org/repo/pull/42');
        expect(result.error).toBeUndefined();
    });

    it('sends API key in request header', async () => {
        fetchMock.mockResolvedValueOnce(jsonResponse({ state: 'RUNNING' }));

        await pollJulesSession({ apiKey: 'poll-key' }, 'sessions/x');

        const [, opts] = fetchMock.mock.calls[0];
        expect((opts?.headers as Record<string, string>)['X-Goog-Api-Key']).toBe('poll-key');
    });

    it('uses correct URL with session name', async () => {
        fetchMock.mockResolvedValueOnce(jsonResponse({ state: 'PENDING' }));

        await pollJulesSession(
            { apiKey: 'key', baseUrl: 'https://custom.api/v1' },
            'sessions/poll-123'
        );

        const [url] = fetchMock.mock.calls[0];
        expect(url).toBe('https://custom.api/v1/sessions/poll-123');
    });

    it('returns error message from response', async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({
                state: 'FAILED',
                error: { message: 'Session crashed' },
            })
        );

        const result = await pollJulesSession(defaultConfig, 'sessions/fail');

        expect(result.state).toBe('FAILED');
        expect(result.error).toBe('Session crashed');
    });

    it('throws on non-ok HTTP response', async () => {
        fetchMock.mockResolvedValueOnce(textResponse('Not Found', 404));

        await expect(
            pollJulesSession(defaultConfig, 'sessions/missing')
        ).rejects.toThrow('Failed to poll session: 404');
    });
});

// ===========================================================================
// sendJulesFollowUp
// ===========================================================================

describe('sendJulesFollowUp', () => {
    it('sends follow-up message to correct endpoint', async () => {
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 200 }));

        await sendJulesFollowUp(defaultConfig, 'sessions/abc', 'Please also update the tests');

        const [url, opts] = fetchMock.mock.calls[0];
        expect(url).toBe('https://jules.googleapis.com/v1alpha/sessions/abc:addUserResponse');
        expect(opts?.method).toBe('POST');

        const body = JSON.parse(opts?.body as string);
        expect(body.userResponse).toBe('Please also update the tests');
    });

    it('sends API key in header', async () => {
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 200 }));

        await sendJulesFollowUp({ apiKey: 'followup-key' }, 'sessions/x', 'msg');

        const [, opts] = fetchMock.mock.calls[0];
        expect((opts?.headers as Record<string, string>)['X-Goog-Api-Key']).toBe('followup-key');
    });

    it('uses custom base URL', async () => {
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 200 }));

        await sendJulesFollowUp(
            { apiKey: 'key', baseUrl: 'https://custom.api/v2' },
            'sessions/y',
            'hello'
        );

        const [url] = fetchMock.mock.calls[0];
        expect(url).toBe('https://custom.api/v2/sessions/y:addUserResponse');
    });

    it('throws on non-ok HTTP response', async () => {
        fetchMock.mockResolvedValueOnce(textResponse('Bad Request', 400));

        await expect(
            sendJulesFollowUp(defaultConfig, 'sessions/bad', 'msg')
        ).rejects.toThrow('Failed to send follow-up: 400');
    });
});
