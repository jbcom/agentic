---
editUrl: false
next: false
prev: false
title: "withTestEnv"
---

> **withTestEnv**(`env?`): () => `void`

Defined in: [fixtures.ts:295](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L295)

Set up test environment variables and return cleanup function.

## Parameters

### env?

[`TestEnvSetup`](/api/vitest/interfaces/testenvsetup/) = `DEFAULT_TEST_ENV`

Environment variables to set

## Returns

Cleanup function to restore original values

> (): `void`

### Returns

`void`

## Example

```typescript
import { withTestEnv, DEFAULT_TEST_ENV } from 'vitest-agentic-control';
import { beforeEach, afterEach } from 'vitest';

describe('My Tests', () => {
  let cleanup: () => void;

  beforeEach(() => {
    cleanup = withTestEnv(DEFAULT_TEST_ENV);
  });

  afterEach(() => {
    cleanup();
  });

  it('should use test tokens', () => {
    expect(process.env.GITHUB_TOKEN).toBe('ghp_test_token_12345');
  });
});
```
