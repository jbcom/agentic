---
editUrl: false
next: false
prev: false
title: "TestReport"
---

Defined in: [packages/triage/src/test-results.ts:83](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L83)

## Properties

### ci?

> `optional` **ci**: `object`

Defined in: [packages/triage/src/test-results.ts:112](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L112)

CI context

#### issueNumbers?

> `optional` **issueNumbers**: `number`[]

#### prNumber?

> `optional` **prNumber**: `number`

#### provider

> **provider**: `string`

#### runId

> **runId**: `string`

#### runUrl?

> `optional` **runUrl**: `string`

***

### coverage?

> `optional` **coverage**: [`CoverageData`](/api/triage/interfaces/coveragedata/)

Defined in: [packages/triage/src/test-results.ts:103](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L103)

Coverage data (if available)

***

### files

> **files**: [`TestFile`](/api/triage/interfaces/testfile/)[]

Defined in: [packages/triage/src/test-results.ts:101](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L101)

Test files

***

### git?

> `optional` **git**: `object`

Defined in: [packages/triage/src/test-results.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L105)

Git context

#### author?

> `optional` **author**: `string`

#### branch

> **branch**: `string`

#### commit

> **commit**: `string`

#### message?

> `optional` **message**: `string`

***

### runner

> **runner**: `string`

Defined in: [packages/triage/src/test-results.ts:89](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L89)

Test runner (vitest, playwright, etc.)

***

### summary

> **summary**: `object`

Defined in: [packages/triage/src/test-results.ts:93](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L93)

Summary statistics

#### duration

> **duration**: `number`

#### failed

> **failed**: `number`

#### passed

> **passed**: `number`

#### skipped

> **skipped**: `number`

#### total

> **total**: `number`

***

### timestamp

> **timestamp**: `string`

Defined in: [packages/triage/src/test-results.ts:87](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L87)

Report generation timestamp

***

### type

> **type**: `"unit"` \| `"integration"` \| `"e2e"`

Defined in: [packages/triage/src/test-results.ts:91](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L91)

Test type

***

### version

> **version**: `"1.0"`

Defined in: [packages/triage/src/test-results.ts:85](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L85)

Report format version
