import { z } from 'zod';
import { createTool, resolveModel } from '../ai.js';
import { sage } from '../handlers/sage.js';

export const sageTool = createTool({
    description: 'Ask Sage for technical advice, task decomposition, or agent routing based on repository context.',
    inputSchema: z.object({
        query: z.string().describe('The question or request for Sage'),
        context: z
            .object({
                repoStructure: z.string().optional().describe('Repository file structure'),
                keyFiles: z.record(z.string(), z.string()).optional().describe('Contents of key files'),
                issueContext: z.string().optional().describe('Context from a GitHub issue or PR'),
                currentContext: z.string().optional().describe('Current working context'),
            })
            .optional(),
    }),
    execute: async ({ query, context }) => {
        const { model } = await resolveModel({});
        return await sage(query, model, context as any);
    },
});
