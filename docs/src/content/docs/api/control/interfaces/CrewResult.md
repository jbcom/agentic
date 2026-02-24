---
editUrl: false
next: false
prev: false
title: "CrewResult"
---

Defined in: [packages/agentic-control/src/crews/types.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L40)

Result from crew execution (matches agentic-crew CLI JSON output)

## Properties

### duration\_ms

> **duration\_ms**: `number`

Defined in: [packages/agentic-control/src/crews/types.ts:50](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L50)

Execution time in milliseconds

***

### error?

> `optional` **error**: `string`

Defined in: [packages/agentic-control/src/crews/types.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L46)

Error message (if failed)

***

### framework\_used?

> `optional` **framework\_used**: `string`

Defined in: [packages/agentic-control/src/crews/types.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L48)

Framework that was used

***

### output?

> `optional` **output**: `string`

Defined in: [packages/agentic-control/src/crews/types.ts:44](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L44)

Crew output (if successful)

***

### success

> **success**: `boolean`

Defined in: [packages/agentic-control/src/crews/types.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L42)

Whether execution succeeded
