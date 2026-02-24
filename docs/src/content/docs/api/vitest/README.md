---
editUrl: false
next: false
prev: false
title: "@jbcom/vitest-agentic"
---

Vitest plugin with fixtures and utilities for agentic-control E2E testing.

This package provides test utilities and mocking fixtures for building
E2E tests with agentic-control components (MCP, providers, sandbox, fleet).

## Examples

```bash
pnpm add -D vitest-agentic-control
```

```typescript
import { describe, it, expect } from 'vitest';
import { createMcpMocker, createProviderMocker } from 'vitest-agentic-control';

describe('My MCP Tests', () => {
  it('should mock MCP server', async () => {
    const mocker = createMcpMocker();
    mocker.mockServer('test-server', {
      tools: [{ name: 'test-tool', handler: () => ({ result: 'ok' }) }],
    });

    // Your test code here
  });
});
```

```typescript
import { createProviderMocker } from 'vitest-agentic-control';

const mocker = createProviderMocker();
mocker.mockAnthropic({ response: 'Mocked Claude response' });
mocker.mockOpenAI({ response: 'Mocked GPT response' });
```

Available exports:
  - `AgenticMocker` - Main mocker class for comprehensive mocking
  - `createMcpMocker` - Factory for MCP server mocking
  - `createProviderMocker` - Factory for AI provider mocking
  - `createSandboxMocker` - Factory for sandbox execution mocking
  - Test fixtures for configs and environment setup

## Classes

- [AgenticMocker](/api/vitest/classes/agenticmocker/)
- [McpMocker](/api/vitest/classes/mcpmocker/)
- [MockMcpServer](/api/vitest/classes/mockmcpserver/)
- [ProviderMocker](/api/vitest/classes/providermocker/)
- [SandboxMocker](/api/vitest/classes/sandboxmocker/)

## Interfaces

- [McpMockerOptions](/api/vitest/interfaces/mcpmockeroptions/)
- [MockContainerConfig](/api/vitest/interfaces/mockcontainerconfig/)
- [MockExecutionResult](/api/vitest/interfaces/mockexecutionresult/)
- [MockMcpResource](/api/vitest/interfaces/mockmcpresource/)
- [MockMcpTool](/api/vitest/interfaces/mockmcptool/)
- [MockProviderResponse](/api/vitest/interfaces/mockproviderresponse/)
- [MockResourceDefinition](/api/vitest/interfaces/mockresourcedefinition/)
- [MockStreamChunk](/api/vitest/interfaces/mockstreamchunk/)
- [MockToolDefinition](/api/vitest/interfaces/mocktooldefinition/)
- [ProviderMockerOptions](/api/vitest/interfaces/providermockeroptions/)
- [SandboxMockerOptions](/api/vitest/interfaces/sandboxmockeroptions/)
- [TestConfig](/api/vitest/interfaces/testconfig/)
- [TestConfigOptions](/api/vitest/interfaces/testconfigoptions/)
- [TestEnvSetup](/api/vitest/interfaces/testenvsetup/)
- [TestFleetConfig](/api/vitest/interfaces/testfleetconfig/)
- [TestSandboxConfig](/api/vitest/interfaces/testsandboxconfig/)
- [TestTokenConfig](/api/vitest/interfaces/testtokenconfig/)
- [TestTriageConfig](/api/vitest/interfaces/testtriageconfig/)

## Variables

- [DEFAULT\_TEST\_ENV](/api/vitest/variables/default_test_env/)
- [SUPPORTED\_PROVIDERS](/api/vitest/variables/supported_providers/)
- [VERSION](/api/vitest/variables/version/)

## Functions

- [createAgenticMocker](/api/vitest/functions/createagenticmocker/)
- [createFleetConfig](/api/vitest/functions/createfleetconfig/)
- [createMcpMocker](/api/vitest/functions/createmcpmocker/)
- [createMockAgentConfig](/api/vitest/functions/createmockagentconfig/)
- [createMockCrewConfig](/api/vitest/functions/createmockcrewconfig/)
- [createMockGitHubIssue](/api/vitest/functions/createmockgithubissue/)
- [createMockGitHubPR](/api/vitest/functions/createmockgithubpr/)
- [createMockTaskConfig](/api/vitest/functions/createmocktaskconfig/)
- [createProviderMocker](/api/vitest/functions/createprovidermocker/)
- [createSandboxConfig](/api/vitest/functions/createsandboxconfig/)
- [createSandboxMocker](/api/vitest/functions/createsandboxmocker/)
- [createTestConfig](/api/vitest/functions/createtestconfig/)
- [createTokenConfig](/api/vitest/functions/createtokenconfig/)
- [createTriageConfig](/api/vitest/functions/createtriageconfig/)
- [withTestEnv](/api/vitest/functions/withtestenv/)
