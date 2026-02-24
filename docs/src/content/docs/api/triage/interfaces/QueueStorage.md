---
editUrl: false
next: false
prev: false
title: "QueueStorage"
---

Defined in: [packages/triage/src/storage/interface.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L14)

Storage backend interface
Implement this for your preferred storage (GitHub Issue, Redis, etc.)

## Type Parameters

### T

`T` *extends* [`QueueItem`](/api/triage/interfaces/queueitem/) = [`QueueItem`](/api/triage/interfaces/queueitem/)

## Methods

### acquireLock()

> **acquireLock**(`holder`, `ttlMs`): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/storage/interface.ts:31](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L31)

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

***

### getLock()

> **getLock**(): `Promise`\<[`QueueLock`](/api/triage/interfaces/queuelock/) \| `null`\>

Defined in: [packages/triage/src/storage/interface.ts:47](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L47)

Get the current lock info

#### Returns

`Promise`\<[`QueueLock`](/api/triage/interfaces/queuelock/) \| `null`\>

***

### isLocked()

> **isLocked**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/storage/interface.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L42)

Check if a lock is currently held

#### Returns

`Promise`\<`boolean`\>

***

### read()

> **read**(): `Promise`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

Defined in: [packages/triage/src/storage/interface.ts:18](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L18)

Read the current queue state

#### Returns

`Promise`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

***

### releaseLock()

> **releaseLock**(`holder`): `Promise`\<`void`\>

Defined in: [packages/triage/src/storage/interface.ts:37](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L37)

Release a distributed lock

#### Parameters

##### holder

`string`

Must match the holder that acquired the lock

#### Returns

`Promise`\<`void`\>

***

### write()

> **write**(`state`): `Promise`\<`void`\>

Defined in: [packages/triage/src/storage/interface.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/interface.ts#L23)

Write the queue state

#### Parameters

##### state

[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>

#### Returns

`Promise`\<`void`\>
