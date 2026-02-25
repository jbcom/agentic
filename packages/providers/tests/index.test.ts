import { describe, expect, it } from 'vitest';
import * as providers from '../src/index.ts';

/**
 * Tests that the barrel index exports the correct public API surface.
 * This catches accidental removal of exports and ensures type re-exports
 * are correctly wired.
 */
describe('index exports', () => {
    // -----------------------------------------------------------------------
    // Ollama exports
    // -----------------------------------------------------------------------
    describe('Ollama exports', () => {
        it('exports createOllamaAgent function', () => {
            expect(typeof providers.createOllamaAgent).toBe('function');
        });

        it('exports createOllamaEvaluator function', () => {
            expect(typeof providers.createOllamaEvaluator).toBe('function');
        });
    });

    // -----------------------------------------------------------------------
    // Jules exports
    // -----------------------------------------------------------------------
    describe('Jules exports', () => {
        it('exports createJulesAgent function', () => {
            expect(typeof providers.createJulesAgent).toBe('function');
        });

        it('exports pollJulesSession function', () => {
            expect(typeof providers.pollJulesSession).toBe('function');
        });

        it('exports sendJulesFollowUp function', () => {
            expect(typeof providers.sendJulesFollowUp).toBe('function');
        });
    });

    // -----------------------------------------------------------------------
    // Cursor exports
    // -----------------------------------------------------------------------
    describe('Cursor exports', () => {
        it('exports createCursorAgent function', () => {
            expect(typeof providers.createCursorAgent).toBe('function');
        });

        it('exports pollCursorAgent function', () => {
            expect(typeof providers.pollCursorAgent).toBe('function');
        });
    });

    // -----------------------------------------------------------------------
    // Complete API surface
    // -----------------------------------------------------------------------
    describe('complete API surface', () => {
        it('exports exactly the expected set of runtime functions', () => {
            const runtimeExports = Object.entries(providers)
                .filter(([, v]) => typeof v === 'function')
                .map(([k]) => k)
                .sort();

            expect(runtimeExports).toEqual([
                'createCursorAgent',
                'createJulesAgent',
                'createOllamaAgent',
                'createOllamaEvaluator',
                'pollCursorAgent',
                'pollJulesSession',
                'sendJulesFollowUp',
            ]);
        });
    });
});

/**
 * Tests for cost routing characteristics across providers.
 * Validates that the tiered cost model is correctly configured.
 */
describe('cost routing logic', () => {
    it('Ollama is free (cost=0) and highest priority (1)', () => {
        const ollama = providers.createOllamaAgent('ollama', {});
        expect(ollama.cost).toBe(0);
        expect(ollama.priority).toBe(1);
    });

    it('Jules is free tier (cost=0) with medium priority (10)', () => {
        const jules = providers.createJulesAgent('jules', { apiKey: 'k' });
        expect(jules.cost).toBe(0);
        expect(jules.priority).toBe(10);
    });

    it('Cursor is expensive (cost=100) and last resort (priority=100)', () => {
        const cursor = providers.createCursorAgent('cursor', { apiKey: 'k' });
        expect(cursor.cost).toBe(100);
        expect(cursor.priority).toBe(100);
    });

    it('providers are ordered by cost: Ollama < Jules < Cursor', () => {
        const ollama = providers.createOllamaAgent('o', {});
        const jules = providers.createJulesAgent('j', { apiKey: 'k' });
        const cursor = providers.createCursorAgent('c', { apiKey: 'k' });

        const sorted = [cursor, ollama, jules].sort((a, b) => a.cost - b.cost);
        expect(sorted.map((a) => a.id)).toEqual(['o', 'j', 'c']);
    });

    it('providers are ordered by priority: Ollama < Jules < Cursor', () => {
        const ollama = providers.createOllamaAgent('o', {});
        const jules = providers.createJulesAgent('j', { apiKey: 'k' });
        const cursor = providers.createCursorAgent('c', { apiKey: 'k' });

        const sorted = [cursor, ollama, jules].sort((a, b) => a.priority - b.priority);
        expect(sorted.map((a) => a.id)).toEqual(['o', 'j', 'c']);
    });

    it('tier coverage spans all complexity levels', () => {
        const ollama = providers.createOllamaAgent('o', {});
        const jules = providers.createJulesAgent('j', { apiKey: 'k' });
        const cursor = providers.createCursorAgent('c', { apiKey: 'k' });

        const allTiers = new Set([
            ...ollama.capabilities.tiers,
            ...jules.capabilities.tiers,
            ...cursor.capabilities.tiers,
        ]);

        expect(allTiers).toEqual(new Set(['trivial', 'simple', 'moderate', 'complex', 'expert']));
    });

    it('only Cursor requires approval by default', () => {
        const ollama = providers.createOllamaAgent('o', {});
        const jules = providers.createJulesAgent('j', { apiKey: 'k' });
        const cursor = providers.createCursorAgent('c', { apiKey: 'k' });

        expect(ollama.requiresApproval).toBe(false);
        expect(jules.requiresApproval).toBe(false);
        expect(cursor.requiresApproval).toBe(true);
    });

    it('Jules and Cursor are async; Ollama is sync', () => {
        const ollama = providers.createOllamaAgent('o', {});
        const jules = providers.createJulesAgent('j', { apiKey: 'k' });
        const cursor = providers.createCursorAgent('c', { apiKey: 'k' });

        expect(ollama.capabilities.async).toBe(false);
        expect(jules.capabilities.async).toBe(true);
        expect(cursor.capabilities.async).toBe(true);
    });

    it('Jules and Cursor can create PRs; Ollama cannot', () => {
        const ollama = providers.createOllamaAgent('o', {});
        const jules = providers.createJulesAgent('j', { apiKey: 'k' });
        const cursor = providers.createCursorAgent('c', { apiKey: 'k' });

        expect(ollama.capabilities.canCreatePR).toBe(false);
        expect(jules.capabilities.canCreatePR).toBe(true);
        expect(cursor.capabilities.canCreatePR).toBe(true);
    });
});
