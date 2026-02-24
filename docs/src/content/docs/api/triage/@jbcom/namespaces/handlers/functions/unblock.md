---
editUrl: false
next: false
prev: false
title: "unblock"
---

> **unblock**(`situation`, `model`, `context?`): `Promise`\<\{ `diagnosis`: `string`; `escalationReason?`: `string`; `immediateAction`: `string`; `needsHuman`: `boolean`; `rootCause`: `string`; `suggestions`: `object`[]; \}\>

Defined in: [packages/triage/src/handlers/sage.ts:217](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/handlers/sage.ts#L217)

Help unblock stuck work

## Parameters

### situation

`string`

Description of the blocked situation

### model

`LanguageModel`

The Vercel AI SDK model to use

### context?

[`SageContext`](/api/triage/jbcom/namespaces/handlers/interfaces/sagecontext/)

Optional context for better diagnosis

## Returns

`Promise`\<\{ `diagnosis`: `string`; `escalationReason?`: `string`; `immediateAction`: `string`; `needsHuman`: `boolean`; `rootCause`: `string`; `suggestions`: `object`[]; \}\>

Unblock suggestions
