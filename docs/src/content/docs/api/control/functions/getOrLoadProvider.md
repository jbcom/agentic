---
editUrl: false
next: false
prev: false
title: "getOrLoadProvider"
---

> **getOrLoadProvider**(`providerName`, `apiKey`): `Promise`\<[`ModelFactory`](/api/control/type-aliases/modelfactory/)\>

Defined in: [packages/agentic-control/src/core/providers.ts:211](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/providers.ts#L211)

Get or create a cached provider instance.

## Parameters

### providerName

`string`

Name of the provider

### apiKey

`string`

API key for the provider

## Returns

`Promise`\<[`ModelFactory`](/api/control/type-aliases/modelfactory/)\>

A function that creates a model instance
