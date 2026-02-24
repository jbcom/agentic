---
editUrl: false
next: false
prev: false
title: "loadJsonDefinitions"
---

> **loadJsonDefinitions**(`options`): `Promise`\<[`DefinitionBundle`](/api/meshy/interfaces/definitionbundle/)\>

Defined in: [core/definitions.ts:31](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/definitions.ts#L31)

Load and validate pipeline/task definitions from disk.

## Parameters

### options

[`DefinitionLoadOptions`](/api/meshy/interfaces/definitionloadoptions/)

Directory paths for pipeline and task definitions.

## Returns

`Promise`\<[`DefinitionBundle`](/api/meshy/interfaces/definitionbundle/)\>

Parsed and validated definition bundle.
