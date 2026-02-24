---
editUrl: false
next: false
prev: false
title: "submitPRReview"
---

> **submitPRReview**(`prNumber`, `event`, `body`): `Promise`\<`void`\>

Defined in: [packages/triage/src/octokit.ts:840](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L840)

Submit a PR review

## Parameters

### prNumber

`number`

### event

`"APPROVE"` | `"REQUEST_CHANGES"` | `"COMMENT"`

### body

`string`

## Returns

`Promise`\<`void`\>
