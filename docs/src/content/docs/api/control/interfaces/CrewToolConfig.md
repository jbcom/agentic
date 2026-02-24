---
editUrl: false
next: false
prev: false
title: "CrewToolConfig"
---

Defined in: [packages/agentic-control/src/crews/types.ts:12](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L12)

Configuration for crew tool execution

## Properties

### defaultTimeout?

> `optional` **defaultTimeout**: `number`

Defined in: [packages/agentic-control/src/crews/types.ts:16](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L16)

Default timeout in milliseconds (default: 300000 = 5 minutes)

***

### env?

> `optional` **env**: `Record`\<`string`, `string`\>

Defined in: [packages/agentic-control/src/crews/types.ts:18](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L18)

Environment variables to pass to crew execution

***

### invokeMethod?

> `optional` **invokeMethod**: `"uv"` \| `"direct"`

Defined in: [packages/agentic-control/src/crews/types.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/crews/types.ts#L14)

How to invoke agentic-crew: 'uv' (default) or 'direct'
