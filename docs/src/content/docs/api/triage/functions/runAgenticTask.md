---
editUrl: false
next: false
prev: false
title: "runAgenticTask"
---

> **runAgenticTask**(`options`): `Promise`\<[`AgenticTaskResult`](/api/triage/interfaces/agentictaskresult/)\>

Defined in: [packages/triage/src/mcp.ts:691](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L691)

Run an agentic task with MCP tools

This is the main entry point for AI tasks that need tool access.
Properly handles the agentic loop via AI SDK's stopWhen.

## Parameters

### options

[`AgenticTaskOptions`](/api/triage/interfaces/agentictaskoptions/)

## Returns

`Promise`\<[`AgenticTaskResult`](/api/triage/interfaces/agentictaskresult/)\>

## Example

```ts
const result = await runAgenticTask({
    systemPrompt: 'You are a code reviewer...',
    userPrompt: 'Review this PR diff...',
    mcpClients: {
        filesystem: process.cwd(),  // Read files for context
        context7: true,              // Check documentation - prevents hallucinations
    },
    maxSteps: 15,
    onToolCall: (name, args) => console.log(`Using ${name}`),
});
```
