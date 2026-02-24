---
editUrl: false
next: false
prev: false
title: "createMockGitHubIssue"
---

> **createMockGitHubIssue**(`overrides?`): `object`

Defined in: [fixtures.ts:387](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L387)

Create a mock GitHub issue for testing.

## Parameters

### overrides?

`Partial`\<\{ `body`: `string`; `labels`: `string`[]; `number`: `number`; `state`: `"open"` \| `"closed"`; `title`: `string`; \}\> = `{}`

Optional overrides

## Returns

`object`

GitHub issue object

### body

> **body**: `string`

### created\_at

> **created\_at**: `string`

### labels

> **labels**: `object`[]

### number

> **number**: `number`

### state

> **state**: `"open"` \| `"closed"`

### title

> **title**: `string`

### updated\_at

> **updated\_at**: `string`

### user

> **user**: `object`

#### user.login

> **login**: `string`
