---
editUrl: false
next: false
prev: false
title: "QueueLock"
---

Defined in: [packages/triage/src/queue/types.ts:78](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L78)

Distributed lock for queue operations

## Properties

### acquiredAt

> **acquiredAt**: `string`

Defined in: [packages/triage/src/queue/types.ts:82](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L82)

When the lock was acquired

***

### expiresAt

> **expiresAt**: `string`

Defined in: [packages/triage/src/queue/types.ts:84](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L84)

Lock expiry time

***

### holder

> **holder**: `string`

Defined in: [packages/triage/src/queue/types.ts:80](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/types.ts#L80)

Who holds the lock
