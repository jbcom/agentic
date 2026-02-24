---
editUrl: false
next: false
prev: false
title: "sage"
---

> **sage**(`query`, `model`, `context?`): `Promise`\<\{ `agentRecommendation?`: \{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `instructions?`: `string`; `reason`: `string`; \}; `answer`: `string`; `confidence`: `number`; `followUp?`: `string`; `queryType`: `"question"` \| `"review"` \| `"fix"` \| `"implement"` \| `"refactor"` \| `"decompose"` \| `"unblock"` \| `"route"` \| `"general"`; `references?`: `string`[]; \} \| \{ `estimatedTotalEffort`: `"epic"` \| `"medium"` \| `"trivial"` \| `"small"` \| `"large"`; `executionOrder?`: `string`[]; `notes?`: `string`; `originalTask`: `string`; `subtasks`: `object`[]; \} \| \{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `alternatives?`: `object`[]; `confidence`: `number`; `instructions`: `string`; `reason`: `string`; \} \| \{ `diagnosis`: `string`; `escalationReason?`: `string`; `immediateAction`: `string`; `needsHuman`: `boolean`; `rootCause`: `string`; `suggestions`: `object`[]; \}\>

Defined in: [packages/triage/src/handlers/sage.ts:254](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L254)

High-level Sage function that auto-routes to the appropriate handler

## Parameters

### query

`string`

The question, task, or situation

### model

`LanguageModel`

The Vercel AI SDK model to use

### context?

[`SageContext`](/api/triage/jbcom/namespaces/handlers/interfaces/sagecontext/)

Optional context

## Returns

`Promise`\<\{ `agentRecommendation?`: \{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `instructions?`: `string`; `reason`: `string`; \}; `answer`: `string`; `confidence`: `number`; `followUp?`: `string`; `queryType`: `"question"` \| `"review"` \| `"fix"` \| `"implement"` \| `"refactor"` \| `"decompose"` \| `"unblock"` \| `"route"` \| `"general"`; `references?`: `string`[]; \} \| \{ `estimatedTotalEffort`: `"epic"` \| `"medium"` \| `"trivial"` \| `"small"` \| `"large"`; `executionOrder?`: `string`[]; `notes?`: `string`; `originalTask`: `string`; `subtasks`: `object`[]; \} \| \{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `alternatives?`: `object`[]; `confidence`: `number`; `instructions`: `string`; `reason`: `string`; \} \| \{ `diagnosis`: `string`; `escalationReason?`: `string`; `immediateAction`: `string`; `needsHuman`: `boolean`; `rootCause`: `string`; `suggestions`: `object`[]; \}\>

Response appropriate to the query type
