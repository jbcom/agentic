---
editUrl: false
next: false
prev: false
title: "AgenticTaskOptions"
---

Defined in: [packages/triage/src/mcp.ts:647](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L647)

## Properties

### maxSteps?

> `optional` **maxSteps**: `number`

Defined in: [packages/triage/src/mcp.ts:655](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L655)

Maximum steps for the agentic loop (default: 15)

***

### mcpClients?

> `optional` **mcpClients**: [`MCPClientOptions`](/api/triage/interfaces/mcpclientoptions/)

Defined in: [packages/triage/src/mcp.ts:653](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L653)

MCP clients to enable

***

### onStepFinish()?

> `optional` **onStepFinish**: (`step`) => `void`

Defined in: [packages/triage/src/mcp.ts:659](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L659)

Callback for each step completion

#### Parameters

##### step

###### text?

`string`

###### toolCalls?

`unknown`[]

#### Returns

`void`

***

### onToolCall()?

> `optional` **onToolCall**: (`toolName`, `args`) => `void`

Defined in: [packages/triage/src/mcp.ts:657](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L657)

Callback for each tool call

#### Parameters

##### toolName

`string`

##### args

`unknown`

#### Returns

`void`

***

### systemPrompt

> **systemPrompt**: `string`

Defined in: [packages/triage/src/mcp.ts:649](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L649)

System prompt defining the AI's role and behavior

***

### userPrompt

> **userPrompt**: `string`

Defined in: [packages/triage/src/mcp.ts:651](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L651)

User prompt with the actual task
