---
editUrl: false
next: false
prev: false
title: "getEnvForRepo"
---

> **getEnvForRepo**(`repoUrl`): `Record`\<`string`, `string`\>

Defined in: [packages/agentic-control/src/core/tokens.ts:373](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/tokens.ts#L373)

Create environment variables object for a subprocess targeting a specific org
Useful when spawning child processes that need the correct GitHub token

## Parameters

### repoUrl

`string`

Repository URL to get token for

## Returns

`Record`\<`string`, `string`\>

Object with GH_TOKEN and GITHUB_TOKEN set

## Example

```ts
import { spawnSync } from 'node:child_process';
const proc = spawnSync('gh', ['pr', 'list'], {
  env: { ...process.env, ...getEnvForRepo("owner/repo") }
});
```
