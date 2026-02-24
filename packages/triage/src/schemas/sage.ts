import { z } from 'zod';

/**
 * Query types that Sage can handle
 */
export const SageQueryTypeSchema = z.enum([
    'question',
    'review',
    'fix',
    'implement',
    'refactor',
    'decompose',
    'unblock',
    'route',
    'general',
]);

export type SageQueryType = z.infer<typeof SageQueryTypeSchema>;

/**
 * Agent types for task routing
 */
export const AgentTypeSchema = z.enum(['cursor', 'jules', 'ollama', 'claude', 'human']);

export type AgentType = z.infer<typeof AgentTypeSchema>;

/**
 * Common effort levels for tasks and suggestions
 */
export const EffortSchema = z.enum(['trivial', 'small', 'medium', 'large', 'epic']);

export type Effort = z.infer<typeof EffortSchema>;

/**
 * Schema for Sage Q&A responses
 */
export const SageResponseSchema = z.object({
    answer: z.string().describe('The answer to the question or query'),
    queryType: SageQueryTypeSchema.describe('The classified type of the query'),
    confidence: z.number().min(0).max(1).describe('Confidence level in the response (0-1)'),
    references: z.array(z.string()).optional().describe('File paths or documentation referenced in the answer'),
    followUp: z.string().optional().describe('Suggested follow-up action or question'),
    agentRecommendation: z
        .object({
            agent: AgentTypeSchema.describe('Recommended agent to handle follow-up work'),
            reason: z.string().describe('Why this agent is recommended'),
            instructions: z.string().optional().describe('Specific instructions for the agent'),
        })
        .optional()
        .describe('If the query requires action, which agent should handle it'),
});

export type SageResponse = z.infer<typeof SageResponseSchema>;

/**
 * Schema for task decomposition
 */
export const SubtaskSchema = z.object({
    id: z.string().describe('Unique identifier for the subtask (e.g., task-001)'),
    title: z.string().describe('Clear, actionable title for the subtask'),
    description: z.string().describe('Detailed description of what needs to be done'),
    agent: AgentTypeSchema.describe('Which agent should handle this subtask'),
    priority: z.number().min(1).max(10).describe('Priority from 1 (highest) to 10 (lowest)'),
    effort: EffortSchema.describe('Estimated effort level'),
    dependencies: z.array(z.string()).optional().describe('IDs of subtasks this depends on'),
});

export type Subtask = z.infer<typeof SubtaskSchema>;

export const TaskDecompositionSchema = z.object({
    originalTask: z.string().describe('The original task that was decomposed'),
    subtasks: z.array(SubtaskSchema).describe('List of subtasks to complete'),
    executionOrder: z.array(z.string()).optional().describe('Recommended order to execute subtasks by ID'),
    estimatedTotalEffort: EffortSchema.describe('Total estimated effort for all subtasks'),
    notes: z.string().optional().describe('Additional notes or considerations'),
});

export type TaskDecomposition = z.infer<typeof TaskDecompositionSchema>;

/**
 * Schema for agent routing decisions
 */
export const AgentRoutingSchema = z.object({
    agent: AgentTypeSchema.describe('The recommended agent to handle the task'),
    reason: z.string().describe('Why this agent was chosen'),
    instructions: z.string().describe('Specific instructions for the agent'),
    confidence: z.number().min(0).max(1).describe('Confidence in the routing decision'),
    alternatives: z
        .array(
            z.object({
                agent: AgentTypeSchema,
                reason: z.string(),
            })
        )
        .optional()
        .describe('Alternative agents that could handle the task'),
});

export type AgentRouting = z.infer<typeof AgentRoutingSchema>;

/**
 * Schema for unblocking stuck work
 */
export const UnblockResponseSchema = z.object({
    diagnosis: z.string().describe('Analysis of what is blocking progress'),
    rootCause: z.string().describe('The underlying root cause of the blockage'),
    suggestions: z
        .array(
            z.object({
                action: z.string().describe('Specific action to take'),
                effort: EffortSchema,
                likelihood: z.number().min(0).max(1).describe('Likelihood this will unblock'),
            })
        )
        .describe('Ordered list of suggestions to unblock'),
    immediateAction: z.string().describe('The single most important thing to do right now'),
    needsHuman: z.boolean().describe('Whether human intervention is required'),
    escalationReason: z.string().optional().describe('If needs human, why escalation is needed'),
});

export type UnblockResponse = z.infer<typeof UnblockResponseSchema>;
