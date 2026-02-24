---
editUrl: false
next: false
prev: false
title: "UnblockResponseSchema"
---

> `const` **UnblockResponseSchema**: `ZodObject`\<\{ `diagnosis`: `ZodString`; `escalationReason`: `ZodOptional`\<`ZodString`\>; `immediateAction`: `ZodString`; `needsHuman`: `ZodBoolean`; `rootCause`: `ZodString`; `suggestions`: `ZodArray`\<`ZodObject`\<\{ `action`: `ZodString`; `effort`: `ZodEnum`\<\{ `epic`: `"epic"`; `large`: `"large"`; `medium`: `"medium"`; `small`: `"small"`; `trivial`: `"trivial"`; \}\>; `likelihood`: `ZodNumber`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [packages/triage/src/schemas/sage.ts:104](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/schemas/sage.ts#L104)

Schema for unblocking stuck work
