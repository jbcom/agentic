---
editUrl: false
next: false
prev: false
title: "PipelineRunner"
---

Defined in: [core/runner.ts:61](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L61)

Executes declarative pipelines against asset manifests.

## Constructors

### Constructor

> **new PipelineRunner**(`options`): `PipelineRunner`

Defined in: [core/runner.ts:67](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L67)

#### Parameters

##### options

[`PipelineRunnerOptions`](/api/meshy/interfaces/pipelinerunneroptions/)

#### Returns

`PipelineRunner`

## Methods

### run()

> **run**(`options`): `Promise`\<\{\[`key`: `string`\]: `unknown`; `id`: `string`; `name`: `string`; `seed?`: `number`; `tasks?`: `Record`\<`string`, \{\[`key`: `string`\]: `unknown`; `artifacts?`: `Record`\<`string`, `string`\>; `outputs?`: `Record`\<`string`, `unknown`\>; `status?`: `"PENDING"` \| `"IN_PROGRESS"` \| `"SUCCEEDED"` \| `"FAILED"` \| `"CANCELED"`; `taskId?`: `string`; \}\>; `type`: `string`; \}\>

Defined in: [core/runner.ts:80](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/meshy-content-generator/src/core/runner.ts#L80)

Run a pipeline against the manifest in the provided asset directory.

#### Parameters

##### options

[`RunOptions`](/api/meshy/interfaces/runoptions/)

Pipeline execution options.

#### Returns

`Promise`\<\{\[`key`: `string`\]: `unknown`; `id`: `string`; `name`: `string`; `seed?`: `number`; `tasks?`: `Record`\<`string`, \{\[`key`: `string`\]: `unknown`; `artifacts?`: `Record`\<`string`, `string`\>; `outputs?`: `Record`\<`string`, `unknown`\>; `status?`: `"PENDING"` \| `"IN_PROGRESS"` \| `"SUCCEEDED"` \| `"FAILED"` \| `"CANCELED"`; `taskId?`: `string`; \}\>; `type`: `string`; \}\>

Updated manifest with task state.
