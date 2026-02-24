---
editUrl: false
next: false
prev: false
title: "DefinitionBundle"
---

Defined in: [core/definitions.ts:10](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/definitions.ts#L10)

Parsed pipeline and task definitions loaded from JSON files.

## Properties

### pipelines

> **pipelines**: `Map`\<`string`, \{ `description?`: `string`; `name`: `string`; `stateMapping?`: `Record`\<`string`, `string`\>; `steps`: `object`[]; `version?`: `string`; \}\>

Defined in: [core/definitions.ts:11](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/definitions.ts#L11)

***

### tasks

> **tasks**: `Map`\<`string`, \{ `apiVersion?`: `string`; `description?`: `string`; `endpoint?`: `string`; `id`: `string`; `inputs`: `object`[]; `method?`: `"GET"` \| `"POST"` \| `"DELETE"`; `outputs`: `object`[]; `poll?`: \{ `intervalMs?`: `number`; `path`: `string`; `strategy`: `"stream"` \| `"poll"`; `timeoutMs?`: `number`; \}; \}\>

Defined in: [core/definitions.ts:12](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/definitions.ts#L12)
