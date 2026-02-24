---
editUrl: false
next: false
prev: false
title: "evaluateComplexity"
---

> **evaluateComplexity**(`llm`, `task`, `context`, `config?`): `Promise`\<[`ComplexityScore`](/api/triage/interfaces/complexityscore/)\>

Defined in: [packages/triage/src/scoring/evaluator.ts:211](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/scoring/evaluator.ts#L211)

Full evaluation using an LLM

## Parameters

### llm

[`LLMEvaluator`](/api/triage/type-aliases/llmevaluator/)

### task

`string`

### context

`string`

### config?

[`EvaluatorConfig`](/api/triage/interfaces/evaluatorconfig/) = `{}`

## Returns

`Promise`\<[`ComplexityScore`](/api/triage/interfaces/complexityscore/)\>

## Example

```typescript
// With Ollama
const evaluate = async (prompt: string) => {
  const res = await fetch('http://localhost:11434/api/generate', {
    method: 'POST',
    body: JSON.stringify({ model: 'qwen2.5-coder', prompt, stream: false })
  });
  return (await res.json()).response;
};

const score = await evaluateComplexity(evaluate, 'Fix the bug', codeDiff);
console.log(score.tier); // 'simple'
console.log(score.weighted); // 3.5
```
