---
title: Quick Start
description: Get up and running with Agentic in under 5 minutes
---

# Quick Start

This guide will get you up and running with Agentic in under 5 minutes. We'll install the core packages, configure authentication, and spawn your first AI agent.

## Prerequisites

- **Node.js 18+** (for @agentic-dev-library/control)
- **Python 3.10+** (for agentic-crew, optional)
- **Git** configured with at least one remote repository
- **GitHub Personal Access Token** with `repo` scope

## Installation

### Option 1: npm/pnpm (Recommended)

```bash
# Install the control package globally
pnpm add -g @agentic-dev-library/control
# or
npm install -g @agentic-dev-library/control

# Verify installation
agentic --version
```

### Option 2: Docker

```bash
# Pull the latest image (includes Python companion)
docker pull jbcom/agentic-control:latest

# Run with your environment
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -e ANTHROPIC_API_KEY=$ANTHROPIC_API_KEY \
  -v $(pwd):/workspace \
  jbcom/agentic-control:latest fleet list
```

### Option 3: From Source

```bash
git clone https://github.com/agentic-dev-library/control.git
cd control
pnpm install
pnpm run build
pnpm run agentic --help
```

## Initialize Configuration

The `agentic init` command intelligently detects your environment:

```bash
agentic init
```

This will:
- Detect your Git repository from `git remote`
- Scan for existing tokens in your environment (`GITHUB_*_TOKEN`, etc.)
- Interactively prompt for missing configuration
- Generate a working `agentic.config.json`

### Generated Configuration

```json
{
  "tokens": {
    "organizations": {
      "my-org": {
        "name": "my-org",
        "tokenEnvVar": "GITHUB_MY_ORG_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_TOKEN"
  },
  "defaultRepository": "my-org/my-repo",
  "fleet": {
    "autoCreatePr": false,
    "openAsCursorGithubApp": false
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

## Set Environment Variables

```bash
# Required: Default GitHub token
export GITHUB_TOKEN="ghp_xxx"

# Optional: Organization-specific tokens
export GITHUB_MY_ORG_TOKEN="ghp_xxx"

# For AI triage features
export ANTHROPIC_API_KEY="sk-xxx"
# Or for other providers:
# export OPENAI_API_KEY="sk-xxx"
# export GOOGLE_API_KEY="xxx"

# For fleet management
export CURSOR_API_KEY="xxx"
```

## Verify Setup

Check that your tokens are configured correctly:

```bash
agentic tokens status
```

Expected output:
```
Token Status
─────────────────────────────────────────────────
✅ GITHUB_TOKEN           (default)          valid
✅ GITHUB_MY_ORG_TOKEN    (my-org)           valid
✅ ANTHROPIC_API_KEY      (triage provider)  valid
✅ CURSOR_API_KEY         (fleet management) valid
```

## Your First Agent

### List Available Repositories

```bash
agentic fleet list
```

### Spawn an Agent

```bash
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Review the README and add a Quick Start section" \
  --auto-pr
```

The agent will:
1. Clone the repository
2. Work on the task you specified
3. Create a pull request with the changes

### Monitor Progress

```bash
# List running agents
agentic fleet list --running

# Get fleet summary
agentic fleet summary
```

### View Conversation

```bash
# Get the conversation from an agent
agentic fleet conversation bc-xxx-xxx

# Analyze with AI
agentic triage analyze bc-xxx-xxx -o report.md
```

## Quick Triage Example

For AI-powered code review without spawning a full agent:

```bash
# Quick triage of text
agentic triage quick "Error: Cannot find module 'express'"

# Review code changes
agentic triage review --base main --head HEAD
```

## Install AI Provider (for Triage)

To use triage features, install at least one AI provider:

```bash
# Anthropic (recommended)
pnpm add @ai-sdk/anthropic

# Or OpenAI
pnpm add @ai-sdk/openai

# Or Google AI
pnpm add @ai-sdk/google

# Or Mistral
pnpm add @ai-sdk/mistral
```

## Next Steps

You're now ready to explore the full power of Agentic!

- **[Configuration Guide](/getting-started/configuration/)** - Deep dive into configuration options
- **[Agent Spawning Guide](/guides/agent-spawning/)** - Master agent creation and monitoring
- **[Fleet Management](/guides/fleet-management/)** - Coordinate multiple agents
- **[Orchestration Patterns](/guides/orchestration-patterns/)** - Advanced multi-agent workflows
