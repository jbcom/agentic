---
editUrl: false
next: false
prev: false
title: "routeToAgent"
---

> **routeToAgent**(`task`, `model`, `context?`): `Promise`\<\{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `alternatives?`: `object`[]; `confidence`: `number`; `instructions`: `string`; `reason`: `string`; \}\>

Defined in: [packages/triage/src/handlers/sage.ts:182](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L182)

Determine which agent should handle a task

## Parameters

### task

`string`

The task description

### model

`LanguageModel`

The Vercel AI SDK model to use

### context?

[`SageContext`](/api/triage/jbcom/namespaces/handlers/interfaces/sagecontext/)

Optional context for better routing

## Returns

`Promise`\<\{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `alternatives?`: `object`[]; `confidence`: `number`; `instructions`: `string`; `reason`: `string`; \}\>

Agent routing decision
