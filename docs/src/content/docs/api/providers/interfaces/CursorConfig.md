---
editUrl: false
next: false
prev: false
title: "CursorConfig"
---

Defined in: [cursor.ts:10](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L10)

@jbcom/agentic-providers

LLM and agent provider implementations for use with @jbcom/agentic-triage.

These implementations connect the provider-agnostic primitives from
@jbcom/agentic-triage to real LLM services like Ollama, Jules, and Cursor.

## Example

```typescript
import { AgentRegistry, evaluateComplexity, TaskRouter } from '@jbcom/agentic-triage';
import { createOllamaAgent, createOllamaEvaluator, createJulesAgent } from '@jbcom/agentic-providers';

// Create evaluator for complexity scoring
const evaluate = createOllamaEvaluator({ url: 'http://ollama:11434' });

// Set up agent registry with your preferred providers
const registry = new AgentRegistry()
  .register(createOllamaAgent('ollama', { url: 'http://ollama:11434' }))
  .register(createJulesAgent('jules', { apiKey: process.env.JULES_API_KEY }));

// Create router
const router = new TaskRouter({ registry });

// Evaluate and route a task
const score = await evaluateComplexity(evaluate, 'Fix the bug', diff);
const result = await router.route(task, score);
```

## Properties

### apiKey

> **apiKey**: `string`

Defined in: [cursor.ts:12](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L12)

Cursor API key

***

### baseUrl?

> `optional` **baseUrl**: `string`

Defined in: [cursor.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L14)

API base URL (default: https://api.cursor.com/v0)

***

### workspacePath?

> `optional` **workspacePath**: `string`

Defined in: [cursor.ts:16](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L16)

Workspace path in the cloud environment
