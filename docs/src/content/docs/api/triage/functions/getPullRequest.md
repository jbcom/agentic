---
editUrl: false
next: false
prev: false
title: "getPullRequest"
---

> **getPullRequest**(`prNumber`): `Promise`\<\{ `base`: \{ `ref`: `string`; \}; `body`: `string`; `draft`: `boolean`; `head`: \{ `ref`: `string`; `sha`: `string`; \}; `mergeable`: `boolean` \| `null`; `number`: `number`; `state`: `string`; `title`: `string`; \}\>

Defined in: [packages/triage/src/octokit.ts:452](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L452)

Get pull request details via MCP

## Parameters

### prNumber

`number`

## Returns

`Promise`\<\{ `base`: \{ `ref`: `string`; \}; `body`: `string`; `draft`: `boolean`; `head`: \{ `ref`: `string`; `sha`: `string`; \}; `mergeable`: `boolean` \| `null`; `number`: `number`; `state`: `string`; `title`: `string`; \}\>
