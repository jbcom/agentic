---
title: "@jbcom/agentic (Control)"
description: AI agent fleet management — token routing, fleet orchestration, sandbox execution, and handoff protocols
---

# @jbcom/agentic — Control

<div class="polyglot-bar">
  <span class="lang-badge lang-badge--ts">TypeScript</span>
</div>

> Spawn, coordinate, and manage fleets of AI agents with automatic token routing, sandboxed execution, and handoff protocols.

[![npm version](https://badge.fury.io/js/@jbcom/agentic.svg)](https://www.npmjs.com/package/@jbcom/agentic)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What It Does

@jbcom/agentic is the orchestration layer of the Agentic ecosystem. It handles the hard coordination problems that come up when you run multiple AI agents across multiple repositories and organizations:

| Capability | Description |
|-----------|-------------|
| **Token Routing** | Automatically selects the right GitHub token based on repository organization — no manual switching |
| **Fleet Orchestration** | Spawn, monitor, and coordinate multiple Cursor Background Agents in parallel |
| **Sandbox Execution** | Run AI agents in isolated Docker containers with resource limits |
| **Handoff Protocol** | Transfer work between agents with full context preservation |
| **AI Triage** | Analyze agent conversations and extract actionable insights |
| **Provider Agnostic** | Works with Anthropic, OpenAI, Google, Mistral, Azure, and Ollama |

## Installation

```bash
# Global CLI
npm install -g @jbcom/agentic

# As a library
npm install @jbcom/agentic
```

Install at least one AI provider for triage features:

```bash
npm install @ai-sdk/anthropic  # Recommended
# or: @ai-sdk/openai, @ai-sdk/google, @ai-sdk/mistral, @ai-sdk/azure
```

## CLI Reference

### Configuration

```bash
agentic init                     # Interactive setup
agentic init --non-interactive   # Auto-detect everything
```

### Token Management

```bash
agentic tokens status            # Show all token status
agentic tokens validate          # Validate required tokens
agentic tokens for-repo my-org/r # Show which token a repo uses
```

### Fleet Management

```bash
agentic fleet list               # List all agents
agentic fleet list --running     # Running agents only
agentic fleet summary            # Fleet overview
agentic fleet models             # Available Cursor models

# Spawn agents
agentic fleet spawn <repo> <task>
agentic fleet spawn <repo> <task> --auto-pr --branch fix/ci

# Communication
agentic fleet followup <agent-id> "Status update?"
agentic fleet coordinate --pr 123 --repo my-org/repo
```

### Sandbox

```bash
# Single agent
agentic sandbox run "Security audit" \
  --runtime claude --workspace . --timeout 300

# Parallel fleet
agentic sandbox fleet \
  "Review auth" "Analyze queries" "Check security" \
  --runtime claude --workspace . --output ./results
```

### Handoff

```bash
agentic handoff initiate <id> --pr 123 --branch my-branch
agentic handoff confirm <id>
agentic handoff takeover <id> 123 new-branch
```

## Programmatic API

### Fleet Management

```typescript
import { Fleet } from '@jbcom/agentic';

const fleet = new Fleet();

// Spawn an agent
const result = await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Fix the failing CI workflow',
  target: { autoCreatePr: true, branchName: 'fix/ci' },
});

// Monitor multiple agents
const results = await fleet.monitorAgents(agentIds, {
  pollInterval: 30000,
  onProgress: (statusMap) => {
    for (const [id, status] of statusMap) {
      console.log(`${id}: ${status}`);
    }
  },
});

// Broadcast to agents
await fleet.broadcast(['agent-1', 'agent-2'], 'Status update?');
```

### Token Routing

```typescript
import { getTokenForRepo, addOrganization } from '@jbcom/agentic';

addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
});

// Automatically returns GITHUB_COMPANY_TOKEN
const token = getTokenForRepo('my-company/my-repo');
```

### Sandbox Execution

```typescript
import { SandboxExecutor } from '@jbcom/agentic';

const sandbox = new SandboxExecutor();

const result = await sandbox.execute({
  runtime: 'claude',
  workspace: './src',
  outputDir: './analysis',
  prompt: 'Analyze for performance bottlenecks',
  timeout: 300000,
  memory: 1024,
});
```

## Architecture

```
src/
├── core/           # Types, tokens, config
│   ├── types.ts    # Shared type definitions
│   ├── tokens.ts   # Token routing logic
│   └── config.ts   # cosmiconfig-based config
├── fleet/          # Agent fleet management
│   ├── fleet.ts    # High-level Fleet API
│   └── cursor-api.ts
├── triage/         # AI analysis
│   └── analyzer.ts # Multi-provider analyzer
├── github/         # Token-aware GitHub client
├── handoff/        # Agent continuity protocols
├── sandbox/        # Docker execution
├── cli.ts          # Commander-based CLI
└── index.ts        # Public API exports
```

## Security

- **No hardcoded values** — All tokens configured via environment variables
- **Safe subprocess execution** — `spawnSync` instead of shell interpolation
- **Token sanitization** — Tokens never logged or exposed in error messages
- **SHA-pinned Actions** — All GitHub Actions pinned to full commit SHA
- **OIDC publishing** — npm publishing via OpenID Connect, no long-lived tokens
- **ReDoS protection** — Regex patterns designed to prevent denial of service

## Related Packages

- **[@jbcom/agentic (Triage)](/packages/triage/)** — Triage primitives consumed by control
- **[agentic-crew](/packages/crew/)** — Framework-agnostic crew orchestration
- **[@agentic/meshy](/packages/meshy-content-generator/)** — 3D asset pipelines
- **[game-generator](/packages/game-generator/)** — AI-powered game generation

## Links

- [GitHub](https://github.com/jbcom/agentic/tree/main/packages/agentic-control)
- [npm](https://www.npmjs.com/package/@jbcom/agentic)
- [API Reference](/api/fleet-management/)
