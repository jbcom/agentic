---
editUrl: false
next: false
prev: false
title: "extractOrg"
---

> **extractOrg**(`repoUrl`): `string` \| `null`

Defined in: [packages/agentic-control/src/core/tokens.ts:176](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/tokens.ts#L176)

Extract organization name from a repository URL or full name
Uses a safe regex pattern to prevent ReDoS attacks

## Parameters

### repoUrl

`string`

## Returns

`string` \| `null`

## Example

```ts
extractOrg("https://github.com/my-org/my-repo") // "my-org"
extractOrg("my-org/my-repo") // "my-org"
extractOrg("git@github.com:my-org/my-repo.git") // "my-org"
```
