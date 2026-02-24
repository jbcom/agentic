---
editUrl: false
next: false
prev: false
title: "addIssueComment"
---

> **addIssueComment**(`issueNumber`, `body`): `Promise`\<`void`\>

Defined in: [packages/triage/src/octokit.ts:249](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L249)

Create a comment on an issue or PR via MCP.

Notes:
- GitHub treats PR comments as "issue comments" on the PR's issue thread.

## Parameters

### issueNumber

`number`

### body

`string`

## Returns

`Promise`\<`void`\>
