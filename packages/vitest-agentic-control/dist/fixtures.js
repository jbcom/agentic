var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// src/fixtures.ts
function createTestConfig(options = {}) {
  const config = {
    logLevel: options.logLevel ?? "info"
  };
  if (options.tokens !== false) {
    config.tokens = createTokenConfig();
  }
  if (options.fleet) {
    config.fleet = createFleetConfig();
  }
  if (options.triage) {
    config.triage = createTriageConfig();
  }
  if (options.sandbox) {
    config.sandbox = createSandboxConfig();
  }
  return config;
}
__name(createTestConfig, "createTestConfig");
function createTokenConfig(overrides = {}) {
  return {
    organizations: overrides.organizations ?? {
      "test-org": {
        name: "test-org",
        tokenEnvVar: "TEST_ORG_TOKEN"
      },
      "another-org": {
        name: "another-org",
        tokenEnvVar: "ANOTHER_ORG_TOKEN"
      }
    },
    defaultTokenEnvVar: overrides.defaultTokenEnvVar ?? "GITHUB_TOKEN",
    prReviewTokenEnvVar: overrides.prReviewTokenEnvVar ?? "GITHUB_TOKEN"
  };
}
__name(createTokenConfig, "createTokenConfig");
function createFleetConfig(overrides = {}) {
  return {
    autoCreatePr: overrides.autoCreatePr ?? false,
    concurrency: overrides.concurrency ?? 3,
    timeout: overrides.timeout ?? 3e4
  };
}
__name(createFleetConfig, "createFleetConfig");
function createTriageConfig(overrides = {}) {
  return {
    provider: overrides.provider ?? "anthropic",
    model: overrides.model ?? "claude-sonnet-4-20250514",
    maxTokens: overrides.maxTokens ?? 4096,
    temperature: overrides.temperature ?? 0.7
  };
}
__name(createTriageConfig, "createTriageConfig");
function createSandboxConfig(overrides = {}) {
  return {
    runtime: overrides.runtime ?? "claude",
    image: overrides.image ?? "node:22-slim",
    memory: overrides.memory ?? 512,
    timeout: overrides.timeout ?? 3e4,
    workdir: overrides.workdir ?? "/workspace"
  };
}
__name(createSandboxConfig, "createSandboxConfig");
var DEFAULT_TEST_ENV = {
  GITHUB_TOKEN: "ghp_test_token_12345",
  ANTHROPIC_API_KEY: "sk-ant-test-key-12345",
  OPENAI_API_KEY: "sk-test-key-12345",
  TEST_ORG_TOKEN: "ghp_test_org_token_12345",
  ANOTHER_ORG_TOKEN: "ghp_another_org_token_12345",
  PR_REVIEW_TOKEN: "ghp_pr_review_token_12345"
};
function withTestEnv(env = DEFAULT_TEST_ENV) {
  const original = {};
  for (const [key, value] of Object.entries(env)) {
    original[key] = process.env[key];
    if (value !== void 0) {
      process.env[key] = value;
    } else {
      delete process.env[key];
    }
  }
  return () => {
    for (const [key, value] of Object.entries(original)) {
      if (value === void 0) {
        delete process.env[key];
      } else {
        process.env[key] = value;
      }
    }
  };
}
__name(withTestEnv, "withTestEnv");
function createMockAgentConfig() {
  return {
    name: "test-agent",
    role: "Test Agent",
    goal: "Complete test tasks accurately and efficiently",
    backstory: "You are a helpful test agent designed for testing purposes."
  };
}
__name(createMockAgentConfig, "createMockAgentConfig");
function createMockTaskConfig() {
  return {
    name: "test-task",
    description: "Complete the test task by following the instructions.",
    expectedOutput: "A successful completion message.",
    agent: "test-agent"
  };
}
__name(createMockTaskConfig, "createMockTaskConfig");
function createMockCrewConfig() {
  return {
    name: "test-crew",
    description: "A test crew for testing purposes",
    agents: {
      "test-agent": createMockAgentConfig()
    },
    tasks: {
      "test-task": createMockTaskConfig()
    }
  };
}
__name(createMockCrewConfig, "createMockCrewConfig");
function createMockGitHubIssue(overrides = {}) {
  return {
    number: overrides.number ?? 1,
    title: overrides.title ?? "Test Issue",
    body: overrides.body ?? "This is a test issue body.",
    state: overrides.state ?? "open",
    labels: (overrides.labels ?? ["bug"]).map((name) => ({ name })),
    user: { login: "test-user" },
    created_at: (/* @__PURE__ */ new Date()).toISOString(),
    updated_at: (/* @__PURE__ */ new Date()).toISOString()
  };
}
__name(createMockGitHubIssue, "createMockGitHubIssue");
function createMockGitHubPR(overrides = {}) {
  const state = overrides.state ?? "open";
  return {
    number: overrides.number ?? 1,
    title: overrides.title ?? "Test Pull Request",
    body: overrides.body ?? "This is a test PR body.",
    state: state === "merged" ? "closed" : state,
    merged: state === "merged",
    base: { ref: overrides.base ?? "main" },
    head: { ref: overrides.head ?? "feature/test" },
    labels: (overrides.labels ?? ["enhancement"]).map((name) => ({ name })),
    user: { login: "test-user" },
    created_at: (/* @__PURE__ */ new Date()).toISOString(),
    updated_at: (/* @__PURE__ */ new Date()).toISOString()
  };
}
__name(createMockGitHubPR, "createMockGitHubPR");

export { DEFAULT_TEST_ENV, createFleetConfig, createMockAgentConfig, createMockCrewConfig, createMockGitHubIssue, createMockGitHubPR, createMockTaskConfig, createSandboxConfig, createTestConfig, createTokenConfig, createTriageConfig, withTestEnv };
//# sourceMappingURL=fixtures.js.map
//# sourceMappingURL=fixtures.js.map