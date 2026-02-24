---
editUrl: false
next: false
prev: false
title: "TaskRouter"
---

Defined in: [packages/triage/src/scoring/router.ts:57](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L57)

Task Router - intelligently routes tasks to agents

## Constructors

### Constructor

> **new TaskRouter**(`config`): `TaskRouter`

Defined in: [packages/triage/src/scoring/router.ts:62](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L62)

#### Parameters

##### config

[`RouterConfig`](/api/triage/interfaces/routerconfig/)

#### Returns

`TaskRouter`

## Methods

### getRemainingBudget()

> **getRemainingBudget**(): `number`

Defined in: [packages/triage/src/scoring/router.ts:236](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L236)

Get remaining daily budget

#### Returns

`number`

***

### getState()

> **getState**(): [`RouterState`](/api/triage/interfaces/routerstate/)

Defined in: [packages/triage/src/scoring/router.ts:229](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L229)

Get current router state

#### Returns

[`RouterState`](/api/triage/interfaces/routerstate/)

***

### route()

> **route**(`task`, `complexity`): `Promise`\<[`RoutingResult`](/api/triage/interfaces/routingresult/)\<`unknown`\>\>

Defined in: [packages/triage/src/scoring/router.ts:78](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/router.ts#L78)

Route a task to the optimal agent

#### Parameters

##### task

`Omit`\<[`AgentTask`](/api/triage/interfaces/agenttask/), `"complexityScore"` \| `"complexityTier"`\>

##### complexity

[`ComplexityScore`](/api/triage/interfaces/complexityscore/)

#### Returns

`Promise`\<[`RoutingResult`](/api/triage/interfaces/routingresult/)\<`unknown`\>\>
