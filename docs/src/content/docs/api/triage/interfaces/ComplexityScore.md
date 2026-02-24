---
editUrl: false
next: false
prev: false
title: "ComplexityScore"
---

Defined in: [packages/triage/src/scoring/evaluator.ts:44](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L44)

Complete complexity score result

## Properties

### raw

> **raw**: [`DimensionScores`](/api/triage/interfaces/dimensionscores/)

Defined in: [packages/triage/src/scoring/evaluator.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L46)

Raw scores for each dimension (0-10)

***

### reasoning

> **reasoning**: `string`

Defined in: [packages/triage/src/scoring/evaluator.ts:52](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L52)

AI's reasoning for the scores

***

### tier

> **tier**: [`ComplexityTier`](/api/triage/type-aliases/complexitytier/)

Defined in: [packages/triage/src/scoring/evaluator.ts:50](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L50)

Complexity tier based on thresholds

***

### weighted

> **weighted**: `number`

Defined in: [packages/triage/src/scoring/evaluator.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L48)

Weighted composite score (0-10)
