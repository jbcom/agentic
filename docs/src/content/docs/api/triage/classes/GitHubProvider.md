---
editUrl: false
next: false
prev: false
title: "GitHubProvider"
---

Defined in: [packages/triage/src/providers/github.ts:22](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L22)

Triage Provider Interface

All issue tracking backends must implement this interface.
This allows commands to work with any provider transparently.

## Implements

- [`TriageProvider`](/api/triage/interfaces/triageprovider/)

## Constructors

### Constructor

> **new GitHubProvider**(`config`): `GitHubProvider`

Defined in: [packages/triage/src/providers/github.ts:29](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L29)

#### Parameters

##### config

[`GitHubProviderConfig`](/api/triage/interfaces/githubproviderconfig/)

#### Returns

`GitHubProvider`

## Properties

### displayName

> `readonly` **displayName**: `"GitHub Issues"` = `'GitHub Issues'`

Defined in: [packages/triage/src/providers/github.ts:24](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L24)

Provider display name

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`displayName`](/api/triage/interfaces/triageprovider/#displayname)

***

### name

> `readonly` **name**: `"github"` = `'github'`

Defined in: [packages/triage/src/providers/github.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L23)

Provider name (e.g., 'github', 'beads', 'jira', 'linear')

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`name`](/api/triage/interfaces/triageprovider/#name)

## Methods

### addLabels()

> **addLabels**(`id`, `labels`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/github.ts:169](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L169)

Add labels to an issue

#### Parameters

##### id

`string`

##### labels

`string`[]

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`addLabels`](/api/triage/interfaces/triageprovider/#addlabels)

***

### closeIssue()

> **closeIssue**(`id`, `reason?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/github.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L105)

Close an issue

#### Parameters

##### id

`string`

##### reason?

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`closeIssue`](/api/triage/interfaces/triageprovider/#closeissue)

***

### createIssue()

> **createIssue**(`options`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/github.ts:43](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L43)

Create a new issue

#### Parameters

##### options

[`CreateIssueOptions`](/api/triage/interfaces/createissueoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`createIssue`](/api/triage/interfaces/triageprovider/#createissue)

***

### getBlockedIssues()

> **getBlockedIssues**(): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/github.ts:160](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L160)

Get blocked issues

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getBlockedIssues`](/api/triage/interfaces/triageprovider/#getblockedissues)

***

### getIssue()

> **getIssue**(`id`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\>

Defined in: [packages/triage/src/providers/github.ts:64](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L64)

Get an issue by ID

#### Parameters

##### id

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getIssue`](/api/triage/interfaces/triageprovider/#getissue)

***

### getReadyWork()

> **getReadyWork**(`options?`): `Promise`\<[`ReadyWork`](/api/triage/interfaces/readywork/)[]\>

Defined in: [packages/triage/src/providers/github.ts:148](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L148)

Get issues ready to work on (no blockers)

#### Parameters

##### options?

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

Defined in: [packages/triage/src/providers/github.ts:177](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L177)

Get provider statistics

#### Returns

`Promise`\<[`ProviderStats`](/api/triage/interfaces/providerstats/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`getStats`](/api/triage/interfaces/triageprovider/#getstats)

***

### isReady()

> **isReady**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/providers/github.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L34)

Whether the provider is initialized and ready

#### Returns

`Promise`\<`boolean`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`isReady`](/api/triage/interfaces/triageprovider/#isready)

***

### listIssues()

> **listIssues**(`options?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/github.ts:123](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L123)

List issues with optional filters

#### Parameters

##### options?

[`ListIssuesOptions`](/api/triage/interfaces/listissuesoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`listIssues`](/api/triage/interfaces/triageprovider/#listissues)

***

### removeLabels()

> **removeLabels**(`id`, `labels`): `Promise`\<`void`\>

Defined in: [packages/triage/src/providers/github.ts:173](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L173)

Remove labels from an issue

#### Parameters

##### id

`string`

##### labels

`string`[]

#### Returns

`Promise`\<`void`\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`removeLabels`](/api/triage/interfaces/triageprovider/#removelabels)

***

### reopenIssue()

> **reopenIssue**(`id`, `reason?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/github.ts:114](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L114)

Reopen an issue

#### Parameters

##### id

`string`

##### reason?

`string`

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`reopenIssue`](/api/triage/interfaces/triageprovider/#reopenissue)

***

### searchIssues()

> **searchIssues**(`query`, `options?`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

Defined in: [packages/triage/src/providers/github.ts:165](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L165)

Search issues by text

#### Parameters

##### query

`string`

##### options?

[`ListIssuesOptions`](/api/triage/interfaces/listissuesoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)[]\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`searchIssues`](/api/triage/interfaces/triageprovider/#searchissues)

***

### updateIssue()

> **updateIssue**(`id`, `options`): `Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

Defined in: [packages/triage/src/providers/github.ts:82](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/github.ts#L82)

Update an existing issue

#### Parameters

##### id

`string`

##### options

[`UpdateIssueOptions`](/api/triage/interfaces/updateissueoptions/)

#### Returns

`Promise`\<[`TriageIssue`](/api/triage/interfaces/triageissue/)\>

#### Implementation of

[`TriageProvider`](/api/triage/interfaces/triageprovider/).[`updateIssue`](/api/triage/interfaces/triageprovider/#updateissue)
