---
editUrl: false
next: false
prev: false
title: "CursorAPIOptions"
---

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L32)

## Extended by

- [`FleetConfig`](/api/control/interfaces/fleetconfig/)

## Properties

### apiKey?

> `optional` **apiKey**: `string`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L34)

API key (defaults to CURSOR_API_KEY env var)

***

### baseUrl?

> `optional` **baseUrl**: `string`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L38)

API base URL (default: https://api.cursor.com/v0)

***

### maxRetries?

> `optional` **maxRetries**: `number`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L40)

Maximum number of retries for transient errors (default: 3)

***

### retryDelay?

> `optional` **retryDelay**: `number`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L42)

Initial delay for exponential backoff in ms (default: 1000)

***

### timeout?

> `optional` **timeout**: `number`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L36)

Request timeout in milliseconds (default: 60000)
