---
editUrl: false
next: false
prev: false
title: "AgentRegistry"
---

Defined in: [packages/triage/src/scoring/agents.ts:132](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L132)

Registry for managing available agents

## Example

```typescript
const registry = new AgentRegistry();

// Register your agents (implementations from @jbcom/agentic or custom)
registry.register(myOllamaAgent);
registry.register(myJulesAgent);
registry.register(myCustomAgent);

// Get best agent for a complexity tier
const agent = registry.optimalFor('moderate');
```

## Constructors

### Constructor

> **new AgentRegistry**(): `AgentRegistry`

#### Returns

`AgentRegistry`

## Accessors

### size

#### Get Signature

> **get** **size**(): `number`

Defined in: [packages/triage/src/scoring/agents.ts:243](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L243)

Get count of registered agents

##### Returns

`number`

## Methods

### all()

> **all**(): [`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

Defined in: [packages/triage/src/scoring/agents.ts:193](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L193)

Get all registered agents

#### Returns

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

***

### clear()

> **clear**(): `void`

Defined in: [packages/triage/src/scoring/agents.ts:250](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L250)

Clear all agents

#### Returns

`void`

***

### enabled()

> **enabled**(): [`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

Defined in: [packages/triage/src/scoring/agents.ts:200](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L200)

Get all enabled agents

#### Returns

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

***

### export()

> **export**(): `Omit`\<[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>, `"execute"`\>[]

Defined in: [packages/triage/src/scoring/agents.ts:257](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L257)

Export registry configuration (for serialization)

#### Returns

`Omit`\<[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>, `"execute"`\>[]

***

### forTier()

> **forTier**(`tier`, `includeDisabled?`): [`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

Defined in: [packages/triage/src/scoring/agents.ts:208](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L208)

Get agents that can handle a specific complexity tier
Sorted by priority (lowest first), then by cost

#### Parameters

##### tier

[`ComplexityTier`](/api/triage/type-aliases/complexitytier/)

##### includeDisabled?

`boolean` = `false`

#### Returns

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

***

### get()

> **get**(`id`): [`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\> \| `undefined`

Defined in: [packages/triage/src/scoring/agents.ts:229](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L229)

Get agent by ID

#### Parameters

##### id

`string`

#### Returns

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\> \| `undefined`

***

### has()

> **has**(`id`): `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:236](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L236)

Check if an agent is registered

#### Parameters

##### id

`string`

#### Returns

`boolean`

***

### optimalFor()

> **optimalFor**(`tier`): [`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\> \| `undefined`

Defined in: [packages/triage/src/scoring/agents.ts:222](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L222)

Get the optimal (first choice) agent for a complexity tier

#### Parameters

##### tier

[`ComplexityTier`](/api/triage/type-aliases/complexitytier/)

#### Returns

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\> \| `undefined`

***

### register()

> **register**\<`T`\>(`agent`): `this`

Defined in: [packages/triage/src/scoring/agents.ts:138](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L138)

Register an agent

#### Type Parameters

##### T

`T`

#### Parameters

##### agent

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`T`\>

#### Returns

`this`

***

### registerAll()

> **registerAll**(`agents`): `this`

Defined in: [packages/triage/src/scoring/agents.ts:146](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L146)

Register multiple agents at once

#### Parameters

##### agents

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`unknown`\>[]

#### Returns

`this`

***

### setCost()

> **setCost**(`id`, `cost`): `void`

Defined in: [packages/triage/src/scoring/agents.ts:183](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L183)

Update an agent's cost (for dynamic pricing)

#### Parameters

##### id

`string`

##### cost

`number`

#### Returns

`void`

***

### setEnabled()

> **setEnabled**(`id`, `enabled`): `void`

Defined in: [packages/triage/src/scoring/agents.ts:163](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L163)

Enable/disable an agent at runtime

#### Parameters

##### id

`string`

##### enabled

`boolean`

#### Returns

`void`

***

### setPriority()

> **setPriority**(`id`, `priority`): `void`

Defined in: [packages/triage/src/scoring/agents.ts:173](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L173)

Update an agent's priority (for dynamic rebalancing)

#### Parameters

##### id

`string`

##### priority

`number`

#### Returns

`void`

***

### unregister()

> **unregister**(`id`): `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:156](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L156)

Unregister an agent

#### Parameters

##### id

`string`

#### Returns

`boolean`
