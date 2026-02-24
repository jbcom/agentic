---
editUrl: false
next: false
prev: false
title: "initializeMCPClients"
---

> **initializeMCPClients**(`options`): `Promise`\<[`MCPClients`](/api/triage/interfaces/mcpclients/)\>

Defined in: [packages/triage/src/mcp.ts:527](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L527)

Initialize multiple MCP clients based on options

## Parameters

### options

[`MCPClientOptions`](/api/triage/interfaces/mcpclientoptions/)

## Returns

`Promise`\<[`MCPClients`](/api/triage/interfaces/mcpclients/)\>

## Example

```ts
const clients = await initializeMCPClients({
    filesystem: process.cwd(),  // Required for file access
    context7: true,             // Documentation lookup - prevents hallucinations
    github: true,               // Issue/PR operations
});
```
