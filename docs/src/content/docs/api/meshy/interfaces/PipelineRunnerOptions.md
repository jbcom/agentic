---
editUrl: false
next: false
prev: false
title: "PipelineRunnerOptions"
---

Defined in: [core/runner.ts:21](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L21)

## Properties

### apiKey

> **apiKey**: `string`

Defined in: [core/runner.ts:25](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L25)

Meshy API key used to authenticate requests.

***

### client?

> `optional` **client**: `MeshyClient`

Defined in: [core/runner.ts:27](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L27)

Optional Meshy client override (useful for testing).

***

### definitions

> **definitions**: [`DefinitionBundle`](/api/meshy/interfaces/definitionbundle/)

Defined in: [core/runner.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L23)

Loaded pipeline/task definitions.

***

### logger()?

> `optional` **logger**: (`message`) => `void`

Defined in: [core/runner.ts:31](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L31)

Optional logger to capture pipeline progress.

#### Parameters

##### message

`string`

#### Returns

`void`

***

### lookups?

> `optional` **lookups**: `Record`\<`string`, `Record`\<`string`, `unknown`\>\>

Defined in: [core/runner.ts:29](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L29)

Lookup tables for binding inputs by key.
