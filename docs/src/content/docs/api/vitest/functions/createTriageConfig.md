---
editUrl: false
next: false
prev: false
title: "createTriageConfig"
---

> **createTriageConfig**(`overrides?`): [`TestTriageConfig`](/api/vitest/interfaces/testtriageconfig/)

Defined in: [fixtures.ts:200](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L200)

Create a triage configuration for testing.

## Parameters

### overrides?

`Partial`\<[`TestTriageConfig`](/api/vitest/interfaces/testtriageconfig/)\> = `{}`

Optional overrides

## Returns

[`TestTriageConfig`](/api/vitest/interfaces/testtriageconfig/)

Triage configuration

## Example

```typescript
import { createTriageConfig } from 'vitest-agentic-control';

const triage = createTriageConfig({
  provider: 'openai',
  model: 'gpt-4',
});
```
