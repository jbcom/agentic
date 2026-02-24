---
editUrl: false
next: false
prev: false
title: "MCPClientOptions"
---

Defined in: [packages/triage/src/mcp.ts:487](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L487)

## Properties

### context7?

> `optional` **context7**: `boolean`

Defined in: [packages/triage/src/mcp.ts:503](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L503)

Enable Context7 documentation lookup (PREVENTS HALLUCINATIONS!)

***

### filesystem?

> `optional` **filesystem**: `string` \| `boolean`

Defined in: [packages/triage/src/mcp.ts:489](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L489)

Enable filesystem access (required for most tasks)

***

### github?

> `optional` **github**: `boolean`

Defined in: [packages/triage/src/mcp.ts:492](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L492)

Enable GitHub API access

***

### graphql?

> `optional` **graphql**: `boolean`

Defined in: [packages/triage/src/mcp.ts:514](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L514)

Enable GitHub GraphQL API access via mcp-graphql

***

### playwright?

> `optional` **playwright**: `boolean` \| \{ `browser?`: `"chromium"` \| `"firefox"` \| `"webkit"`; `headless?`: `boolean`; \}

Defined in: [packages/triage/src/mcp.ts:495](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L495)

Enable Playwright browser automation

***

### viteReact?

> `optional` **viteReact**: `boolean` \| \{ `port?`: `number`; `url?`: `string`; \}

Defined in: [packages/triage/src/mcp.ts:506](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L506)

Enable Vite React component debugging
