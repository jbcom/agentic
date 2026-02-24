---
editUrl: false
next: false
prev: false
title: "safeSpawn"
---

> **safeSpawn**(`command`, `args?`, `options?`): `Promise`\<\{ `code`: `number` \| `null`; `stderr`: `string`; `stdout`: `string`; `success`: `boolean`; \}\>

Defined in: [packages/agentic-control/src/core/subprocess.ts:54](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/subprocess.ts#L54)

Safely execute a command asynchronously with array-based arguments

## Parameters

### command

`string`

### args?

`string`[] = `[]`

### options?

`SpawnOptions` = `{}`

## Returns

`Promise`\<\{ `code`: `number` \| `null`; `stderr`: `string`; `stdout`: `string`; `success`: `boolean`; \}\>
