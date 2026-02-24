---
editUrl: false
next: false
prev: false
title: "EscalationLadder"
---

Defined in: [packages/triage/src/escalation/ladder.ts:73](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L73)

Escalation Ladder - intelligently routes tasks through 7 levels

## Constructors

### Constructor

> **new EscalationLadder**(`config?`): `EscalationLadder`

Defined in: [packages/triage/src/escalation/ladder.ts:79](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L79)

#### Parameters

##### config?

`Partial`\<[`EscalationConfig`](/api/triage/interfaces/escalationconfig/)\>

#### Returns

`EscalationLadder`

## Methods

### getAllStates()

> **getAllStates**(): [`EscalationState`](/api/triage/interfaces/escalationstate/)[]

Defined in: [packages/triage/src/escalation/ladder.ts:241](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L241)

Get all states

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)[]

***

### getConfig()

> **getConfig**(): [`EscalationConfig`](/api/triage/interfaces/escalationconfig/)

Defined in: [packages/triage/src/escalation/ladder.ts:255](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L255)

Get configuration

#### Returns

[`EscalationConfig`](/api/triage/interfaces/escalationconfig/)

***

### getCostTracker()

> **getCostTracker**(): [`CostTracker`](/api/triage/classes/costtracker/)

Defined in: [packages/triage/src/escalation/ladder.ts:248](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L248)

Get cost tracker

#### Returns

[`CostTracker`](/api/triage/classes/costtracker/)

***

### getState()

> **getState**(`taskId`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/ladder.ts:227](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L227)

Get state for a task

#### Parameters

##### taskId

`string`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### process()

> **process**(`task`): `Promise`\<[`ProcessResult`](/api/triage/interfaces/processresult/)\>

Defined in: [packages/triage/src/escalation/ladder.ts:96](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L96)

Process a task through the escalation ladder

#### Parameters

##### task

[`Task`](/api/triage/interfaces/task/)

#### Returns

`Promise`\<[`ProcessResult`](/api/triage/interfaces/processresult/)\>

***

### registerHandler()

> **registerHandler**(`level`, `handler`): `this`

Defined in: [packages/triage/src/escalation/ladder.ts:88](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L88)

Register a handler for a specific level

#### Parameters

##### level

[`EscalationLevel`](/api/triage/type-aliases/escalationlevel/)

##### handler

[`LevelHandler`](/api/triage/type-aliases/levelhandler/)

#### Returns

`this`

***

### resetState()

> **resetState**(`taskId`): `void`

Defined in: [packages/triage/src/escalation/ladder.ts:234](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L234)

Reset state for a task

#### Parameters

##### taskId

`string`

#### Returns

`void`

***

### updateConfig()

> **updateConfig**(`update`): `void`

Defined in: [packages/triage/src/escalation/ladder.ts:262](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L262)

Update configuration

#### Parameters

##### update

`Partial`\<[`EscalationConfig`](/api/triage/interfaces/escalationconfig/)\>

#### Returns

`void`
