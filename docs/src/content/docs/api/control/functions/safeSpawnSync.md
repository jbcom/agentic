---
editUrl: false
next: false
prev: false
title: "safeSpawnSync"
---

> **safeSpawnSync**(`command`, `args?`, `options?`): `object`

Defined in: [packages/agentic-control/src/core/subprocess.ts:15](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/subprocess.ts#L15)

Safely execute a command with array-based arguments (no shell interpolation)

## Parameters

### command

`string`

### args?

`string`[] = `[]`

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
