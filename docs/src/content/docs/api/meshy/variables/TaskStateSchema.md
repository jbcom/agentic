---
editUrl: false
next: false
prev: false
title: "TaskStateSchema"
---

> `const` **TaskStateSchema**: `ZodObject`\<\{ `artifacts`: `ZodOptional`\<`ZodRecord`\<`ZodString`, `ZodString`\>\>; `outputs`: `ZodOptional`\<`ZodRecord`\<`ZodString`, `ZodUnknown`\>\>; `status`: `ZodOptional`\<`ZodEnum`\<\{ `CANCELED`: `"CANCELED"`; `FAILED`: `"FAILED"`; `IN_PROGRESS`: `"IN_PROGRESS"`; `PENDING`: `"PENDING"`; `SUCCEEDED`: `"SUCCEEDED"`; \}\>\>; `taskId`: `ZodOptional`\<`ZodString`\>; \}, `$loose`\>

Defined in: [schemas/manifest.ts:3](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/schemas/manifest.ts#L3)
