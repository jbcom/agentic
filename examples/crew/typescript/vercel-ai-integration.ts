/**
 * Vercel AI SDK Integration Example
 *
 * This example shows how to integrate agentic-crew with the Vercel AI SDK
 * to enable AI agents to delegate tasks to specialized crews.
 *
 * Prerequisites:
 * - npm install ai @ai-sdk/anthropic zod
 * - Python with agentic-crew installed
 * - ANTHROPIC_API_KEY environment variable set
 *
 * @see https://sdk.vercel.ai/docs
 * @see https://github.com/jbcom/agentic-crew/blob/main/docs/INTEGRATION.md
 */

import { tool, generateText, streamText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { z } from 'zod';
import { CrewTool } from './crew-tool';
import type { CrewRunResult, CrewSummary } from './agentic-crew-types';

// ============================================================================
// TOOL DEFINITIONS
// ============================================================================

const crewTool = new CrewTool({
  // Enable debug logging in development
  debug: process.env.NODE_ENV === 'development',
  // 10 minute timeout for long-running crews
  defaultTimeoutMs: 600000,
});

/**
 * Tool for listing available crews.
 * Use this to discover what crews are available before invoking them.
 */
export const listCrewsTool = tool({
  description:
    'List available AI crews. Use this to discover what specialized crews are available ' +
    'before delegating tasks. Returns crew names, descriptions, and required frameworks.',
  parameters: z.object({
    packageFilter: z
      .string()
      .optional()
      .describe('Optional: filter to a specific package (e.g., "otterfall", "vendor-connectors")'),
  }),
  execute: async ({ packageFilter }) => {
    const result = await crewTool.listCrews(packageFilter);

    // Format for LLM consumption
    const formatted = result.crews
      .map(
        (c: CrewSummary) =>
          `- **${c.package}/${c.name}**: ${c.description}` +
          (c.required_framework ? ` [requires: ${c.required_framework}]` : '')
      )
      .join('\n');

    return `Available crews:\n${formatted}`;
  },
});

/**
 * Tool for invoking a crew.
 * Use this to delegate complex tasks to specialized multi-agent crews.
 */
export const invokeCrewTool = tool({
  description:
    'Delegate a task to a specialized AI crew. Use this when a task requires ' +
    'specialized agents working together, such as:\n' +
    '- Code generation (game components, connectors, APIs)\n' +
    '- Content creation (documentation, specifications)\n' +
    '- Analysis tasks (API documentation, code review)\n\n' +
    'The crew will execute autonomously and return results. This may take several minutes.',
  parameters: z.object({
    package: z
      .string()
      .describe('Package containing the crew (e.g., "otterfall", "vendor-connectors")'),
    crew: z.string().describe('Crew name (e.g., "game_builder", "connector_builder")'),
    input: z
      .string()
      .describe(
        'Detailed task specification for the crew. Be specific about requirements, ' +
          'constraints, and expected output format.'
      ),
  }),
  execute: async ({ package: pkg, crew, input }): Promise<string> => {
    console.log(`\n🚀 Invoking crew: ${pkg}/${crew}`);
    console.log(`   Input: ${input.substring(0, 100)}...`);

    const result: CrewRunResult = await crewTool.invokeCrew({
      package: pkg,
      crew,
      input,
    });

    if (!result.success) {
      const errorMsg = `Crew execution failed: ${result.error}`;
      if (result.available_packages) {
        throw new Error(`${errorMsg}\nAvailable packages: ${result.available_packages.join(', ')}`);
      }
      throw new Error(errorMsg);
    }

    console.log(`✅ Crew completed in ${result.duration_ms}ms using ${result.framework_used}`);

    return result.output || 'Crew completed with no output';
  },
});

// ============================================================================
// EXAMPLE USAGE
// ============================================================================

/**
 * Example: Basic agent with crew delegation
 */
async function basicExample() {
  console.log('\n=== Basic Crew Delegation Example ===\n');

  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: {
      listCrews: listCrewsTool,
      invokeCrew: invokeCrewTool,
    },
    maxSteps: 5,
    prompt:
      'First, list the available crews. Then, using the game_builder crew from ' +
      'the otterfall package, create a simple HealthComponent for an ECS system.',
  });

  console.log('\n--- Result ---');
  console.log(result.text);
}

/**
 * Example: Streaming response with crew delegation
 */
async function streamingExample() {
  console.log('\n=== Streaming Crew Delegation Example ===\n');

  const result = streamText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: {
      listCrews: listCrewsTool,
      invokeCrew: invokeCrewTool,
    },
    maxSteps: 5,
    prompt:
      'Help me build an HTTP connector for a REST API. First check what crews are ' +
      'available, then delegate to the appropriate crew to build the connector.',
  });

  // Stream the text response
  for await (const chunk of result.textStream) {
    process.stdout.write(chunk);
  }

  console.log('\n');
}

/**
 * Example: Multi-step workflow with validation
 */
async function multiStepExample() {
  console.log('\n=== Multi-Step Workflow Example ===\n');

  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: {
      listCrews: listCrewsTool,
      invokeCrew: invokeCrewTool,
    },
    maxSteps: 10,
    system: `You are a project coordinator that delegates specialized tasks to AI crews.
Your workflow:
1. Understand the user's request
2. List available crews to find the best match
3. Break down complex requests into crew-appropriate tasks
4. Invoke crews with detailed, specific instructions
5. Synthesize and present the final results

Always provide clear context when invoking crews. Include:
- Specific requirements
- Expected output format
- Any constraints or preferences`,
    prompt:
      'I need to build a quest system for a game. It should have:\n' +
      '- Quest objectives that can be tracked\n' +
      '- Rewards for completion\n' +
      '- Support for main quests and side quests\n\n' +
      'Please coordinate with the appropriate crews to design and implement this.',
  });

  console.log('\n--- Final Result ---');
  console.log(result.text);

  // Show tool usage statistics
  console.log('\n--- Tool Usage ---');
  console.log(`Steps taken: ${result.steps.length}`);
  for (const step of result.steps) {
    if (step.toolCalls) {
      for (const call of step.toolCalls) {
        console.log(`- ${call.toolName}(${JSON.stringify(call.args).substring(0, 100)}...)`);
      }
    }
  }
}

// ============================================================================
// ADVANCED PATTERNS
// ============================================================================

/**
 * Create a crew tool with caching for the crew list.
 * Useful when the same crews are invoked repeatedly.
 */
export function createCachedCrewTools(cacheTtlMs = 60000) {
  let cachedList: { crews: CrewSummary[]; timestamp: number } | null = null;

  const getCachedList = async () => {
    if (cachedList && Date.now() - cachedList.timestamp < cacheTtlMs) {
      return cachedList.crews;
    }

    const result = await crewTool.listCrews();
    cachedList = { crews: result.crews, timestamp: Date.now() };
    return result.crews;
  };

  return {
    listCrews: tool({
      description: 'List available AI crews (cached)',
      parameters: z.object({}),
      execute: async () => {
        const crews = await getCachedList();
        return crews
          .map((c) => `- ${c.package}/${c.name}: ${c.description}`)
          .join('\n');
      },
    }),

    invokeCrew: tool({
      description: 'Invoke an AI crew with validation',
      parameters: z.object({
        package: z.string(),
        crew: z.string(),
        input: z.string(),
      }),
      execute: async ({ package: pkg, crew, input }) => {
        // Validate crew exists before invoking
        const crews = await getCachedList();
        const exists = crews.some(
          (c) => c.package === pkg && c.name === crew
        );

        if (!exists) {
          const available = crews
            .map((c) => `${c.package}/${c.name}`)
            .join(', ');
          throw new Error(
            `Crew ${pkg}/${crew} not found. Available: ${available}`
          );
        }

        const result = await crewTool.invokeCrew({ package: pkg, crew, input });

        if (!result.success) {
          throw new Error(result.error || 'Crew execution failed');
        }

        return result.output || '';
      },
    }),
  };
}

/**
 * Create specialized tool for a specific crew.
 * Useful when you want to expose a single crew with custom parameters.
 */
export function createSpecializedCrewTool(
  packageName: string,
  crewName: string,
  description: string,
  inputSchema: z.ZodType
) {
  return tool({
    description,
    parameters: z.object({
      input: inputSchema,
    }),
    execute: async ({ input }) => {
      const inputStr =
        typeof input === 'string' ? input : JSON.stringify(input, null, 2);

      const result = await crewTool.invokeCrew({
        package: packageName,
        crew: crewName,
        input: inputStr,
      });

      if (!result.success) {
        throw new Error(result.error || 'Crew execution failed');
      }

      return result.output || '';
    },
  });
}

// Example: Specialized connector builder tool
export const buildConnectorTool = createSpecializedCrewTool(
  'vendor-connectors',
  'connector_builder',
  'Build an HTTP connector for a REST API. Provide the API documentation URL or specification.',
  z.object({
    apiDocsUrl: z.string().url().describe('URL to the API documentation'),
    apiName: z.string().describe('Name for the generated connector'),
    authType: z
      .enum(['api_key', 'oauth2', 'bearer', 'none'])
      .optional()
      .describe('Authentication type to implement'),
  })
);

// ============================================================================
// RUN EXAMPLES
// ============================================================================

// Uncomment to run examples:
// basicExample().catch(console.error);
// streamingExample().catch(console.error);
// multiStepExample().catch(console.error);

export { crewTool };
