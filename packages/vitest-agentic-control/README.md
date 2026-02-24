# @jbcom/vitest-agentic

[![npm version](https://img.shields.io/npm/v/@jbcom/vitest-agentic.svg)](https://www.npmjs.com/package/@jbcom/vitest-agentic)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Vitest plugin with fixtures and utilities for end-to-end testing of `@jbcom/agentic` components. Provides comprehensive mocking for MCP servers, AI providers (Anthropic, OpenAI, Google, Mistral, Azure, Ollama), Docker sandbox execution, and pre-configured test fixtures for tokens, fleet, and triage configurations.

[Full Documentation](https://agentic.coach) | [Package Docs](https://agentic.coach/packages/control/)

## Installation

```bash
pnpm add -D @jbcom/vitest-agentic
# or
npm install --save-dev @jbcom/vitest-agentic
```

## Quick Start

```typescript
import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { createAgenticMocker, withTestEnv, DEFAULT_TEST_ENV } from '@jbcom/vitest-agentic';

describe('My Agentic Tests', () => {
  let cleanup: () => void;
  let mocker: ReturnType<typeof createAgenticMocker>;

  beforeEach(() => {
    cleanup = withTestEnv(DEFAULT_TEST_ENV);
    mocker = createAgenticMocker();
  });

  afterEach(() => {
    mocker.restoreAll();
    cleanup();
  });

  it('should mock MCP server', async () => {
    const server = mocker.mcp.mockServer('test-server', {
      tools: [{ name: 'get_data', handler: () => ({ data: 'mocked' }) }],
    });

    await server.connect();
    const result = await server.callTool('get_data', {});
    expect(result).toEqual({ data: 'mocked' });
  });

  it('should mock AI provider', async () => {
    mocker.providers.mockAnthropic({ response: 'Hello from mocked Claude!' });
    // Your test code that uses Anthropic
  });

  it('should mock sandbox execution', async () => {
    mocker.sandbox.mockExecution({
      success: true,
      stdout: 'Task completed successfully',
      exitCode: 0,
    });
    // Your test code that uses SandboxExecutor
  });
});
```

## Key Features

- **MCP mocking** -- mock MCP servers, tools, and resources without real implementations
- **Provider mocking** -- mock all six AI providers with configurable responses, streaming, and errors
- **Sandbox mocking** -- mock Docker container execution with queued results
- **Test fixtures** -- pre-configured configs for tokens, fleet, triage, and sandbox
- **Environment helpers** -- `withTestEnv()` for safe setup and teardown of env vars
- **GitHub mocking** -- `createMockGitHubIssue()` and `createMockGitHubPR()` factories

## Exports

| Subpath | Description |
|---------|-------------|
| `@jbcom/vitest-agentic` | All exports |
| `@jbcom/vitest-agentic/mocking` | `AgenticMocker`, `createAgenticMocker` |
| `@jbcom/vitest-agentic/mcp` | `McpMocker`, `createMcpMocker` |
| `@jbcom/vitest-agentic/providers` | `ProviderMocker`, `createProviderMocker` |
| `@jbcom/vitest-agentic/sandbox` | `SandboxMocker`, `createSandboxMocker` |
| `@jbcom/vitest-agentic/fixtures` | Test config factories, environment helpers |

## Requirements

- Node.js >= 22.0.0
- Vitest >= 1.0.0

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation.

## License

MIT
