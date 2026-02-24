---
editUrl: false
next: false
prev: false
title: "createTestConfig"
---

> **createTestConfig**(`options?`): [`TestConfig`](/api/vitest/interfaces/testconfig/)

Defined in: [fixtures.ts:102](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L102)

Create a test configuration with sensible defaults.

## Parameters

### options?

[`TestConfigOptions`](/api/vitest/interfaces/testconfigoptions/) = `{}`

Configuration options

## Returns

[`TestConfig`](/api/vitest/interfaces/testconfig/)

A test configuration object

## Example

```typescript
import { createTestConfig } from 'vitest-agentic-control';

const config = createTestConfig({
  logLevel: 'debug',
  tokens: true,
  fleet: true,
});

// Use in your tests
expect(config.tokens?.organizations).toBeDefined();
```
