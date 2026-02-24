---
editUrl: false
next: false
prev: false
title: "TriageIssue"
---

Defined in: [packages/triage/src/providers/types.ts:28](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L28)

Normalized issue representation across all providers

## Properties

### assignee?

> `optional` **assignee**: `string`

Defined in: [packages/triage/src/providers/types.ts:44](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L44)

Assignee username/identifier

***

### closedAt?

> `optional` **closedAt**: `string`

Defined in: [packages/triage/src/providers/types.ts:50](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L50)

Closed timestamp if applicable

***

### createdAt

> **createdAt**: `string`

Defined in: [packages/triage/src/providers/types.ts:46](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L46)

Creation timestamp (ISO 8601)

***

### description?

> `optional` **description**: `string`

Defined in: [packages/triage/src/providers/types.ts:34](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L34)

Optional description/body

***

### id

> **id**: `string`

Defined in: [packages/triage/src/providers/types.ts:30](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L30)

Provider-specific ID (e.g., "123" for GitHub, "bd-a1b2" for Beads)

***

### labels

> **labels**: `string`[]

Defined in: [packages/triage/src/providers/types.ts:42](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L42)

Labels/tags

***

### metadata?

> `optional` **metadata**: `Record`\<`string`, `unknown`\>

Defined in: [packages/triage/src/providers/types.ts:54](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L54)

Provider-specific metadata

***

### priority

> **priority**: [`IssuePriority`](/api/triage/type-aliases/issuepriority/)

Defined in: [packages/triage/src/providers/types.ts:38](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L38)

Priority level

***

### status

> **status**: [`IssueStatus`](/api/triage/type-aliases/issuestatus/)

Defined in: [packages/triage/src/providers/types.ts:36](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L36)

Issue status

***

### title

> **title**: `string`

Defined in: [packages/triage/src/providers/types.ts:32](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L32)

Human-readable title

***

### type

> **type**: [`IssueType`](/api/triage/type-aliases/issuetype/)

Defined in: [packages/triage/src/providers/types.ts:40](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L40)

Issue type

***

### updatedAt

> **updatedAt**: `string`

Defined in: [packages/triage/src/providers/types.ts:48](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L48)

Last update timestamp (ISO 8601)

***

### url?

> `optional` **url**: `string`

Defined in: [packages/triage/src/providers/types.ts:52](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L52)

URL to view the issue (if available)
