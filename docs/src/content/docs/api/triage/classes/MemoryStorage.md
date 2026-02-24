---
editUrl: false
next: false
prev: false
title: "MemoryStorage"
---

Defined in: [packages/triage/src/storage/memory.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L14)

In-memory storage for testing

## Type Parameters

### T

`T` *extends* [`QueueItem`](/api/triage/interfaces/queueitem/) = [`QueueItem`](/api/triage/interfaces/queueitem/)

## Implements

- [`QueueStorage`](/api/triage/interfaces/queuestorage/)\<`T`\>

## Constructors

### Constructor

> **new MemoryStorage**\<`T`\>(`initial?`): `MemoryStorage`\<`T`\>

Defined in: [packages/triage/src/storage/memory.ts:17](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L17)

#### Parameters

##### initial?

`Partial`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

#### Returns

`MemoryStorage`\<`T`\>

## Methods

### acquireLock()

> **acquireLock**(`holder`, `ttlMs`): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/storage/memory.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L42)

Acquire a distributed lock

#### Parameters

##### holder

`string`

Identifier for the lock holder

##### ttlMs

`number`

Time-to-live in milliseconds

#### Returns

`Promise`\<`boolean`\>

true if lock acquired, false if already locked

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`acquireLock`](/api/triage/interfaces/queuestorage/#acquirelock)

***

### getLock()

> **getLock**(): `Promise`\<[`QueueLock`](/api/triage/interfaces/queuelock/) \| `null`\>

Defined in: [packages/triage/src/storage/memory.ts:69](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L69)

Get the current lock info

#### Returns

`Promise`\<[`QueueLock`](/api/triage/interfaces/queuelock/) \| `null`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`getLock`](/api/triage/interfaces/queuestorage/#getlock)

***

### isLocked()

> **isLocked**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/storage/memory.ts:64](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L64)

Check if a lock is currently held

#### Returns

`Promise`\<`boolean`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`isLocked`](/api/triage/interfaces/queuestorage/#islocked)

***

### read()

> **read**(): `Promise`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

Defined in: [packages/triage/src/storage/memory.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L34)

Read the current queue state

#### Returns

`Promise`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`read`](/api/triage/interfaces/queuestorage/#read)

***

### releaseLock()

> **releaseLock**(`holder`): `Promise`\<`void`\>

Defined in: [packages/triage/src/storage/memory.ts:58](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L58)

Release a distributed lock

#### Parameters

##### holder

`string`

Must match the holder that acquired the lock

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`releaseLock`](/api/triage/interfaces/queuestorage/#releaselock)

***

### write()

> **write**(`state`): `Promise`\<`void`\>

Defined in: [packages/triage/src/storage/memory.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/memory.ts#L38)

Write the queue state

#### Parameters

##### state

[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`write`](/api/triage/interfaces/queuestorage/#write)
