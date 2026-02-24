---
title: "@agentic/triage"
description: Portable triage primitives for AI agents - Vercel AI SDK tools, MCP server, and direct API
---

# @agentic/triage

> Portable triage primitives for AI agents - Vercel AI SDK tools, MCP server, and direct TypeScript API

[![npm version](https://img.shields.io/npm/v/@agentic-dev-library/triage.svg)](https://www.npmjs.com/package/@agentic-dev-library/triage)
[![Coverage Status](https://coveralls.io/repos/github/agentic-dev-library/triage/badge.svg?branch=main)](https://coveralls.io/github/agentic-dev-library/triage?branch=main)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**@agentic/triage** provides reusable triage primitives for AI agent systems. It offers three integration patterns:

1. **Vercel AI SDK Tools** - Portable tools for any Vercel AI SDK application
2. **MCP Server** - Model Context Protocol server for Claude Desktop, Cursor, etc.
3. **Direct TypeScript API** - Programmatic access for non-AI use cases

## Installation

```bash
# npm
npm install @agentic-dev-library/triage

# pnpm
pnpm add @agentic-dev-library/triage
```

## Quick Start

### 1. Vercel AI SDK Tools (Recommended for AI Agents)

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all open high-priority bugs and create a triage plan',
});

console.log(result.text);
```

### 2. Selective Tool Import

Import only the tools your agent needs to minimize the tool space:

```typescript
import { 
  getIssueTools, 
  getReviewTools, 
  getProjectTools 
} from '@agentic-dev-library/triage';

// Only import what your agent needs
const myAgentTools = {
  ...getIssueTools(),    // Issue CRUD, search, labels
  ...getReviewTools(),   // PR review, comments, approval
};
```

### 3. Individual Tools

For maximum control, import individual tools:

```typescript
import {
  listIssuesTool,
  createIssueTool,
  getIssueTool,
  updateIssueTool,
  closeIssueTool,
  searchIssuesTool,
  addLabelsTool,
  removeLabelsTool,
} from '@agentic-dev-library/triage';

const tools = { 
  listIssues: listIssuesTool, 
  createIssue: createIssueTool 
};
```

### 4. MCP Server (For Claude Desktop, Cursor, etc.)

Add to your MCP configuration:

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@agentic-dev-library/triage", "mcp-server"]
    }
  }
}
```

### 5. Direct TypeScript API

For non-AI use cases or custom integrations:

```typescript
import { TriageConnectors } from '@agentic-dev-library/triage';

const triage = new TriageConnectors({ provider: 'github' });

// Issue operations
const issues = await triage.issues.list({ 
  status: 'open', 
  priority: 'high' 
});

const issue = await triage.issues.create({
  title: 'Fix login bug',
  body: 'Users cannot login with SSO',
  type: 'bug',
  priority: 'critical',
});

await triage.issues.addLabels(issue.id, ['needs-triage', 'auth']);
await triage.issues.close(issue.id, 'Fixed in PR #123');

// Project operations
const sprints = await triage.projects.getSprints();
const currentSprint = await triage.projects.getCurrentSprint();

// Review operations
const comments = await triage.reviews.getPRComments(144);
```

## Provider Support

| Provider | Status | Use Case |
|----------|--------|----------|
| **GitHub Issues** | ✅ Complete | GitHub-native projects |
| **Beads** | ✅ Complete | Local-first, AI-native issue tracking |
| **Jira** | ✅ Complete | Enterprise projects |
| **Linear** | ✅ Complete | Modern team workflows |

### Auto-Detection

The provider is auto-detected based on environment:
- `.beads/` directory present → Beads provider
- `GITHUB_REPOSITORY` set or `.git` remote → GitHub provider

### Explicit Configuration

```typescript
import { TriageConnectors } from '@agentic-dev-library/triage';

// GitHub
const github = new TriageConnectors({
  provider: 'github',
  github: { owner: 'myorg', repo: 'myrepo' }
});

// Beads
const beads = new TriageConnectors({
  provider: 'beads',
  beads: { workingDir: '/path/to/project' }
});

// Jira
const jira = new TriageConnectors({
  provider: 'jira',
  jira: { 
    host: 'mycompany.atlassian.net',
    projectKey: 'PROJ'
  }
});

// Linear
const linear = new TriageConnectors({
  provider: 'linear',
  linear: { teamId: 'TEAM123' }
});
```

## Available Tools

### Issue Tools (`getIssueTools()`)

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

### Review Tools (`getReviewTools()`)

| Tool | Description |
|------|-------------|
| `getPRComments` | Get comments on a PR |
| `addPRComment` | Add a comment to a PR |
| `approvePR` | Approve a pull request |
| `requestChanges` | Request changes on a PR |

### Project Tools (`getProjectTools()`)

| Tool | Description |
|------|-------------|
| `getSprints` | Get all sprints |
| `getCurrentSprint` | Get the current sprint |
| `getSprintIssues` | Get issues in a sprint |
| `moveToSprint` | Move issue to a sprint |

## Example: AI-Powered Triage Agent

```typescript
import { anthropic } from '@ai-sdk/anthropic';
import { generateText } from 'ai';
import { getTriageTools } from '@agentic-dev-library/triage';

async function triageAgent() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: `
      You are a triage agent. Please:
      1. List all open critical bugs
      2. For each bug, assess severity and add appropriate labels
      3. Create a summary report of your findings
    `,
  });

  console.log('Triage Complete:', result.text);
  return result;
}
```

## Example: Selective Tools for Code Review

```typescript
import { anthropic } from '@ai-sdk/anthropic';
import { generateText } from 'ai';
import { 
  listIssuesTool, 
  createIssueTool, 
  searchIssuesTool 
} from '@agentic-dev-library/triage';

async function codeReviewAgent() {
  const minimalTools = {
    listIssues: listIssuesTool,
    createIssue: createIssueTool,
    searchIssues: searchIssuesTool,
  };

  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: minimalTools,
    maxSteps: 5,
    prompt: 'List the open issues and create a summary report.',
  });

  console.log(result.text);
}
```

## CLI (Development & Testing)

The CLI is primarily for development and testing the primitives:

```bash
# Test issue assessment
triage assess 123

# Test PR review
triage review 144
```

## Environment Variables

```bash
# For GitHub provider
GH_TOKEN=ghp_xxx              # GitHub PAT with repo scope

# For AI operations (when using CLI)
ANTHROPIC_API_KEY=xxx         # Anthropic API key
OPENAI_API_KEY=xxx            # Or OpenAI API key
OLLAMA_API_KEY=xxx            # Ollama Cloud API key
```

## Migration from agentic-triage

This package was previously published as `agentic-triage` or `@agentic/triage`. Starting with v0.3.0, it has been consolidated as `@agentic-dev-library/triage`.

To migrate:

```typescript
// Old
import { getTriageTools } from '@agentic/triage';

// New
import { getTriageTools } from '@agentic-dev-library/triage';
```

## Integration with @agentic/control

@agentic/triage tools are consumed by [@agentic/control](/packages/control/) for AI-powered agent analysis:

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';

// Use in your agent configurations
const result = await generateText({
  model: yourModel,
  tools: {
    ...getTriageTools(),
    ...yourOtherTools,
  },
  prompt: 'Triage open issues...',
});
```

## Links

- [GitHub Repository](https://github.com/agentic-dev-library/triage)
- [npm Package](https://www.npmjs.com/package/@agentic-dev-library/triage)
- [MCP Server Guide](/integrations/mcp-server/)
- [Vercel AI SDK Integration](/integrations/vercel-ai-sdk/)
