---
editUrl: false
next: false
prev: false
title: "addOrganization"
---

> **addOrganization**(`org`): `void`

Defined in: [packages/agentic-control/src/core/tokens.ts:152](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/tokens.ts#L152)

Add or update an organization configuration

## Parameters

### org

[`OrganizationConfig`](/api/control/interfaces/organizationconfig/)

## Returns

`void`

## Example

```ts
addOrganization({
  name: "my-company",
  tokenEnvVar: "GITHUB_MYCOMPANY_TOKEN",
  defaultBranch: "main",
  isEnterprise: true
});
```
