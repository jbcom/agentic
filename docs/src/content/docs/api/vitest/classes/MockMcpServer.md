---
editUrl: false
next: false
prev: false
title: "MockMcpServer"
---

Defined in: [mcp.ts:104](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L104)

Mock MCP server instance.

## Constructors

### Constructor

> **new MockMcpServer**(`config`): `MockMcpServer`

Defined in: [mcp.ts:116](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L116)

#### Parameters

##### config

`MockMcpServerConfig`

#### Returns

`MockMcpServer`

## Properties

### capabilities

> `readonly` **capabilities**: `object`

Defined in: [mcp.ts:108](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L108)

#### prompts

> **prompts**: `boolean`

#### resources

> **resources**: `boolean`

#### tools

> **tools**: `boolean`

***

### name

> `readonly` **name**: `string`

Defined in: [mcp.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L105)

***

### resources

> `readonly` **resources**: `Map`\<`string`, [`MockMcpResource`](/api/vitest/interfaces/mockmcpresource/)\>

Defined in: [mcp.ts:107](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L107)

***

### tools

> `readonly` **tools**: `Map`\<`string`, [`MockMcpTool`](/api/vitest/interfaces/mockmcptool/)\>

Defined in: [mcp.ts:106](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L106)

## Methods

### callTool()

> **callTool**(`name`, `args`): `Promise`\<`unknown`\>

Defined in: [mcp.ts:201](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L201)

Call a tool.

#### Parameters

##### name

`string`

##### args

`unknown`

#### Returns

`Promise`\<`unknown`\>

***

### connect()

> **connect**(): `Promise`\<`void`\>

Defined in: [mcp.ts:163](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L163)

Connect to this mock server.

#### Returns

`Promise`\<`void`\>

***

### disconnect()

> **disconnect**(): `Promise`\<`void`\>

Defined in: [mcp.ts:170](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L170)

Disconnect from this mock server.

#### Returns

`Promise`\<`void`\>

***

### isConnected()

> **isConnected**(): `boolean`

Defined in: [mcp.ts:177](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L177)

Check if connected.

#### Returns

`boolean`

***

### listResources()

> **listResources**(): `Promise`\<`object`[]\>

Defined in: [mcp.ts:213](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L213)

List available resources.

#### Returns

`Promise`\<`object`[]\>

***

### listTools()

> **listTools**(): `Promise`\<`object`[]\>

Defined in: [mcp.ts:184](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L184)

List available tools.

#### Returns

`Promise`\<`object`[]\>

***

### readResource()

> **readResource**(`uri`): `Promise`\<\{ `content`: `string` \| `Buffer`\<`ArrayBufferLike`\>; `mimeType?`: `string`; `uri`: `string`; \}\>

Defined in: [mcp.ts:232](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L232)

Read a resource.

#### Parameters

##### uri

`string`

#### Returns

`Promise`\<\{ `content`: `string` \| `Buffer`\<`ArrayBufferLike`\>; `mimeType?`: `string`; `uri`: `string`; \}\>

***

### registerResource()

> **registerResource**(`definition`): [`MockMcpResource`](/api/vitest/interfaces/mockmcpresource/)

Defined in: [mcp.ts:151](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L151)

Register a resource with this mock server.

#### Parameters

##### definition

[`MockResourceDefinition`](/api/vitest/interfaces/mockresourcedefinition/)

#### Returns

[`MockMcpResource`](/api/vitest/interfaces/mockmcpresource/)

***

### registerTool()

> **registerTool**(`definition`): [`MockMcpTool`](/api/vitest/interfaces/mockmcptool/)

Defined in: [mcp.ts:138](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L138)

Register a tool with this mock server.

#### Parameters

##### definition

[`MockToolDefinition`](/api/vitest/interfaces/mocktooldefinition/)

#### Returns

[`MockMcpTool`](/api/vitest/interfaces/mockmcptool/)

***

### reset()

> **reset**(): `void`

Defined in: [mcp.ts:252](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L252)

Reset all call history.

#### Returns

`void`
