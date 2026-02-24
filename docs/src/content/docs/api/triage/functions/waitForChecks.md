---
editUrl: false
next: false
prev: false
title: "waitForChecks"
---

> **waitForChecks**(`ref`, `options?`): `Promise`\<\{ `failed`: `string`[]; `passing`: `boolean`; \}\>

Defined in: [packages/triage/src/octokit.ts:982](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/octokit.ts#L982)

Wait for checks to complete and return status

## Parameters

### ref

`string`

### options?

#### pollInterval?

`number`

#### timeout?

`number`

## Returns

`Promise`\<\{ `failed`: `string`[]; `passing`: `boolean`; \}\>
