---
editUrl: false
next: false
prev: false
title: "Fleet"
---

Defined in: [packages/agentic-control/src/fleet/fleet.ts:86](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L86)

Fleet management for Cursor Background Agents

Provides high-level operations for managing AI agents across multiple
GitHub organizations with automatic token switching and coordination.

## Example

```typescript
const fleet = new Fleet();

// Spawn a new agent
const result = await fleet.spawn({
  repository: 'https://github.com/owner/repo',
  task: 'Fix the authentication bug',
  target: { autoCreatePr: true }
});

// Monitor agent status
const status = await fleet.status(result.data.id);
```

## Constructors

### Constructor

> **new Fleet**(`config?`): `Fleet`

Defined in: [packages/agentic-control/src/fleet/fleet.ts:96](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L96)

Create a new Fleet instance

#### Parameters

##### config?

[`FleetConfig`](/api/control/interfaces/fleetconfig/) = `{}`

Fleet configuration options including API key and timeout

#### Returns

`Fleet`

## Methods

### archive()

> **archive**(`agentId`, `outputPath?`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`string`\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:278](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L278)

Archive agent conversation to disk

#### Parameters

##### agentId

`string`

##### outputPath?

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`string`\>\>

***

### broadcast()

> **broadcast**(`agentIds`, `message`): `Promise`\<`Map`\<`string`, [`Result`](/api/control/interfaces/result/)\<`void`\>\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:249](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L249)

Broadcast message to multiple agents

#### Parameters

##### agentIds

`string`[]

##### message

`string`

#### Returns

`Promise`\<`Map`\<`string`, [`Result`](/api/control/interfaces/result/)\<`void`\>\>\>

***

### conversation()

> **conversation**(`agentId`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Conversation`](/api/control/interfaces/conversation/)\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:268](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L268)

Get agent conversation

#### Parameters

##### agentId

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Conversation`](/api/control/interfaces/conversation/)\>\>

***

### coordinate()

> **coordinate**(`config`): `Promise`\<`void`\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:495](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L495)

Run bidirectional coordination loop with intelligent token switching

#### Parameters

##### config

[`CoordinationConfig`](/api/control/interfaces/coordinationconfig/)

#### Returns

`Promise`\<`void`\>

***

### createDiamond()

> **createDiamond**(`config`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<\{ `counterpartyAgent`: [`Agent`](/api/control/interfaces/agent/); `targetAgents`: [`Agent`](/api/control/interfaces/agent/)[]; \}\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:324](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L324)

Create a diamond pattern orchestration

#### Parameters

##### config

[`DiamondConfig`](/api/control/interfaces/diamondconfig/)

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<\{ `counterpartyAgent`: [`Agent`](/api/control/interfaces/agent/); `targetAgents`: [`Agent`](/api/control/interfaces/agent/)[]; \}\>\>

***

### find()

> **find**(`agentId`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/) \| `undefined`\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:159](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L159)

Find agent by ID

#### Parameters

##### agentId

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/) \| `undefined`\>\>

***

### followup()

> **followup**(`agentId`, `message`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:239](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L239)

Send a follow-up message to an agent

#### Parameters

##### agentId

`string`

##### message

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

***

### isApiAvailable()

> **isApiAvailable**(): `boolean`

Defined in: [packages/agentic-control/src/fleet/fleet.ts:119](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L119)

Check if direct API is available

#### Returns

`boolean`

***

### list()

> **list**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:130](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L130)

List all agents

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

***

### listByStatus()

> **listByStatus**(`status`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:140](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L140)

List agents filtered by status

#### Parameters

##### status

[`AgentStatus`](/api/control/type-aliases/agentstatus/)

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

***

### listModels()

> **listModels**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`string`[]\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:310](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L310)

List available Cursor models

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`string`[]\>\>

***

### monitorAgents()

> **monitorAgents**(`agentIds`, `options?`): `Promise`\<`Map`\<`string`, [`Agent`](/api/control/interfaces/agent/)\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:451](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L451)

Monitor specific agents until all complete

#### Parameters

##### agentIds

`string`[]

##### options?

###### onProgress?

(`status`) => `void`

###### pollInterval?

`number`

#### Returns

`Promise`\<`Map`\<`string`, [`Agent`](/api/control/interfaces/agent/)\>\>

***

### repositories()

> **repositories**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)[]\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:300](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L300)

List available repositories

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)[]\>\>

***

### running()

> **running**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:152](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L152)

Get running agents only

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

***

### spawn()

> **spawn**(`options`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:186](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L186)

Spawn a new agent

API Spec: https://cursor.com/docs/cloud-agent/api/endpoints

#### Parameters

##### options

[`SpawnOptions`](/api/control/interfaces/spawnoptions/) & `object`

Spawn options including repository, task, ref, target, and webhook

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

***

### status()

> **status**(`agentId`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:168](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L168)

Get agent status

#### Parameters

##### agentId

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

***

### summary()

> **summary**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<\{ `agents`: [`Agent`](/api/control/interfaces/agent/)[]; `completed`: `number`; `failed`: `number`; `running`: `number`; `total`: `number`; \}\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:395](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L395)

Get fleet summary

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<\{ `agents`: [`Agent`](/api/control/interfaces/agent/)[]; `completed`: `number`; `failed`: `number`; `running`: `number`; `total`: `number`; \}\>\>

***

### waitFor()

> **waitFor**(`agentId`, `options?`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

Defined in: [packages/agentic-control/src/fleet/fleet.ts:423](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L423)

Wait for agent to complete

#### Parameters

##### agentId

`string`

##### options?

###### pollInterval?

`number`

###### timeout?

`number`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>
