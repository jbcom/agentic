---
editUrl: false
next: false
prev: false
title: "CostTracker"
---

Defined in: [packages/triage/src/escalation/cost-tracker.ts:43](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L43)

Cost tracker for managing cloud agent budgets

## Constructors

### Constructor

> **new CostTracker**(`dailyBudget`, `options?`): `CostTracker`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L48)

#### Parameters

##### dailyBudget

`number`

##### options?

###### onBudgetWarning?

(`remaining`, `total`) => `void`

#### Returns

`CostTracker`

## Methods

### canAfford()

> **canAfford**(`amount`, `date?`): `boolean`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:89](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L89)

Check if operation is within budget

#### Parameters

##### amount

`number`

##### date?

`string`

#### Returns

`boolean`

***

### cleanup()

> **cleanup**(`keepDays?`): `void`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:162](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L162)

Clear old entries (keep last N days)

#### Parameters

##### keepDays?

`number` = `30`

#### Returns

`void`

***

### export()

> **export**(): `Record`\<`string`, [`CostEntry`](/api/triage/interfaces/costentry/)[]\>

Defined in: [packages/triage/src/escalation/cost-tracker.ts:178](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L178)

Export all data (for persistence)

#### Returns

`Record`\<`string`, [`CostEntry`](/api/triage/interfaces/costentry/)[]\>

***

### getDailyBudget()

> **getDailyBudget**(): `number`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:214](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L214)

Get current daily budget

#### Returns

`number`

***

### getDailyStats()

> **getDailyStats**(`date?`): [`DailyCostStats`](/api/triage/interfaces/dailycoststats/)

Defined in: [packages/triage/src/escalation/cost-tracker.ts:108](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L108)

Get daily statistics

#### Parameters

##### date?

`string`

#### Returns

[`DailyCostStats`](/api/triage/interfaces/dailycoststats/)

***

### getRemainingBudget()

> **getRemainingBudget**(`date?`): `number`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:99](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L99)

Get remaining budget for today

#### Parameters

##### date?

`string`

#### Returns

`number`

***

### getStatsInRange()

> **getStatsInRange**(`startDate`, `endDate`): [`DailyCostStats`](/api/triage/interfaces/dailycoststats/)[]

Defined in: [packages/triage/src/escalation/cost-tracker.ts:132](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L132)

Get stats for a date range

#### Parameters

##### startDate

`string`

##### endDate

`string`

#### Returns

[`DailyCostStats`](/api/triage/interfaces/dailycoststats/)[]

***

### getTotalCost()

> **getTotalCost**(): `number`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:150](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L150)

Get all-time total cost

#### Returns

`number`

***

### import()

> **import**(`data`): `void`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:190](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L190)

Import data (from persistence)

#### Parameters

##### data

`Record`\<`string`, [`CostEntry`](/api/triage/interfaces/costentry/)[]\>

#### Returns

`void`

***

### record()

> **record**(`taskId`, `agent`, `amount`, `description?`): [`CostEntry`](/api/triage/interfaces/costentry/)

Defined in: [packages/triage/src/escalation/cost-tracker.ts:61](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L61)

Record a cost entry

#### Parameters

##### taskId

`string`

##### agent

`string`

##### amount

`number`

##### description?

`string` = `'Cloud agent operation'`

#### Returns

[`CostEntry`](/api/triage/interfaces/costentry/)

***

### reset()

> **reset**(): `void`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:200](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L200)

Reset all tracking data

#### Returns

`void`

***

### setDailyBudget()

> **setDailyBudget**(`budget`): `void`

Defined in: [packages/triage/src/escalation/cost-tracker.ts:207](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/cost-tracker.ts#L207)

Update daily budget

#### Parameters

##### budget

`number`

#### Returns

`void`
