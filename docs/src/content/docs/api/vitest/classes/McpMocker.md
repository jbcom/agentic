---
editUrl: false
next: false
prev: false
title: "McpMocker"
---

Defined in: [mcp.ts:297](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L297)

MCP mocking utilities class.

Provides methods for mocking MCP servers, tools, and resources
during testing.

## Example

```typescript
import { McpMocker } from 'vitest-agentic-control';

const mocker = new McpMocker();

// Create a mock MCP server
const server = mocker.mockServer('test-server', {
  tools: [
    {
      name: 'get_weather',
      description: 'Get weather for a location',
      handler: (args) => ({ temp: 72, condition: 'sunny' }),
    },
  ],
  resources: [
    {
      uri: 'file:///config.json',
      content: '{"key": "value"}',
    },
  ],
});

// Use the mock server in tests
await server.connect();
const result = await server.callTool('get_weather', { location: 'NYC' });
```

## Constructors

### Constructor

> **new McpMocker**(`options?`): `McpMocker`

Defined in: [mcp.ts:304](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L304)

#### Parameters

##### options?

[`McpMockerOptions`](/api/vitest/interfaces/mcpmockeroptions/) = `{}`

#### Returns

`McpMocker`

## Properties

### servers

> `readonly` **servers**: `Map`\<`string`, [`MockMcpServer`](/api/vitest/classes/mockmcpserver/)\>

Defined in: [mcp.ts:299](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L299)

Map of mock servers by name

## Methods

### createMockResource()

> **createMockResource**(`uri`, `content`, `options?`): [`MockResourceDefinition`](/api/vitest/interfaces/mockresourcedefinition/)

Defined in: [mcp.ts:424](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L424)

Create a mock resource that can be registered with servers.

#### Parameters

##### uri

`string`

Resource URI

##### content

Resource content

`string` | `Buffer`\<`ArrayBufferLike`\>

##### options?

Additional options

###### description?

`string`

###### mimeType?

`string`

###### name?

`string`

#### Returns

[`MockResourceDefinition`](/api/vitest/interfaces/mockresourcedefinition/)

Resource definition

***

### createMockTool()

> **createMockTool**(`name`, `handler`, `options?`): [`MockToolDefinition`](/api/vitest/interfaces/mocktooldefinition/)

Defined in: [mcp.ts:400](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L400)

Create a mock tool that can be registered with servers.

#### Parameters

##### name

`string`

Tool name

##### handler

(`args`) => `unknown`

Tool handler

##### options?

Additional options

###### description?

`string`

###### inputSchema?

`Record`\<`string`, `unknown`\>

#### Returns

[`MockToolDefinition`](/api/vitest/interfaces/mocktooldefinition/)

Tool definition

***

### getServer()

> **getServer**(`name`): [`MockMcpServer`](/api/vitest/classes/mockmcpserver/) \| `undefined`

Defined in: [mcp.ts:329](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L329)

Get a mock server by name.

#### Parameters

##### name

`string`

Server name

#### Returns

[`MockMcpServer`](/api/vitest/classes/mockmcpserver/) \| `undefined`

The mock server or undefined

***

### mockAllModules()

> **mockAllModules**(): `Record`\<`string`, `unknown`\>

Defined in: [mcp.ts:360](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L360)

Mock all MCP-related modules.

#### Returns

`Record`\<`string`, `unknown`\>

Dictionary of mocked modules

***

### mockClient()

> **mockClient**(): `Mock`

Defined in: [mcp.ts:338](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L338)

Mock the MCP client module.

#### Returns

`Mock`

Mock client factory

***

### mockServer()

> **mockServer**(`name`, `config?`): [`MockMcpServer`](/api/vitest/classes/mockmcpserver/)

Defined in: [mcp.ts:317](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L317)

Create a mock MCP server.

#### Parameters

##### name

`string`

Server name

##### config?

`Omit`\<`MockMcpServerConfig`, `"name"`\> = `{}`

Server configuration

#### Returns

[`MockMcpServer`](/api/vitest/classes/mockmcpserver/)

The mock server

***

### resetAll()

> **resetAll**(): `void`

Defined in: [mcp.ts:453](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L453)

Reset all mock servers.

#### Returns

`void`

***

### restoreAll()

> **restoreAll**(): `void`

Defined in: [mcp.ts:445](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L445)

Restore all mocked modules.

#### Returns

`void`
