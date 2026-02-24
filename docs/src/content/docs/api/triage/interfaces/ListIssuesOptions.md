---
editUrl: false
next: false
prev: false
title: "ListIssuesOptions"
---

Defined in: [packages/triage/src/providers/types.ts:105](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L105)

Options for listing issues

## Properties

### assignee?

> `optional` **assignee**: `string`

Defined in: [packages/triage/src/providers/types.ts:117](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L117)

Filter by assignee

***

### createdAfter?

> `optional` **createdAfter**: `string`

Defined in: [packages/triage/src/providers/types.ts:123](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L123)

Created after date

***

### createdBefore?

> `optional` **createdBefore**: `string`

Defined in: [packages/triage/src/providers/types.ts:125](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L125)

Created before date

***

### descriptionContains?

> `optional` **descriptionContains**: `string`

Defined in: [packages/triage/src/providers/types.ts:121](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L121)

Search in description

***

### labels?

> `optional` **labels**: `string`[]

Defined in: [packages/triage/src/providers/types.ts:113](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L113)

Filter by labels (AND logic)

***

### labelsAny?

> `optional` **labelsAny**: `string`[]

Defined in: [packages/triage/src/providers/types.ts:115](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L115)

Filter by any of these labels (OR logic)

***

### limit?

> `optional` **limit**: `number`

Defined in: [packages/triage/src/providers/types.ts:127](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L127)

Maximum results

***

### priority?

> `optional` **priority**: [`IssuePriority`](/api/triage/type-aliases/issuepriority/) \| [`IssuePriority`](/api/triage/type-aliases/issuepriority/)[]

Defined in: [packages/triage/src/providers/types.ts:109](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L109)

Filter by priority

***

### sortBy?

> `optional` **sortBy**: `"priority"` \| `"created"` \| `"updated"`

Defined in: [packages/triage/src/providers/types.ts:129](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L129)

Sort field

***

### sortOrder?

> `optional` **sortOrder**: `"asc"` \| `"desc"`

Defined in: [packages/triage/src/providers/types.ts:131](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L131)

Sort direction

***

### status?

> `optional` **status**: [`IssueStatus`](/api/triage/type-aliases/issuestatus/) \| [`IssueStatus`](/api/triage/type-aliases/issuestatus/)[]

Defined in: [packages/triage/src/providers/types.ts:107](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L107)

Filter by status

***

### titleContains?

> `optional` **titleContains**: `string`

Defined in: [packages/triage/src/providers/types.ts:119](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L119)

Search in title

***

### type?

> `optional` **type**: [`IssueType`](/api/triage/type-aliases/issuetype/) \| [`IssueType`](/api/triage/type-aliases/issuetype/)[]

Defined in: [packages/triage/src/providers/types.ts:111](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/providers/types.ts#L111)

Filter by type
