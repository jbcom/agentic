---
editUrl: false
next: false
prev: false
title: "EscalationState"
---

Defined in: [packages/triage/src/escalation/state.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L23)

State for a single task progressing through escalation levels

## Properties

### approved

> **approved**: `boolean`

Defined in: [packages/triage/src/escalation/state.ts:49](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L49)

Whether human approval has been granted for cloud agents

***

### attempts

> **attempts**: `Record`\<`string`, `number`\>

Defined in: [packages/triage/src/escalation/state.ts:31](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L31)

Attempt counts per level

***

### cost

> **cost**: `number`

Defined in: [packages/triage/src/escalation/state.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L40)

Total cost incurred (in cents)

***

### createdAt

> **createdAt**: `string`

Defined in: [packages/triage/src/escalation/state.ts:43](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L43)

Timestamp when state was created

***

### errors

> **errors**: `string`[]

Defined in: [packages/triage/src/escalation/state.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L34)

Error messages encountered

***

### level

> **level**: [`EscalationLevel`](/api/triage/type-aliases/escalationlevel/)

Defined in: [packages/triage/src/escalation/state.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L28)

Current escalation level (0-6)

***

### resolved

> **resolved**: `boolean`

Defined in: [packages/triage/src/escalation/state.ts:37](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L37)

Whether the task has been resolved

***

### taskId

> **taskId**: `string`

Defined in: [packages/triage/src/escalation/state.ts:25](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L25)

Unique task identifier

***

### updatedAt

> **updatedAt**: `string`

Defined in: [packages/triage/src/escalation/state.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/state.ts#L46)

Timestamp of last update
