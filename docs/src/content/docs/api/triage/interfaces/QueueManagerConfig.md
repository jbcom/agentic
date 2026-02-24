---
editUrl: false
next: false
prev: false
title: "QueueManagerConfig"
---

Defined in: [packages/triage/src/queue/manager.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L14)

Configuration for the queue manager

## Properties

### instanceId?

> `optional` **instanceId**: `string`

Defined in: [packages/triage/src/queue/manager.ts:20](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L20)

Unique identifier for this manager instance

***

### lockTimeout?

> `optional` **lockTimeout**: `number`

Defined in: [packages/triage/src/queue/manager.ts:16](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L16)

Lock timeout in milliseconds (default: 5 minutes)

***

### maxRetries?

> `optional` **maxRetries**: `number`

Defined in: [packages/triage/src/queue/manager.ts:18](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L18)

Maximum retries before marking as failed (default: 3)
