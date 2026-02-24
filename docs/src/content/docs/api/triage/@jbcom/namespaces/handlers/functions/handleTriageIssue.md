---
editUrl: false
next: false
prev: false
title: "handleTriageIssue"
---

> **handleTriageIssue**(`id`, `analysis`, `customConnectors?`): `Promise`\<\{ `analysis`: \{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\>

Defined in: [packages/triage/src/handlers/issue.ts:11](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/issue.ts#L11)

Handler for triaging an issue

## Parameters

### id

`string`

### analysis

#### actionItems

`string`[] = `...`

#### estimate?

`number` = `...`

#### labels

`string`[] = `...`

#### priority

`"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"` = `...`

#### summary

`string` = `...`

#### title

`string` = `...`

#### type

`"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"` = `...`

### customConnectors?

[`TriageConnectors`](/api/triage/classes/triageconnectors/)

## Returns

`Promise`\<\{ `analysis`: \{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\>
