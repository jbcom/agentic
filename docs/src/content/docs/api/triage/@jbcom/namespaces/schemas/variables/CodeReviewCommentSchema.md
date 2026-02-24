---
editUrl: false
next: false
prev: false
title: "CodeReviewCommentSchema"
---

> `const` **CodeReviewCommentSchema**: `ZodObject`\<\{ `content`: `ZodString`; `file`: `ZodString`; `line`: `ZodOptional`\<`ZodNumber`\>; `severity`: `ZodOptional`\<`ZodEnum`\<\{ `high`: `"high"`; `low`: `"low"`; `medium`: `"medium"`; \}\>\>; `type`: `ZodEnum`\<\{ `issue`: `"issue"`; `praise`: `"praise"`; `question`: `"question"`; `suggestion`: `"suggestion"`; \}\>; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/review.ts:5](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/review.ts#L5)
