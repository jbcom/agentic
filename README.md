# Agentic

[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![npm: @jbcom/agentic](https://img.shields.io/npm/v/@jbcom/agentic.svg?label=@jbcom/agentic)](https://www.npmjs.com/package/@jbcom/agentic)
[![PyPI: agentic-crew](https://img.shields.io/pypi/v/agentic-crew.svg?label=agentic-crew)](https://pypi.org/project/agentic-crew/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Polyglot AI agent orchestration toolkit.** TypeScript for fleet management and triage, Python for multi-framework crew orchestration, Rust for game generation.

[Documentation](https://agentic.coach) | [Getting Started](https://agentic.coach/getting-started)

## Packages

| Package | Language | Description |
|---------|----------|-------------|
| [`@jbcom/agentic`](packages/agentic-control) | TypeScript | Fleet management, multi-agent routing, CI resolution, GitHub Actions |
| [`@jbcom/agentic-triage`](packages/triage) | TypeScript | AI-powered issue triage, PR review, sprint planning (Vercel AI SDK + MCP) |
| [`agentic-crew`](packages/agentic-crew) | Python | Framework-agnostic crew orchestration (CrewAI, LangGraph, Strands) |
| [`@jbcom/agentic-meshy`](packages/meshy-content-generator) | TypeScript | Declarative Meshy 3D asset generation pipelines |
| [`@jbcom/agentic-providers`](packages/providers) | TypeScript | LLM provider implementations (Ollama, Jules, Cursor) |
| [`game-generator`](packages/game-generator) | Rust | Visual-first vintage game generator with AI assistance |

### Testing Plugins

| Package | Language | Description |
|---------|----------|-------------|
| [`@jbcom/vitest-agentic`](packages/vitest-agentic-control) | TypeScript | Vitest fixtures and utilities for agentic E2E testing |
| [`pytest-agentic-crew`](packages/pytest-agentic-crew) | Python | Pytest plugin with fixtures for agentic-crew E2E testing |

## Quick Start

### Fleet Management (TypeScript)

```bash
npm install @jbcom/agentic
```

```typescript
import { Fleet } from '@jbcom/agentic/fleet';

const fleet = new Fleet({ dailyBudget: 50 });
// Spawn agents, route tasks by complexity, coordinate across providers
```

### Crew Orchestration (Python)

```bash
pip install agentic-crew
```

```python
from agentic_crew import Crew

# Declare once, run on CrewAI, LangGraph, or Strands
crew = Crew.from_yaml("crew.yaml")
result = await crew.kickoff(task="Triage open issues")
```

### Triage Primitives (TypeScript)

```bash
npm install @jbcom/agentic-triage
```

```typescript
import { getTriageTools } from '@jbcom/agentic-triage';
import { generateText } from 'ai';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all high-priority bugs and create a triage plan',
});
```

## Monorepo Structure

```
agentic/
├── packages/
│   ├── agentic-control/          # @jbcom/agentic (TypeScript)
│   ├── triage/                   # @jbcom/agentic-triage (TypeScript)
│   ├── agentic-crew/             # agentic-crew (Python)
│   ├── meshy-content-generator/  # @jbcom/agentic-meshy (TypeScript)
│   ├── providers/                # @jbcom/agentic-providers (TypeScript)
│   ├── game-generator/           # game-generator (Rust)
│   ├── vitest-agentic-control/   # @jbcom/vitest-agentic (TypeScript)
│   └── pytest-agentic-crew/      # pytest-agentic-crew (Python)
├── actions/                      # GitHub Marketplace actions
├── docs/                         # Documentation site (Astro + Starlight)
└── scripts/                      # Ecosystem automation
```

## Development

### TypeScript

```bash
pnpm install
pnpm run build
pnpm run test
pnpm run check          # biome lint + format
```

### Python

```bash
uv sync --all-extras
uv run pytest
uvx ruff check --fix .
uvx ruff format .
```

### Rust

```bash
cargo check
cargo test
cargo clippy
cargo fmt
```

## GitHub Actions

Four GitHub Marketplace actions for CI/CD integration:

- **[agentic-issue-triage](actions/agentic-issue-triage)** - AI-powered issue assessment and labeling
- **[agentic-pr-review](actions/agentic-pr-review)** - Automated PR code review
- **[agentic-ci-resolution](actions/agentic-ci-resolution)** - Automatic CI failure resolution
- **[agentic-orchestrator](actions/agentic-orchestrator)** - Fleet coordination and status

## License

MIT
