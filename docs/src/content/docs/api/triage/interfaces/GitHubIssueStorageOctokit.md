---
editUrl: false
next: false
prev: false
title: "GitHubIssueStorageOctokit"
---

Defined in: [packages/triage/src/storage/github-issue.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L30)

Minimal Octokit interface for GitHub operations
Allows dependency injection for testing

## Properties

### rest

> **rest**: `object`

Defined in: [packages/triage/src/storage/github-issue.ts:31](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L31)

#### issues

> **issues**: `object`

##### issues.create()

> **create**(`params`): `Promise`\<\{ `data`: \{ `number`: `number`; \}; \}\>

###### Parameters

###### params

###### body

`string`

###### labels?

`string`[]

###### owner

`string`

###### repo

`string`

###### title

`string`

###### Returns

`Promise`\<\{ `data`: \{ `number`: `number`; \}; \}\>

##### issues.createComment()

> **createComment**(`params`): `Promise`\<\{ `data`: \{ `id`: `number`; \}; \}\>

###### Parameters

###### params

###### body

`string`

###### issue_number

`number`

###### owner

`string`

###### repo

`string`

###### Returns

`Promise`\<\{ `data`: \{ `id`: `number`; \}; \}\>

##### issues.deleteComment()

> **deleteComment**(`params`): `Promise`\<`unknown`\>

###### Parameters

###### params

###### comment_id

`number`

###### owner

`string`

###### repo

`string`

###### Returns

`Promise`\<`unknown`\>

##### issues.get()

> **get**(`params`): `Promise`\<\{ `data`: \{ `body`: `string` \| `null`; `number`: `number`; \}; \}\>

###### Parameters

###### params

###### issue_number

`number`

###### owner

`string`

###### repo

`string`

###### Returns

`Promise`\<\{ `data`: \{ `body`: `string` \| `null`; `number`: `number`; \}; \}\>

##### issues.listComments()

> **listComments**(`params`): `Promise`\<\{ `data`: `object`[]; \}\>

###### Parameters

###### params

###### issue_number

`number`

###### owner

`string`

###### per_page?

`number`

###### repo

`string`

###### Returns

`Promise`\<\{ `data`: `object`[]; \}\>

##### issues.update()

> **update**(`params`): `Promise`\<`unknown`\>

###### Parameters

###### params

###### body

`string`

###### issue_number

`number`

###### owner

`string`

###### repo

`string`

###### Returns

`Promise`\<`unknown`\>
