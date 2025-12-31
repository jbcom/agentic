/**
 * Test fixtures for agentic-control testing.
 *
 * This module provides pre-configured test fixtures for common
 * agentic-control configurations, making it easy to set up tests.
 *
 * @module fixtures
 */
/**
 * Options for creating test configurations.
 */
interface TestConfigOptions {
    /** Log level */
    logLevel?: 'debug' | 'info' | 'warn' | 'error';
    /** Whether to enable token configuration */
    tokens?: boolean;
    /** Whether to enable fleet configuration */
    fleet?: boolean;
    /** Whether to enable triage configuration */
    triage?: boolean;
    /** Whether to enable sandbox configuration */
    sandbox?: boolean;
}
/**
 * Token configuration for testing.
 */
interface TestTokenConfig {
    organizations: Record<string, {
        name: string;
        tokenEnvVar: string;
    }>;
    defaultTokenEnvVar: string;
    prReviewTokenEnvVar: string;
}
/**
 * Fleet configuration for testing.
 */
interface TestFleetConfig {
    autoCreatePr: boolean;
    concurrency: number;
    timeout: number;
}
/**
 * Triage configuration for testing.
 */
interface TestTriageConfig {
    provider: 'anthropic' | 'openai' | 'google' | 'mistral' | 'azure' | 'ollama';
    model: string;
    maxTokens: number;
    temperature: number;
}
/**
 * Sandbox configuration for testing.
 */
interface TestSandboxConfig {
    runtime: 'claude' | 'cursor' | 'custom';
    image: string;
    memory: number;
    timeout: number;
    workdir: string;
}
/**
 * Full test configuration.
 */
interface TestConfig {
    logLevel: 'debug' | 'info' | 'warn' | 'error';
    tokens?: TestTokenConfig;
    fleet?: TestFleetConfig;
    triage?: TestTriageConfig;
    sandbox?: TestSandboxConfig;
}
/**
 * Create a test configuration with sensible defaults.
 *
 * @param options - Configuration options
 * @returns A test configuration object
 *
 * @example
 * ```typescript
 * import { createTestConfig } from 'vitest-agentic-control';
 *
 * const config = createTestConfig({
 *   logLevel: 'debug',
 *   tokens: true,
 *   fleet: true,
 * });
 *
 * // Use in your tests
 * expect(config.tokens?.organizations).toBeDefined();
 * ```
 */
declare function createTestConfig(options?: TestConfigOptions): TestConfig;
/**
 * Create a token configuration for testing.
 *
 * @param overrides - Optional overrides
 * @returns Token configuration
 *
 * @example
 * ```typescript
 * import { createTokenConfig } from 'vitest-agentic-control';
 *
 * const tokens = createTokenConfig({
 *   organizations: {
 *     'my-org': { name: 'my-org', tokenEnvVar: 'MY_ORG_TOKEN' },
 *   },
 * });
 * ```
 */
declare function createTokenConfig(overrides?: Partial<TestTokenConfig>): TestTokenConfig;
/**
 * Create a fleet configuration for testing.
 *
 * @param overrides - Optional overrides
 * @returns Fleet configuration
 *
 * @example
 * ```typescript
 * import { createFleetConfig } from 'vitest-agentic-control';
 *
 * const fleet = createFleetConfig({
 *   concurrency: 5,
 *   timeout: 60000,
 * });
 * ```
 */
declare function createFleetConfig(overrides?: Partial<TestFleetConfig>): TestFleetConfig;
/**
 * Create a triage configuration for testing.
 *
 * @param overrides - Optional overrides
 * @returns Triage configuration
 *
 * @example
 * ```typescript
 * import { createTriageConfig } from 'vitest-agentic-control';
 *
 * const triage = createTriageConfig({
 *   provider: 'openai',
 *   model: 'gpt-4',
 * });
 * ```
 */
declare function createTriageConfig(overrides?: Partial<TestTriageConfig>): TestTriageConfig;
/**
 * Create a sandbox configuration for testing.
 *
 * @param overrides - Optional overrides
 * @returns Sandbox configuration
 *
 * @example
 * ```typescript
 * import { createSandboxConfig } from 'vitest-agentic-control';
 *
 * const sandbox = createSandboxConfig({
 *   runtime: 'cursor',
 *   memory: 1024,
 * });
 * ```
 */
declare function createSandboxConfig(overrides?: Partial<TestSandboxConfig>): TestSandboxConfig;
/**
 * Environment variable setup for testing.
 */
interface TestEnvSetup {
    /** GitHub token */
    GITHUB_TOKEN?: string;
    /** Anthropic API key */
    ANTHROPIC_API_KEY?: string;
    /** OpenAI API key */
    OPENAI_API_KEY?: string;
    /** Test organization token */
    TEST_ORG_TOKEN?: string;
    /** Another organization token */
    ANOTHER_ORG_TOKEN?: string;
    /** PR review token */
    PR_REVIEW_TOKEN?: string;
    /** Additional environment variables */
    [key: string]: string | undefined;
}
/**
 * Default test environment variables.
 */
declare const DEFAULT_TEST_ENV: TestEnvSetup;
/**
 * Set up test environment variables and return cleanup function.
 *
 * @param env - Environment variables to set
 * @returns Cleanup function to restore original values
 *
 * @example
 * ```typescript
 * import { withTestEnv, DEFAULT_TEST_ENV } from 'vitest-agentic-control';
 * import { beforeEach, afterEach } from 'vitest';
 *
 * describe('My Tests', () => {
 *   let cleanup: () => void;
 *
 *   beforeEach(() => {
 *     cleanup = withTestEnv(DEFAULT_TEST_ENV);
 *   });
 *
 *   afterEach(() => {
 *     cleanup();
 *   });
 *
 *   it('should use test tokens', () => {
 *     expect(process.env.GITHUB_TOKEN).toBe('ghp_test_token_12345');
 *   });
 * });
 * ```
 */
declare function withTestEnv(env?: TestEnvSetup): () => void;
/**
 * Create a mock agent configuration for testing.
 *
 * @returns Agent configuration
 */
declare function createMockAgentConfig(): {
    name: string;
    role: string;
    goal: string;
    backstory: string;
};
/**
 * Create a mock task configuration for testing.
 *
 * @returns Task configuration
 */
declare function createMockTaskConfig(): {
    name: string;
    description: string;
    expectedOutput: string;
    agent: string;
};
/**
 * Create a mock crew configuration for testing.
 *
 * @returns Crew configuration
 */
declare function createMockCrewConfig(): {
    name: string;
    description: string;
    agents: Record<string, unknown>;
    tasks: Record<string, unknown>;
};
/**
 * Create a mock GitHub issue for testing.
 *
 * @param overrides - Optional overrides
 * @returns GitHub issue object
 */
declare function createMockGitHubIssue(overrides?: Partial<{
    number: number;
    title: string;
    body: string;
    state: 'open' | 'closed';
    labels: string[];
}>): {
    number: number;
    title: string;
    body: string;
    state: 'open' | 'closed';
    labels: Array<{
        name: string;
    }>;
    user: {
        login: string;
    };
    created_at: string;
    updated_at: string;
};
/**
 * Create a mock GitHub pull request for testing.
 *
 * @param overrides - Optional overrides
 * @returns GitHub pull request object
 */
declare function createMockGitHubPR(overrides?: Partial<{
    number: number;
    title: string;
    body: string;
    state: 'open' | 'closed' | 'merged';
    base: string;
    head: string;
    labels: string[];
}>): {
    number: number;
    title: string;
    body: string;
    state: 'open' | 'closed';
    merged: boolean;
    base: {
        ref: string;
    };
    head: {
        ref: string;
    };
    labels: Array<{
        name: string;
    }>;
    user: {
        login: string;
    };
    created_at: string;
    updated_at: string;
};

export { DEFAULT_TEST_ENV, type TestConfig, type TestConfigOptions, type TestEnvSetup, type TestFleetConfig, type TestSandboxConfig, type TestTokenConfig, type TestTriageConfig, createFleetConfig, createMockAgentConfig, createMockCrewConfig, createMockGitHubIssue, createMockGitHubPR, createMockTaskConfig, createSandboxConfig, createTestConfig, createTokenConfig, createTriageConfig, withTestEnv };
