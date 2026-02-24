---
editUrl: false
next: false
prev: false
title: "CursorAgentResult"
---

Defined in: [cursor.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L19)

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

### agentId

> **agentId**: `string`

Defined in: [cursor.ts:21](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L21)

Agent ID

***

### messages?

> `optional` **messages**: `object`[]

Defined in: [cursor.ts:25](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L25)

Output messages

#### content

> **content**: `string`

#### role

> **role**: `string`

***

### status

> **status**: `string`

Defined in: [cursor.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L23)

Current status
