---
editUrl: false
next: false
prev: false
title: "setTokenConfig"
---

> **setTokenConfig**(`config`): `void`

Defined in: [packages/agentic-control/src/core/tokens.ts:122](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/tokens.ts#L122)

Update the token configuration

## Parameters

### config

`Partial`\<[`TokenConfig`](/api/control/interfaces/tokenconfig/)\>

## Returns

`void`

## Example

```ts
setTokenConfig({
  organizations: {
    "my-org": { name: "my-org", tokenEnvVar: "MY_ORG_TOKEN" }
  },
  prReviewTokenEnvVar: "PR_REVIEW_TOKEN"
});
```
