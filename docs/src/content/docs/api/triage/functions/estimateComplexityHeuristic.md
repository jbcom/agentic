---
editUrl: false
next: false
prev: false
title: "estimateComplexityHeuristic"
---

> **estimateComplexityHeuristic**(`options`, `config?`): [`ComplexityScore`](/api/triage/interfaces/complexityscore/)

Defined in: [packages/triage/src/scoring/evaluator.ts:236](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L236)

Quick complexity estimation without AI (heuristic-based)
Useful when LLM is unavailable or for fast pre-filtering

## Parameters

### options

#### filesChanged?

`number`

#### hasDependencyChanges?

`boolean`

#### hasTests?

`boolean`

#### isCriticalPath?

`boolean`

#### isRefactor?

`boolean`

#### linesChanged?

`number`

### config?

[`EvaluatorConfig`](/api/triage/interfaces/evaluatorconfig/) = `{}`

## Returns

[`ComplexityScore`](/api/triage/interfaces/complexityscore/)
