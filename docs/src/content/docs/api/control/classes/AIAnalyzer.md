---
editUrl: false
next: false
prev: false
title: "AIAnalyzer"
---

Defined in: [packages/agentic-control/src/triage/analyzer.ts:174](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L174)

## Constructors

### Constructor

> **new AIAnalyzer**(`options?`): `Analyzer`

Defined in: [packages/agentic-control/src/triage/analyzer.ts:181](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L181)

#### Parameters

##### options?

[`AIAnalyzerOptions`](/api/control/interfaces/aianalyzeroptions/) = `{}`

#### Returns

`Analyzer`

## Methods

### analyzeConversation()

> **analyzeConversation**(`conversation`): `Promise`\<[`AnalysisResult`](/api/control/interfaces/analysisresult/)\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:210](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L210)

Analyze a conversation to extract completed/outstanding tasks

#### Parameters

##### conversation

[`Conversation`](/api/control/interfaces/conversation/)

#### Returns

`Promise`\<[`AnalysisResult`](/api/control/interfaces/analysisresult/)\>

***

### analyzePR()

> **analyzePR**(`github`, `prNumber`): `Promise`\<\{ `blockers`: `object`[]; `ci`: \{ `allPassing`: `boolean`; `anyPending`: `boolean`; `checks`: `object`[]; `failures`: `object`[]; \}; `feedback`: \{ `items`: `object`[]; `total`: `number`; `unaddressed`: `number`; \}; `nextActions`: `object`[]; `prNumber`: `number`; `prTitle`: `string`; `prUrl`: `string`; `status`: `"blocked"` \| `"closed"` \| `"merged"` \| `"needs_work"` \| `"needs_review"` \| `"needs_ci"` \| `"ready_to_merge"`; `summary`: `string`; `timestamp`: `string`; \}\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:351](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L351)

Analyze a Pull Request for triage

#### Parameters

##### github

`GitHubClientInterface`

##### prNumber

`number`

#### Returns

`Promise`\<\{ `blockers`: `object`[]; `ci`: \{ `allPassing`: `boolean`; `anyPending`: `boolean`; `checks`: `object`[]; `failures`: `object`[]; \}; `feedback`: \{ `items`: `object`[]; `total`: `number`; `unaddressed`: `number`; \}; `nextActions`: `object`[]; `prNumber`: `number`; `prTitle`: `string`; `prUrl`: `string`; `status`: `"blocked"` \| `"closed"` \| `"merged"` \| `"needs_work"` \| `"needs_review"` \| `"needs_ci"` \| `"ready_to_merge"`; `summary`: `string`; `timestamp`: `string`; \}\>

***

### createIssuesFromAnalysis()

> **createIssuesFromAnalysis**(`analysis`, `options?`): `Promise`\<`string`[]\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:449](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L449)

Create GitHub issues from analysis.
Always uses PR review token for consistent identity.

#### Parameters

##### analysis

[`AnalysisResult`](/api/control/interfaces/analysisresult/)

##### options?

###### assignCopilot?

`boolean`

###### dryRun?

`boolean`

###### labels?

`string`[]

###### repo?

`string`

#### Returns

`Promise`\<`string`[]\>

***

### generateFeedbackResponse()

> **generateFeedbackResponse**(`feedback`, `context`): `Promise`\<\{ `content`: `string`; `type`: `"fix"` \| `"justification"`; \}\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:396](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L396)

Generate a response for feedback (fix or justification)

#### Parameters

##### feedback

###### author

`string` = `...`

GitHub username of the reviewer

###### body

`string` = `...`

Content of the feedback comment

###### createdAt

`string` = `...`

ISO timestamp when the feedback was created

###### id

`string` = `...`

Unique identifier for the feedback item

###### isAutoResolvable

`boolean` = `...`

Whether AI can automatically resolve this feedback

###### line

`number` \| `null` = `...`

Line number in the file, if applicable

###### path

`string` \| `null` = `...`

File path the feedback relates to, if any

###### resolution

`string` \| `null` = `...`

Description of how the feedback was resolved

###### severity

`"critical"` \| `"high"` \| `"medium"` \| `"low"` \| `"info"` = `FeedbackSeveritySchema`

Severity level of the feedback

###### status

`"unaddressed"` \| `"addressed"` \| `"dismissed"` \| `"wont_fix"` = `FeedbackStatusSchema`

Current status of the feedback

###### suggestedAction

`string` \| `null` = `...`

AI-suggested action to address the feedback

###### url

`string` = `...`

URL to the feedback comment on GitHub

##### context

###### files

`string`[]

###### prTitle

`string`

#### Returns

`Promise`\<\{ `content`: `string`; `type`: `"fix"` \| `"justification"`; \}\>

***

### generateReport()

> **generateReport**(`conversation`): `Promise`\<`string`\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:554](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L554)

Generate a comprehensive assessment report

#### Parameters

##### conversation

[`Conversation`](/api/control/interfaces/conversation/)

#### Returns

`Promise`\<`string`\>

***

### quickTriage()

> **quickTriage**(`input`): `Promise`\<[`TriageResult`](/api/control/interfaces/triageresult/)\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:320](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L320)

Quick triage - fast assessment of what needs attention

#### Parameters

##### input

`string`

#### Returns

`Promise`\<[`TriageResult`](/api/control/interfaces/triageresult/)\>

***

### reviewCode()

> **reviewCode**(`diff`, `context?`): `Promise`\<[`CodeReviewResult`](/api/control/interfaces/codereviewresult/)\>

Defined in: [packages/agentic-control/src/triage/analyzer.ts:273](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L273)

Review code changes and identify issues

#### Parameters

##### diff

`string`

##### context?

`string`

#### Returns

`Promise`\<[`CodeReviewResult`](/api/control/interfaces/codereviewresult/)\>

***

### setRepo()

> **setRepo**(`repo`): `void`

Defined in: [packages/agentic-control/src/triage/analyzer.ts:199](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/triage/analyzer.ts#L199)

Set the repository for GitHub operations

#### Parameters

##### repo

`string`

#### Returns

`void`
