---
editUrl: false
next: false
prev: false
title: "triageTools"
---

> `const` **triageTools**: `object`

Defined in: [packages/triage/src/tools/index.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/tools/index.ts#L23)

## Type Declaration

### addLabels

> **addLabels**: `Tool`\<\{ `id`: `string`; `labels`: `string`[]; \}, \{ `id`: `string`; `labelsAdded`: `string`[]; \}\> = `addLabelsTool`

### analyzePR

> **analyzePR**: `Tool`\<\{ `analysis`: \{ `breakingChanges`: `string`[]; `relatedIssues`: `string`[]; `riskLevel`: `"high"` \| `"medium"` \| `"low"`; `scope`: `"minor"` \| `"major"` \| `"patch"` \| `"breaking"`; `summary`: `string`; `testingCoverage`: `"none"` \| `"partial"` \| `"full"`; `title`: `string`; \}; `prNumber`: `number`; \}, \{ `analysis`: \{ `breakingChanges`: `string`[]; `relatedIssues`: `string`[]; `riskLevel`: `"high"` \| `"medium"` \| `"low"`; `scope`: `"minor"` \| `"major"` \| `"patch"` \| `"breaking"`; `summary`: `string`; `testingCoverage`: `"none"` \| `"partial"` \| `"full"`; `title`: `string`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\> = `analyzePRTool`

### closeIssue

> **closeIssue**: `Tool`\<\{ `id`: `string`; `reason?`: `string`; \}, [`TriageIssue`](/api/triage/interfaces/triageissue/)\> = `closeIssueTool`

### createIssue

> **createIssue**: `Tool`\<\{ `assignee?`: `string`; `description?`: `string`; `labels?`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}, [`TriageIssue`](/api/triage/interfaces/triageissue/)\> = `createIssueTool`

### getIssue

> **getIssue**: `Tool`\<\{ `id`: `string`; \}, [`TriageIssue`](/api/triage/interfaces/triageissue/) \| `null`\> = `getIssueTool`

### listIssues

> **listIssues**: `Tool`\<\{ `assignee?`: `string`; `labels?`: `string`[]; `limit?`: `number`; `priority?`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `status?`: `"open"` \| `"in_progress"` \| `"blocked"` \| `"closed"`; `type?`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}, [`TriageIssue`](/api/triage/interfaces/triageissue/)[]\> = `listIssuesTool`

### removeLabels

> **removeLabels**: `Tool`\<\{ `id`: `string`; `labels`: `string`[]; \}, \{ `id`: `string`; `labelsRemoved`: `string`[]; \}\> = `removeLabelsTool`

### sage

> **sage**: `Tool`\<\{ `context?`: \{ `currentContext?`: `string`; `issueContext?`: `string`; `keyFiles?`: `Record`\<`string`, `string`\>; `repoStructure?`: `string`; \}; `query`: `string`; \}, `unknown`\> = `sageTool`

### searchIssues

> **searchIssues**: `Tool`\<\{ `query`: `string`; \}, [`TriageIssue`](/api/triage/interfaces/triageissue/)[]\> = `searchIssuesTool`

### submitReview

> **submitReview**: `Tool`\<\{ `prNumber`: `number`; `review`: \{ `comments`: `object`[]; `impact`: `"critical"` \| `"high"` \| `"medium"` \| `"low"`; `status`: `"approve"` \| `"request_changes"` \| `"comment"`; `suggestedLabels`: `string`[]; `summary`: `string`; \}; \}, \{ `error?`: `undefined`; `message`: `string`; `review`: \{ `comments`: `object`[]; `impact`: `"critical"` \| `"high"` \| `"medium"` \| `"low"`; `status`: `"approve"` \| `"request_changes"` \| `"comment"`; `suggestedLabels`: `string`[]; `summary`: `string`; \}; `success`: `boolean`; \} \| \{ `error`: `unknown`; `message`: `string`; `review?`: `undefined`; `success`: `boolean`; \}\> = `submitReviewTool`

### triageIssue

> **triageIssue**: `Tool`\<\{ `analysis`: \{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; `id`: `string`; \}, \{ `analysis`: \{ `actionItems`: `string`[]; `estimate?`: `number`; `labels`: `string`[]; `priority`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `summary`: `string`; `title`: `string`; `type`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; `error?`: `undefined`; `message`: `string`; `success`: `boolean`; \} \| \{ `analysis?`: `undefined`; `error`: `unknown`; `message`: `string`; `success`: `boolean`; \}\> = `triageIssueTool`

### updateIssue

> **updateIssue**: `Tool`\<\{ `id`: `string`; `updates`: \{ `assignee?`: `string`; `description?`: `string`; `priority?`: `"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"backlog"`; `status?`: `"open"` \| `"in_progress"` \| `"blocked"` \| `"closed"`; `title?`: `string`; `type?`: `"feature"` \| `"docs"` \| `"chore"` \| `"bug"` \| `"task"` \| `"epic"`; \}; \}, [`TriageIssue`](/api/triage/interfaces/triageissue/)\> = `updateIssueTool`

### visualReview

> **visualReview**: `Tool`\<\{ `scenario?`: `string`; `url`: `string`; `viewport?`: \{ `height`: `number`; `width`: `number`; \}; \}, `unknown`\> = `visualReviewTool`
