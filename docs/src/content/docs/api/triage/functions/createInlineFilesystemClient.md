---
editUrl: false
next: false
prev: false
title: "createInlineFilesystemClient"
---

> **createInlineFilesystemClient**(`workingDirectory`): `Promise`\<`MCPClient`\>

Defined in: [packages/triage/src/mcp.ts:58](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L58)

Inline filesystem MCP server

Provides: read_file, write_file, list_files, search_files
All paths are sandboxed to workingDirectory for security.

## Parameters

### workingDirectory

`string`

## Returns

`Promise`\<`MCPClient`\>
