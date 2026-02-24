---
editUrl: false
next: false
prev: false
title: "createViteReactClient"
---

> **createViteReactClient**(`options?`): `Promise`\<`MCPClient`\>

Defined in: [packages/triage/src/mcp.ts:403](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L403)

Create Vite React MCP client for React component debugging

Connects to a running Vite dev server with the vite-react-mcp plugin.
Used for debugging React apps during development and testing.

Tools provided:
- highlight-component: Highlight a React component by name
- get-component-states: Get props, states, and contexts
- get-component-tree: Get component tree in ASCII format
- get-unnecessary-rerenders: Track wasted renders

Requires the app to be running with vite-react-mcp plugin installed.

## Parameters

### options?

#### port?

`number`

Port number (alternative to full URL)

#### url?

`string`

URL of the Vite dev server SSE endpoint (default: http://localhost:5173/sse)

## Returns

`Promise`\<`MCPClient`\>
