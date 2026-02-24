---
editUrl: false
next: false
prev: false
title: "TriageConnectorsConfig"
---

Defined in: [packages/triage/src/triage/connectors.ts:49](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L49)

## Properties

### preferBeads?

> `optional` **preferBeads**: `boolean`

Defined in: [packages/triage/src/triage/connectors.ts:70](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L70)

Prefer Beads over GitHub when both are available

#### Default

```ts
true
```

***

### provider?

> `optional` **provider**: [`ProviderConfig`](/api/triage/type-aliases/providerconfig/)

Defined in: [packages/triage/src/triage/connectors.ts:54](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L54)

Provider configuration. If not provided, will auto-detect.
Can be a single config or multiple for different providers.

***

### repo?

> `optional` **repo**: `string`

Defined in: [packages/triage/src/triage/connectors.ts:64](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L64)

Repository for GitHub provider (owner/repo format)

***

### workingDir?

> `optional` **workingDir**: `string`

Defined in: [packages/triage/src/triage/connectors.ts:59](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L59)

Working directory for local providers (Beads)
