---
editUrl: false
next: false
prev: false
title: "TestResult"
---

Defined in: [packages/triage/src/test-results.ts:8](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L8)

Strata Test Results Format

Custom test result format designed for AI-powered triage and diagnosis.
Used by both Vitest and Playwright reporters.

## Properties

### duration

> **duration**: `number`

Defined in: [packages/triage/src/test-results.ts:22](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L22)

Duration in milliseconds

***

### error?

> `optional` **error**: [`TestError`](/api/triage/interfaces/testerror/)

Defined in: [packages/triage/src/test-results.ts:24](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L24)

Error details if failed

***

### file

> **file**: `string`

Defined in: [packages/triage/src/test-results.ts:16](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L16)

Source file containing the test

***

### fullName

> **fullName**: `string`

Defined in: [packages/triage/src/test-results.ts:14](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L14)

Full test path (describe blocks)

***

### id

> **id**: `string`

Defined in: [packages/triage/src/test-results.ts:10](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L10)

Unique test identifier

***

### line?

> `optional` **line**: `number`

Defined in: [packages/triage/src/test-results.ts:18](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L18)

Line number in source file

***

### name

> **name**: `string`

Defined in: [packages/triage/src/test-results.ts:12](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L12)

Test name/title

***

### retry?

> `optional` **retry**: `number`

Defined in: [packages/triage/src/test-results.ts:26](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L26)

Retry attempt number

***

### status

> **status**: `"failed"` \| `"skipped"` \| `"passed"` \| `"todo"`

Defined in: [packages/triage/src/test-results.ts:20](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L20)

Test status

***

### tags?

> `optional` **tags**: `string`[]

Defined in: [packages/triage/src/test-results.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/test-results.ts#L28)

Tags/annotations
