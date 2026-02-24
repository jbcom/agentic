---
editUrl: false
next: false
prev: false
title: "EscalationStateManager"
---

Defined in: [packages/triage/src/escalation/state.ts:55](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L55)

Manager for escalation states across multiple tasks

## Constructors

### Constructor

> **new EscalationStateManager**(): `EscalationStateManager`

#### Returns

`EscalationStateManager`

## Methods

### addCost()

> **addCost**(`taskId`, `cost`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:138](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L138)

Add cost to task

#### Parameters

##### taskId

`string`

##### cost

`number`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### clear()

> **clear**(): `void`

Defined in: [packages/triage/src/escalation/state.ts:174](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L174)

Clear all states

#### Returns

`void`

***

### escalate()

> **escalate**(`taskId`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:122](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L122)

Escalate to next level

#### Parameters

##### taskId

`string`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### getAllStates()

> **getAllStates**(): [`EscalationState`](/api/triage/interfaces/escalationstate/)[]

Defined in: [packages/triage/src/escalation/state.ts:160](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L160)

Get all states

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)[]

***

### getState()

> **getState**(`taskId`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:61](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L61)

Create or get state for a task

#### Parameters

##### taskId

`string`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### getTotalCost()

> **getTotalCost**(): `number`

Defined in: [packages/triage/src/escalation/state.ts:181](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L181)

Get total cost across all tasks

#### Returns

`number`

***

### getUnresolved()

> **getUnresolved**(): [`EscalationState`](/api/triage/interfaces/escalationstate/)[]

Defined in: [packages/triage/src/escalation/state.ts:167](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L167)

Get unresolved states

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)[]

***

### recordAttempt()

> **recordAttempt**(`taskId`, `level`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:100](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L100)

Record an attempt at a level

#### Parameters

##### taskId

`string`

##### level

[`EscalationLevel`](/api/triage/type-aliases/escalationlevel/)

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### recordError()

> **recordError**(`taskId`, `error`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:113](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L113)

Record an error

#### Parameters

##### taskId

`string`

##### error

`string`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### resetState()

> **resetState**(`taskId`): `void`

Defined in: [packages/triage/src/escalation/state.ts:153](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L153)

Reset state for a task

#### Parameters

##### taskId

`string`

#### Returns

`void`

***

### resolve()

> **resolve**(`taskId`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:131](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L131)

Mark task as resolved

#### Parameters

##### taskId

`string`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### setApproval()

> **setApproval**(`taskId`, `approved`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:146](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L146)

Set approval status

#### Parameters

##### taskId

`string`

##### approved

`boolean`

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)

***

### updateState()

> **updateState**(`taskId`, `update`): [`EscalationState`](/api/triage/interfaces/escalationstate/)

Defined in: [packages/triage/src/escalation/state.ts:86](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L86)

Update state for a task

#### Parameters

##### taskId

`string`

##### update

`Partial`\<`Omit`\<[`EscalationState`](/api/triage/interfaces/escalationstate/), `"taskId"` \| `"createdAt"`\>\>

#### Returns

[`EscalationState`](/api/triage/interfaces/escalationstate/)
