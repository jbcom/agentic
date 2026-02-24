---
editUrl: false
next: false
prev: false
title: "QueueItem"
---

Defined in: [packages/triage/src/queue/types.ts:22](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L22)

Base queue item interface

## Properties

### addedAt

> **addedAt**: `string`

Defined in: [packages/triage/src/queue/types.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L30)

When the item was added

***

### completedAt?

> `optional` **completedAt**: `string`

Defined in: [packages/triage/src/queue/types.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L34)

When processing completed (if applicable)

***

### id

> **id**: `string`

Defined in: [packages/triage/src/queue/types.ts:24](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L24)

Unique identifier (e.g., "org/repo#123")

***

### lastError?

> `optional` **lastError**: `string`

Defined in: [packages/triage/src/queue/types.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L38)

Last error message (if failed)

***

### metadata?

> `optional` **metadata**: `Record`\<`string`, `unknown`\>

Defined in: [packages/triage/src/queue/types.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L40)

Additional metadata

***

### priority

> **priority**: [`Priority`](/api/triage/type-aliases/priority/)

Defined in: [packages/triage/src/queue/types.ts:26](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L26)

Priority level

***

### retries

> **retries**: `number`

Defined in: [packages/triage/src/queue/types.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L36)

Number of retry attempts

***

### startedAt?

> `optional` **startedAt**: `string`

Defined in: [packages/triage/src/queue/types.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L32)

When processing started (if applicable)

***

### status

> **status**: [`QueueItemStatus`](/api/triage/type-aliases/queueitemstatus/)

Defined in: [packages/triage/src/queue/types.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L28)

Current status
