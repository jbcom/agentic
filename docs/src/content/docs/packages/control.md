---
title: "@agentic/control"
description: Unified AI agent fleet management, triage, and orchestration toolkit
---

# @agentic/control

> 🚀 **Unified AI agent fleet management, triage, and orchestration toolkit**

[![npm version](https://badge.fury.io/js/@agentic-dev-library/control.svg)](https://www.npmjs.com/package/@agentic-dev-library/control)
[![Docker Pulls](https://img.shields.io/docker/pulls/jbcom/agentic-control)](https://hub.docker.com/r/jbcom/agentic-control)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](https://www.typescriptlang.org/)

**Transform your development workflow with AI-powered agent orchestration.** Spawn, coordinate, and manage fleets of AI agents across your repositories with intelligent token switching, advanced triage capabilities, and secure sandbox execution.

## ✨ What Makes @agentic/control Special?

| Feature | Description |
|---------|-------------|
| 🎯 **Smart Token Management** | Automatically routes operations to the right GitHub tokens based on organization |
| 🚀 **Fleet Orchestration** | Spawn and coordinate multiple Cursor Background Agents simultaneously |
| 🔍 **AI-Powered Triage** | Analyze conversations, review code, and extract actionable insights |
| 🏗️ **Sandbox Execution** | Run AI agents in isolated Docker containers for safe local development |
| 🤝 **Seamless Handoffs** | Transfer work between agents with full context preservation |
| 🔐 **Security First** | Token sanitization, safe subprocess execution, and zero hardcoded credentials |
| 🔌 **Provider Agnostic** | Works with Anthropic, OpenAI, Google, Mistral, and Azure |

## Installation

### npm/pnpm (Recommended)

```bash
# Install globally
pnpm add -g @agentic-dev-library/control
# or
npm install -g @agentic-dev-library/control

# Verify installation
agentic --version
```

### Docker

```bash
# Pull the latest image
docker pull jbcom/agentic-control:latest

# Run with your environment
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -e ANTHROPIC_API_KEY=$ANTHROPIC_API_KEY \
  -v $(pwd):/workspace \
  jbcom/agentic-control:latest fleet list
```

### Installing AI Providers

AI triage features require installing a provider SDK:

```bash
# Anthropic (recommended)
pnpm add @ai-sdk/anthropic

# OpenAI
pnpm add @ai-sdk/openai

# Google AI
pnpm add @ai-sdk/google

# Mistral
pnpm add @ai-sdk/mistral

# Azure OpenAI
pnpm add @ai-sdk/azure
```

## CLI Commands

### Configuration

```bash
# Initialize configuration (interactive)
agentic init

# Non-interactive initialization
agentic init --non-interactive
```

### Token Management

```bash
# Check all token status
agentic tokens status

# Validate required tokens
agentic tokens validate

# Show token for a specific repo
agentic tokens for-repo my-org/my-repo
```

### Fleet Management

```bash
# List all agents
agentic fleet list

# List only running agents
agentic fleet list --running

# List available Cursor models
agentic fleet models

# Get fleet summary
agentic fleet summary

# Spawn a new agent
agentic fleet spawn <repo> <task>

# Spawn with options
agentic fleet spawn <repo> <task> \
  --ref feature-branch \
  --auto-pr \
  --branch my-branch

# Send followup message
agentic fleet followup <agent-id> "Status update?"

# Run coordination loop
agentic fleet coordinate --pr 123 --repo my-org/my-repo
```

### AI Triage

```bash
# Quick triage of text
agentic triage quick "Error in deployment pipeline"

# Review code changes
agentic triage review --base main --head HEAD

# Analyze agent conversation
agentic triage analyze <agent-id> -o report.md

# Create issues from analysis
agentic triage analyze <agent-id> --create-issues

# Use specific model (overrides config)
agentic triage analyze <agent-id> --model claude-opus-4-20250514
```

### Sandbox Execution

```bash
# Run a single AI agent in sandbox
agentic sandbox run "Analyze this codebase" \
  --runtime claude \
  --workspace . \
  --output ./analysis-results \
  --timeout 300

# Run multiple agents in parallel
agentic sandbox fleet \
  "Review authentication system" \
  "Analyze database queries" \
  "Check for security vulnerabilities" \
  --runtime claude \
  --workspace . \
  --output ./fleet-results

# With custom environment and resource limits
agentic sandbox run "Refactor the API layer" \
  --workspace ./src/api \
  --output ./refactor-results \
  --memory 2048 \
  --timeout 600 \
  --env "NODE_ENV=development,LOG_LEVEL=debug"
```

### Handoff Protocol

```bash
# Initiate handoff to successor
agentic handoff initiate <predecessor-id> \
  --pr 123 \
  --branch my-branch \
  --repo https://github.com/my-org/my-repo

# Confirm health as successor
agentic handoff confirm <predecessor-id>

# Take over from predecessor
agentic handoff takeover <predecessor-id> 123 my-new-branch
```

## Programmatic Usage

### Fleet Management

```typescript
import { Fleet, GitHubClient, getTokenForRepo } from '@agentic-dev-library/control';

// Initialize Fleet manager
const fleet = new Fleet();

// Spawn an agent
const result = await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Fix the failing CI workflow',
  ref: 'main',
  target: {
    autoCreatePr: true,
    branchName: 'fix/ci-workflow',
  },
});

if (result.success && result.data) {
  console.log(`Agent spawned: ${result.data.id}`);
  
  // Wait for completion
  const finalResult = await fleet.waitFor(result.data.id, {
    timeout: 600000, // 10 minutes
    pollInterval: 15000, // 15 seconds
  });
  
  console.log(`Final status: ${finalResult.data?.status}`);
}

// List all agents
const agents = await fleet.list();

// Get running agents only
const running = await fleet.running();

// Broadcast message to multiple agents
const broadcastResults = await fleet.broadcast(
  ['agent-1', 'agent-2'],
  'Please provide a status update.'
);
```

### Token-Aware Operations

```typescript
import { 
  getTokenForRepo,
  setTokenConfig,
  addOrganization,
} from '@agentic-dev-library/control';

// Configure organizations
addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
});

// Get token for a specific repo (auto-routed)
const token = getTokenForRepo('my-company/my-repo');
// Returns value of GITHUB_COMPANY_TOKEN

// Configure everything at once
setTokenConfig({
  organizations: {
    'my-company': { name: 'my-company', tokenEnvVar: 'GITHUB_COMPANY_TOKEN' },
    'partner-org': { name: 'partner-org', tokenEnvVar: 'PARTNER_GH_PAT' },
  },
  defaultTokenEnvVar: 'GITHUB_TOKEN',
  prReviewTokenEnvVar: 'GITHUB_BOT_TOKEN',
});
```

### AI Analysis

```typescript
import { AIAnalyzer } from '@agentic-dev-library/control';

// Use default provider (from config)
const analyzer = new AIAnalyzer({ repo: 'my-company/my-repo' });
const result = await analyzer.quickTriage('Error in deployment');

// Use specific provider
const openaiAnalyzer = new AIAnalyzer({ 
  repo: 'my-company/my-repo',
  provider: 'openai',
  model: 'gpt-4o',
  apiKey: process.env.OPENAI_API_KEY,
});
```

### Sandbox Execution

```typescript
import { SandboxExecutor } from '@agentic-dev-library/control';

const sandbox = new SandboxExecutor();

// Single execution
const result = await sandbox.execute({
  runtime: 'claude',
  workspace: './src',
  outputDir: './analysis',
  prompt: 'Analyze this code for performance bottlenecks',
  timeout: 300000, // 5 minutes
  memory: 1024, // 1GB
});

// Parallel fleet execution
const results = await sandbox.executeFleet([
  {
    runtime: 'claude',
    workspace: './frontend',
    outputDir: './frontend-analysis',
    prompt: 'Review React components for accessibility issues',
  },
  {
    runtime: 'cursor', 
    workspace: './backend',
    outputDir: './backend-analysis',
    prompt: 'Analyze API endpoints for security vulnerabilities',
  }
]);
```

## Real-World Use Cases

### Automated Code Maintenance

```bash
# Update dependencies across multiple repos
agentic fleet spawn "my-org/frontend" \
  "Update React to v18 and fix breaking changes" --auto-pr

agentic fleet spawn "my-org/backend" \
  "Update Node.js dependencies and fix vulnerabilities" --auto-pr

agentic fleet spawn "my-org/mobile" \
  "Update React Native and test on latest iOS" --auto-pr
```

### Security Auditing

```bash
# Run security analysis in isolated sandbox
agentic sandbox run \
  "Perform comprehensive security audit focusing on authentication, authorization, and data validation" \
  --workspace . --output ./security-audit --timeout 900
```

### Code Review Automation

```bash
# AI-powered code review for PRs
agentic triage review --base main --head feature/user-auth

# Auto-create follow-up issues
agentic triage analyze <agent-id> --create-issues
```

### Release Coordination

```bash
# Coordinate multiple agents for release preparation
agentic fleet coordinate --repo my-org/app --pr 200 \
  --agents docs-agent,test-agent,deploy-agent
```

## Architecture

```
agentic-control/
├── src/
│   ├── core/           # Types, tokens, config
│   │   ├── types.ts    # Shared type definitions
│   │   ├── tokens.ts   # Intelligent token switching
│   │   └── config.ts   # Configuration management
│   ├── fleet/          # Cursor agent fleet management
│   │   ├── fleet.ts    # High-level Fleet API
│   │   └── cursor-api.ts   # Direct Cursor API client
│   ├── triage/         # AI-powered analysis
│   │   └── analyzer.ts # Multi-provider AI analysis
│   ├── github/         # Token-aware GitHub ops
│   │   └── client.ts   # Multi-org GitHub client
│   ├── handoff/        # Agent continuity
│   │   └── manager.ts  # Handoff protocols
│   ├── cli.ts          # Command-line interface
│   └── index.ts        # Main exports
└── tests/
```

## Security

This package is designed with security in mind:

- **No hardcoded values** - All tokens and organizations are user-configured
- **Safe subprocess execution** - Uses `spawnSync` instead of shell interpolation
- **Token sanitization** - Tokens are never logged or exposed in error messages
- **ReDoS protection** - Regex patterns are designed to prevent denial of service
- **No credential patterns in docs** - We don't document third-party API key formats

### GitHub Actions SHA Pinning

All GitHub Actions are pinned to their full commit SHA instead of semantic version tags:

```yaml
# Secure
- uses: actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5 # v4.3.1

# Vulnerable to tag manipulation
- uses: actions/checkout@v4  # ⚠️ Avoid
```

### npm Trusted Publishing (OIDC)

Package publishing uses OpenID Connect authentication instead of long-lived tokens, eliminating token leakage risk and providing cryptographic proof of provenance.

## Related Packages

- **[@agentic/triage](/packages/triage/)** - Triage primitives consumed by control
- **[@agentic/crew](/packages/crew/)** - Framework-agnostic crew orchestration
- **[game-generator](/packages/game-generator/)** - AI-powered game generation

## Links

- [GitHub Repository](https://github.com/agentic-dev-library/control)
- [npm Package](https://www.npmjs.com/package/@agentic-dev-library/control)
- [Docker Hub](https://hub.docker.com/r/jbcom/agentic-control)
- [API Reference](/api/fleet-management/)
