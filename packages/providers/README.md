# @jbcom/agentic-providers

[![npm version](https://img.shields.io/npm/v/@jbcom/agentic-providers.svg)](https://www.npmjs.com/package/@jbcom/agentic-providers)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

LLM and agent provider implementations for the Agentic ecosystem. Connects the provider-agnostic primitives from `@jbcom/agentic-triage` to real services including Ollama (free, self-hosted), Google Jules (free tier, async), and Cursor (premium, last resort). Includes cost-aware routing and complexity evaluation.

[Full Documentation](https://agentic.coach) | [Package Docs](https://agentic.coach/packages/control/)

## Installation

```bash
npm install @jbcom/agentic-providers @jbcom/agentic-triage
# or
pnpm add @jbcom/agentic-providers @jbcom/agentic-triage
```

## Quick Start

```typescript
import { createOllamaAgent, createOllamaEvaluator, createJulesAgent, createCursorAgent } from '@jbcom/agentic-providers';

// Create a complexity evaluator
const evaluate = createOllamaEvaluator({
  url: 'http://localhost:11434',
  model: 'qwen2.5-coder:32b',
});

// Register agents with a task router
import { AgentRegistry, TaskRouter, evaluateComplexity } from '@jbcom/agentic-triage';

const registry = new AgentRegistry()
  .register(createOllamaAgent('ollama', { url: 'http://localhost:11434' }))
  .register(createJulesAgent('jules', { apiKey: process.env.JULES_API_KEY! }))
  .register(createCursorAgent('cursor', { apiKey: process.env.CURSOR_API_KEY! }, {
    requiresApproval: true,
  }));

const router = new TaskRouter({ registry, dailyBudget: 50 });

// Evaluate and route a task
const score = await evaluateComplexity(evaluate, 'Fix the login bug', diff);
const result = await router.route({ id: 'task-1', description: 'Fix the login bug', context: diff }, score);
```

## Providers

| Provider | Cost | Use Case | Export |
|----------|------|----------|--------|
| **Ollama** | Free | Simple tasks, local inference | `@jbcom/agentic-providers/ollama` |
| **Jules** | Free tier | Complex tasks, PR creation | `@jbcom/agentic-providers/jules` |
| **Cursor** | Premium | Expert-level, last resort | `@jbcom/agentic-providers/cursor` |

## Exports

| Subpath | Exports |
|---------|---------|
| `@jbcom/agentic-providers` | All providers |
| `@jbcom/agentic-providers/ollama` | `createOllamaAgent`, `createOllamaEvaluator` |
| `@jbcom/agentic-providers/jules` | `createJulesAgent`, `pollJulesSession`, `sendJulesFollowUp` |
| `@jbcom/agentic-providers/cursor` | `createCursorAgent`, `pollCursorAgent` |

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation.

## License

MIT
