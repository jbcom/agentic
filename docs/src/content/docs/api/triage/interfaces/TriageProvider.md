---
editUrl: false
next: false
prev: false
title: "TriageProvider"
---

Defined in: [packages/triage/src/providers/types.ts:166](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L166)

Triage Provider Interface

All issue tracking backends must implement this interface.
This allows commands to work with any provider transparently.

## Properties

### displayName

> `readonly` **displayName**: `string`

Defined in: [packages/triage/src/providers/types.ts:171](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L171)

Provider display name

***

### name

> `readonly` **name**: `string`

Defined in: [packages/triage/src/providers/types.ts:168](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L168)

Provider name (e.g., 'github', 'beads', 'jira', 'linear')

## Methods

### addLabels()

> **addLabels**(`id`, `labels`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/types.ts:241](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L241)

Add labels to an issue

#### Parameters

##### id

`string`

##### labels

`string`[]

#### Returns

`Promise`\<`void`\>

***

### closeIssue()

> **closeIssue**(`id`, `reason?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/types.ts:198](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L198)

Close an issue

#### Parameters

##### id

`string`

##### reason?

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

***

### createIssue()

> **createIssue**(`options`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/types.ts:183](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L183)

Create a new issue

#### Parameters

##### options

[`CreateIssueOptions`](/api/triage/interfaces/createissueoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

***

### deleteIssue()?

> `optional` **deleteIssue**(`id`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/types.ts:208](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L208)

Delete an issue (if supported)

#### Parameters

##### id

`string`

#### Returns

`Promise`\<`void`\>

***

### getAvailableLabels()?

> `optional` **getAvailableLabels**(): `Promise`\<`string`[]\>

Defined in: [packages/triage/src/providers/types.ts:251](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L251)

Get all available labels

#### Returns

`Promise`\<`string`[]\>

***

### getBlockedIssues()

> **getBlockedIssues**(): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/types.ts:227](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L227)

Get blocked issues

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

***

### getIssue()

> **getIssue**(`id`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\>

Defined in: [packages/triage/src/providers/types.ts:188](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L188)

Get an issue by ID

#### Parameters

##### id

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\>

***

### getReadyWork()

> **getReadyWork**(`options?`): `Promise`\<[`ReadyWork`](/api/triage/interfaces/readywork/)[]\>

Defined in: [packages/triage/src/providers/types.ts:222](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L222)

Get issues ready to work on (no blockers)

#### Parameters

##### options?

###### limit?

`number`

###### priority?

[`IssuePriority`](/api/triage/type-aliases/issuepriority/)

#### Returns

`Promise`\<[`ReadyWork`](/api/triage/interfaces/readywork/)[]\>

***

### getStats()

> **getStats**(): `Promise`\<[`ProviderStats`](/api/triage/interfaces/providerstats/)\>

Defined in: [packages/triage/src/providers/types.ts:260](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L260)

Get provider statistics

#### Returns

`Promise`\<[`ProviderStats`](/api/triage/interfaces/providerstats/)\>

***

### init()?

> `optional` **init**(`directory?`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/types.ts:270](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L270)

Initialize the provider in a directory (for local providers like Beads)

#### Parameters

##### directory?

`string`

#### Returns

`Promise`\<`void`\>

***

### isReady()

> **isReady**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/providers/types.ts:174](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L174)

Whether the provider is initialized and ready

#### Returns

`Promise`\<`boolean`\>

***

### listIssues()

> **listIssues**(`options?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/types.ts:217](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L217)

List issues with optional filters

#### Parameters

##### options?

[`ListIssuesOptions`](/api/triage/interfaces/listissuesoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

***

### removeLabels()

> **removeLabels**(`id`, `labels`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/types.ts:246](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L246)

Remove labels from an issue

#### Parameters

##### id

`string`

##### labels

`string`[]

#### Returns

`Promise`\<`void`\>

***

### reopenIssue()

> **reopenIssue**(`id`, `reason?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/types.ts:203](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L203)

Reopen an issue

#### Parameters

##### id

`string`

##### reason?

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

***

### searchIssues()

> **searchIssues**(`query`, `options?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/types.ts:232](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L232)

Search issues by text

#### Parameters

##### query

`string`

##### options?

[`ListIssuesOptions`](/api/triage/interfaces/listissuesoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

***

### sync()?

> `optional` **sync**(): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/types.ts:265](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L265)

Sync with remote (for providers that support it)

#### Returns

`Promise`\<`void`\>

***

### updateIssue()

> **updateIssue**(`id`, `options`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/types.ts:193](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L193)

Update an existing issue

#### Parameters

##### id

`string`

##### options

[`UpdateIssueOptions`](/api/triage/interfaces/updateissueoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>
