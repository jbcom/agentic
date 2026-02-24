---
editUrl: false
next: false
prev: false
title: "AgentDefinition"
---

Defined in: [packages/triage/src/scoring/agents.ts:84](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L84)

Complete agent definition

## Type Parameters

### T

`T` = `unknown`

## Properties

### capabilities

> **capabilities**: [`AgentCapabilities`](/api/triage/interfaces/agentcapabilities/)

Defined in: [packages/triage/src/scoring/agents.ts:100](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L100)

What this agent can do

***

### cost

> **cost**: `number`

Defined in: [packages/triage/src/scoring/agents.ts:93](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L93)

Cost per invocation in your chosen units
Could be cents, tokens, or relative units (0=free, 100=expensive)

***

### enabled

> **enabled**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:102](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L102)

Whether this agent is currently enabled

***

### execute

> **execute**: [`AgentExecutor`](/api/triage/type-aliases/agentexecutor/)\<`T`\>

Defined in: [packages/triage/src/scoring/agents.ts:109](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L109)

The executor function - implement this for your provider

***

### id

> **id**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:86](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L86)

Unique identifier (e.g., 'ollama-qwen', 'jules', 'openai-gpt4')

***

### name

> **name**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:88](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L88)

Human-readable name

***

### priority

> **priority**: `number`

Defined in: [packages/triage/src/scoring/agents.ts:98](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L98)

Priority when multiple agents can handle same tier
Lower = preferred (will be tried first)

***

### requiresApproval?

> `optional` **requiresApproval**: `boolean`

Defined in: [packages/triage/src/scoring/agents.ts:107](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L107)

Require explicit approval before using?
Useful for expensive agents - task.metadata.approved must include this agent's ID
