import { tool } from 'ai';
import type { z } from 'zod';

/**
 * Create a simple tool definition helper
 * Wraps the AI SDK's tool() function for convenience
 */
export function createTool<T extends z.ZodType>(config: {
    description: string;
    inputSchema: T;
    execute: (input: z.infer<T>) => Promise<unknown>;
}) {
    return tool({
        description: config.description,
        inputSchema: config.inputSchema,
        execute: config.execute,
    });
}

export { tool } from 'ai';
export { z } from 'zod';
