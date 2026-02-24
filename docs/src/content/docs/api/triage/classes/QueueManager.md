---
editUrl: false
next: false
prev: false
title: "QueueManager"
---

Defined in: [packages/triage/src/queue/manager.ts:26](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L26)

Queue Manager - handles all queue operations

## Type Parameters

### T

`T` *extends* [`QueueItem`](/api/triage/interfaces/queueitem/) = [`QueueItem`](/api/triage/interfaces/queueitem/)

## Constructors

### Constructor

> **new QueueManager**\<`T`\>(`storage`, `config?`): `QueueManager`\<`T`\>

Defined in: [packages/triage/src/queue/manager.ts:29](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L29)

#### Parameters

##### storage

[`QueueStorage`](/api/triage/interfaces/queuestorage/)\<`T`\>

##### config?

[`QueueManagerConfig`](/api/triage/interfaces/queuemanagerconfig/) = `{}`

#### Returns

`QueueManager`\<`T`\>

## Methods

### add()

> **add**(`item`): `Promise`\<`T`\>

Defined in: [packages/triage/src/queue/manager.ts:43](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L43)

Add an item to the queue

#### Parameters

##### item

`Omit`\<`T`, `"status"` \| `"addedAt"` \| `"retries"`\> & `Partial`\<`Pick`\<`T`, `"status"` \| `"addedAt"` \| `"retries"`\>\>

#### Returns

`Promise`\<`T`\>

***

### cancel()

> **cancel**(`id`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:178](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L178)

Cancel an item

#### Parameters

##### id

`string`

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### clear()

> **clear**(): `Promise`\<`void`\>

Defined in: [packages/triage/src/queue/manager.ts:212](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L212)

Clear all items (dangerous!)

#### Returns

`Promise`\<`void`\>

***

### complete()

> **complete**(`id`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:130](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L130)

Mark an item as completed and remove from queue

#### Parameters

##### id

`string`

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### fail()

> **fail**(`id`, `error`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:151](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L151)

Mark an item as failed
If under max retries, requeue as pending

#### Parameters

##### id

`string`

##### error

`string`

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### get()

> **get**(`id`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:95](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L95)

Get an item by ID

#### Parameters

##### id

`string`

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### isLocked()

> **isLocked**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/queue/manager.ts:236](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L236)

Check if queue is locked

#### Returns

`Promise`\<`boolean`\>

***

### length()

> **length**(): `Promise`\<`number`\>

Defined in: [packages/triage/src/queue/manager.ts:204](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L204)

Get current queue length

#### Returns

`Promise`\<`number`\>

***

### list()

> **list**(`status?`): `Promise`\<`T`[]\>

Defined in: [packages/triage/src/queue/manager.ts:185](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L185)

List all items (optionally filtered by status)

#### Parameters

##### status?

[`QueueItemStatus`](/api/triage/type-aliases/queueitemstatus/)

#### Returns

`Promise`\<`T`[]\>

***

### lock()

> **lock**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/queue/manager.ts:222](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L222)

Acquire a lock for processing

#### Returns

`Promise`\<`boolean`\>

***

### next()

> **next**(): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:87](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L87)

Get the next item to process (highest priority, oldest first)

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### processNext()

> **processNext**\<`R`\>(`handler`): `Promise`\<\{ `item`: `T`; `result`: `R`; \} \| `null`\>

Defined in: [packages/triage/src/queue/manager.ts:244](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L244)

Process the next item with a handler
Automatically handles locking, status updates, and error handling

#### Type Parameters

##### R

`R`

#### Parameters

##### handler

(`item`) => `Promise`\<`R`\>

#### Returns

`Promise`\<\{ `item`: `T`; `result`: `R`; \} \| `null`\>

***

### remove()

> **remove**(`id`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:71](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L71)

Remove an item from the queue

#### Parameters

##### id

`string`

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### startProcessing()

> **startProcessing**(`id`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:120](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L120)

Mark an item as processing

#### Parameters

##### id

`string`

#### Returns

`Promise`\<`T` \| `undefined`\>

***

### stats()

> **stats**(): `Promise`\<[`QueueStats`](/api/triage/interfaces/queuestats/)\>

Defined in: [packages/triage/src/queue/manager.ts:196](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L196)

Get queue statistics

#### Returns

`Promise`\<[`QueueStats`](/api/triage/interfaces/queuestats/)\>

***

### unlock()

> **unlock**(): `Promise`\<`void`\>

Defined in: [packages/triage/src/queue/manager.ts:229](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L229)

Release the lock

#### Returns

`Promise`\<`void`\>

***

### update()

> **update**(`id`, `updates`): `Promise`\<`T` \| `undefined`\>

Defined in: [packages/triage/src/queue/manager.ts:103](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/manager.ts#L103)

Update an item's properties

#### Parameters

##### id

`string`

##### updates

`Partial`\<`Omit`\<`T`, `"id"`\>\>

#### Returns

`Promise`\<`T` \| `undefined`\>
