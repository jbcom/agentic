---
editUrl: false
next: false
prev: false
title: "analyzeIssue"
---

> **analyzeIssue**(`issueBody`, `model`): `Promise`\<\{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}\>

Defined in: [packages/triage/src/handlers/issue.ts:74](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/issue.ts#L74)

Analyze an issue using the provided language model.

## Parameters

### issueBody

`string`

The content of the issue to analyze

### model

`LanguageModel`

The Vercel AI SDK model to use

## Returns

`Promise`\<\{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}\>

The structured analysis result
