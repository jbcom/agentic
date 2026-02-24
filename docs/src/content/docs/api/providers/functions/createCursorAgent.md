---
editUrl: false
next: false
prev: false
title: "createCursorAgent"
---

> **createCursorAgent**(`id`, `config`, `options?`): `AgentDefinition`\<[`CursorAgentResult`](/api/providers/interfaces/cursoragentresult/)\>

Defined in: [cursor.ts:35](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/providers/src/cursor.ts#L35)

Create a Cursor Cloud Agent for the registry

⚠️ EXPENSIVE - Requires explicit approval by default

## Parameters

### id

`string`

### config

[`CursorConfig`](/api/providers/interfaces/cursorconfig/)

### options?

#### capabilities?

`Partial`\<`AgentCapabilities`\>

#### cost?

`number`

#### name?

`string`

#### priority?

`number`

#### requiresApproval?

`boolean`

## Returns

`AgentDefinition`\<[`CursorAgentResult`](/api/providers/interfaces/cursoragentresult/)\>
