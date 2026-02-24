---
editUrl: false
next: false
prev: false
title: "FleetConfig"
---

Defined in: [packages/agentic-control/src/fleet/fleet.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L36)

## Extends

- [`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/)

## Properties

### apiKey?

> `optional` **apiKey**: `string`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L34)

API key (defaults to CURSOR_API_KEY env var)

#### Inherited from

[`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/).[`apiKey`](/api/control/interfaces/cursorapioptions/#apikey)

***

### archivePath?

> `optional` **archivePath**: `string`

Defined in: [packages/agentic-control/src/fleet/fleet.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/fleet.ts#L38)

Path to archive conversations (default: ./memory-bank/recovery)

***

### baseUrl?

> `optional` **baseUrl**: `string`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L38)

API base URL (default: https://api.cursor.com/v0)

#### Inherited from

[`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/).[`baseUrl`](/api/control/interfaces/cursorapioptions/#baseurl)

***

### maxRetries?

> `optional` **maxRetries**: `number`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L40)

Maximum number of retries for transient errors (default: 3)

#### Inherited from

[`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/).[`maxRetries`](/api/control/interfaces/cursorapioptions/#maxretries)

***

### retryDelay?

> `optional` **retryDelay**: `number`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L42)

Initial delay for exponential backoff in ms (default: 1000)

#### Inherited from

[`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/).[`retryDelay`](/api/control/interfaces/cursorapioptions/#retrydelay)

***

### timeout?

> `optional` **timeout**: `number`

Defined in: [packages/agentic-control/src/fleet/cursor-api.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/fleet/cursor-api.ts#L36)

Request timeout in milliseconds (default: 60000)

#### Inherited from

[`CursorAPIOptions`](/api/control/interfaces/cursorapioptions/).[`timeout`](/api/control/interfaces/cursorapioptions/#timeout)
