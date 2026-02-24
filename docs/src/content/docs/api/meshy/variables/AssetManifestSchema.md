---
editUrl: false
next: false
prev: false
title: "AssetManifestSchema"
---

> `const` **AssetManifestSchema**: `ZodObject`\<\{ `id`: `ZodString`; `name`: `ZodString`; `seed`: `ZodOptional`\<`ZodNumber`\>; `tasks`: `ZodOptional`\<`ZodRecord`\<`ZodString`, `ZodObject`\<\{ `artifacts`: `ZodOptional`\<`ZodRecord`\<`ZodString`, `ZodString`\>\>; `outputs`: `ZodOptional`\<`ZodRecord`\<`ZodString`, `ZodUnknown`\>\>; `status`: `ZodOptional`\<`ZodEnum`\<\{ `CANCELED`: `"CANCELED"`; `FAILED`: `"FAILED"`; `IN_PROGRESS`: `"IN_PROGRESS"`; `PENDING`: `"PENDING"`; `SUCCEEDED`: `"SUCCEEDED"`; \}\>\>; `taskId`: `ZodOptional`\<`ZodString`\>; \}, `$loose`\>\>\>; `type`: `ZodString`; \}, `$loose`\>

Defined in: [schemas/manifest.ts:12](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/schemas/manifest.ts#L12)
