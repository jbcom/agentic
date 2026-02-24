---
editUrl: false
next: false
prev: false
title: "createMockGitHubPR"
---

> **createMockGitHubPR**(`overrides?`): `object`

Defined in: [fixtures.ts:423](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L423)

Create a mock GitHub pull request for testing.

## Parameters

### overrides?

`Partial`\<\{ `base`: `string`; `body`: `string`; `head`: `string`; `labels`: `string`[]; `number`: `number`; `state`: `"open"` \| `"closed"` \| `"merged"`; `title`: `string`; \}\> = `{}`

Optional overrides

## Returns

`object`

GitHub pull request object

### base

> **base**: `object`

#### base.ref

> **ref**: `string`

### body

> **body**: `string`

### created\_at

> **created\_at**: `string`

### head

> **head**: `object`

#### head.ref

> **ref**: `string`

### labels

> **labels**: `object`[]

### merged

> **merged**: `boolean`

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
