---
editUrl: false
next: false
prev: false
title: "ContainerManager"
---

Defined in: [packages/agentic-control/src/sandbox/container.ts:9](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L9)

## Constructors

### Constructor

> **new ContainerManager**(): `ContainerManager`

#### Returns

`ContainerManager`

## Methods

### create()

> **create**(`config`): `Promise`\<`string`\>

Defined in: [packages/agentic-control/src/sandbox/container.ts:10](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L10)

#### Parameters

##### config

[`ContainerConfig`](/api/control/interfaces/containerconfig/)

#### Returns

`Promise`\<`string`\>

***

### exec()

> **exec**(`containerId`, `command`): `Promise`\<[`ContainerResult`](/api/control/interfaces/containerresult/)\>

Defined in: [packages/agentic-control/src/sandbox/container.ts:73](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L73)

#### Parameters

##### containerId

`string`

##### command

`string`[]

#### Returns

`Promise`\<[`ContainerResult`](/api/control/interfaces/containerresult/)\>

***

### logs()

> **logs**(`containerId`): `Promise`\<`string`\>

Defined in: [packages/agentic-control/src/sandbox/container.ts:100](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L100)

#### Parameters

##### containerId

`string`

#### Returns

`Promise`\<`string`\>

***

### remove()

> **remove**(`containerId`): `Promise`\<`void`\>

Defined in: [packages/agentic-control/src/sandbox/container.ts:65](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L65)

#### Parameters

##### containerId

`string`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`containerId`): `Promise`\<`void`\>

Defined in: [packages/agentic-control/src/sandbox/container.ts:50](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L50)

#### Parameters

##### containerId

`string`

#### Returns

`Promise`\<`void`\>

***

### stop()

> **stop**(`containerId`): `Promise`\<`void`\>

Defined in: [packages/agentic-control/src/sandbox/container.ts:57](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/sandbox/container.ts#L57)

#### Parameters

##### containerId

`string`

#### Returns

`Promise`\<`void`\>
