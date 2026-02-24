---
editUrl: false
next: false
prev: false
title: "AgentResult"
---

Defined in: [packages/triage/src/scoring/agents.ts:60](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L60)

Result from an agent execution

## Type Parameters

### T

`T` = `unknown`

## Properties

### cost

> **cost**: `number`

Defined in: [packages/triage/src/scoring/agents.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L70)

Actual cost incurred (in your cost units)

***

### data?

> `optional` **data**: `T`

Defined in: [packages/triage/src/scoring/agents.ts:64](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L64)

Result data (provider-specific)

***

### error?

> `optional` **error**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:66](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L66)

Error message if failed

***

### escalate?

> `optional` **escalate**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:68](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L68)

Should router escalate to next agent?

***

### jobId?

> `optional` **jobId**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:72](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L72)

For async agents: job ID to poll for completion

***

### success

> **success**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:62](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L62)

Whether the task was completed successfully
