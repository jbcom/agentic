---
editUrl: false
next: false
prev: false
title: "createFilesystemClient"
---

> **createFilesystemClient**(`workingDirectory`): `Promise`\<`MCPClient`\>

Defined in: [packages/triage/src/mcp.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L48)

Create Filesystem MCP client for file operations

Uses an inline server for reliability (no npx dependency).
This is the MOST IMPORTANT MCP - it lets the AI read and write files
since Ollama can't fit everything in context.

## Parameters

### workingDirectory

`string`

## Returns

`Promise`\<`MCPClient`\>
