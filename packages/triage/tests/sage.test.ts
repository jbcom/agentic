import { describe, expect, it } from 'vitest';
import { classifyQuery } from '../src/handlers/sage.js';
import {
    AgentRoutingSchema,
    AgentTypeSchema,
    SageQueryTypeSchema,
    SageResponseSchema,
    TaskDecompositionSchema,
    UnblockResponseSchema,
} from '../src/schemas/sage.js';

describe('Sage Schemas', () => {
    describe('SageQueryTypeSchema', () => {
        it('accepts valid query types', () => {
            const validTypes = [
                'question',
                'review',
                'fix',
                'implement',
                'refactor',
                'decompose',
                'unblock',
                'route',
                'general',
            ];
            for (const type of validTypes) {
                expect(SageQueryTypeSchema.parse(type)).toBe(type);
            }
        });

        it('rejects invalid query types', () => {
            expect(() => SageQueryTypeSchema.parse('invalid')).toThrow();
        });
    });

    describe('AgentTypeSchema', () => {
        it('accepts valid agent types', () => {
            const validAgents = ['cursor', 'jules', 'ollama', 'claude', 'human'];
            for (const agent of validAgents) {
                expect(AgentTypeSchema.parse(agent)).toBe(agent);
            }
        });
    });

    describe('SageResponseSchema', () => {
        it('validates a complete response', () => {
            const response = {
                answer: 'Use the useEffect hook for side effects.',
                queryType: 'question',
                confidence: 0.95,
                references: ['src/components/App.tsx'],
                followUp: 'Consider using useMemo for expensive computations',
                agentRecommendation: {
                    agent: 'cursor',
                    reason: 'Simple single-file change',
                    instructions: 'Add useEffect to handle the subscription',
                },
            };
            expect(SageResponseSchema.parse(response)).toEqual(response);
        });

        it('validates a minimal response', () => {
            const response = {
                answer: 'Yes, that approach works.',
                queryType: 'general',
                confidence: 0.8,
            };
            expect(SageResponseSchema.parse(response)).toEqual(response);
        });

        it('rejects invalid confidence', () => {
            const response = {
                answer: 'Test',
                queryType: 'general',
                confidence: 1.5, // Invalid: > 1
            };
            expect(() => SageResponseSchema.parse(response)).toThrow();
        });
    });

    describe('TaskDecompositionSchema', () => {
        it('validates a complete decomposition', () => {
            const decomposition = {
                originalTask: 'Implement user authentication',
                subtasks: [
                    {
                        id: 'task-001',
                        title: 'Create auth schema',
                        description: 'Define Zod schemas for login/register',
                        agent: 'cursor',
                        priority: 1,
                        effort: 'small',
                    },
                    {
                        id: 'task-002',
                        title: 'Implement auth handlers',
                        description: 'Create login, register, logout handlers',
                        agent: 'jules',
                        priority: 2,
                        effort: 'medium',
                        dependencies: ['task-001'],
                    },
                ],
                executionOrder: ['task-001', 'task-002'],
                estimatedTotalEffort: 'medium',
                notes: 'Consider using OAuth for social login',
            };
            expect(TaskDecompositionSchema.parse(decomposition)).toEqual(decomposition);
        });
    });

    describe('AgentRoutingSchema', () => {
        it('validates a routing decision', () => {
            const routing = {
                agent: 'jules',
                reason: 'Multi-file refactor requiring documentation updates',
                instructions: 'Refactor the auth module and update README',
                confidence: 0.9,
                alternatives: [
                    {
                        agent: 'cursor',
                        reason: 'Could handle if split into smaller tasks',
                    },
                ],
            };
            expect(AgentRoutingSchema.parse(routing)).toEqual(routing);
        });
    });

    describe('UnblockResponseSchema', () => {
        it('validates an unblock response', () => {
            const unblock = {
                diagnosis: 'CI is failing due to missing environment variable',
                rootCause: 'The DATABASE_URL secret was not added to the workflow',
                suggestions: [
                    {
                        action: 'Add DATABASE_URL to repository secrets',
                        effort: 'trivial',
                        likelihood: 0.95,
                    },
                    {
                        action: 'Check if the database is accessible',
                        effort: 'small',
                        likelihood: 0.3,
                    },
                ],
                immediateAction: 'Add DATABASE_URL to repository secrets',
                needsHuman: false,
            };
            expect(UnblockResponseSchema.parse(unblock)).toEqual(unblock);
        });

        it('validates response requiring human', () => {
            const unblock = {
                diagnosis: 'Architecture decision needed',
                rootCause: 'Conflicting requirements from stakeholders',
                suggestions: [],
                immediateAction: 'Schedule architecture review meeting',
                needsHuman: true,
                escalationReason: 'Requires product decision on feature scope',
            };
            expect(UnblockResponseSchema.parse(unblock)).toEqual(unblock);
        });
    });
});

describe('Sage Handlers', () => {
    describe('classifyQuery', () => {
        it('classifies questions', () => {
            expect(classifyQuery('How do I add a new endpoint?')).toBe('question');
            expect(classifyQuery('What is the purpose of this file?')).toBe('question');
            expect(classifyQuery('Why is this failing?')).toBe('question');
            expect(classifyQuery('Explain the auth flow')).toBe('question');
        });

        it('classifies review requests', () => {
            expect(classifyQuery('Review this code')).toBe('review');
            expect(classifyQuery('Can you give me feedback on this?')).toBe('review');
            expect(classifyQuery('Look at my implementation')).toBe('review');
        });

        it('classifies fix requests', () => {
            expect(classifyQuery('Fix the bug in auth.ts')).toBe('fix');
            expect(classifyQuery('There is an error in the login')).toBe('fix');
            expect(classifyQuery('The tests are broken')).toBe('fix');
        });

        it('classifies implementation requests', () => {
            expect(classifyQuery('Implement user registration')).toBe('implement');
            expect(classifyQuery('Create a new API endpoint')).toBe('implement');
            expect(classifyQuery('Add a dark mode toggle')).toBe('implement');
            expect(classifyQuery('Build the dashboard')).toBe('implement');
        });

        it('classifies refactor requests', () => {
            expect(classifyQuery('Refactor the auth module')).toBe('refactor');
            expect(classifyQuery('Cleanup this code')).toBe('refactor');
            expect(classifyQuery('Improve the performance')).toBe('refactor');
        });

        it('classifies decomposition requests', () => {
            expect(classifyQuery('Decompose this epic into tasks')).toBe('decompose');
            expect(classifyQuery('Break down the feature')).toBe('decompose');
            expect(classifyQuery('Plan the implementation')).toBe('decompose');
        });

        it('classifies unblock requests', () => {
            expect(classifyQuery("I'm blocked on the CI")).toBe('unblock');
            expect(classifyQuery("I'm stuck and need help")).toBe('unblock');
            expect(classifyQuery("Can't figure out why this fails")).toBe('unblock');
        });

        it('classifies routing requests', () => {
            expect(classifyQuery('Route this to the right agent')).toBe('route');
            expect(classifyQuery('Who should handle this task?')).toBe('route');
            expect(classifyQuery('Assign this issue')).toBe('route');
        });

        it('defaults to general', () => {
            expect(classifyQuery('Hello')).toBe('general');
            expect(classifyQuery('Thanks for the update')).toBe('general');
        });
    });
});
