# TypeScript Integration Examples

This directory contains ready-to-use TypeScript code for integrating with `agentic-crew`.

## Files

| File | Description |
|------|-------------|
| `agentic-crew-types.ts` | TypeScript type definitions for CLI JSON output |
| `crew-tool.ts` | Complete client implementation for invoking crews |
| `vercel-ai-integration.ts` | Vercel AI SDK integration with tool definitions |

## Quick Start

### 1. Install Dependencies

```bash
npm install ai @ai-sdk/anthropic zod
```

### 2. Install agentic-crew (Python)

```bash
pip install 'agentic-crew[crewai]'
```

### 3. Copy Files

Copy the TypeScript files to your project:

```bash
cp agentic-crew-types.ts your-project/src/types/
cp crew-tool.ts your-project/src/crews/
cp vercel-ai-integration.ts your-project/src/tools/
```

### 4. Use in Your Code

```typescript
import { CrewTool } from './crews/crew-tool';
import { invokeCrewTool, listCrewsTool } from './tools/vercel-ai-integration';

// Direct invocation
const crewTool = new CrewTool();
const result = await crewTool.invokeCrew({
  package: 'vendor-connectors',
  crew: 'connector_builder',
  input: 'Build a connector for the Stripe API',
});

// With Vercel AI SDK
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const response = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: { invokeCrew: invokeCrewTool, listCrews: listCrewsTool },
  prompt: 'List available crews and build a game component',
});
```

## Environment Variables

Set these environment variables:

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

## Documentation

See the full [Integration Guide](../../docs/INTEGRATION.md) for:

- CLI contract documentation
- JSON output schemas
- Exit codes
- Best practices
- Error handling

## License

MIT - Same as agentic-crew
