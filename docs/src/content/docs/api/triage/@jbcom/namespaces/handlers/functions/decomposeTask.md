---
editUrl: false
next: false
prev: false
title: "decomposeTask"
---

> **decomposeTask**(`task`, `model`, `context?`): `Promise`\<\{ `estimatedTotalEffort`: `"epic"` \| `"medium"` \| `"trivial"` \| `"small"` \| `"large"`; `executionOrder?`: `string`[]; `notes?`: `string`; `originalTask`: `string`; `subtasks`: `object`[]; \}\>

Defined in: [packages/triage/src/handlers/sage.ts:141](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L141)

Decompose a complex task into subtasks with agent assignments

## Parameters

### task

`string`

The task to decompose

### model

`LanguageModel`

The Vercel AI SDK model to use

### context?

[`SageContext`](/api/triage/jbcom/namespaces/handlers/interfaces/sagecontext/)

Optional context for better decomposition

## Returns

`Promise`\<\{ `estimatedTotalEffort`: `"epic"` \| `"medium"` \| `"trivial"` \| `"small"` \| `"large"`; `executionOrder?`: `string`[]; `notes?`: `string`; `originalTask`: `string`; `subtasks`: `object`[]; \}\>

Structured task decomposition
