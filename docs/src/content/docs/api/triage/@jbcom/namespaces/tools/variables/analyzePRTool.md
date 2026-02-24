---
editUrl: false
next: false
prev: false
title: "analyzePRTool"
---

> `const` **analyzePRTool**: `Tool`\<\{ `analysis`: \{ `breakingChanges`: `string`[]; `relatedIssues`: `string`[]; `riskLevel`: `"high"` \| `"medium"` \| `"low"`; `scope`: `"minor"` \| `"major"` \| `"patch"` \| `"breaking"`; `summary`: `string`; `testingCoverage`: `"none"` \| `"partial"` \| `"full"`; `title`: `string`; \}; `prNumber`: `number`; \}, \{ `analysis`: \{ `breakingChanges`: `string`[]; `relatedIssues`: `string`[]; `riskLevel`: `"high"` \| `"medium"` \| `"low"`; `scope`: `"minor"` \| `"major"` \| `"patch"` \| `"breaking"`; `summary`: `string`; `testingCoverage`: `"none"` \| `"partial"` \| `"full"`; `title`: `string`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\>

Defined in: [packages/triage/src/tools/pr.ts:6](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/tools/pr.ts#L6)
