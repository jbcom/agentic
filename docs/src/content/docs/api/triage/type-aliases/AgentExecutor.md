---
editUrl: false
next: false
prev: false
title: "AgentExecutor"
---

> **AgentExecutor**\<`T`\> = (`task`) => `Promise`\<[`AgentResult`](/api/triage/interfaces/agentresult/)\<`T`\>\>

Defined in: [packages/triage/src/scoring/agents.ts:79](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L79)

Function signature for agent execution
Implement this interface to add any LLM/agent provider

## Type Parameters

### T

`T` = `unknown`

## Parameters

### task

[`AgentTask`](/api/triage/interfaces/agenttask/)

## Returns

`Promise`\<[`AgentResult`](/api/triage/interfaces/agentresult/)\<`T`\>\>
