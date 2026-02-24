---
editUrl: false
next: false
prev: false
title: "AgentFactory"
---

> **AgentFactory**\<`TConfig`, `TResult`\> = (`id`, `config`, `options?`) => [`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`TResult`\>

Defined in: [packages/triage/src/scoring/agents.ts:276](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L276)

Factory function type for creating agents
Providers implement this to create configured agents

## Type Parameters

### TConfig

`TConfig`

### TResult

`TResult` = `unknown`

## Parameters

### id

`string`

### config

`TConfig`

### options?

`Partial`\<[`AgentConfig`](/api/triage/type-aliases/agentconfig/)\>

## Returns

[`AgentDefinition`](/api/triage/interfaces/agentdefinition/)\<`TResult`\>
