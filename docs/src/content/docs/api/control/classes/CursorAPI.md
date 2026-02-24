---
editUrl: false
next: false
prev: false
title: "CursorAPI"
---

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:156](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L156)

## Constructors

### Constructor

> **new CursorAPI**(`options?`): `CursorAPI`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:163](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L163)

#### Parameters

##### options?

[`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/) = `{}`

#### Returns

`CursorAPI`

## Methods

### addFollowup()

> **addFollowup**(`agentId`, `prompt`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:361](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L361)

Send a follow-up message to an agent

#### Parameters

##### agentId

`string`

##### prompt

###### text

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`void`\>\>

***

### getAgentConversation()

> **getAgentConversation**(`agentId`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Conversation`](/api/control/interfaces/conversation/)\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:310](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L310)

Get conversation history for an agent

#### Parameters

##### agentId

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Conversation`](/api/control/interfaces/conversation/)\>\>

***

### getAgentStatus()

> **getAgentStatus**(`agentId`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:301](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L301)

Get status of a specific agent

#### Parameters

##### agentId

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

***

### launchAgent()

> **launchAgent**(`options`): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:321](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L321)

Launch a new agent

API Spec: https://cursor.com/docs/cloud-agent/api/endpoints

#### Parameters

##### options

###### prompt

\{ `images?`: `object`[]; `text`: `string`; \}

###### prompt.images?

`object`[]

###### prompt.text

`string`

###### source

\{ `ref?`: `string`; `repository`: `string`; \}

###### source.ref?

`string`

###### source.repository

`string`

###### target?

\{ `autoCreatePr?`: `boolean`; `branchName?`: `string`; `openAsCursorGithubApp?`: `boolean`; `skipReviewerRequest?`: `boolean`; \}

###### target.autoCreatePr?

`boolean`

###### target.branchName?

`string`

###### target.openAsCursorGithubApp?

`boolean`

###### target.skipReviewerRequest?

`boolean`

###### webhook?

\{ `secret?`: `string`; `url`: `string`; \}

###### webhook.secret?

`string`

###### webhook.url

`string`

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)\>\>

***

### listAgents()

> **listAgents**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:292](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L292)

List all agents

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Agent`](/api/control/interfaces/agent/)[]\>\>

***

### listModels()

> **listModels**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<`string`[]\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:382](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L382)

List available models

API Spec: https://cursor.com/docs/cloud-agent/api/endpoints#list-models

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<`string`[]\>\>

***

### listRepositories()

> **listRepositories**(): `Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)[]\>\>

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:371](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L371)

List available repositories

#### Returns

`Promise`\<[`Result`](/api/control/interfaces/result/)\<[`Repository`](/api/control/interfaces/repository/)[]\>\>

***

### isAvailable()

> `static` **isAvailable**(): `boolean`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:181](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L181)

Check if API key is available

#### Returns

`boolean`
