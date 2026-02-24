---
editUrl: false
next: false
prev: false
title: "PriorityScorer"
---

Defined in: [packages/triage/src/queue/priority.ts:27](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/priority.ts#L27)

Priority scorer for queue items

## Constructors

### Constructor

> **new PriorityScorer**(): `PriorityScorer`

#### Returns

`PriorityScorer`

## Methods

### score()

> **score**(`pr`): [`Priority`](/api/triage/type-aliases/priority/)

Defined in: [packages/triage/src/queue/priority.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/priority.ts#L32)

Calculate priority score from PR metadata
Returns 1 (critical), 2 (normal), or 3 (low)

#### Parameters

##### pr

[`PRMetadata`](/api/triage/interfaces/prmetadata/)

#### Returns

[`Priority`](/api/triage/type-aliases/priority/)

***

### fromLabels()

> `static` **fromLabels**(`labels`): [`Priority`](/api/triage/type-aliases/priority/)

Defined in: [packages/triage/src/queue/priority.ts:74](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/priority.ts#L74)

Calculate priority from labels
Looks for priority/critical, priority/high, priority/low labels

#### Parameters

##### labels

`string`[]

#### Returns

[`Priority`](/api/triage/type-aliases/priority/)

***

### fromType()

> `static` **fromType**(`type`): [`Priority`](/api/triage/type-aliases/priority/)

Defined in: [packages/triage/src/queue/priority.ts:137](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/queue/priority.ts#L137)

Calculate priority from PR type

#### Parameters

##### type

`"ci-fix"` | `"security"` | `"feature"` | `"docs"` | `"bugfix"` | `"chore"`

#### Returns

[`Priority`](/api/triage/type-aliases/priority/)
