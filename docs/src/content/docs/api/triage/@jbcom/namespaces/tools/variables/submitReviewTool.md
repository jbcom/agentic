---
editUrl: false
next: false
prev: false
title: "submitReviewTool"
---

> `const` **submitReviewTool**: `Tool`\<\{ `prNumber`: `number`; `review`: \{ `comments`: `object`[]; `impact`: `"critical"` \| `"high"` \| `"medium"` \| `"low"`; `status`: `"approve"` \| `"request_changes"` \| `"comment"`; `suggestedLabels`: `string`[]; `summary`: `string`; \}; \}, \{ `error?`: `undefined`; `message`: `string`; `review`: \{ `comments`: `object`[]; `impact`: `"critical"` \| `"high"` \| `"medium"` \| `"low"`; `status`: `"approve"` \| `"request_changes"` \| `"comment"`; `suggestedLabels`: `string`[]; `summary`: `string`; \}; `success`: `boolean`; \} \| \{ `error`: `unknown`; `message`: `string`; `review?`: `undefined`; `success`: `boolean`; \}\>

Defined in: [packages/triage/src/tools/review.ts:6](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/tools/review.ts#L6)
