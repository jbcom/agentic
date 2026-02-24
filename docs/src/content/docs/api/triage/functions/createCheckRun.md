---
editUrl: false
next: false
prev: false
title: "createCheckRun"
---

> **createCheckRun**(`_name`, `_headSha`, `_options?`): `Promise`\<`number`\>

Defined in: [packages/triage/src/octokit.ts:953](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L953)

## Parameters

### \_name

`string`

### \_headSha

`string`

### \_options?

#### conclusion?

`"cancelled"` \| `"success"` \| `"failure"` \| `"neutral"` \| `"skipped"` \| `"timed_out"` \| `"action_required"`

#### status?

`"completed"` \| `"in_progress"` \| `"queued"`

#### summary?

`string`

#### text?

`string`

#### title?

`string`

## Returns

`Promise`\<`number`\>
