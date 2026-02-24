---
editUrl: false
next: false
prev: false
title: "getIssue"
---

> **getIssue**(`issueNumber`, `repoContext?`): `Promise`\<\{ `body`: `string`; `labels`: `string`[]; `number`: `number`; `state`: `string`; `title`: `string`; \}\>

Defined in: [packages/triage/src/octokit.ts:211](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L211)

Get issue details via MCP

## Parameters

### issueNumber

`number`

### repoContext?

#### owner

`string`

#### repo

`string`

## Returns

`Promise`\<\{ `body`: `string`; `labels`: `string`[]; `number`: `number`; `state`: `string`; `title`: `string`; \}\>
