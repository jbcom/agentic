---
editUrl: false
next: false
prev: false
title: "MockExecutionResult"
---

Defined in: [sandbox.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L36)

Mock execution result.

## Properties

### duration?

> `optional` **duration**: `number`

Defined in: [sandbox.ts:51](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L51)

Execution duration in ms

***

### error?

> `optional` **error**: `Error`

Defined in: [sandbox.ts:53](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L53)

Error if execution failed

***

### exitCode?

> `optional` **exitCode**: `number`

Defined in: [sandbox.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L40)

Exit code

***

### files?

> `optional` **files**: `object`[]

Defined in: [sandbox.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L46)

Output files

#### content

> **content**: `string` \| `Buffer`\<`ArrayBufferLike`\>

#### path

> **path**: `string`

***

### stderr?

> `optional` **stderr**: `string`

Defined in: [sandbox.ts:44](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L44)

Standard error

***

### stdout?

> `optional` **stdout**: `string`

Defined in: [sandbox.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L42)

Standard output

***

### success

> **success**: `boolean`

Defined in: [sandbox.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L38)

Whether execution succeeded
