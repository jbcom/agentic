---
editUrl: false
next: false
prev: false
title: "AgentRoutingSchema"
---

> `const` **AgentRoutingSchema**: `ZodObject`\<\{ `agent`: `ZodEnum`\<\{ `claude`: `"claude"`; `cursor`: `"cursor"`; `human`: `"human"`; `jules`: `"jules"`; `ollama`: `"ollama"`; \}\>; `alternatives`: `ZodOptional`\<`ZodArray`\<`ZodObject`\<\{ `agent`: `ZodEnum`\<\{ `claude`: `"claude"`; `cursor`: `"cursor"`; `human`: `"human"`; `jules`: `"jules"`; `ollama`: `"ollama"`; \}\>; `reason`: `ZodString`; \}, `$strip`\>\>\>; `confidence`: `ZodNumber`; `instructions`: `ZodString`; `reason`: `ZodString`; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/sage.ts:83](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/sage.ts#L83)

Schema for agent routing decisions
