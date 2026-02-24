---
editUrl: false
next: false
prev: false
title: "SandboxMocker"
---

Defined in: [sandbox.ts:120](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L120)

Sandbox execution mocking utilities class.

Provides methods for mocking Docker container execution during testing.

## Example

```typescript
import { SandboxMocker } from 'vitest-agentic-control';

const mocker = new SandboxMocker();

// Mock successful execution
mocker.mockExecution({
  success: true,
  stdout: 'Hello from container!',
  exitCode: 0,
});

// Create a mock container
const container = mocker.createMockContainer({
  image: 'node:22',
  workdir: '/app',
});

// Execute command in mock container
const result = await container.exec(['npm', 'test']);
expect(result.success).toBe(true);
```

## Constructors

### Constructor

> **new SandboxMocker**(`options?`): `SandboxMocker`

Defined in: [sandbox.ts:139](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L139)

#### Parameters

##### options?

[`SandboxMockerOptions`](/api/vitest/interfaces/sandboxmockeroptions/) = `{}`

#### Returns

`SandboxMocker`

## Properties

### containers

> `readonly` **containers**: `Map`\<`string`, `MockContainer`\>

Defined in: [sandbox.ts:128](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L128)

Track mock containers

## Methods

### createMockContainer()

> **createMockContainer**(`config?`): `MockContainer`

Defined in: [sandbox.ts:197](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L197)

Create a mock container.

#### Parameters

##### config?

[`MockContainerConfig`](/api/vitest/interfaces/mockcontainerconfig/) = `{}`

Container configuration

#### Returns

`MockContainer`

Mock container instance

***

### createMockRuntime()

> **createMockRuntime**(`name`, `command?`): `object`

Defined in: [sandbox.ts:423](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L423)

Create a mock runtime adapter.

#### Parameters

##### name

`string`

Runtime name

##### command?

`string`[] = `...`

Command to return from prepareCommand

#### Returns

`object`

##### name

> **name**: `string`

##### parseOutput

> **parseOutput**: `Mock`\<(`output`) => `unknown`\>

##### prepareCommand

> **prepareCommand**: `Mock`\<(`prompt`, `options?`) => `string`[]\>

***

### getContainer()

> **getContainer**(`id`): `MockContainer` \| `undefined`

Defined in: [sandbox.ts:448](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L448)

Get a container by ID.

#### Parameters

##### id

`string`

#### Returns

`MockContainer` \| `undefined`

***

### getContainerIds()

> **getContainerIds**(): `string`[]

Defined in: [sandbox.ts:441](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L441)

Get all container IDs.

#### Returns

`string`[]

***

### mockContainerManager()

> **mockContainerManager**(): `Mock`

Defined in: [sandbox.ts:356](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L356)

Mock the ContainerManager class from agentic-control.

#### Returns

`Mock`

***

### mockDockerCommands()

> **mockDockerCommands**(): `void`

Defined in: [sandbox.ts:255](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L255)

Mock Docker CLI commands.

#### Returns

`void`

***

### mockExecution()

> **mockExecution**(`result`): `void`

Defined in: [sandbox.ts:162](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L162)

Set the default execution result.

#### Parameters

##### result

[`MockExecutionResult`](/api/vitest/interfaces/mockexecutionresult/)

The result to return from executions

#### Returns

`void`

***

### mockSandboxExecutor()

> **mockSandboxExecutor**(): `Mock`

Defined in: [sandbox.ts:399](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L399)

Mock the SandboxExecutor class from agentic-control.

#### Returns

`Mock`

***

### queueResult()

> **queueResult**(`result`): `void`

Defined in: [sandbox.ts:171](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L171)

Queue a result to be returned from the next execution.

#### Parameters

##### result

[`MockExecutionResult`](/api/vitest/interfaces/mockexecutionresult/)

The result to queue

#### Returns

`void`

***

### queueResults()

> **queueResults**(`results`): `void`

Defined in: [sandbox.ts:180](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L180)

Queue multiple results to be returned from executions.

#### Parameters

##### results

[`MockExecutionResult`](/api/vitest/interfaces/mockexecutionresult/)[]

The results to queue

#### Returns

`void`

***

### resetAll()

> **resetAll**(): `void`

Defined in: [sandbox.ts:464](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L464)

Reset all mocks.

#### Returns

`void`

***

### restoreAll()

> **restoreAll**(): `void`

Defined in: [sandbox.ts:455](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/sandbox.ts#L455)

Restore all mocked modules.

#### Returns

`void`
