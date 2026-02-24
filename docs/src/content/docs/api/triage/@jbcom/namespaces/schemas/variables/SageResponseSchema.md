---
editUrl: false
next: false
prev: false
title: "SageResponseSchema"
---

> `const` **SageResponseSchema**: `ZodObject`\<\{ `agentRecommendation`: `ZodOptional`\<`ZodObject`\<\{ `agent`: `ZodEnum`\<\{ `claude`: `"claude"`; `cursor`: `"cursor"`; `human`: `"human"`; `jules`: `"jules"`; `ollama`: `"ollama"`; \}\>; `instructions`: `ZodOptional`\<`ZodString`\>; `reason`: `ZodString`; \}, `$strip`\>\>; `answer`: `ZodString`; `confidence`: `ZodNumber`; `followUp`: `ZodOptional`\<`ZodString`\>; `queryType`: `ZodEnum`\<\{ `decompose`: `"decompose"`; `fix`: `"fix"`; `general`: `"general"`; `implement`: `"implement"`; `question`: `"question"`; `refactor`: `"refactor"`; `review`: `"review"`; `route`: `"route"`; `unblock`: `"unblock"`; \}\>; `references`: `ZodOptional`\<`ZodArray`\<`ZodString`\>\>; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/sage.ts:37](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/sage.ts#L37)

Schema for Sage Q&A responses
