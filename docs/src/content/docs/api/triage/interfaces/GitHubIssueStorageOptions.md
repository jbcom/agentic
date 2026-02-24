---
editUrl: false
next: false
prev: false
title: "GitHubIssueStorageOptions"
---

Defined in: [packages/triage/src/storage/github-issue.ts:13](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L13)

## Properties

### issueNumber

> **issueNumber**: `number` \| `"auto"`

Defined in: [packages/triage/src/storage/github-issue.ts:17](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L17)

Issue number, or 'auto' to create if doesn't exist

***

### issueTitle?

> `optional` **issueTitle**: `string`

Defined in: [packages/triage/src/storage/github-issue.ts:19](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L19)

Issue title when auto-creating

***

### octokit?

> `optional` **octokit**: [`GitHubIssueStorageOctokit`](/api/triage/interfaces/githubissuestorageoctokit/)

Defined in: [packages/triage/src/storage/github-issue.ts:23](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L23)

Optional Octokit instance (for testing)

***

### repo

> **repo**: `string`

Defined in: [packages/triage/src/storage/github-issue.ts:15](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L15)

Repository in format "owner/repo"

***

### token

> **token**: `string`

Defined in: [packages/triage/src/storage/github-issue.ts:21](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/storage/github-issue.ts#L21)

GitHub token for authentication
