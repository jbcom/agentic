---
editUrl: false
next: false
prev: false
title: "createTriageConnectors"
---

> **createTriageConnectors**(`config?`): `Promise`\<[`TriageConnectors`](/api/triage/classes/triageconnectors/)\>

Defined in: [packages/triage/src/triage/connectors.ts:409](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L409)

Create a TriageConnectors instance with auto-detection

## Parameters

### config?

[`TriageConnectorsConfig`](/api/triage/interfaces/triageconnectorsconfig/)

## Returns

`Promise`\<[`TriageConnectors`](/api/triage/classes/triageconnectors/)\>

## Example

```typescript
const triage = await createTriageConnectors();
const issues = await triage.issues.list({ status: 'open' });
```
