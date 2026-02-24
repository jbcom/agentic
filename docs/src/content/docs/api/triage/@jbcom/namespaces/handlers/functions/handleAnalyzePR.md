---
editUrl: false
next: false
prev: false
title: "handleAnalyzePR"
---

> **handleAnalyzePR**(`prNumber`, `analysis`): `Promise`\<\{ `analysis`: \{ `breakingChanges`: `string`[]; `relatedIssues`: `string`[]; `riskLevel`: `"high"` \| `"medium"` \| `"low"`; `scope`: `"minor"` \| `"major"` \| `"patch"` \| `"breaking"`; `summary`: `string`; `testingCoverage`: `"none"` \| `"partial"` \| `"full"`; `title`: `string`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\>

Defined in: [packages/triage/src/handlers/pr.ts:6](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/pr.ts#L6)

Handler for analyzing a PR

## Parameters

### prNumber

`number`

### analysis

#### breakingChanges

`string`[] = `...`

#### relatedIssues

`string`[] = `...`

#### riskLevel

`"high"` \| `"medium"` \| `"low"` = `...`

#### scope

`"minor"` \| `"major"` \| `"patch"` \| `"breaking"` = `...`

#### summary

`string` = `...`

#### testingCoverage

`"none"` \| `"partial"` \| `"full"` = `...`

#### title

`string` = `...`

## Returns

`Promise`\<\{ `analysis`: \{ `breakingChanges`: `string`[]; `relatedIssues`: `string`[]; `riskLevel`: `"high"` \| `"medium"` \| `"low"`; `scope`: `"minor"` \| `"major"` \| `"patch"` \| `"breaking"`; `summary`: `string`; `testingCoverage`: `"none"` \| `"partial"` \| `"full"`; `title`: `string`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\>
