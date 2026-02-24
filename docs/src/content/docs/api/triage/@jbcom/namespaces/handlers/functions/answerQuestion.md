---
editUrl: false
next: false
prev: false
title: "answerQuestion"
---

> **answerQuestion**(`query`, `model`, `context?`): `Promise`\<\{ `agentRecommendation?`: \{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `instructions?`: `string`; `reason`: `string`; \}; `answer`: `string`; `confidence`: `number`; `followUp?`: `string`; `queryType`: `"question"` \| `"review"` \| `"fix"` \| `"implement"` \| `"refactor"` \| `"decompose"` \| `"unblock"` \| `"route"` \| `"general"`; `references?`: `string`[]; \}\>

Defined in: [packages/triage/src/handlers/sage.ts:106](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L106)

Answer a question or provide guidance using Sage

## Parameters

### query

`string`

The question or request

### model

`LanguageModel`

The Vercel AI SDK model to use

### context?

[`SageContext`](/api/triage/jbcom/namespaces/handlers/interfaces/sagecontext/)

Optional context for better responses

## Returns

`Promise`\<\{ `agentRecommendation?`: \{ `agent`: `"cursor"` \| `"jules"` \| `"ollama"` \| `"claude"` \| `"human"`; `instructions?`: `string`; `reason`: `string`; \}; `answer`: `string`; `confidence`: `number`; `followUp?`: `string`; `queryType`: `"question"` \| `"review"` \| `"fix"` \| `"implement"` \| `"refactor"` \| `"decompose"` \| `"unblock"` \| `"route"` \| `"general"`; `references?`: `string`[]; \}\>

Structured Sage response
