---
editUrl: false
next: false
prev: false
title: "AgentTask"
---

Defined in: [packages/triage/src/scoring/agents.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L40)

A task to be processed by an agent

## Properties

### complexityScore

> **complexityScore**: `number`

Defined in: [packages/triage/src/scoring/agents.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L48)

Pre-computed complexity score (0-10)

***

### complexityTier

> **complexityTier**: [`ComplexityTier`](/api/triage/type-aliases/complexitytier/)

Defined in: [packages/triage/src/scoring/agents.ts:50](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L50)

Pre-computed complexity tier

***

### context

> **context**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L46)

Code/diff context

***

### description

> **description**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:44](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L44)

Task description/prompt

***

### id

> **id**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L42)

Unique task identifier

***

### metadata?

> `optional` **metadata**: `Record`\<`string`, `unknown`\>

Defined in: [packages/triage/src/scoring/agents.ts:54](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L54)

Additional metadata for provider-specific needs

***

### repo?

> `optional` **repo**: `string`

Defined in: [packages/triage/src/scoring/agents.ts:52](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/agents.ts#L52)

Repository reference (optional)
