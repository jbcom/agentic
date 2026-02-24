import { generateObject, type LanguageModel } from 'ai';
import {
    type AgentRouting,
    AgentRoutingSchema,
    type SageQueryType,
    type SageResponse,
    SageResponseSchema,
    type TaskDecomposition,
    TaskDecompositionSchema,
    type UnblockResponse,
    UnblockResponseSchema,
} from '../schemas/sage.js';

export type { AgentRouting, SageResponse, TaskDecomposition, UnblockResponse, SageQueryType };

/**
 * Context that can be provided to Sage for better responses
 */
export interface SageContext {
    /** Repository structure (file list) */
    repoStructure?: string;
    /** Content from key files (README, CLAUDE.md, etc.) */
    keyFiles?: Record<string, string>;
    /** Issue or PR context if available */
    issueContext?: string;
    /** Current working directory or file context */
    currentContext?: string;
}

/**
 * Classify a query into a SageQueryType
 */
export function classifyQuery(query: string): SageQueryType {
    const lower = query.toLowerCase();

    if (/\b(review|feedback|look at|check)\b/.test(lower)) return 'review';
    if (/\b(blocked|stuck|help|unblock|can't|cannot)\b/.test(lower)) return 'unblock';
    if (/\b(how|what|why|explain|where|when)\b/.test(lower)) return 'question';
    if (/\b(fix|bug|error|broken|failing|crash)\b/.test(lower)) return 'fix';
    if (/\b(implement|create|add|build|make|write)\b/.test(lower)) return 'implement';
    if (/\b(refactor|cleanup|improve|optimize|reorganize)\b/.test(lower)) return 'refactor';
    if (/\b(decompose|break down|plan|tasks|subtasks|steps)\b/.test(lower)) return 'decompose';
    if (/\b(route|assign|delegate|who should)\b/.test(lower)) return 'route';

    return 'general';
}

/**
 * Build the system prompt for Sage
 */
function buildSystemPrompt(context?: SageContext): string {
    let prompt = `You are the Ecosystem Sage - an intelligent advisor for software development.

Your role:
1. Answer technical questions accurately and concisely
2. Provide code review feedback when asked
3. Decompose complex tasks into actionable subtasks
4. Help unblock stuck developers and agents
5. Route work to the appropriate agent (Cursor, Jules, Claude, or Human)

Agent Capabilities:
- CURSOR: Best for quick fixes (<10 lines), single-file changes, debugging, CI failure resolution
- JULES: Best for multi-file refactors, documentation, complex feature architecture
- CLAUDE: Best for complex reasoning, implementation of new features, deep analysis
- OLLAMA: Best for quick answers, simple query classification, local task execution
- HUMAN: Required for product decisions, security reviews, architecture approval, human-in-the-loop validation

Guidelines:
- Be concise and actionable
- Reference specific files when relevant
- Never hallucinate - if unsure, say so
- Provide confidence levels honestly
- Format responses in Markdown
- Align with jbcom/control-center ecosystem workflows`;

    if (context?.repoStructure) {
        prompt += `\n\nRepository Structure:\n${context.repoStructure}`;
    }

    if (context?.keyFiles) {
        prompt += '\n\nKey Files:';
        for (const [file, content] of Object.entries(context.keyFiles)) {
            prompt += `\n\n=== ${file} ===\n${content.slice(0, 2000)}`;
        }
    }

    if (context?.issueContext) {
        prompt += `\n\nIssue/PR Context:\n${context.issueContext}`;
    }

    if (context?.currentContext) {
        prompt += `\n\nCurrent Context:\n${context.currentContext}`;
    }

    return prompt;
}

/**
 * Answer a question or provide guidance using Sage
 *
 * @param query - The question or request
 * @param model - The Vercel AI SDK model to use
 * @param context - Optional context for better responses
 * @returns Structured Sage response
 */
export async function answerQuestion(
    query: string,
    model: LanguageModel,
    context?: SageContext
): Promise<SageResponse> {
    if (!query?.trim()) {
        throw new Error('Query is required');
    }

    const queryType = classifyQuery(query);

    const result = await generateObject({
        model,
        schema: SageResponseSchema,
        system: buildSystemPrompt(context),
        prompt: `Query Type: ${queryType}

Query:
${query}

Provide a helpful, accurate response. Include file references if relevant.
If the query suggests work that should be delegated, include an agent recommendation.`,
    });

    return result.object;
}

/**
 * Decompose a complex task into subtasks with agent assignments
 *
 * @param task - The task to decompose
 * @param model - The Vercel AI SDK model to use
 * @param context - Optional context for better decomposition
 * @returns Structured task decomposition
 */
export async function decomposeTask(
    task: string,
    model: LanguageModel,
    context?: SageContext
): Promise<TaskDecomposition> {
    if (!task?.trim()) {
        throw new Error('Task is required');
    }

    const result = await generateObject({
        model,
        schema: TaskDecompositionSchema,
        system: buildSystemPrompt(context),
        prompt: `Decompose this task into specific, actionable subtasks.

Task:
${task}

For each subtask:
1. Assign a unique ID (task-001, task-002, etc.)
2. Write a clear, actionable title
3. Provide detailed description
4. Assign to the most appropriate agent (cursor/jules/ollama/human)
5. Set priority (1=highest, 10=lowest)
6. Estimate effort (small/medium/large)
7. Note dependencies on other subtasks

Order subtasks logically for execution.`,
    });

    return result.object;
}

/**
 * Determine which agent should handle a task
 *
 * @param task - The task description
 * @param model - The Vercel AI SDK model to use
 * @param context - Optional context for better routing
 * @returns Agent routing decision
 */
export async function routeToAgent(task: string, model: LanguageModel, context?: SageContext): Promise<AgentRouting> {
    if (!task?.trim()) {
        throw new Error('Task is required');
    }

    const result = await generateObject({
        model,
        schema: AgentRoutingSchema,
        system: buildSystemPrompt(context),
        prompt: `Determine which agent should handle this task.

Task:
${task}

Consider:
- Task complexity and scope
- Number of files likely affected
- Need for human judgment
- Security implications
- Time sensitivity

Provide clear reasoning and specific instructions for the chosen agent.`,
    });

    return result.object;
}

/**
 * Help unblock stuck work
 *
 * @param situation - Description of the blocked situation
 * @param model - The Vercel AI SDK model to use
 * @param context - Optional context for better diagnosis
 * @returns Unblock suggestions
 */
export async function unblock(
    situation: string,
    model: LanguageModel,
    context?: SageContext
): Promise<UnblockResponse> {
    if (!situation?.trim()) {
        throw new Error('Situation description is required');
    }

    const result = await generateObject({
        model,
        schema: UnblockResponseSchema,
        system: buildSystemPrompt(context),
        prompt: `Analyze this blocked situation and provide suggestions to unblock.

Situation:
${situation}

Diagnose the root cause and provide:
1. Clear diagnosis of what's blocking
2. Root cause analysis
3. Ordered list of suggestions (most likely to work first)
4. The single most important immediate action
5. Whether human intervention is required`,
    });

    return result.object;
}

/**
 * High-level Sage function that auto-routes to the appropriate handler
 *
 * @param query - The question, task, or situation
 * @param model - The Vercel AI SDK model to use
 * @param context - Optional context
 * @returns Response appropriate to the query type
 */
export async function sage(
    query: string,
    model: LanguageModel,
    context?: SageContext
): Promise<SageResponse | TaskDecomposition | AgentRouting | UnblockResponse> {
    const queryType = classifyQuery(query);

    switch (queryType) {
        case 'decompose':
            return decomposeTask(query, model, context);
        case 'route':
            return routeToAgent(query, model, context);
        case 'unblock':
            return unblock(query, model, context);
        default:
            return answerQuestion(query, model, context);
    }
}
