---
editUrl: false
next: false
prev: false
title: "ProcessResult"
---

Defined in: [packages/triage/src/escalation/ladder.ts:39](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L39)

Result from processing a task

## Properties

### attempts

> **attempts**: `number`

Defined in: [packages/triage/src/escalation/ladder.ts:51](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L51)

Number of attempts made

***

### cost

> **cost**: `number`

Defined in: [packages/triage/src/escalation/ladder.ts:49](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L49)

Total cost incurred (in cents)

***

### data?

> `optional` **data**: `unknown`

Defined in: [packages/triage/src/escalation/ladder.ts:45](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L45)

Result data (agent-specific)

***

### error?

> `optional` **error**: `string`

Defined in: [packages/triage/src/escalation/ladder.ts:47](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L47)

Error message if failed

***

### level

> **level**: [`EscalationLevel`](/api/triage/type-aliases/escalationlevel/)

Defined in: [packages/triage/src/escalation/ladder.ts:43](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L43)

Final escalation level reached

***

### success

> **success**: `boolean`

Defined in: [packages/triage/src/escalation/ladder.ts:41](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L41)

Whether the task was resolved

***

### trail

> **trail**: `object`[]

Defined in: [packages/triage/src/escalation/ladder.ts:53](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L53)

Trail of levels attempted

#### error?

> `optional` **error**: `string`

#### level

> **level**: [`EscalationLevel`](/api/triage/type-aliases/escalationlevel/)

#### success

> **success**: `boolean`
