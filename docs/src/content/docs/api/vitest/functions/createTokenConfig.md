---
editUrl: false
next: false
prev: false
title: "createTokenConfig"
---

> **createTokenConfig**(`overrides?`): [`TestTokenConfig`](/api/vitest/interfaces/testtokenconfig/)

Defined in: [fixtures.ts:143](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L143)

Create a token configuration for testing.

## Parameters

### overrides?

`Partial`\<[`TestTokenConfig`](/api/vitest/interfaces/testtokenconfig/)\> = `{}`

Optional overrides

## Returns

[`TestTokenConfig`](/api/vitest/interfaces/testtokenconfig/)

Token configuration

## Example

```typescript
import { createTokenConfig } from 'vitest-agentic-control';

const tokens = createTokenConfig({
  organizations: {
    'my-org': { name: 'my-org', tokenEnvVar: 'MY_ORG_TOKEN' },
  },
});
```
