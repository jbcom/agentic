---
editUrl: false
next: false
prev: false
title: "TriageConnectors"
---

Defined in: [packages/triage/src/triage/connectors.ts:83](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L83)

Unified triage connector providing issue, project, and review APIs.

This is the main entry point for programmatic triage operations.
For AI agent tools, see `getTriageTools()` in tools.ts.

## Constructors

### Constructor

> **new TriageConnectors**(`config?`): `TriageConnectors`

Defined in: [packages/triage/src/triage/connectors.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L105)

#### Parameters

##### config?

[`TriageConnectorsConfig`](/api/triage/interfaces/triageconnectorsconfig/) = `{}`

#### Returns

`TriageConnectors`

## Properties

### issues

> `readonly` **issues**: `IssueAPI`

Defined in: [packages/triage/src/triage/connectors.ts:91](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L91)

Issue operations API

***

### projects

> `readonly` **projects**: `ProjectAPI`

Defined in: [packages/triage/src/triage/connectors.ts:97](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L97)

Project operations API (boards, sprints, epics)

#### Remarks

Coming soon - currently returns stubs

***

### reviews

> `readonly` **reviews**: `ReviewAPI`

Defined in: [packages/triage/src/triage/connectors.ts:103](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L103)

Review operations API (PR feedback, comments)

#### Remarks

Coming soon - currently returns stubs

## Methods

### getProvider()

> **getProvider**(): `Promise`\<[`TriageProvider`](/api/triage/interfaces/triageprovider/)\>

Defined in: [packages/triage/src/triage/connectors.ts:117](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L117)

Get or initialize the underlying provider

#### Returns

`Promise`\<[`TriageProvider`](/api/triage/interfaces/triageprovider/)\>

***

### getProviderName()

> **getProviderName**(): `Promise`\<`string`\>

Defined in: [packages/triage/src/triage/connectors.ts:165](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L165)

Get the provider name

#### Returns

`Promise`\<`string`\>

***

### isReady()

> **isReady**(): `Promise`\<`boolean`\>

Defined in: [packages/triage/src/triage/connectors.ts:173](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L173)

Check if the connector is ready

#### Returns

`Promise`\<`boolean`\>

***

### reconfigure()

> **reconfigure**(`config`): `Promise`\<`void`\>

Defined in: [packages/triage/src/triage/connectors.ts:143](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L143)

Reconfigure the connectors with a new configuration.
This will reset the underlying provider.

#### Parameters

##### config

[`TriageConnectorsConfig`](/api/triage/interfaces/triageconnectorsconfig/)

#### Returns

`Promise`\<`void`\>

***

### sync()

> **sync**(): `Promise`\<`void`\>

Defined in: [packages/triage/src/triage/connectors.ts:185](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/triage/connectors.ts#L185)

Sync with remote (for providers that support it)

#### Returns

`Promise`\<`void`\>
