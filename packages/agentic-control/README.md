# @jbcom/agentic

[![npm version](https://img.shields.io/npm/v/@jbcom/agentic.svg)](https://www.npmjs.com/package/@jbcom/agentic)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Orchestration layer for AI agent fleet management -- multi-agent routing, CI resolution, sandbox execution, and GitHub Marketplace actions. Spawn, coordinate, and manage fleets of AI agents across repositories with intelligent token switching, advanced triage capabilities, and secure Docker-based sandbox execution.

[Full Documentation](https://agentic.coach) | [API Reference](https://agentic.coach/api/control/) | [Getting Started](https://agentic.coach/getting-started)

## Installation

```bash
npm install @jbcom/agentic
# or
pnpm add @jbcom/agentic
```

Install an AI provider SDK for triage features:

```bash
pnpm add @ai-sdk/anthropic   # Anthropic (recommended)
pnpm add @ai-sdk/openai      # OpenAI
pnpm add @ai-sdk/google      # Google AI
```

## Quick Start

### Fleet Management

```typescript
import { Fleet } from '@jbcom/agentic/fleet';

const fleet = new Fleet();

// Spawn an agent to work on a task
await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Fix the failing CI workflow',
  target: { autoCreatePr: true },
});

// List running agents
const agents = await fleet.list();
```

### AI-Powered Triage

```typescript
import { AIAnalyzer } from '@jbcom/agentic';

const analyzer = new AIAnalyzer({ repo: 'my-org/my-repo' });
const result = await analyzer.quickTriage('Error in deployment pipeline');
```

### Sandbox Execution

```typescript
import { SandboxExecutor } from '@jbcom/agentic/sandbox';

const sandbox = new SandboxExecutor();
const result = await sandbox.execute({
  runtime: 'claude',
  workspace: './src',
  outputDir: './analysis',
  prompt: 'Analyze this codebase for security vulnerabilities',
  timeout: 300000,
});
```

### CLI

```bash
# Initialize configuration
agentic init

# Spawn an agent
agentic fleet spawn "https://github.com/my-org/repo" "Fix the bug" --auto-pr

# Run sandbox analysis
agentic sandbox run "Review code for vulnerabilities" --workspace .

# AI-powered code review
agentic triage review --base main --head feature-branch
```

## Key Features

- **Fleet orchestration** -- spawn and coordinate multiple Cursor Background Agents simultaneously
- **Intelligent token switching** -- routes GitHub operations to the correct org-specific tokens automatically
- **Sandbox execution** -- run AI agents in isolated Docker containers with resource limits
- **AI triage and analysis** -- multi-provider support (Anthropic, OpenAI, Google, Mistral, Azure)
- **Station-to-station handoffs** -- transfer work between agents with full context preservation
- **GitHub Actions integration** -- four Marketplace actions for CI/CD automation
- **Provider agnostic** -- works with any Vercel AI SDK-compatible provider

## Exports

| Subpath | Description |
|---------|-------------|
| `@jbcom/agentic` | All exports (core, fleet, triage, sandbox, etc.) |
| `@jbcom/agentic/fleet` | Fleet management (`Fleet`, `CursorAPI`) |
| `@jbcom/agentic/orchestrators` | Multi-agent routing and orchestration |
| `@jbcom/agentic/pipelines` | CI resolution and PR lifecycle pipelines |
| `@jbcom/agentic/sandbox` | Docker-based sandbox execution |
| `@jbcom/agentic/github` | Token-aware GitHub operations |
| `@jbcom/agentic/handoff` | Agent continuity and handoff protocols |
| `@jbcom/agentic/core` | Types, config, tokens, validation |
| `@jbcom/agentic/actions` | GitHub Actions integration |
| `@jbcom/agentic/triage` | AI-powered analysis |

## Requirements

- Node.js >= 22.0.0
- Docker (for sandbox execution)
- At least one AI provider SDK installed

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation including:

- [Fleet Management Guide](https://agentic.coach/guides/fleet-management/)
- [Sandbox Execution Guide](https://agentic.coach/guides/sandbox-execution/)
- [AI Triage Guide](https://agentic.coach/guides/ai-triage/)
- [Configuration Reference](https://agentic.coach/api/configuration/)

## License

MIT
