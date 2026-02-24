---
editUrl: false
next: false
prev: false
title: "createContext7Client"
---

> **createContext7Client**(): `Promise`\<`MCPClient`\>

Defined in: [packages/triage/src/mcp.ts:359](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L359)

Create Context7 MCP client for library documentation lookup

CRITICAL for preventing hallucinations!
Provides access to up-to-date library documentation so the AI can
reference-check instead of making up API details.

Tools provided:
- resolve-library-id: Find Context7 library ID from name
- get-library-docs: Get documentation for a library

Uses HTTP transport to Context7's cloud service.
Requires CONTEXT7_API_KEY environment variable.

## Returns

`Promise`\<`MCPClient`\>
