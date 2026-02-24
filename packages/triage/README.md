# @jbcom/agentic-triage

[![npm version](https://img.shields.io/npm/v/@jbcom/agentic-triage.svg)](https://www.npmjs.com/package/@jbcom/agentic-triage)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

AI-powered GitHub issue triage, PR review, and sprint planning primitives built on the Vercel AI SDK. Provides portable tools for any AI agent system, an MCP server for Claude Desktop and Cursor, and a direct TypeScript API for programmatic use. Supports GitHub, Linear, Jira, and Beads as triage providers.

[Full Documentation](https://agentic.coach) | [API Reference](https://agentic.coach/api/triage-tools/) | [Package Docs](https://agentic.coach/packages/triage/)

## Installation

```bash
npm install @jbcom/agentic-triage
# or
pnpm add @jbcom/agentic-triage
```

## Quick Start

### Vercel AI SDK Tools (Recommended)

```typescript
import { getTriageTools } from '@jbcom/agentic-triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all open high-priority bugs and create a triage plan',
});
```

### Selective Tool Import

```typescript
import { getIssueTools, getReviewTools, getProjectTools } from '@jbcom/agentic-triage';

// Only import what your agent needs
const tools = {
  ...getIssueTools(),    // Issue CRUD, search, labels
  ...getReviewTools(),   // PR review, comments, approval
  ...getProjectTools(),  // Sprint planning, backlog
};
```

### MCP Server (Claude Desktop, Cursor)

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@jbcom/agentic-triage", "mcp-server"]
    }
  }
}
```

### Direct TypeScript API

```typescript
import { TriageConnectors } from '@jbcom/agentic-triage';

const triage = new TriageConnectors({ provider: 'github' });
const issues = await triage.issues.list({ status: 'open', priority: 'high' });
```

## Key Features

- **Three integration patterns** -- Vercel AI SDK tools, MCP server, and direct API
- **Multi-provider support** -- GitHub Issues, Linear, Jira, and Beads
- **Issue triage** -- AI-powered assessment, labeling, prioritization
- **Code review** -- automated PR reviews with feedback handling
- **Sprint planning** -- weighted prioritization, backlog balancing
- **Escalation ladder** -- configurable cost-aware escalation policies
- **Queue management** -- priority-based task queuing with locking
- **Test result parsing** -- Vitest and Playwright reporter integration

## Triage Providers

| Provider | Status | Use Case |
|----------|--------|----------|
| **GitHub Issues** | Complete | GitHub-native projects |
| **Beads** | Complete | Local-first, AI-native issue tracking |
| **Jira** | Complete | Enterprise projects |
| **Linear** | Complete | Modern team workflows |

## Environment Variables

```bash
GH_TOKEN=ghp_xxx              # GitHub PAT with repo scope
OLLAMA_API_KEY=xxx             # Ollama Cloud API key (optional)
ANTHROPIC_API_KEY=xxx          # Anthropic API key (optional)
```

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation including:

- [AI Triage Guide](https://agentic.coach/guides/ai-triage/)
- [Vercel AI SDK Integration](https://agentic.coach/integrations/vercel-ai-sdk/)
- [MCP Server Integration](https://agentic.coach/integrations/mcp-server/)

## License

MIT
