---
editUrl: false
next: false
prev: false
title: "createSandboxConfig"
---

> **createSandboxConfig**(`overrides?`): [`TestSandboxConfig`](/api/vitest/interfaces/testsandboxconfig/)

Defined in: [fixtures.ts:225](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/tooling/vitest-agentic/src/fixtures.ts#L225)

Create a sandbox configuration for testing.

## Parameters

### overrides?

`Partial`\<[`TestSandboxConfig`](/api/vitest/interfaces/testsandboxconfig/)\> = `{}`

Optional overrides

## Returns

[`TestSandboxConfig`](/api/vitest/interfaces/testsandboxconfig/)

Sandbox configuration

## Example

```typescript
import { createSandboxConfig } from '/vitest-agentic';

const sandbox = createSandboxConfig({
  runtime: 'cursor',
  memory: 1024,
});
```
