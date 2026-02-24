---
editUrl: false
next: false
prev: false
title: "createAgenticMocker"
---

> **createAgenticMocker**(`options?`): [`AgenticMocker`](/api/vitest/classes/agenticmocker/)

Defined in: [mocking.ts:229](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/vitest-agentic-control/src/mocking.ts#L229)

Factory function to create an AgenticMocker instance.

## Parameters

### options?

`AgenticMockerOptions` = `{}`

Configuration options

## Returns

[`AgenticMocker`](/api/vitest/classes/agenticmocker/)

A new AgenticMocker instance

## Example

```typescript
import { createAgenticMocker } from 'vitest-agentic-control';

const mocker = createAgenticMocker();
mocker.mcp.mockServer('test', { tools: [] });
```
