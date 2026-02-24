---
editUrl: false
next: false
prev: false
title: "RoutingResult"
---

Defined in: [packages/triage/src/scoring/router.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L30)

## Type Parameters

### T

`T` = `unknown`

## Properties

### agent

> **agent**: `string`

Defined in: [packages/triage/src/scoring/router.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L34)

Which agent handled the task

***

### attempts

> **attempts**: `number`

Defined in: [packages/triage/src/scoring/router.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L40)

Number of attempts made

***

### result

> **result**: [`AgentResult`](/api/triage/interfaces/agentresult/)\<`T`\>

Defined in: [packages/triage/src/scoring/router.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L36)

The result from the agent

***

### success

> **success**: `boolean`

Defined in: [packages/triage/src/scoring/router.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L32)

Whether the task was successfully completed

***

### totalCost

> **totalCost**: `number`

Defined in: [packages/triage/src/scoring/router.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L38)

Total cost incurred

***

### trail

> **trail**: `object`[]

Defined in: [packages/triage/src/scoring/router.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L42)

Trail of agents tried

#### agent

> **agent**: `string`

#### error?

> `optional` **error**: `string`

#### success

> **success**: `boolean`
