---
editUrl: false
next: false
prev: false
title: "LevelHandler"
---

> **LevelHandler** = (`task`, `state`) => `Promise`\<\{ `cost?`: `number`; `data?`: `unknown`; `error?`: `string`; `escalate`: `boolean`; `success`: `boolean`; \}\>

Defined in: [packages/triage/src/escalation/ladder.ts:59](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/escalation/ladder.ts#L59)

Handler function for a specific escalation level

## Parameters

### task

[`Task`](/api/triage/interfaces/task/)

### state

[`EscalationState`](/api/triage/interfaces/escalationstate/)

## Returns

`Promise`\<\{ `cost?`: `number`; `data?`: `unknown`; `error?`: `string`; `escalate`: `boolean`; `success`: `boolean`; \}\>
