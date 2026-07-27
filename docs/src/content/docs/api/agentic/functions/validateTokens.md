---
editUrl: false
next: false
prev: false
title: "validateTokens"
---

> **validateTokens**(`orgs?`): [`Result`](/api/agentic/interfaces/result/)\<`string`[]\>

Defined in: [packages/agentic/src/core/tokens.ts:278](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic/src/core/tokens.ts#L278)

Validate that required tokens are available

## Parameters

### orgs?

`string`[]

Organization names to validate (optional, validates all configured if not specified)

## Returns

[`Result`](/api/agentic/interfaces/result/)\<`string`[]\>

Validation result with any missing tokens
