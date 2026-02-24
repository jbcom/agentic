---
editUrl: false
next: false
prev: false
title: "HandoffManager"
---

Defined in: [packages/agentic-control/src/handoff/manager.ts:55](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/handoff/manager.ts#L55)

## Constructors

### Constructor

> **new HandoffManager**(`options?`): `HandoffManager`

Defined in: [packages/agentic-control/src/handoff/manager.ts:60](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/handoff/manager.ts#L60)

#### Parameters

##### options?

###### anthropicKey?

`string`

###### cursorApiKey?

`string`

###### repo?

`string`

#### Returns

`HandoffManager`

## Methods

### confirmHealthAndBegin()

> **confirmHealthAndBegin**(`successorId`, `predecessorId`): `Promise`\<`void`\>

Defined in: [packages/agentic-control/src/handoff/manager.ts:193](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/handoff/manager.ts#L193)

Called by successor to confirm health

#### Parameters

##### successorId

`string`

##### predecessorId

`string`

#### Returns

`Promise`\<`void`\>

***

### initiateHandoff()

> **initiateHandoff**(`predecessorId`, `options`): `Promise`\<[`HandoffResult`](/api/control/interfaces/handoffresult/)\>

Defined in: [packages/agentic-control/src/handoff/manager.ts:91](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/handoff/manager.ts#L91)

Initiate handoff to successor agent

#### Parameters

##### predecessorId

`string`

##### options

[`HandoffOptions`](/api/control/interfaces/handoffoptions/)

#### Returns

`Promise`\<[`HandoffResult`](/api/control/interfaces/handoffresult/)\>

***

### setRepo()

> **setRepo**(`repo`): `void`

Defined in: [packages/agentic-control/src/handoff/manager.ts:84](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/handoff/manager.ts#L84)

Set the repository for GitHub operations

#### Parameters

##### repo

`string`

#### Returns

`void`

***

### takeover()

> **takeover**(`predecessorId`, `predecessorPr`, `newBranchName`, `options?`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

Defined in: [packages/agentic-control/src/handoff/manager.ts:218](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/handoff/manager.ts#L218)

Called by successor to merge predecessor and take over

#### Parameters

##### predecessorId

`string`

##### predecessorPr

`number`

##### newBranchName

`string`

##### options?

[`TakeoverOptions`](/api/control/interfaces/takeoveroptions/)

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>
