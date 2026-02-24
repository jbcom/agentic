---
editUrl: false
next: false
prev: false
title: "parseEvaluationResponse"
---

> **parseEvaluationResponse**(`response`, `weights?`): `object`

Defined in: [packages/triage/src/scoring/evaluator.ts:137](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L137)

Parse and validate LLM response into dimension scores

## Parameters

### response

`string`

### weights?

[`ComplexityWeights`](/api/triage/interfaces/complexityweights/) = `DEFAULT_WEIGHTS`

## Returns

`object`

### reasoning

> **reasoning**: `string`

### scores

> **scores**: [`DimensionScores`](/api/triage/interfaces/dimensionscores/)
