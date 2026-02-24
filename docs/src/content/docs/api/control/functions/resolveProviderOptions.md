---
editUrl: false
next: false
prev: false
title: "resolveProviderOptions"
---

> **resolveProviderOptions**(`options?`): `object`

Defined in: [packages/agentic-control/src/core/providers.ts:253](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/providers.ts#L253)

Resolve provider options with defaults from config.

## Parameters

### options?

[`ProviderOptions`](/api/control/interfaces/provideroptions/) = `{}`

User-provided options

## Returns

`object`

Resolved provider name, model, and API key

### apiKey

> **apiKey**: `string`

### model

> **model**: `string`

### providerName

> **providerName**: `string`

## Throws

Error if API key is not available
