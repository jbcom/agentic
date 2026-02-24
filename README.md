# Agentic

[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![npm: @jbcom/agentic](https://img.shields.io/npm/v/@jbcom/agentic.svg?label=@jbcom/agentic)](https://www.npmjs.com/package/@jbcom/agentic)
[![npm: @jbcom/agentic-triage](https://img.shields.io/npm/v/@jbcom/agentic-triage.svg?label=@jbcom/agentic-triage)](https://www.npmjs.com/package/@jbcom/agentic-triage)
[![PyPI: agentic-crew](https://img.shields.io/pypi/v/agentic-crew.svg?label=agentic-crew)](https://pypi.org/project/agentic-crew/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Polyglot AI agent orchestration toolkit.** TypeScript for fleet management and triage, Python for multi-framework crew orchestration, Rust for game generation.

## Documentation

**[agentic.coach](https://agentic.coach)** -- full documentation, guides, API reference, and integration tutorials.

- [Getting Started](https://agentic.coach/getting-started)
- [Fleet Management Guide](https://agentic.coach/guides/fleet-management/)
- [AI Triage Guide](https://agentic.coach/guides/ai-triage/)
- [Architecture Overview](https://agentic.coach/guides/architecture/)
- [Troubleshooting](https://agentic.coach/guides/troubleshooting/)

## Packages

| Package | Language | Description | Docs |
|---------|----------|-------------|------|
| [`@jbcom/agentic`](packages/agentic-control) | TypeScript | Fleet management, multi-agent routing, CI resolution, GitHub Actions | [Docs](https://agentic.coach/packages/control/) |
| [`@jbcom/agentic-triage`](packages/triage) | TypeScript | AI-powered issue triage, PR review, sprint planning (Vercel AI SDK + MCP) | [Docs](https://agentic.coach/packages/triage/) |
| [`agentic-crew`](packages/agentic-crew) | Python | Framework-agnostic crew orchestration (CrewAI, LangGraph, Strands) | [Docs](https://agentic.coach/packages/crew/) |
| [`@jbcom/agentic-meshy`](packages/meshy-content-generator) | TypeScript | Declarative Meshy 3D asset generation pipelines | [Docs](https://agentic.coach/packages/meshy-content-generator/) |
| [`@jbcom/agentic-providers`](packages/providers) | TypeScript | LLM provider implementations (Ollama, Jules, Cursor) | [Docs](https://agentic.coach/packages/control/) |
| [`game-generator`](packages/game-generator) | Rust | Visual-first vintage game generator with AI assistance | [Docs](https://agentic.coach/packages/game-generator/) |

### Testing Plugins

| Package | Language | Description | Docs |
|---------|----------|-------------|------|
| [`@jbcom/vitest-agentic`](packages/vitest-agentic-control) | TypeScript | Vitest fixtures and utilities for agentic E2E testing | [Docs](https://agentic.coach/packages/control/) |
| [`pytest-agentic-crew`](packages/pytest-agentic-crew) | Python | Pytest plugin with fixtures for agentic-crew E2E testing | [Docs](https://agentic.coach/packages/crew/) |

## Quick Start

### Fleet Management (TypeScript)

```bash
npm install @jbcom/agentic
```

```typescript
import { Fleet } from '@jbcom/agentic/fleet';

const fleet = new Fleet();

// Spawn an agent to fix CI, automatically open a PR
await fleet.spawn({
  repository: 'https://github.com/my-org/my-repo',
  task: 'Fix the failing GitHub Actions workflow',
  target: { autoCreatePr: true },
});

// List and coordinate running agents
const agents = await fleet.list();
```

### AI Triage (TypeScript)

```bash
npm install @jbcom/agentic-triage
```

```typescript
import { getTriageTools } from '@jbcom/agentic-triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all high-priority bugs and create a triage plan',
});
```

### Crew Orchestration (Python)

```bash
pip install agentic-crew[crewai]
```

```python
from agentic_crew import run_crew_auto, get_crew_config, discover_packages

# Discover and run a crew -- framework auto-detected
packages = discover_packages()
config = get_crew_config(packages["my-package"], "analyzer")
result = run_crew_auto(config, inputs={"code": "..."})
```

### CLI

```bash
# Fleet operations
agentic fleet spawn "https://github.com/org/repo" "Fix the bug" --auto-pr
agentic fleet list --running

# AI triage
agentic triage review --base main --head feature-branch
agentic triage analyze <agent-id> --create-issues

# Sandbox execution
agentic sandbox run "Analyze codebase for vulnerabilities" --workspace .

# Crew orchestration
agentic-crew run my-package analyzer --input "Review this code"
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

- **[agentic-issue-triage](actions/agentic-issue-triage)** -- AI-powered issue assessment and labeling
- **[agentic-pr-review](actions/agentic-pr-review)** -- Automated PR code review
- **[agentic-ci-resolution](actions/agentic-ci-resolution)** -- Automatic CI failure resolution
- **[agentic-orchestrator](actions/agentic-orchestrator)** -- Fleet coordination and status

## License

MIT
