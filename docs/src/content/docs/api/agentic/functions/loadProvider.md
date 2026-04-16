---
editUrl: false
next: false
prev: false
title: "loadProvider"
---

> **loadProvider**(`providerName`, `apiKey`): `Promise`\<[`ModelFactory`](/api/agentic/type-aliases/modelfactory/)\>

Defined in: [packages/agentic/src/core/providers.ts:97](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic/src/core/providers.ts#L97)

Load an AI provider dynamically.

## Parameters

### providerName

`string`

Name of the provider (anthropic, openai, etc.)

### apiKey

`string`

API key for the provider

## Returns

`Promise`\<[`ModelFactory`](/api/agentic/type-aliases/modelfactory/)\>

A function that creates a model instance

## Throws

Error if provider is unknown or package not installed
