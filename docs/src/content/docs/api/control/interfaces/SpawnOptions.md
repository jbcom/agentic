---
editUrl: false
next: false
prev: false
title: "SpawnOptions"
---

Defined in: [packages/agentic-control/src/core/types.ts:293](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L293)

Options for spawning a new agent

API Spec: https://cursor.com/docs/cloud-agent/api/endpoints

## Properties

### context?

> `optional` **context**: `Record`\<`string`, `unknown`\>

Defined in: [packages/agentic-control/src/core/types.ts:301](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L301)

Coordination context (for multi-agent orchestration)

***

### ref?

> `optional` **ref**: `string`

Defined in: [packages/agentic-control/src/core/types.ts:299](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L299)

Git ref (branch, tag, commit) - defaults to "main"

***

### repository

> **repository**: `string`

Defined in: [packages/agentic-control/src/core/types.ts:295](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L295)

GitHub repository URL (e.g., https://github.com/org/repo)

***

### target?

> `optional` **target**: `object`

Defined in: [packages/agentic-control/src/core/types.ts:303](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L303)

Target configuration

#### autoCreatePr?

> `optional` **autoCreatePr**: `boolean`

Auto-create PR when agent completes

#### branchName?

> `optional` **branchName**: `string`

Custom branch name

#### openAsCursorGithubApp?

> `optional` **openAsCursorGithubApp**: `boolean`

Open PR as Cursor GitHub App instead of user

#### skipReviewerRequest?

> `optional` **skipReviewerRequest**: `boolean`

Skip adding user as reviewer

***

### task

> **task**: `string`

Defined in: [packages/agentic-control/src/core/types.ts:297](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L297)

Task description for the agent

***

### webhook?

> `optional` **webhook**: `object`

Defined in: [packages/agentic-control/src/core/types.ts:314](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/agentic-control/src/core/types.ts#L314)

Webhook for status notifications

#### secret?

> `optional` **secret**: `string`

Secret for payload verification (min 32 chars)

#### url

> **url**: `string`

URL to receive notifications
