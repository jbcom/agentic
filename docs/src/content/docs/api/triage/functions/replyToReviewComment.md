---
editUrl: false
next: false
prev: false
title: "replyToReviewComment"
---

> **replyToReviewComment**(`prNumber`, `commentNodeId`, `body`): `Promise`\<`void`\>

Defined in: [packages/triage/src/octokit.ts:815](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L815)

Reply to a review comment

## Parameters

### prNumber

`number`

The PR number

### commentNodeId

`string`

The GraphQL node ID of the comment to reply to (from ReviewComment.nodeId)

### body

`string`

The reply body

## Returns

`Promise`\<`void`\>
