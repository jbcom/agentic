---
editUrl: false
next: false
prev: false
title: "triageItem"
---

> **triageItem**(`content`, `model`): `Promise`\<\{ `codeReview?`: \{ `comments`: `object`[]; `impact`: `"critical"` \| `"high"` \| `"medium"` \| `"low"`; `status`: `"approve"` \| `"request_changes"` \| `"comment"`; `suggestedLabels`: `string`[]; `summary`: `string`; \}; `issueAnalysis?`: \{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; `triage`: `string`; \}\>

Defined in: [packages/triage/src/handlers/triage.ts:11](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/triage.ts#L11)

Triage an issue or pull request using the provided language model.

## Parameters

### content

`string`

The content of the issue or PR (including metadata)

### model

`LanguageModel`

The Vercel AI SDK model to use

## Returns

`Promise`\<\{ `codeReview?`: \{ `comments`: `object`[]; `impact`: `"critical"` \| `"high"` \| `"medium"` \| `"low"`; `status`: `"approve"` \| `"request_changes"` \| `"comment"`; `suggestedLabels`: `string`[]; `summary`: `string`; \}; `issueAnalysis?`: \{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; `triage`: `string`; \}\>

The structured triage result
