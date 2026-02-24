---
editUrl: false
next: false
prev: false
title: "TaskDecompositionSchema"
---

> `const` **TaskDecompositionSchema**: `ZodObject`\<\{ `estimatedTotalEffort`: `ZodEnum`\<\{ `epic`: `"epic"`; `large`: `"large"`; `medium`: `"medium"`; `small`: `"small"`; `trivial`: `"trivial"`; \}\>; `executionOrder`: `ZodOptional`\<`ZodArray`\<`ZodString`\>\>; `notes`: `ZodOptional`\<`ZodString`\>; `originalTask`: `ZodString`; `subtasks`: `ZodArray`\<`ZodObject`\<\{ `agent`: `ZodEnum`\<\{ `claude`: `"claude"`; `cursor`: `"cursor"`; `human`: `"human"`; `jules`: `"jules"`; `ollama`: `"ollama"`; \}\>; `dependencies`: `ZodOptional`\<`ZodArray`\<`ZodString`\>\>; `description`: `ZodString`; `effort`: `ZodEnum`\<\{ `epic`: `"epic"`; `large`: `"large"`; `medium`: `"medium"`; `small`: `"small"`; `trivial`: `"trivial"`; \}\>; `id`: `ZodString`; `priority`: `ZodNumber`; `title`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/sage.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/sage.ts#L70)
