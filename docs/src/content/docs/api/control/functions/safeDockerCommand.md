---
editUrl: false
next: false
prev: false
title: "safeDockerCommand"
---

> **safeDockerCommand**(`args`, `options?`): `object`

Defined in: [packages/agentic-control/src/core/subprocess.ts:168](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/subprocess.ts#L168)

Safely execute docker commands with validation

## Parameters

### args

`string`[]

### options?

`SpawnSyncOptions` = `{}`

## Returns

`object`

### code

> **code**: `number` \| `null`

### stderr

> **stderr**: `string`

### stdout

> **stdout**: `string`

### success

> **success**: `boolean`
