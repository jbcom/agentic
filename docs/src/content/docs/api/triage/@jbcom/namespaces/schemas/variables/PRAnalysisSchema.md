---
editUrl: false
next: false
prev: false
title: "PRAnalysisSchema"
---

> `const` **PRAnalysisSchema**: `ZodObject`\<\{ `breakingChanges`: `ZodArray`\<`ZodString`\>; `relatedIssues`: `ZodArray`\<`ZodString`\>; `riskLevel`: `ZodEnum`\<\{ `high`: `"high"`; `low`: `"low"`; `medium`: `"medium"`; \}\>; `scope`: `ZodEnum`\<\{ `breaking`: `"breaking"`; `major`: `"major"`; `minor`: `"minor"`; `patch`: `"patch"`; \}\>; `summary`: `ZodString`; `testingCoverage`: `ZodEnum`\<\{ `full`: `"full"`; `none`: `"none"`; `partial`: `"partial"`; \}\>; `title`: `ZodString`; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/pr.ts:3](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/pr.ts#L3)
