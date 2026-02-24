---
editUrl: false
next: false
prev: false
title: "QueueStats"
---

Defined in: [packages/triage/src/queue/types.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L46)

Queue statistics

## Properties

### avgProcessingTime

> **avgProcessingTime**: `number`

Defined in: [packages/triage/src/queue/types.ts:56](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L56)

Average processing time in minutes

***

### byStatus

> **byStatus**: `Record`\<[`QueueItemStatus`](/api/triage/type-aliases/queueitemstatus/), `number`\>

Defined in: [packages/triage/src/queue/types.ts:50](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L50)

Items by status

***

### completed24h

> **completed24h**: `number`

Defined in: [packages/triage/src/queue/types.ts:52](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L52)

Items completed in last 24h

***

### failed24h

> **failed24h**: `number`

Defined in: [packages/triage/src/queue/types.ts:54](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L54)

Items failed in last 24h

***

### total

> **total**: `number`

Defined in: [packages/triage/src/queue/types.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L48)

Total items currently in queue
