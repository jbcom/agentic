---
editUrl: false
next: false
prev: false
title: "EscalationConfig"
---

Defined in: [packages/triage/src/escalation/config.ts:11](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L11)

Configuration for the escalation ladder

## Properties

### cloudAgentApprovalRequired

> **cloudAgentApprovalRequired**: `boolean`

Defined in: [packages/triage/src/escalation/config.ts:25](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L25)

Whether cloud agents require explicit approval via label (Default: true)

***

### cloudAgentEnabled

> **cloudAgentEnabled**: `boolean`

Defined in: [packages/triage/src/escalation/config.ts:22](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L22)

Whether cloud agents (e.g., Cursor) are enabled (Default: false)

***

### costBudgetDaily

> **costBudgetDaily**: `number`

Defined in: [packages/triage/src/escalation/config.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L28)

Daily cost budget for cloud agents in cents (Default: 0 = no cloud)

***

### maxJulesAttempts

> **maxJulesAttempts**: `number`

Defined in: [packages/triage/src/escalation/config.ts:16](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L16)

Maximum number of initial Jules attempts before escalating (Default: 3)

***

### maxJulesBoostAttempts

> **maxJulesBoostAttempts**: `number`

Defined in: [packages/triage/src/escalation/config.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L19)

Maximum number of Jules attempts with boosted context (Default: 3)

***

### maxOllamaAttempts

> **maxOllamaAttempts**: `number`

Defined in: [packages/triage/src/escalation/config.ts:13](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/config.ts#L13)

Maximum number of Ollama fix attempts before escalating (Default: 2)
