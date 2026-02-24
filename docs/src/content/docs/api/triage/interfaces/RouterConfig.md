---
editUrl: false
next: false
prev: false
title: "RouterConfig"
---

Defined in: [packages/triage/src/scoring/router.ts:15](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L15)

## Properties

### dailyBudget?

> `optional` **dailyBudget**: `number`

Defined in: [packages/triage/src/scoring/router.ts:21](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L21)

Daily cost budget (0 = unlimited)

***

### maxRetries?

> `optional` **maxRetries**: `number`

Defined in: [packages/triage/src/scoring/router.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L19)

Maximum retries per agent before escalating

***

### onAgentSelected()?

> `optional` **onAgentSelected**: (`agent`, `task`) => `void`

Defined in: [packages/triage/src/scoring/router.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L23)

Callback when an agent is selected

#### Parameters

##### agent

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)

##### task

[`AgentTask`](/api/triage/interfaces/agenttask/)

#### Returns

`void`

***

### onCostIncurred()?

> `optional` **onCostIncurred**: (`agent`, `cost`, `task`) => `void`

Defined in: [packages/triage/src/scoring/router.ts:27](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L27)

Callback for cost tracking

#### Parameters

##### agent

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)

##### cost

`number`

##### task

[`AgentTask`](/api/triage/interfaces/agenttask/)

#### Returns

`void`

***

### onEscalate()?

> `optional` **onEscalate**: (`fromAgent`, `toAgent`, `reason`) => `void`

Defined in: [packages/triage/src/scoring/router.ts:25](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L25)

Callback when escalating

#### Parameters

##### fromAgent

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)

##### toAgent

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)

##### reason

`string`

#### Returns

`void`

***

### registry

> **registry**: [`AgentRegistry`](/api/triage/classes/agentregistry/)

Defined in: [packages/triage/src/scoring/router.ts:17](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L17)

Agent registry with available agents
