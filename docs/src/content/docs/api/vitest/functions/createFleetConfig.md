---
editUrl: false
next: false
prev: false
title: "createFleetConfig"
---

> **createFleetConfig**(`overrides?`): [`TestFleetConfig`](/api/vitest/interfaces/testfleetconfig/)

Defined in: [fixtures.ts:176](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/fixtures.ts#L176)

Create a fleet configuration for testing.

## Parameters

### overrides?

`Partial`\<[`TestFleetConfig`](/api/vitest/interfaces/testfleetconfig/)\> = `{}`

Optional overrides

## Returns

[`TestFleetConfig`](/api/vitest/interfaces/testfleetconfig/)

Fleet configuration

## Example

```typescript
import { createFleetConfig } from 'vitest-agentic-control';

const fleet = createFleetConfig({
  concurrency: 5,
  timeout: 60000,
});
```
