---
editUrl: false
next: false
prev: false
title: "GitHubIssueStorage"
---

Defined in: [packages/triage/src/storage/github-issue.ts:68](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L68)

GitHub Issue storage implementation

## Type Parameters

### T

`T` *extends* [`QueueItem`](/api/triage/interfaces/queueitem/) = [`QueueItem`](/api/triage/interfaces/queueitem/)

## Implements

- [`QueueStorage`](/api/triage/interfaces/queuestorage/)\<`T`\>

## Constructors

### Constructor

> **new GitHubIssueStorage**\<`T`\>(`options`): `GitHubIssueStorage`\<`T`\>

Defined in: [packages/triage/src/storage/github-issue.ts:78](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L78)

#### Parameters

##### options

[`GitHubIssueStorageOptions`](/api/triage/interfaces/githubissuestorageoptions/)

#### Returns

`GitHubIssueStorage`\<`T`\>

## Methods

### acquireLock()

> **acquireLock**(`holder`, `ttlMs`): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/storage/github-issue.ts:124](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L124)

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

Defined in: [packages/triage/src/storage/github-issue.ts:200](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L200)

Get the current lock info

#### Returns

`Promise`\<[`QueueLock`](/api/triage/interfaces/queuelock/) \| `null`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`getLock`](/api/triage/interfaces/queuestorage/#getlock)

***

### isLocked()

> **isLocked**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/storage/github-issue.ts:195](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L195)

Check if a lock is currently held

#### Returns

`Promise`\<`boolean`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`isLocked`](/api/triage/interfaces/queuestorage/#islocked)

***

### read()

> **read**(): `Promise`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

Defined in: [packages/triage/src/storage/github-issue.ts:92](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L92)

Read the current queue state

#### Returns

`Promise`\<[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`read`](/api/triage/interfaces/queuestorage/#read)

***

### releaseLock()

> **releaseLock**(`holder`): `Promise`\<`void`\>

Defined in: [packages/triage/src/storage/github-issue.ts:170](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L170)

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

Defined in: [packages/triage/src/storage/github-issue.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L105)

Write the queue state

#### Parameters

##### state

[`QueueState`](/api/triage/interfaces/queuestate/)\<`T`\>

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`QueueStorage`](/api/triage/interfaces/queuestorage/).[`write`](/api/triage/interfaces/queuestorage/#write)
