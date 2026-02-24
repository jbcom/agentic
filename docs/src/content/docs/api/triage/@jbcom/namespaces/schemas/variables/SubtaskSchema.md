---
editUrl: false
next: false
prev: false
title: "SubtaskSchema"
---

> `const` **SubtaskSchema**: `ZodObject`\<\{ `agent`: `ZodEnum`\<\{ `claude`: `"claude"`; `cursor`: `"cursor"`; `human`: `"human"`; `jules`: `"jules"`; `ollama`: `"ollama"`; \}\>; `dependencies`: `ZodOptional`\<`ZodArray`\<`ZodString`\>\>; `description`: `ZodString`; `effort`: `ZodEnum`\<\{ `epic`: `"epic"`; `large`: `"large"`; `medium`: `"medium"`; `small`: `"small"`; `trivial`: `"trivial"`; \}\>; `id`: `ZodString`; `priority`: `ZodNumber`; `title`: `ZodString`; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/sage.ts:58](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/sage.ts#L58)

Schema for task decomposition
