---
editUrl: false
next: false
prev: false
title: "EvaluatorConfig"
---

Defined in: [packages/triage/src/scoring/evaluator.ts:64](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L64)

Configuration for complexity evaluation

## Properties

### maxContextLength?

> `optional` **maxContextLength**: `number`

Defined in: [packages/triage/src/scoring/evaluator.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L70)

Maximum context length to send to LLM

***

### thresholds?

> `optional` **thresholds**: [`TierThresholds`](/api/triage/interfaces/tierthresholds/)

Defined in: [packages/triage/src/scoring/evaluator.ts:68](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L68)

Custom tier thresholds (defaults to DEFAULT_THRESHOLDS)

***

### weights?

> `optional` **weights**: [`ComplexityWeights`](/api/triage/interfaces/complexityweights/)

Defined in: [packages/triage/src/scoring/evaluator.ts:66](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L66)

Custom weights (defaults to DEFAULT_WEIGHTS)
