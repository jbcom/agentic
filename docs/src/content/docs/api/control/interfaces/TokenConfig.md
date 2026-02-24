---
editUrl: false
next: false
prev: false
title: "TokenConfig"
---

Defined in: [packages/agentic-control/src/core/types.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L28)

Token configuration for intelligent switching between orgs

## Properties

### defaultTokenEnvVar

> **defaultTokenEnvVar**: `string`

Defined in: [packages/agentic-control/src/core/types.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L32)

Default token env var when org is unknown

***

### organizations

> **organizations**: `Record`\<`string`, [`OrganizationConfig`](/api/control/interfaces/organizationconfig/)\>

Defined in: [packages/agentic-control/src/core/types.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L30)

Mapping of org name to configuration

***

### prReviewTokenEnvVar

> **prReviewTokenEnvVar**: `string`

Defined in: [packages/agentic-control/src/core/types.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L34)

Token env var to ALWAYS use for PR reviews (ensures consistent identity)
