---
editUrl: false
next: false
prev: false
title: "AgenticConfig"
---

Defined in: [packages/agentic-control/src/core/config.ts:88](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L88)

Core module for agentic-control

Exports types, token management, configuration, and AI providers

## Properties

### coordinationPr?

> `optional` **coordinationPr**: `number`

Defined in: [packages/agentic-control/src/core/config.ts:96](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L96)

Coordination PR number for fleet communication

***

### cursor?

> `optional` **cursor**: `object`

Defined in: [packages/agentic-control/src/core/config.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L105)

Cursor API configuration

#### apiKeyEnvVar?

> `optional` **apiKeyEnvVar**: `string`

API key environment variable name

#### baseUrl?

> `optional` **baseUrl**: `string`

Base URL for Cursor API

***

### defaultRepository?

> `optional` **defaultRepository**: `string`

Defined in: [packages/agentic-control/src/core/config.ts:93](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L93)

Default repository for fleet operations

***

### fleet?

> `optional` **fleet**: `FleetConfig`

Defined in: [packages/agentic-control/src/core/config.ts:113](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L113)

Fleet default options

***

### logLevel?

> `optional` **logLevel**: `"info"` \| `"debug"` \| `"warn"` \| `"error"`

Defined in: [packages/agentic-control/src/core/config.ts:99](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L99)

Log level

***

### mcp?

> `optional` **mcp**: `MCPConfig`

Defined in: [packages/agentic-control/src/core/config.ts:119](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L119)

MCP server configuration

***

### tokens?

> `optional` **tokens**: `Partial`\<[`TokenConfig`](/api/control/interfaces/tokenconfig/)\>

Defined in: [packages/agentic-control/src/core/config.ts:90](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L90)

Token configuration for multi-org access

***

### triage?

> `optional` **triage**: [`TriageConfig`](/api/control/interfaces/triageconfig/)

Defined in: [packages/agentic-control/src/core/config.ts:116](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L116)

Triage (AI analysis) configuration

***

### verbose?

> `optional` **verbose**: `boolean`

Defined in: [packages/agentic-control/src/core/config.ts:102](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/config.ts#L102)

Whether to enable verbose output
