---
editUrl: false
next: false
prev: false
title: "MockProviderResponse"
---

Defined in: [providers.ts:47](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L47)

Mock provider response configuration.

## Properties

### error?

> `optional` **error**: `Error`

Defined in: [providers.ts:60](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L60)

Error to throw instead of returning response

***

### latency?

> `optional` **latency**: `number`

Defined in: [providers.ts:58](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L58)

Simulated latency in ms

***

### response?

> `optional` **response**: `string`

Defined in: [providers.ts:49](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L49)

The text response to return

***

### stream?

> `optional` **stream**: `boolean`

Defined in: [providers.ts:56](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L56)

Whether the response should stream

***

### toolCalls?

> `optional` **toolCalls**: `object`[]

Defined in: [providers.ts:51](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L51)

Tool calls to include in response

#### args

> **args**: `Record`\<`string`, `unknown`\>

#### name

> **name**: `string`

***

### usage?

> `optional` **usage**: `object`

Defined in: [providers.ts:62](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L62)

Usage information

#### completionTokens

> **completionTokens**: `number`

#### promptTokens

> **promptTokens**: `number`

#### totalTokens

> **totalTokens**: `number`
