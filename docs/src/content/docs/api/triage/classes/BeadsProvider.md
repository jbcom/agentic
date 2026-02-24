---
editUrl: false
next: false
prev: false
title: "BeadsProvider"
---

Defined in: [packages/triage/src/providers/beads.ts:13](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L13)

Triage Provider Interface

All issue tracking backends must implement this interface.
This allows commands to work with any provider transparently.

## Implements

- [`TriageProvider`](/api/triage/interfaces/triageprovider/)

## Constructors

### Constructor

> **new BeadsProvider**(`config?`): `BeadsProvider`

Defined in: [packages/triage/src/providers/beads.ts:17](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L17)

#### Parameters

##### config?

[`BeadsProviderConfig`](/api/triage/interfaces/beadsproviderconfig/) = `...`

#### Returns

`BeadsProvider`

## Properties

### config

> `readonly` **config**: [`BeadsProviderConfig`](/api/triage/interfaces/beadsproviderconfig/)

Defined in: [packages/triage/src/providers/beads.ts:17](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L17)

***

### displayName

> `readonly` **displayName**: `"Beads"` = `'Beads'`

Defined in: [packages/triage/src/providers/beads.ts:15](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L15)

Provider display name

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`displayName`](/api/triage/interfaces/triageprovider/#displayname)

***

### name

> `readonly` **name**: `"beads"` = `'beads'`

Defined in: [packages/triage/src/providers/beads.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L14)

Provider name (e.g., 'github', 'beads', 'jira', 'linear')

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`name`](/api/triage/interfaces/triageprovider/#name)

## Methods

### addLabels()

> **addLabels**(`_id`, `_labels`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/beads.ts:59](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L59)

Add labels to an issue

#### Parameters

##### \_id

`string`

##### \_labels

`string`[]

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`addLabels`](/api/triage/interfaces/triageprovider/#addlabels)

***

### closeIssue()

> **closeIssue**(`_id`, `_reason?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/beads.ts:35](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L35)

Close an issue

#### Parameters

##### \_id

`string`

##### \_reason?

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`closeIssue`](/api/triage/interfaces/triageprovider/#closeissue)

***

### createIssue()

> **createIssue**(`_options`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/beads.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L23)

Create a new issue

#### Parameters

##### \_options

[`CreateIssueOptions`](/api/triage/interfaces/createissueoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`createIssue`](/api/triage/interfaces/triageprovider/#createissue)

***

### getBlockedIssues()

> **getBlockedIssues**(): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/beads.ts:51](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L51)

Get blocked issues

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getBlockedIssues`](/api/triage/interfaces/triageprovider/#getblockedissues)

***

### getIssue()

> **getIssue**(`_id`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\>

Defined in: [packages/triage/src/providers/beads.ts:27](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L27)

Get an issue by ID

#### Parameters

##### \_id

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getIssue`](/api/triage/interfaces/triageprovider/#getissue)

***

### getReadyWork()

> **getReadyWork**(`_options?`): `Promise`\<[`ReadyWork`](/api/triage/interfaces/readywork/)[]\>

Defined in: [packages/triage/src/providers/beads.ts:47](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L47)

Get issues ready to work on (no blockers)

#### Parameters

##### \_options?

###### limit?

`number`

###### priority?

[`IssuePriority`](/api/triage/type-aliases/issuepriority/)

#### Returns

`Promise`\<[`ReadyWork`](/api/triage/interfaces/readywork/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getReadyWork`](/api/triage/interfaces/triageprovider/#getreadywork)

***

### getStats()

> **getStats**(): `Promise`\<[`ProviderStats`](/api/triage/interfaces/providerstats/)\>

Defined in: [packages/triage/src/providers/beads.ts:67](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L67)

Get provider statistics

#### Returns

`Promise`\<[`ProviderStats`](/api/triage/interfaces/providerstats/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getStats`](/api/triage/interfaces/triageprovider/#getstats)

***

### isReady()

> **isReady**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/providers/beads.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L19)

Whether the provider is initialized and ready

#### Returns

`Promise`\<`boolean`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`isReady`](/api/triage/interfaces/triageprovider/#isready)

***

### listIssues()

> **listIssues**(`_options?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/beads.ts:43](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L43)

List issues with optional filters

#### Parameters

##### \_options?

[`ListIssuesOptions`](/api/triage/interfaces/listissuesoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`listIssues`](/api/triage/interfaces/triageprovider/#listissues)

***

### removeLabels()

> **removeLabels**(`_id`, `_labels`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/beads.ts:63](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L63)

Remove labels from an issue

#### Parameters

##### \_id

`string`

##### \_labels

`string`[]

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`removeLabels`](/api/triage/interfaces/triageprovider/#removelabels)

***

### reopenIssue()

> **reopenIssue**(`_id`, `_reason?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/beads.ts:39](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L39)

Reopen an issue

#### Parameters

##### \_id

`string`

##### \_reason?

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`reopenIssue`](/api/triage/interfaces/triageprovider/#reopenissue)

***

### searchIssues()

> **searchIssues**(`_query`, `_options?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/beads.ts:55](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L55)

Search issues by text

#### Parameters

##### \_query

`string`

##### \_options?

[`ListIssuesOptions`](/api/triage/interfaces/listissuesoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`searchIssues`](/api/triage/interfaces/triageprovider/#searchissues)

***

### updateIssue()

> **updateIssue**(`_id`, `_options`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/beads.ts:31](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/beads.ts#L31)

Update an existing issue

#### Parameters

##### \_id

`string`

##### \_options

[`UpdateIssueOptions`](/api/triage/interfaces/updateissueoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`updateIssue`](/api/triage/interfaces/triageprovider/#updateissue)
