---
editUrl: false
next: false
prev: false
title: "AgenticMocker"
---

Defined in: [mocking.ts:59](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L59)

Main mocker class for comprehensive agentic-control testing.

This class provides a unified interface for mocking all agentic-control
components: MCP servers, AI providers, sandbox execution, and more.

## Example

```typescript
import { AgenticMocker } from 'vitest-agentic-control';

const mocker = new AgenticMocker();

// Mock MCP server
mocker.mcp.mockServer('my-server', {
  tools: [{ name: 'tool1', handler: () => 'result' }],
});

// Mock AI provider
mocker.providers.mockAnthropic({ response: 'Hello!' });

// Mock sandbox execution
mocker.sandbox.mockExecution({ success: true, output: 'Done!' });

// Clean up after test
mocker.restoreAll();
```

## Constructors

### Constructor

> **new AgenticMocker**(`options?`): `AgenticMocker`

Defined in: [mocking.ts:77](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L77)

Creates a new AgenticMocker instance.

#### Parameters

##### options?

`AgenticMockerOptions` = `{}`

Configuration options for the mocker

#### Returns

`AgenticMocker`

## Properties

### mcp

> `readonly` **mcp**: [`McpMocker`](/api/vitest/classes/mcpmocker/)

Defined in: [mocking.ts:61](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L61)

MCP mocking utilities

***

### providers

> `readonly` **providers**: [`ProviderMocker`](/api/vitest/classes/providermocker/)

Defined in: [mocking.ts:63](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L63)

AI provider mocking utilities

***

### sandbox

> `readonly` **sandbox**: [`SandboxMocker`](/api/vitest/classes/sandboxmocker/)

Defined in: [mocking.ts:65](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L65)

Sandbox execution mocking utilities

## Methods

### createSpy()

> **createSpy**\<`T`\>(`implementation?`): `Mock`\<`T`\>

Defined in: [mocking.ts:102](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L102)

Create a spy on a function.

#### Type Parameters

##### T

`T` *extends* (...`args`) => `unknown`

#### Parameters

##### implementation?

`T`

Optional implementation for the spy

#### Returns

`Mock`\<`T`\>

The mock function

***

### mockAllFrameworks()

> **mockAllFrameworks**(): `Record`\<`string`, `unknown`\>

Defined in: [mocking.ts:116](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L116)

Mock all agentic-control framework modules.

This mocks the common modules used in agentic-control:
- MCP SDK modules
- AI SDK modules
- GitHub client modules

#### Returns

`Record`\<`string`, `unknown`\>

Dictionary of all mocked modules

***

### mockEnv()

> **mockEnv**(`env`): () => `void`

Defined in: [mocking.ts:167](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L167)

Mock environment variables temporarily.

#### Parameters

##### env

`Record`\<`string`, `string`\>

Environment variables to set

#### Returns

Cleanup function to restore original values

> (): `void`

##### Returns

`void`

***

### mockGitHubClient()

> **mockGitHubClient**(`options?`): `unknown`

Defined in: [mocking.ts:136](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L136)

Mock the GitHub client.

#### Parameters

##### options?

Options for the mock

###### issues?

`unknown`[]

###### pullRequests?

`unknown`[]

###### repositories?

`unknown`[]

#### Returns

`unknown`

The mock GitHub client

***

### mockModule()

> **mockModule**\<`T`\>(`modulePath`, `mockValue`): `T`

Defined in: [mocking.ts:90](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L90)

Mock a module by path.

#### Type Parameters

##### T

`T` *extends* `Record`\<`string`, `unknown`\>

#### Parameters

##### modulePath

`string`

The module path to mock

##### mockValue

`T`

The mock value to use

#### Returns

`T`

The mock value for chaining

***

### resetAll()

> **resetAll**(): `void`

Defined in: [mocking.ts:207](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L207)

Reset all mocks without restoring.

#### Returns

`void`

***

### restoreAll()

> **restoreAll**(): `void`

Defined in: [mocking.ts:189](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L189)

Restore all mocked modules to their original values.

#### Returns

`void`
