---
editUrl: false
next: false
prev: false
title: "AgentCapabilities"
---

Defined in: [packages/triage/src/scoring/agents.ts:22](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L22)

What an agent is capable of doing

## Indexable

\[`key`: `string`\]: `unknown`

Custom capability flags

## Properties

### async?

> `optional` **async**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L32)

Is this agent async (returns job ID to poll)?

***

### canCreatePR?

> `optional` **canCreatePR**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L28)

Can this agent create PRs?

***

### canExecute?

> `optional` **canExecute**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L30)

Can this agent run commands?

***

### maxContext?

> `optional` **maxContext**: `number`

Defined in: [packages/triage/src/scoring/agents.ts:26](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L26)

Maximum context length (tokens/chars)

***

### tiers

> **tiers**: [`ComplexityTier`](/api/triage/type-aliases/complexitytier/)[]

Defined in: [packages/triage/src/scoring/agents.ts:24](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L24)

Complexity tiers this agent can handle
