---
editUrl: false
next: false
prev: false
title: "generateEvaluationPrompt"
---

> **generateEvaluationPrompt**(`task`, `context`, `maxContext?`): `string`

Defined in: [packages/triage/src/scoring/evaluator.ts:81](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L81)

Generate the evaluation prompt for an LLM
This prompt is provider-agnostic - works with any LLM

## Parameters

### task

`string`

### context

`string`

### maxContext?

`number` = `8000`

## Returns

`string`
