import { generateText, stepCountIs } from 'ai';
import { resolveModel } from './model.js';
import type { GenerateOptions, GenerateWithToolsOptions, GenerateWithToolsResult, ToolSet } from './types.js';

/**
 * Generate text using the AI model (no tools)
 */
export async function generate(prompt: string, options: GenerateOptions = {}): Promise<string> {
    const resolved = await resolveModel(options);

    const result = await generateText({
        model: resolved.model,
        system: options.systemPrompt,
        prompt,
        temperature: options.temperature,
        maxOutputTokens: options.maxTokens,
    });

    return result.text;
}

/**
 * Generate text with tools - uses AI SDK's built-in multi-step support
 */
export async function generateWithTools(
    prompt: string,
    tools: ToolSet,
    options: GenerateWithToolsOptions = {}
): Promise<GenerateWithToolsResult> {
    const resolved = await resolveModel(options);
    const maxSteps = options.maxSteps ?? 10;

    const result = await generateText({
        model: resolved.model,
        system: options.systemPrompt,
        prompt,
        tools,
        stopWhen: stepCountIs(maxSteps),
        temperature: options.temperature,
        maxOutputTokens: options.maxTokens,
        onStepFinish: options.onStepFinish
            ? (step) => {
                  options.onStepFinish?.({
                      toolCalls: step.toolCalls,
                      toolResults: step.toolResults,
                      text: step.text,
                  });
              }
            : undefined,
    });

    const allToolCalls: unknown[] = [];
    const allToolResults: unknown[] = [];

    if (result.steps) {
        for (const step of result.steps) {
            if (step.toolCalls) {
                allToolCalls.push(...step.toolCalls);
            }
            if (step.toolResults) {
                allToolResults.push(...step.toolResults);
            }
        }
    }

    return {
        text: result.text,
        toolCalls: allToolCalls,
        toolResults: allToolResults,
        steps: result.steps || [],
        finishReason: result.finishReason,
    };
}

export { stepCountIs } from 'ai';
