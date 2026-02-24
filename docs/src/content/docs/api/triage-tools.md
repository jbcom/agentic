---
title: "Triage Tools API"
description: "Complete API reference for the AI-powered triage tools from @jbcom/agentic-triage, compatible with the Vercel AI SDK."
---

# Triage Tools API Reference

The triage tools package provides AI-compatible tool definitions for issue management, code review, PR analysis, visual testing, and intelligent routing. All tools are built with the [Vercel AI SDK](https://sdk.vercel.ai/) `tool()` function and use [Zod](https://zod.dev/) schemas for input validation.

## Installation

```bash
npm install @jbcom/agentic-triage
```

## Import

```typescript
import {
  // Tool collections
  getTriageTools,
  triageTools,

  // Issue tools
  listIssuesTool,
  getIssueTool,
  createIssueTool,
  updateIssueTool,
  closeIssueTool,
  searchIssuesTool,
  addLabelsTool,
  removeLabelsTool,
  triageIssueTool,

  // Review tools
  submitReviewTool,

  // PR tools
  analyzePRTool,

  // Sage (AI advisor)
  sageTool,

  // Visual review
  visualReviewTool,
} from '@jbcom/agentic-triage';
```

## Tool Collections

### getTriageTools()

Returns the complete set of all available triage tools as a single object.

```typescript
import { getTriageTools } from '@jbcom/agentic-triage';

const tools = getTriageTools();
// Returns object with all 13 tools
```

### triageTools

The same collection exported as a named constant:

```typescript
import { triageTools } from '@jbcom/agentic-triage';

// triageTools.listIssues
// triageTools.getIssue
// triageTools.createIssue
// triageTools.updateIssue
// triageTools.closeIssue
// triageTools.searchIssues
// triageTools.addLabels
// triageTools.removeLabels
// triageTools.triageIssue
// triageTools.submitReview
// triageTools.analyzePR
// triageTools.sage
// triageTools.visualReview
```

---

## Usage with Vercel AI SDK

All tools work directly with `generateText` and `streamText`:

```typescript
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { getTriageTools } from '@jbcom/agentic-triage';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all open bugs and triage the most critical one.',
  maxSteps: 10,
});

console.log(result.text);
```

For streaming:

```typescript
import { streamText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { triageTools } from '@jbcom/agentic-triage';

const result = streamText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: triageTools,
  prompt: 'Review PR #42 and provide feedback.',
  maxSteps: 5,
});

for await (const chunk of result.textStream) {
  process.stdout.write(chunk);
}
```

---

## Issue Tools

### listIssuesTool

List issues from the configured issue tracker with optional filters.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | `'open' \| 'in_progress' \| 'blocked' \| 'closed'` | No | Filter by issue status |
| `priority` | `'critical' \| 'high' \| 'medium' \| 'low' \| 'backlog'` | No | Filter by priority |
| `type` | `'bug' \| 'feature' \| 'task' \| 'epic' \| 'chore' \| 'docs'` | No | Filter by issue type |
| `labels` | `string[]` | No | Filter by labels |
| `limit` | `number` | No | Maximum number of results |
| `assignee` | `string` | No | Filter by assignee username |

**Returns:** Array of issue objects.

```typescript
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { listIssuesTool } from '@jbcom/agentic-triage';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: { listIssues: listIssuesTool },
  prompt: 'Show me all open bugs with critical priority',
});
```

---

### getIssueTool

Get detailed information about a specific issue by ID.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | The issue ID |

**Returns:** Complete issue object with title, description, labels, assignees, and metadata.

---

### createIssueTool

Create a new issue in the configured issue tracker.

**Input Schema:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `title` | `string` | Yes | -- | Issue title |
| `description` | `string` | No | -- | Issue body/description |
| `type` | `'bug' \| 'feature' \| 'task' \| 'epic' \| 'chore' \| 'docs'` | No | `'task'` | Issue type |
| `priority` | `'critical' \| 'high' \| 'medium' \| 'low' \| 'backlog'` | No | `'medium'` | Priority level |
| `labels` | `string[]` | No | -- | Labels to apply |
| `assignee` | `string` | No | -- | Username to assign |

**Returns:** The created issue object.

```typescript
import { createIssueTool } from '@jbcom/agentic-triage';

// Used by AI:
// "Create a bug report for the login page timeout issue"
// -> Calls createIssueTool with { title: "Login page timeout", type: "bug", priority: "high" }
```

---

### updateIssueTool

Update fields on an existing issue.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | The issue ID to update |
| `updates.title` | `string` | No | New title |
| `updates.description` | `string` | No | New description |
| `updates.status` | `'open' \| 'in_progress' \| 'blocked' \| 'closed'` | No | New status |
| `updates.priority` | `'critical' \| 'high' \| 'medium' \| 'low' \| 'backlog'` | No | New priority |
| `updates.type` | `'bug' \| 'feature' \| 'task' \| 'epic' \| 'chore' \| 'docs'` | No | New type |
| `updates.assignee` | `string` | No | New assignee |

**Returns:** The updated issue object.

---

### triageIssueTool

Apply structured triage analysis to an issue. This tool applies a comprehensive analysis including title optimization, priority assessment, type categorization, label recommendations, and action items.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | The issue ID to triage |
| `analysis.title` | `string` | Yes | Cleaned up / optimized title |
| `analysis.summary` | `string` | Yes | Concise summary of the issue |
| `analysis.type` | `IssueType` | Yes | Categorized issue type |
| `analysis.priority` | `IssuePriority` | Yes | Determined priority based on impact and urgency |
| `analysis.labels` | `string[]` | Yes | Recommended labels |
| `analysis.estimate` | `number` | No | Optional story point estimate |
| `analysis.actionItems` | `string[]` | Yes | Concrete next steps discovered from the description |

The `IssueTriageSchema` validates the analysis object:

```typescript
import { z } from 'zod';

const IssueTriageSchema = z.object({
  title: z.string(),
  summary: z.string(),
  type: z.enum(['bug', 'feature', 'task', 'epic', 'chore', 'docs']),
  priority: z.enum(['critical', 'high', 'medium', 'low', 'backlog']),
  labels: z.array(z.string()),
  estimate: z.number().optional(),
  actionItems: z.array(z.string()),
});
```

---

### closeIssueTool

Close an issue with an optional reason.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | The issue ID to close |
| `reason` | `string` | No | Closing comment/reason |

---

### searchIssuesTool

Full-text search across all issues.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | `string` | Yes | Search query string |

**Returns:** Array of matching issue objects.

---

### addLabelsTool

Add labels to an issue.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | The issue ID |
| `labels` | `string[]` | Yes | Labels to add |

**Returns:** `{ id, labelsAdded }` confirmation object.

---

### removeLabelsTool

Remove labels from an issue.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | The issue ID |
| `labels` | `string[]` | Yes | Labels to remove |

**Returns:** `{ id, labelsRemoved }` confirmation object.

---

## Review Tools

### submitReviewTool

Submit a structured code review for a pull request. The review includes per-file comments, an overall summary, a review decision, and suggested labels.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `prNumber` | `number` | Yes | The pull request number |
| `review.summary` | `string` | Yes | Overall review summary |
| `review.status` | `'approve' \| 'request_changes' \| 'comment'` | Yes | Review decision |
| `review.comments` | `CodeReviewComment[]` | Yes | Individual file comments |
| `review.impact` | `'low' \| 'medium' \| 'high' \| 'critical'` | Yes | Estimated impact of changes |
| `review.suggestedLabels` | `string[]` | Yes | Labels suggested based on code changes |

**CodeReviewComment:**

```typescript
interface CodeReviewComment {
  file: string;           // File path
  line?: number;          // Line number (optional)
  content: string;        // Review comment text
  type: 'suggestion' | 'issue' | 'question' | 'praise';
  severity?: 'low' | 'medium' | 'high';
}
```

```typescript
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { submitReviewTool } from '@jbcom/agentic-triage';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: { submitReview: submitReviewTool },
  prompt: `Review PR #42. The diff adds a new authentication middleware.
           Check for security issues, error handling, and test coverage.`,
  maxSteps: 5,
});
```

---

## PR Analysis Tools

### analyzePRTool

Submit a structured analysis of a pull request covering scope, risk, testing, and breaking changes.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `prNumber` | `number` | Yes | The pull request number |
| `analysis.title` | `string` | Yes | Suggested optimized PR title |
| `analysis.summary` | `string` | Yes | Executive summary of changes |
| `analysis.scope` | `'minor' \| 'major' \| 'patch' \| 'breaking'` | Yes | Impact scope |
| `analysis.riskLevel` | `'low' \| 'medium' \| 'high'` | Yes | Risk level of merging |
| `analysis.testingCoverage` | `'none' \| 'partial' \| 'full'` | Yes | Testing assessment |
| `analysis.breakingChanges` | `string[]` | Yes | List of breaking changes |
| `analysis.relatedIssues` | `string[]` | Yes | Related issue IDs or URLs |

The `PRAnalysisSchema` validates the analysis:

```typescript
const PRAnalysisSchema = z.object({
  title: z.string(),
  summary: z.string(),
  scope: z.enum(['minor', 'major', 'patch', 'breaking']),
  riskLevel: z.enum(['low', 'medium', 'high']),
  testingCoverage: z.enum(['none', 'partial', 'full']),
  breakingChanges: z.array(z.string()),
  relatedIssues: z.array(z.string()),
});
```

---

## Sage Tool

### sageTool

Ask Sage for technical advice, task decomposition, or agent routing based on repository context. Sage uses the configured AI model to provide intelligent recommendations.

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | `string` | Yes | The question or request for Sage |
| `context` | `object` | No | Additional repository context |
| `context.repoStructure` | `string` | No | Repository file structure |
| `context.keyFiles` | `Record<string, string>` | No | Contents of key files (filename to content mapping) |
| `context.issueContext` | `string` | No | Context from a GitHub issue or PR |
| `context.currentContext` | `string` | No | Current working context |

Sage automatically resolves the AI model using the triage configuration. It uses the `createTool` helper (which integrates with the escalation system) rather than the standard `tool()` function.

```typescript
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { sageTool } from '@jbcom/agentic-triage';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: { sage: sageTool },
  prompt: 'Should I refactor the auth module into separate files, or keep it monolithic?',
  maxSteps: 3,
});
```

---

## Visual Review Tool

### visualReviewTool

Perform a visual review of a web page using Playwright for screenshot capture and AI for image analysis.

**Input Schema:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `url` | `string` | Yes | -- | URL of the page to review |
| `scenario` | `string` | No | -- | Description of the test scenario |
| `viewport.width` | `number` | No | `1280` | Viewport width in pixels |
| `viewport.height` | `number` | No | `720` | Viewport height in pixels |

**Returns:**

```typescript
{
  url: string;
  status: 'success' | 'error';
  screenshotTaken?: boolean;
  analysis?: string;
  message?: string;  // Error message if status is 'error'
}
```

This tool launches a Chromium browser via Playwright, navigates to the URL, waits for `networkidle`, and captures a full-page screenshot. The Anthropic model is used for image analysis.

**Requirements:** `@playwright/test` must be installed and browsers must be available (`npx playwright install chromium`).

```typescript
import { visualReviewTool } from '@jbcom/agentic-triage';

// Example usage in AI pipeline:
// "Take a screenshot of our staging site and check for visual regressions"
```

---

## Provider Configuration

The triage tools use `TriageConnectors` internally, which supports multiple issue tracking providers:

```typescript
import { TriageConnectors } from '@jbcom/agentic-triage';

// GitHub (default -- auto-detected from .git)
const github = new TriageConnectors();

// Jira
const jira = new TriageConnectors({
  provider: 'jira',
  jira: {
    host: 'company.atlassian.net',
    projectKey: 'PROJ',
  },
});

// Linear
const linear = new TriageConnectors({
  provider: 'linear',
  linear: { teamId: 'TEAM123' },
});

// Beads (local-first)
const beads = new TriageConnectors({
  provider: 'beads',
  beads: { workingDir: '/path/to/project' },
});
```

---

## Types

### Issue

```typescript
interface Issue {
  id: string;
  title: string;
  description?: string;
  status: 'open' | 'in_progress' | 'blocked' | 'closed';
  priority: 'critical' | 'high' | 'medium' | 'low' | 'backlog';
  type: 'bug' | 'feature' | 'task' | 'epic' | 'chore' | 'docs';
  labels: string[];
  assignee?: string;
  createdAt: string;
  updatedAt: string;
}
```

### IssueTriage

```typescript
interface IssueTriage {
  title: string;
  summary: string;
  type: 'bug' | 'feature' | 'task' | 'epic' | 'chore' | 'docs';
  priority: 'critical' | 'high' | 'medium' | 'low' | 'backlog';
  labels: string[];
  estimate?: number;
  actionItems: string[];
}
```

### CodeReview

```typescript
interface CodeReview {
  summary: string;
  status: 'approve' | 'request_changes' | 'comment';
  comments: CodeReviewComment[];
  impact: 'low' | 'medium' | 'high' | 'critical';
  suggestedLabels: string[];
}
```

### PRAnalysis

```typescript
interface PRAnalysis {
  title: string;
  summary: string;
  scope: 'minor' | 'major' | 'patch' | 'breaking';
  riskLevel: 'low' | 'medium' | 'high';
  testingCoverage: 'none' | 'partial' | 'full';
  breakingChanges: string[];
  relatedIssues: string[];
}
```

---

## Related Pages

- [Fleet API Reference](/api/fleet-management/) -- Agent fleet management
- [Token Management API](/api/token-management/) -- Multi-org token routing
- [AI Triage Guide](/guides/ai-triage/) -- Usage guide with examples
- [Configuration API](/api/configuration/) -- Provider and model configuration
