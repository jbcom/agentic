---
editUrl: false
next: false
prev: false
title: "LockManager"
---

Defined in: [packages/triage/src/queue/lock.ts:13](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L13)

Lock manager for distributed coordination

## Type Parameters

### T

`T` *extends* [`QueueItem`](/api/triage/interfaces/queueitem/) = [`QueueItem`](/api/triage/interfaces/queueitem/)

## Constructors

### Constructor

> **new LockManager**\<`T`\>(`storage`, `defaultTimeout?`): `LockManager`\<`T`\>

Defined in: [packages/triage/src/queue/lock.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L14)

#### Parameters

##### storage

[`QueueStorage`](/api/triage/interfaces/queuestorage/)\<`T`\>

##### defaultTimeout?

`number` = `...`

#### Returns

`LockManager`\<`T`\>

## Methods

### getLockHolder()

> **getLockHolder**(): `Promise`\<`string` \| `null`\>

Defined in: [packages/triage/src/queue/lock.ts:63](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L63)

Get current lock holder

#### Returns

`Promise`\<`string` \| `null`\>

***

### isLocked()

> **isLocked**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/queue/lock.ts:56](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L56)

Check if currently locked

#### Returns

`Promise`\<`boolean`\>

***

### tryWithLock()

> **tryWithLock**\<`R`\>(`holder`, `fn`, `timeout?`): `Promise`\<`R` \| `null`\>

Defined in: [packages/triage/src/queue/lock.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L40)

Try to execute a function with a lock
Returns null if lock cannot be acquired

#### Type Parameters

##### R

`R`

#### Parameters

##### holder

`string`

##### fn

() => `Promise`\<`R`\>

##### timeout?

`number`

#### Returns

`Promise`\<`R` \| `null`\>

***

### waitForRelease()

> **waitForRelease**(`maxWaitMs?`, `checkIntervalMs?`): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/queue/lock.ts:72](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L72)

Wait for lock to be released
Returns true if lock was released, false if timeout

#### Parameters

##### maxWaitMs?

`number` = `30000`

##### checkIntervalMs?

`number` = `1000`

#### Returns

`Promise`\<`boolean`\>

***

### withLock()

> **withLock**\<`R`\>(`holder`, `fn`, `timeout?`): `Promise`\<`R`\>

Defined in: [packages/triage/src/queue/lock.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/lock.ts#L23)

Execute a function with a lock
Automatically acquires and releases the lock

#### Type Parameters

##### R

`R`

#### Parameters

##### holder

`string`

##### fn

() => `Promise`\<`R`\>

##### timeout?

`number`

#### Returns

`Promise`\<`R`\>
