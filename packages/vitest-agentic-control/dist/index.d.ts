export { DEFAULT_TEST_ENV, TestConfig, TestConfigOptions, TestEnvSetup, TestFleetConfig, TestSandboxConfig, TestTokenConfig, TestTriageConfig, createFleetConfig, createMockAgentConfig, createMockCrewConfig, createMockGitHubIssue, createMockGitHubPR, createMockTaskConfig, createSandboxConfig, createTestConfig, createTokenConfig, createTriageConfig, withTestEnv } from './fixtures.js';
export { McpMocker, McpMockerOptions, MockMcpResource, MockMcpServer, MockMcpTool, MockResourceDefinition, MockToolDefinition, createMcpMocker } from './mcp.js';
export { AgenticMocker, createAgenticMocker } from './mocking.js';
export { MockProviderResponse, MockStreamChunk, ProviderMocker, ProviderMockerOptions, SUPPORTED_PROVIDERS, createProviderMocker } from './providers.js';
export { MockContainerConfig, MockExecutionResult, SandboxMocker, SandboxMockerOptions, createSandboxMocker } from './sandbox.js';
import 'vitest';

/**
 * Vitest plugin with fixtures and utilities for agentic-control E2E testing.
 *
 * This package provides test utilities and mocking fixtures for building
 * E2E tests with agentic-control components (MCP, providers, sandbox, fleet).
 *
 * @packageDocumentation
 *
 * @example Installation
 * ```bash
 * pnpm add -D vitest-agentic-control
 * ```
 *
 * @example Basic Usage
 * ```typescript
 * import { describe, it, expect } from 'vitest';
 * import { createMcpMocker, createProviderMocker } from 'vitest-agentic-control';
 *
 * describe('My MCP Tests', () => {
 *   it('should mock MCP server', async () => {
 *     const mocker = createMcpMocker();
 *     mocker.mockServer('test-server', {
 *       tools: [{ name: 'test-tool', handler: () => ({ result: 'ok' }) }],
 *     });
 *
 *     // Your test code here
 *   });
 * });
 * ```
 *
 * @example Provider Mocking
 * ```typescript
 * import { createProviderMocker } from 'vitest-agentic-control';
 *
 * const mocker = createProviderMocker();
 * mocker.mockAnthropic({ response: 'Mocked Claude response' });
 * mocker.mockOpenAI({ response: 'Mocked GPT response' });
 * ```
 *
 * Available exports:
 *   - `AgenticMocker` - Main mocker class for comprehensive mocking
 *   - `createMcpMocker` - Factory for MCP server mocking
 *   - `createProviderMocker` - Factory for AI provider mocking
 *   - `createSandboxMocker` - Factory for sandbox execution mocking
 *   - Test fixtures for configs and environment setup
 */

declare const VERSION = "1.0.0";

export { VERSION };
