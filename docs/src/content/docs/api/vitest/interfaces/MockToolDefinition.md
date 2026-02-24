---
editUrl: false
next: false
prev: false
title: "MockToolDefinition"
---

Defined in: [mcp.ts:26](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L26)

Definition for a mock MCP tool.

## Properties

### description?

> `optional` **description**: `string`

Defined in: [mcp.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L30)

Tool description

***

### handler()

> **handler**: (`args`) => `unknown`

Defined in: [mcp.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L34)

Handler function for the tool

#### Parameters

##### args

`unknown`

#### Returns

`unknown`

***

### inputSchema?

> `optional` **inputSchema**: `Record`\<`string`, `unknown`\>

Defined in: [mcp.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L32)

Input schema (JSON Schema)

***

### name

> **name**: `string`

Defined in: [mcp.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mcp.ts#L28)

Tool name
