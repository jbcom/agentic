---
title: Quick Start
description: Install Agentic and spawn your first AI agent in under 5 minutes
---

# Quick Start

Get from zero to a running AI agent in five steps.

## Prerequisites

- **Node.js 22+** (for @jbcom/agentic)
- **Python 3.10+** (for agentic-crew, optional)
- **Git** with at least one configured remote
- **GitHub Personal Access Token** with `repo` scope

## 1. Install

```bash
# Install the control package globally
npm install -g @jbcom/agentic

# Verify
agentic --version
```

For Python crew orchestration:

```bash
pip install agentic-crew[crewai]
```

## 2. Initialize

The `agentic init` command detects your environment — Git remotes, existing tokens, AI provider keys — and generates a config file:

```bash
agentic init
```

This creates `agentic.config.json`:

```json
{
  "tokens": {
    "organizations": {
      "my-org": {
        "name": "my-org",
        "tokenEnvVar": "GITHUB_MY_ORG_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN"
  },
  "defaultRepository": "my-org/my-repo",
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

## 3. Set Environment Variables

```bash
# Required: GitHub token
export GITHUB_TOKEN="ghp_xxx"

# Optional: org-specific tokens (auto-routed by config)
export GITHUB_MY_ORG_TOKEN="ghp_xxx"

# For AI triage features (pick one)
export ANTHROPIC_API_KEY="sk-ant-xxx"
# export OPENAI_API_KEY="sk-xxx"
# export GOOGLE_API_KEY="xxx"
```

Verify your tokens:

```bash
agentic tokens status
```

```
Token Status
─────────────────────────────────────────────────
✅ GITHUB_TOKEN           (default)          valid
✅ GITHUB_MY_ORG_TOKEN    (my-org)           valid
✅ ANTHROPIC_API_KEY      (triage provider)  valid
```

## 4. Spawn Your First Agent

```bash
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Review the README and add a Quick Start section" \
  --auto-pr
```

The agent will clone the repo, work on the task, and open a pull request.

Monitor progress:

```bash
# List running agents
agentic fleet list --running

# Get fleet summary
agentic fleet summary
```

## 5. Try AI Triage

For quick AI analysis without spawning a full agent:

```bash
# Quick triage
agentic triage quick "Error: Cannot find module 'express'"

# Code review
agentic triage review --base main --head HEAD
```

Or use the TypeScript API:

```typescript
import { getTriageTools } from '@jbcom/agentic/tools';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all open high-priority bugs',
});
```

## What's Next

You're up and running. Here's where to go from here:

- **[Configuration](/getting-started/configuration/)** — Multi-org tokens, fleet defaults, AI provider setup
- **[Agent Spawning](/guides/agent-spawning/)** — Advanced agent creation and monitoring
- **[Fleet Management](/guides/fleet-management/)** — Coordinate multiple agents
- **[Orchestration Patterns](/guides/orchestration-patterns/)** — Diamond patterns, handoffs, coordination loops
- **[Python Crews](/packages/crew/)** — Framework-agnostic crew orchestration
