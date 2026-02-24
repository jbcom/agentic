---
editUrl: false
next: false
prev: false
title: "getTokenForRepo"
---

> **getTokenForRepo**(`repoUrl`): `string` \| `undefined`

Defined in: [packages/agentic-control/src/core/tokens.ts:243](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/tokens.ts#L243)

Get the token for a repository URL
Automatically extracts the organization and returns the appropriate token

## Parameters

### repoUrl

`string`

Repository URL or owner/repo format

## Returns

`string` \| `undefined`

Token value or undefined if not set

## Example

```ts
// If configured: addOrganization({ name: "myorg", tokenEnvVar: "MYORG_TOKEN" })
getTokenForRepo("https://github.com/myorg/my-repo")
// Returns value of MYORG_TOKEN
```
