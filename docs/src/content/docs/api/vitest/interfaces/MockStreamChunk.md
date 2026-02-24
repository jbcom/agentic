---
editUrl: false
next: false
prev: false
title: "MockStreamChunk"
---

Defined in: [providers.ts:72](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L72)

Mock stream chunk for streaming responses.

## Properties

### isFinished?

> `optional` **isFinished**: `boolean`

Defined in: [providers.ts:86](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L86)

Whether this is the final chunk

***

### textDelta?

> `optional` **textDelta**: `string`

Defined in: [providers.ts:74](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L74)

Text delta

***

### toolCallDelta?

> `optional` **toolCallDelta**: `object`

Defined in: [providers.ts:81](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L81)

Tool call delta (args)

#### args

> **args**: `string`

#### id

> **id**: `string`

***

### toolCallStart?

> `optional` **toolCallStart**: `object`

Defined in: [providers.ts:76](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/providers.ts#L76)

Tool call start

#### id

> **id**: `string`

#### name

> **name**: `string`
