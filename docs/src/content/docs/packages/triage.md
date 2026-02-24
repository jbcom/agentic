---
title: "@jbcom/agentic (Triage)"
description: Portable triage primitives — Vercel AI SDK tools, MCP server, and direct TypeScript API for issue management and PR review
---

# @jbcom/agentic — Triage

<div class="polyglot-bar">
  <span class="lang-badge lang-badge--ts">TypeScript</span>
</div>

> Composable triage tools built on the Vercel AI SDK. Issue management, PR review, sprint planning — for GitHub, Jira, Linear, and Beads.

[![npm version](https://img.shields.io/npm/v/@jbcom/agentic.svg)](https://www.npmjs.com/package/@jbcom/agentic)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Three Ways to Use It

1. **Vercel AI SDK Tools** — Drop triage tools into any `generateText()` or `streamText()` call
2. **MCP Server** — Expose tools to Claude Desktop, Cursor, or any MCP client
3. **Direct TypeScript API** — Programmatic access for scripts, CI, and custom integrations

## Installation

```bash
npm install @jbcom/agentic
```

## Quick Start

### Vercel AI SDK (Recommended)

```typescript
import { getTriageTools } from '@jbcom/agentic/tools';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  maxSteps: 10,
  prompt: 'List all open critical bugs and create a triage plan',
});
```

### Selective Import

Import only what your agent needs — smaller tool space means more focused behavior:

```typescript
import { getIssueTools, getReviewTools } from '@jbcom/agentic/tools';

const myAgentTools = {
  ...getIssueTools(),    // Issue CRUD, search, labels
  ...getReviewTools(),   // PR review, comments, approval
};
```

### MCP Server

Add to your Claude Desktop or Cursor MCP config:

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server"]
    }
  }
}
```

### Direct TypeScript API

For scripts and CI pipelines:

```typescript
import { TriageConnectors } from '@jbcom/agentic';

const triage = new TriageConnectors({ provider: 'github' });

const issues = await triage.issues.list({ status: 'open', priority: 'high' });
const issue = await triage.issues.create({
  title: 'Fix login bug',
  body: 'Users cannot login with SSO',
  type: 'bug',
  priority: 'critical',
});
await triage.issues.addLabels(issue.id, ['needs-triage', 'auth']);
```

## Provider Support

| Provider | Status | Use Case |
|----------|--------|----------|
| **GitHub Issues** | Complete | GitHub-native projects |
| **Beads** | Complete | Local-first, AI-native tracking |
| **Jira** | Complete | Enterprise projects |
| **Linear** | Complete | Modern team workflows |

Auto-detected from environment — `.beads/` directory uses Beads, `.git` remote uses GitHub. Or configure explicitly:

```typescript
const jira = new TriageConnectors({
  provider: 'jira',
  jira: { host: 'mycompany.atlassian.net', projectKey: 'PROJ' }
});

const linear = new TriageConnectors({
  provider: 'linear',
  linear: { teamId: 'TEAM123' }
});
```

## Available Tools

### Issue Tools — `getIssueTools()`

| Tool | Description |
|------|-------------|
| `listIssues` | List issues with filtering |
| `getIssue` | Get a specific issue by ID |
| `createIssue` | Create a new issue |
| `updateIssue` | Update an existing issue |
| `closeIssue` | Close an issue with reason |
| `searchIssues` | Search issues by query |
| `addLabels` | Add labels to an issue |
| `removeLabels` | Remove labels from an issue |

### Review Tools — `getReviewTools()`

| Tool | Description |
|------|-------------|
| `getPRComments` | Get comments on a PR |
| `addPRComment` | Add a comment to a PR |
| `approvePR` | Approve a pull request |
| `requestChanges` | Request changes on a PR |

### Project Tools — `getProjectTools()`

| Tool | Description |
|------|-------------|
| `getSprints` | Get all sprints |
| `getCurrentSprint` | Get the current sprint |
| `getSprintIssues` | Get issues in a sprint |
| `moveToSprint` | Move issue to a sprint |

## Example: Full Triage Agent

```typescript
import { anthropic } from '@ai-sdk/anthropic';
import { generateText } from 'ai';
import { getTriageTools } from '@jbcom/agentic/tools';

async function triageAgent() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: `
      You are a triage agent. Please:
      1. List all open critical bugs
      2. Assess severity and add appropriate labels
      3. Create a summary report
    `,
  });

  console.log('Triage Complete:', result.text);
}
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `GH_TOKEN` | GitHub PAT with repo scope |
| `ANTHROPIC_API_KEY` | For AI-powered triage |
| `OPENAI_API_KEY` | Alternative AI provider |

## Migration

Previously published as `@agentic-dev-library/triage`. To migrate:

```typescript
// Old
import { getTriageTools } from '@agentic-dev-library/triage';

// New
import { getTriageTools } from '@jbcom/agentic/tools';
```

## Related

- **[@jbcom/agentic (Control)](/packages/control/)** — Consumes triage tools for fleet analysis
- **[agentic-crew](/packages/crew/)** — Multi-agent workflows
- **[Vercel AI SDK Integration](/integrations/vercel-ai-sdk/)** — Detailed usage patterns
- **[MCP Server Guide](/integrations/mcp-server/)** — Claude Desktop setup

## Links

- [GitHub](https://github.com/jbcom/agentic/tree/main/packages/triage)
- [npm](https://www.npmjs.com/package/@jbcom/agentic)
