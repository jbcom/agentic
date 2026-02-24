import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createOllamaAgent, createOllamaEvaluator } from '../src/ollama.ts';

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

const sampleTask = {
    id: 'task-1',
    description: 'Fix a typo in README',
    context: 'The word "teh" should be "the".',
    complexityScore: 1,
    complexityTier: 'trivial' as const,
};

// ===========================================================================
// createOllamaEvaluator
// ===========================================================================

describe('createOllamaEvaluator', () => {
    it('returns a function', () => {
        const evaluator = createOllamaEvaluator();
        expect(typeof evaluator).toBe('function');
    });

    it('sends correct request to Ollama /api/generate', async () => {
        fetchMock.mockResolvedValueOnce(jsonResponse({ response: '{"score":2}' }));

        const evaluator = createOllamaEvaluator({
            url: 'http://my-ollama:11434',
            model: 'llama3',
        });

        const result = await evaluator('Evaluate this task');

        expect(fetchMock).toHaveBeenCalledOnce();
        const [url, opts] = fetchMock.mock.calls[0];
        expect(url).toBe('http://my-ollama:11434/api/generate');
        expect(opts?.method).toBe('POST');

        const body = JSON.parse(opts?.body as string);
        expect(body.model).toBe('llama3');
        expect(body.prompt).toBe('Evaluate this task');
        expect(body.stream).toBe(false);
        expect(body.format).toBe('json');

        expect(result).toBe('{"score":2}');
    });

    it('uses default URL and model when no config provided', async () => {
        fetchMock.mockResolvedValueOnce(jsonResponse({ response: 'ok' }));

        const evaluator = createOllamaEvaluator();
        await evaluator('prompt');

        const [url, opts] = fetchMock.mock.calls[0];
        expect(url).toBe('http://localhost:11434/api/generate');

        const body = JSON.parse(opts?.body as string);
        expect(body.model).toBe('qwen2.5-coder:32b');
    });

    it('throws on non-ok response', async () => {
        fetchMock.mockResolvedValueOnce(textResponse('Internal Server Error', 500));

        const evaluator = createOllamaEvaluator();

        await expect(evaluator('prompt')).rejects.toThrow('Ollama API error: 500');
    });

    it('handles network errors', async () => {
        fetchMock.mockRejectedValueOnce(new Error('ECONNREFUSED'));

        const evaluator = createOllamaEvaluator();
        await expect(evaluator('prompt')).rejects.toThrow('ECONNREFUSED');
    });

    it('respects timeout by using AbortController signal', async () => {
        fetchMock.mockResolvedValueOnce(jsonResponse({ response: 'ok' }));

        const evaluator = createOllamaEvaluator({ timeout: 5000 });
        await evaluator('prompt');

        const [, opts] = fetchMock.mock.calls[0];
        expect(opts?.signal).toBeInstanceOf(AbortSignal);
    });
});

// ===========================================================================
// createOllamaAgent - Definition shape
// ===========================================================================

describe('createOllamaAgent', () => {
    describe('agent definition shape', () => {
        it('returns a valid AgentDefinition with required fields', () => {
            const agent = createOllamaAgent('ollama-1', {});

            expect(agent.id).toBe('ollama-1');
            expect(agent.name).toBe('Ollama (qwen2.5-coder:32b)');
            expect(agent.cost).toBe(0);
            expect(agent.priority).toBe(1);
            expect(agent.enabled).toBe(true);
            expect(agent.requiresApproval).toBe(false);
            expect(typeof agent.execute).toBe('function');
        });

        it('applies custom options', () => {
            const agent = createOllamaAgent('o2', { model: 'codellama' }, {
                name: 'My Ollama',
                cost: 5,
                priority: 3,
            });

            expect(agent.name).toBe('My Ollama');
            expect(agent.cost).toBe(5);
            expect(agent.priority).toBe(3);
        });

        it('uses custom model name in default name', () => {
            const agent = createOllamaAgent('o3', { model: 'deepseek-coder' });
            expect(agent.name).toBe('Ollama (deepseek-coder)');
        });

        it('has correct default capabilities', () => {
            const agent = createOllamaAgent('o4', {});

            expect(agent.capabilities.tiers).toEqual(['trivial', 'simple']);
            expect(agent.capabilities.maxContext).toBe(8000);
            expect(agent.capabilities.canCreatePR).toBe(false);
            expect(agent.capabilities.canExecute).toBe(false);
            expect(agent.capabilities.async).toBe(false);
        });

        it('merges custom capabilities', () => {
            const agent = createOllamaAgent('o5', {}, {
                capabilities: {
                    tiers: ['trivial', 'simple', 'moderate'],
                    maxContext: 16000,
                    canCreatePR: true,
                },
            });

            expect(agent.capabilities.tiers).toEqual(['trivial', 'simple', 'moderate']);
            expect(agent.capabilities.maxContext).toBe(16000);
            expect(agent.capabilities.canCreatePR).toBe(true);
            // Overridden defaults should still be in place for non-overridden keys
            expect(agent.capabilities.canExecute).toBe(false);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Successful response
    // -----------------------------------------------------------------------
    describe('execute() - success', () => {
        it('returns successful result with response data', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ response: 'Here is the fix: change teh to the.' })
            );

            const agent = createOllamaAgent('ollama-test', {
                url: 'http://test:11434',
                model: 'test-model',
            });

            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(true);
            expect(result.data).toBe('Here is the fix: change teh to the.');
            expect(result.cost).toBe(0);
            expect(result.escalate).toBeUndefined();
        });

        it('sends correct prompt formatting', async () => {
            fetchMock.mockResolvedValueOnce(jsonResponse({ response: 'done' }));

            const agent = createOllamaAgent('ollama-prompt', { model: 'test' });
            await agent.execute(sampleTask);

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            expect(body.prompt).toContain('TASK: Fix a typo in README');
            expect(body.prompt).toContain('CONTEXT:');
            expect(body.prompt).toContain('teh');
            expect(body.stream).toBe(false);
        });

        it('trims whitespace from response', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ response: '  trimmed output  \n' })
            );

            const agent = createOllamaAgent('ollama-trim', {});
            const result = await agent.execute(sampleTask);

            expect(result.data).toBe('trimmed output');
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Escalation detection
    // -----------------------------------------------------------------------
    describe('execute() - escalation', () => {
        it('detects ESCALATE: prefix and returns escalation result', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ response: 'ESCALATE: This task requires multi-file refactoring' })
            );

            const agent = createOllamaAgent('ollama-esc', {});
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.escalate).toBe(true);
            expect(result.error).toContain('ESCALATE:');
            expect(result.cost).toBe(0);
        });

        it('detects escalation case-insensitively', async () => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ response: 'escalate: too complex for me' })
            );

            const agent = createOllamaAgent('ollama-esc2', {});
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.escalate).toBe(true);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - HTTP error
    // -----------------------------------------------------------------------
    describe('execute() - HTTP errors', () => {
        it('returns error result on non-ok HTTP status', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Service Unavailable', 503));

            const agent = createOllamaAgent('ollama-err', {});
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Ollama error: 503');
            expect(result.escalate).toBe(true);
            expect(result.cost).toBe(0);
        });

        it('returns error result on 404', async () => {
            fetchMock.mockResolvedValueOnce(textResponse('Not Found', 404));

            const agent = createOllamaAgent('ollama-404', {});
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Ollama error: 404');
            expect(result.escalate).toBe(true);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Network / exception errors
    // -----------------------------------------------------------------------
    describe('execute() - network errors', () => {
        it('catches fetch exceptions and returns error result', async () => {
            fetchMock.mockRejectedValueOnce(new Error('ECONNREFUSED'));

            const agent = createOllamaAgent('ollama-net', {});
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('ECONNREFUSED');
            expect(result.escalate).toBe(true);
            expect(result.cost).toBe(0);
        });

        it('handles non-Error exceptions gracefully', async () => {
            fetchMock.mockRejectedValueOnce('string error');

            const agent = createOllamaAgent('ollama-nonerr', {});
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('Unknown error');
            expect(result.escalate).toBe(true);
        });

        it('handles abort signal timeout', async () => {
            fetchMock.mockRejectedValueOnce(new DOMException('The operation was aborted.', 'AbortError'));

            const agent = createOllamaAgent('ollama-abort', { timeout: 1 });
            const result = await agent.execute(sampleTask);

            expect(result.success).toBe(false);
            expect(result.error).toBe('The operation was aborted.');
            expect(result.escalate).toBe(true);
        });
    });

    // -----------------------------------------------------------------------
    // execute() - Context truncation
    // -----------------------------------------------------------------------
    describe('execute() - context handling', () => {
        it('truncates context to maxContext (8000 chars) in prompt', async () => {
            fetchMock.mockResolvedValueOnce(jsonResponse({ response: 'ok' }));

            const longContext = 'x'.repeat(20000);
            const agent = createOllamaAgent('ollama-ctx', {});
            await agent.execute({
                ...sampleTask,
                context: longContext,
            });

            const body = JSON.parse(fetchMock.mock.calls[0][1]?.body as string);
            // The prompt uses task.context.slice(0, 8000)
            expect(body.prompt).not.toContain('x'.repeat(8001));
            expect(body.prompt.length).toBeLessThan(longContext.length);
        });
    });

    // -----------------------------------------------------------------------
    // Config defaults
    // -----------------------------------------------------------------------
    describe('config defaults', () => {
        it('uses default URL http://localhost:11434', async () => {
            fetchMock.mockResolvedValueOnce(jsonResponse({ response: 'ok' }));

            const agent = createOllamaAgent('ollama-def', {});
            await agent.execute(sampleTask);

            const [url] = fetchMock.mock.calls[0];
            expect(url).toBe('http://localhost:11434/api/generate');
        });

        it('uses custom URL when provided', async () => {
            fetchMock.mockResolvedValueOnce(jsonResponse({ response: 'ok' }));

            const agent = createOllamaAgent('ollama-cust', { url: 'http://custom:9999' });
            await agent.execute(sampleTask);

            const [url] = fetchMock.mock.calls[0];
            expect(url).toBe('http://custom:9999/api/generate');
        });
    });
});
