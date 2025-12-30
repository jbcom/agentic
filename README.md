# @agentic-dev-library/control

**AI agent fleet management, orchestration, and automation toolkit.**

[![npm](https://img.shields.io/npm/v/@agentic-dev-library/control)](https://www.npmjs.com/package/@agentic-dev-library/control)
[![CI](https://github.com/agentic-dev-library/control/actions/workflows/ci.yml/badge.svg)](https://github.com/agentic-dev-library/control/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/agentic-dev-library/control/badge.svg?branch=main)](https://coveralls.io/github/agentic-dev-library/control?branch=main)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=agentic-dev-library_control&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=agentic-dev-library_control)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Control is the orchestration layer for AI agent fleets. It provides:

- **Fleet Management** - Spawn, monitor, and coordinate multiple AI agents
- **Sandboxed Execution** - Run untrusted code safely in Docker containers
- **Multi-Provider Support** - Claude, GPT-4, Gemini, Ollama, and more
- **GitHub Integration** - PR automation, issue triage, CI resolution
- **Cascade Routing** - Intelligent task routing to optimal agents

## Installation

```bash
npm install @agentic-dev-library/control
```

## Quick Start

```typescript
import { createFleet, cascade } from '@agentic-dev-library/control';

// Create an agent fleet
const fleet = await createFleet({
  agents: ['claude', 'cursor', 'jules'],
});

// Route a task to the optimal agent
const result = await cascade('Fix the failing tests', {
  context: { repo: 'my-org/my-repo', pr: 123 },
});
```

## Packages

| Package | Description |
|---------|-------------|
| `@agentic-dev-library/control` | Main CLI and runtime |
| `vitest-agentic-control` | Vitest plugin for E2E testing |

## GitHub Actions

```yaml
- uses: agentic-dev-library/control@v1
  with:
    command: fleet
    args: status
    github_token: ${{ secrets.GITHUB_TOKEN }}
```

## Documentation

- [API Reference](./docs/api.md)
- [Fleet Management](./docs/fleet.md)
- [Sandbox Execution](./docs/sandbox.md)
- [GitHub Integration](./docs/github.md)

## License

MIT © [Jon Bogaty](https://jonbogaty.com)
