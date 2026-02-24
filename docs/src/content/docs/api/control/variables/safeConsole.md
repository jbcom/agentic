---
editUrl: false
next: false
prev: false
title: "safeConsole"
---

> `const` **safeConsole**: `object`

Defined in: [packages/agentic-control/src/core/security.ts:77](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/security.ts#L77)

Safe console logging that sanitizes output

## Type Declaration

### error()

> **error**: (`message`, ...`args`) => `void`

#### Parameters

##### message

`string`

##### args

...`unknown`[]

#### Returns

`void`

### log()

> **log**: (`message`, ...`args`) => `void`

#### Parameters

##### message

`string`

##### args

...`unknown`[]

#### Returns

`void`

### warn()

> **warn**: (`message`, ...`args`) => `void`

#### Parameters

##### message

`string`

##### args

...`unknown`[]

#### Returns

`void`
