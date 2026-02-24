---
editUrl: false
next: false
prev: false
title: "QueueState"
---

Defined in: [packages/triage/src/queue/types.ts:62](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L62)

Queue state for serialization

## Type Parameters

### T

`T` *extends* [`QueueItem`](/api/triage/interfaces/queueitem/) = [`QueueItem`](/api/triage/interfaces/queueitem/)

## Properties

### items

> **items**: `T`[]

Defined in: [packages/triage/src/queue/types.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L70)

The queue items

***

### lock

> **lock**: [`QueueLock`](/api/triage/interfaces/queuelock/) \| `null`

Defined in: [packages/triage/src/queue/types.ts:68](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L68)

Current lock holder (if any)

***

### stats

> **stats**: [`QueueStats`](/api/triage/interfaces/queuestats/)

Defined in: [packages/triage/src/queue/types.ts:72](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L72)

Statistics

***

### updatedAt

> **updatedAt**: `string`

Defined in: [packages/triage/src/queue/types.ts:66](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L66)

Last update timestamp

***

### version

> **version**: `number`

Defined in: [packages/triage/src/queue/types.ts:64](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L64)

Schema version for migrations
