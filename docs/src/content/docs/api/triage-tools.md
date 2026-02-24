---
title: Triage Tools API
description: Complete API reference for @agentic/triage tools
---

# Triage Tools API Reference

Complete reference for the triage tools from `@agentic-dev-library/triage`.

## Installation

```bash
npm install @agentic-dev-library/triage
```

## Tool Collections

### getTriageTools()

Get all available triage tools.

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';

const tools = getTriageTools();
// Returns all issue, review, and project tools
```

---

### getIssueTools()

Get issue management tools.

```typescript
import { getIssueTools } from '@agentic-dev-library/triage';

const tools = getIssueTools();
// Returns: listIssues, getIssue, createIssue, updateIssue,
//          closeIssue, searchIssues, addLabels, removeLabels
```

---

### getReviewTools()

Get code review tools.

```typescript
import { getReviewTools } from '@agentic-dev-library/triage';

const tools = getReviewTools();
// Returns: getPRComments, addPRComment, approvePR, requestChanges
```

---

### getProjectTools()

Get project management tools.

```typescript
import { getProjectTools } from '@agentic-dev-library/triage';

const tools = getProjectTools();
// Returns: getSprints, getCurrentSprint, getSprintIssues, moveToSprint
```

## Issue Tools

### listIssuesTool

List issues with optional filtering.

```typescript
import { listIssuesTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `status` | `'open' \| 'closed' \| 'all'` | Filter by status (default: `'open'`) |
| `labels` | `string[]` | Filter by labels |
| `assignee` | `string` | Filter by assignee |
| `limit` | `number` | Maximum results (default: 30) |

**Returns:** `Issue[]`

---

### getIssueTool

Get a specific issue by number.

```typescript
import { getIssueTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `issueNumber` | `number` | Issue number |

**Returns:** `Issue`

---

### createIssueTool

Create a new issue.

```typescript
import { createIssueTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `title` | `string` | Issue title (required) |
| `body` | `string` | Issue body/description |
| `labels` | `string[]` | Labels to add |
| `assignees` | `string[]` | Users to assign |
| `milestone` | `string` | Milestone name or number |

**Returns:** `Issue`

---

### updateIssueTool

Update an existing issue.

```typescript
import { updateIssueTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `issueNumber` | `number` | Issue number (required) |
| `title` | `string` | New title |
| `body` | `string` | New body |
| `state` | `'open' \| 'closed'` | New state |

**Returns:** `Issue`

---

### closeIssueTool

Close an issue.

```typescript
import { closeIssueTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `issueNumber` | `number` | Issue number (required) |
| `reason` | `string` | Closing comment |

**Returns:** `Issue`

---

### searchIssuesTool

Search issues by query.

```typescript
import { searchIssuesTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `query` | `string` | Search query (required) |
| `sort` | `'created' \| 'updated' \| 'comments'` | Sort field |
| `order` | `'asc' \| 'desc'` | Sort order |

**Returns:** `Issue[]`

---

### addLabelsTool

Add labels to an issue.

```typescript
import { addLabelsTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `issueNumber` | `number` | Issue number (required) |
| `labels` | `string[]` | Labels to add (required) |

**Returns:** `Label[]`

---

### removeLabelsTool

Remove labels from an issue.

```typescript
import { removeLabelsTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `issueNumber` | `number` | Issue number (required) |
| `labels` | `string[]` | Labels to remove (required) |

**Returns:** `Label[]`

## Review Tools

### getPRCommentsTool

Get comments on a pull request.

```typescript
import { getPRCommentsTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `prNumber` | `number` | PR number (required) |

**Returns:** `Comment[]`

---

### addPRCommentTool

Add a comment to a pull request.

```typescript
import { addPRCommentTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `prNumber` | `number` | PR number (required) |
| `body` | `string` | Comment body (required) |
| `path` | `string` | File path (for line comments) |
| `line` | `number` | Line number (for line comments) |

**Returns:** `Comment`

---

### approvePRTool

Approve a pull request.

```typescript
import { approvePRTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `prNumber` | `number` | PR number (required) |
| `comment` | `string` | Approval comment |

**Returns:** `Review`

---

### requestChangesTool

Request changes on a pull request.

```typescript
import { requestChangesTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `prNumber` | `number` | PR number (required) |
| `body` | `string` | Review body (required) |

**Returns:** `Review`

## Project Tools

### getSprintsTool

Get all sprints/iterations.

```typescript
import { getSprintsTool } from '@agentic-dev-library/triage';
```

**Returns:** `Sprint[]`

---

### getCurrentSprintTool

Get the current active sprint.

```typescript
import { getCurrentSprintTool } from '@agentic-dev-library/triage';
```

**Returns:** `Sprint`

---

### getSprintIssuesTool

Get issues in a sprint.

```typescript
import { getSprintIssuesTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `sprintId` | `string` | Sprint ID (required) |

**Returns:** `Issue[]`

---

### moveToSprintTool

Move an issue to a sprint.

```typescript
import { moveToSprintTool } from '@agentic-dev-library/triage';
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `issueNumber` | `number` | Issue number (required) |
| `sprintId` | `string` | Target sprint ID (required) |

**Returns:** `Issue`

## Types

### Issue

```typescript
interface Issue {
  id: number;
  number: number;
  title: string;
  body: string;
  state: 'open' | 'closed';
  labels: Label[];
  assignees: User[];
  milestone?: Milestone;
  createdAt: string;
  updatedAt: string;
  closedAt?: string;
  url: string;
}
```

### Label

```typescript
interface Label {
  id: number;
  name: string;
  color: string;
  description?: string;
}
```

### Comment

```typescript
interface Comment {
  id: number;
  body: string;
  user: User;
  createdAt: string;
  updatedAt: string;
  path?: string;
  line?: number;
}
```

### Review

```typescript
interface Review {
  id: number;
  state: 'APPROVED' | 'CHANGES_REQUESTED' | 'COMMENTED' | 'PENDING';
  body: string;
  user: User;
  submittedAt: string;
}
```

### Sprint

```typescript
interface Sprint {
  id: string;
  title: string;
  state: 'PLANNED' | 'ACTIVE' | 'CLOSED';
  startDate?: string;
  endDate?: string;
  description?: string;
}
```

### User

```typescript
interface User {
  id: number;
  login: string;
  avatarUrl: string;
  url: string;
}
```

## Direct API

For non-AI use cases, use the `TriageConnectors` class directly:

```typescript
import { TriageConnectors } from '@agentic-dev-library/triage';

const triage = new TriageConnectors({ provider: 'github' });

// Issue operations
const issues = await triage.issues.list({ status: 'open' });
const issue = await triage.issues.create({ title: 'Bug', body: 'Details' });
await triage.issues.addLabels(issue.id, ['bug', 'critical']);

// Review operations
const comments = await triage.reviews.getPRComments(123);
await triage.reviews.approve(123, 'LGTM!');

// Project operations
const sprint = await triage.projects.getCurrentSprint();
await triage.projects.moveToSprint(456, sprint.id);
```

## Provider Configuration

```typescript
import { TriageConnectors } from '@agentic-dev-library/triage';

// GitHub (auto-detected if .git present)
const github = new TriageConnectors({
  provider: 'github',
  github: { owner: 'my-org', repo: 'my-repo' }
});

// Jira
const jira = new TriageConnectors({
  provider: 'jira',
  jira: {
    host: 'company.atlassian.net',
    projectKey: 'PROJ'
  }
});

// Linear
const linear = new TriageConnectors({
  provider: 'linear',
  linear: { teamId: 'TEAM123' }
});

// Beads (local-first)
const beads = new TriageConnectors({
  provider: 'beads',
  beads: { workingDir: '/path/to/project' }
});
```

## Next Steps

- [Fleet API Reference](/api/fleet-management/) - Fleet management
- [Token Management API](/api/token-management/) - Token configuration
- [Vercel AI SDK Integration](/integrations/vercel-ai-sdk/) - AI integration
