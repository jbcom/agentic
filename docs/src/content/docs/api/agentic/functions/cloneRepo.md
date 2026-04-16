---
editUrl: false
next: false
prev: false
title: "cloneRepo"
---

> **cloneRepo**(`repoUrl`, `destPath`): [`Result`](/api/agentic/interfaces/result/)\<`void`\>

Defined in: [packages/agentic/src/github/client.ts:712](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic/src/github/client.ts#L712)

Clone a repository with appropriate token.
Uses spawnSync for safe command execution (no shell injection).

## Parameters

### repoUrl

`string`

### destPath

`string`

## Returns

[`Result`](/api/agentic/interfaces/result/)\<`void`\>
