import { vi } from 'vitest';

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
var MockMcpServer = class {
  static {
    __name(this, "MockMcpServer");
  }
  name;
  tools = /* @__PURE__ */ new Map();
  resources = /* @__PURE__ */ new Map();
  capabilities;
  connected = false;
  constructor(config) {
    this.name = config.name;
    this.capabilities = {
      tools: config.capabilities?.tools ?? true,
      resources: config.capabilities?.resources ?? true,
      prompts: config.capabilities?.prompts ?? false
    };
    for (const tool of config.tools ?? []) {
      this.registerTool(tool);
    }
    for (const resource of config.resources ?? []) {
      this.registerResource(resource);
    }
  }
  /**
   * Register a tool with this mock server.
   */
  registerTool(definition) {
    const mockTool = {
      definition,
      handler: vi.fn(definition.handler),
      calls: []
    };
    this.tools.set(definition.name, mockTool);
    return mockTool;
  }
  /**
   * Register a resource with this mock server.
   */
  registerResource(definition) {
    const mockResource = {
      definition,
      readCount: 0
    };
    this.resources.set(definition.uri, mockResource);
    return mockResource;
  }
  /**
   * Connect to this mock server.
   */
  async connect() {
    this.connected = true;
  }
  /**
   * Disconnect from this mock server.
   */
  async disconnect() {
    this.connected = false;
  }
  /**
   * Check if connected.
   */
  isConnected() {
    return this.connected;
  }
  /**
   * List available tools.
   */
  async listTools() {
    return Array.from(this.tools.values()).map((tool) => ({
      name: tool.definition.name,
      description: tool.definition.description,
      inputSchema: tool.definition.inputSchema
    }));
  }
  /**
   * Call a tool.
   */
  async callTool(name, args) {
    const tool = this.tools.get(name);
    if (!tool) {
      throw new Error(`Tool not found: ${name}`);
    }
    tool.calls.push(args);
    return tool.handler(args);
  }
  /**
   * List available resources.
   */
  async listResources() {
    return Array.from(this.resources.values()).map((resource) => ({
      uri: resource.definition.uri,
      name: resource.definition.name,
      description: resource.definition.description,
      mimeType: resource.definition.mimeType
    }));
  }
  /**
   * Read a resource.
   */
  async readResource(uri) {
    const resource = this.resources.get(uri);
    if (!resource) {
      throw new Error(`Resource not found: ${uri}`);
    }
    resource.readCount++;
    return {
      uri: resource.definition.uri,
      mimeType: resource.definition.mimeType,
      content: resource.definition.content
    };
  }
  /**
   * Reset all call history.
   */
  reset() {
    for (const tool of this.tools.values()) {
      tool.handler.mockClear();
      tool.calls = [];
    }
    for (const resource of this.resources.values()) {
      resource.readCount = 0;
    }
  }
};
var McpMocker = class {
  static {
    __name(this, "McpMocker");
  }
  /** Map of mock servers by name */
  servers = /* @__PURE__ */ new Map();
  /** Track mocked modules */
  mockedModules = /* @__PURE__ */ new Set();
  constructor(options = {}) {
    if (options.autoMock) {
      this.mockAllModules();
    }
  }
  /**
   * Create a mock MCP server.
   *
   * @param name - Server name
   * @param config - Server configuration
   * @returns The mock server
   */
  mockServer(name, config = {}) {
    const server = new MockMcpServer({ name, ...config });
    this.servers.set(name, server);
    return server;
  }
  /**
   * Get a mock server by name.
   *
   * @param name - Server name
   * @returns The mock server or undefined
   */
  getServer(name) {
    return this.servers.get(name);
  }
  /**
   * Mock the MCP client module.
   *
   * @returns Mock client factory
   */
  mockClient() {
    const mockClientFactory = vi.fn((serverName) => {
      const server = this.servers.get(serverName);
      if (!server) {
        throw new Error(`No mock server registered: ${serverName}`);
      }
      return server;
    });
    vi.doMock("@modelcontextprotocol/sdk/client", () => ({
      Client: mockClientFactory
    }));
    this.mockedModules.add("@modelcontextprotocol/sdk/client");
    return mockClientFactory;
  }
  /**
   * Mock all MCP-related modules.
   *
   * @returns Dictionary of mocked modules
   */
  mockAllModules() {
    const mocks = {};
    mocks["@modelcontextprotocol/sdk/client"] = {
      Client: vi.fn().mockImplementation((serverName) => {
        return this.servers.get(serverName) ?? new MockMcpServer({ name: serverName });
      })
    };
    mocks["@modelcontextprotocol/sdk/server"] = {
      Server: vi.fn().mockImplementation((config) => {
        return new MockMcpServer(config);
      })
    };
    mocks["@ai-sdk/mcp"] = {
      experimental_createMCPClient: vi.fn().mockImplementation(async (config) => {
        return this.servers.get(config.name) ?? new MockMcpServer({ name: config.name });
      })
    };
    for (const [modulePath, mock] of Object.entries(mocks)) {
      vi.doMock(modulePath, () => mock);
      this.mockedModules.add(modulePath);
    }
    return mocks;
  }
  /**
   * Create a mock tool that can be registered with servers.
   *
   * @param name - Tool name
   * @param handler - Tool handler
   * @param options - Additional options
   * @returns Tool definition
   */
  createMockTool(name, handler, options = {}) {
    return {
      name,
      description: options.description,
      inputSchema: options.inputSchema,
      handler
    };
  }
  /**
   * Create a mock resource that can be registered with servers.
   *
   * @param uri - Resource URI
   * @param content - Resource content
   * @param options - Additional options
   * @returns Resource definition
   */
  createMockResource(uri, content, options = {}) {
    return {
      uri,
      content,
      name: options.name,
      description: options.description,
      mimeType: options.mimeType
    };
  }
  /**
   * Restore all mocked modules.
   */
  restoreAll() {
    this.mockedModules.clear();
    this.servers.clear();
  }
  /**
   * Reset all mock servers.
   */
  resetAll() {
    for (const server of this.servers.values()) {
      server.reset();
    }
  }
};
function createMcpMocker(options = {}) {
  return new McpMocker(options);
}
__name(createMcpMocker, "createMcpMocker");
var SUPPORTED_PROVIDERS = [
  "anthropic",
  "openai",
  "google",
  "mistral",
  "azure",
  "ollama"
];
var PROVIDER_MODULES = {
  anthropic: "@ai-sdk/anthropic",
  openai: "@ai-sdk/openai",
  google: "@ai-sdk/google",
  mistral: "@ai-sdk/mistral",
  azure: "@ai-sdk/azure",
  ollama: "ai-sdk-ollama"
};
var ProviderMocker = class {
  static {
    __name(this, "ProviderMocker");
  }
  /** Default response configuration */
  defaultResponse;
  /** Track mocked modules */
  mockedModules = /* @__PURE__ */ new Set();
  /** Track provider-specific configurations */
  providerConfigs = /* @__PURE__ */ new Map();
  /** Track mock models */
  models = /* @__PURE__ */ new Map();
  constructor(options = {}) {
    this.defaultResponse = options.defaultResponse ?? {
      response: "This is a mock AI response."
    };
    if (options.autoMock) {
      this.mockAllModules();
    }
  }
  /**
   * Mock the Anthropic provider.
   *
   * @param config - Response configuration
   */
  mockAnthropic(config = {}) {
    this.providerConfigs.set("anthropic", { ...this.defaultResponse, ...config });
    this.mockProviderModule("anthropic");
  }
  /**
   * Mock the OpenAI provider.
   *
   * @param config - Response configuration
   */
  mockOpenAI(config = {}) {
    this.providerConfigs.set("openai", { ...this.defaultResponse, ...config });
    this.mockProviderModule("openai");
  }
  /**
   * Mock the Google provider.
   *
   * @param config - Response configuration
   */
  mockGoogle(config = {}) {
    this.providerConfigs.set("google", { ...this.defaultResponse, ...config });
    this.mockProviderModule("google");
  }
  /**
   * Mock the Mistral provider.
   *
   * @param config - Response configuration
   */
  mockMistral(config = {}) {
    this.providerConfigs.set("mistral", { ...this.defaultResponse, ...config });
    this.mockProviderModule("mistral");
  }
  /**
   * Mock the Azure provider.
   *
   * @param config - Response configuration
   */
  mockAzure(config = {}) {
    this.providerConfigs.set("azure", { ...this.defaultResponse, ...config });
    this.mockProviderModule("azure");
  }
  /**
   * Mock the Ollama provider.
   *
   * @param config - Response configuration
   */
  mockOllama(config = {}) {
    this.providerConfigs.set("ollama", { ...this.defaultResponse, ...config });
    this.mockProviderModule("ollama");
  }
  /**
   * Mock a specific provider module.
   *
   * @param provider - The provider to mock
   */
  mockProviderModule(provider) {
    const modulePath = PROVIDER_MODULES[provider];
    const config = this.providerConfigs.get(provider) ?? this.defaultResponse;
    const mockProvider = this.createMockProviderFactory(provider, config);
    vi.doMock(modulePath, () => mockProvider);
    this.mockedModules.add(modulePath);
  }
  /**
   * Create a mock provider factory function.
   *
   * @param provider - The provider type
   * @param config - Response configuration
   * @returns Mock provider factory
   */
  createMockProviderFactory(provider, config) {
    const createModel = /* @__PURE__ */ __name((modelId) => {
      const model = this.createMockModel(provider, modelId, config);
      return model;
    }, "createModel");
    switch (provider) {
      case "anthropic":
        return {
          anthropic: vi.fn().mockReturnValue({
            chat: createModel,
            messages: createModel
          }),
          createAnthropic: vi.fn().mockReturnValue({
            chat: createModel,
            messages: createModel
          })
        };
      case "openai":
        return {
          openai: vi.fn().mockReturnValue({
            chat: createModel,
            completion: createModel
          }),
          createOpenAI: vi.fn().mockReturnValue({
            chat: createModel,
            completion: createModel
          })
        };
      case "google":
        return {
          google: vi.fn().mockReturnValue({
            generativeAI: createModel
          }),
          createGoogle: vi.fn().mockReturnValue({
            generativeAI: createModel
          })
        };
      case "mistral":
        return {
          mistral: vi.fn().mockReturnValue({
            chat: createModel
          }),
          createMistral: vi.fn().mockReturnValue({
            chat: createModel
          })
        };
      case "azure":
        return {
          azure: vi.fn().mockReturnValue({
            chat: createModel
          }),
          createAzure: vi.fn().mockReturnValue({
            chat: createModel
          })
        };
      case "ollama":
        return {
          ollama: vi.fn().mockReturnValue({
            chat: createModel
          }),
          createOllama: vi.fn().mockReturnValue({
            chat: createModel
          })
        };
    }
  }
  /**
   * Create a mock model for a provider.
   *
   * @param provider - The provider type
   * @param modelId - The model ID
   * @param config - Response configuration
   * @returns Mock model instance
   */
  createMockModel(provider, modelId, config = {}) {
    const mergedConfig = { ...this.defaultResponse, ...config };
    const generate = vi.fn(async () => {
      if (mergedConfig.latency) {
        await new Promise((resolve) => setTimeout(resolve, mergedConfig.latency));
      }
      if (mergedConfig.error) {
        throw mergedConfig.error;
      }
      return {
        text: mergedConfig.response ?? "Mock response",
        toolCalls: mergedConfig.toolCalls ?? [],
        usage: mergedConfig.usage ?? {
          promptTokens: 10,
          completionTokens: 20,
          totalTokens: 30
        }
      };
    });
    const generateStream = vi.fn(async function* () {
      if (mergedConfig.latency) {
        await new Promise((resolve) => setTimeout(resolve, mergedConfig.latency));
      }
      if (mergedConfig.error) {
        throw mergedConfig.error;
      }
      const text = mergedConfig.response ?? "Mock response";
      const chunks = text.split(" ");
      for (const chunk of chunks) {
        yield { textDelta: `${chunk} ` };
      }
      yield { isFinished: true };
    });
    const model = {
      generate,
      generateStream,
      provider,
      modelId
    };
    this.models.set(`${provider}:${modelId}`, model);
    return model;
  }
  /**
   * Mock the core AI SDK module.
   */
  mockAiSdk() {
    vi.doMock("ai", () => ({
      generateText: vi.fn(async (options) => {
        return options.model.generate(options.prompt, options);
      }),
      streamText: vi.fn(async function* (options) {
        yield* options.model.generateStream(options.prompt, options);
      }),
      generateObject: vi.fn(async () => ({ object: {} })),
      streamObject: vi.fn(async function* () {
        yield { object: {} };
      })
    }));
    this.mockedModules.add("ai");
  }
  /**
   * Mock all provider modules.
   *
   * @returns Dictionary of mocked modules
   */
  mockAllModules() {
    const mocks = {};
    this.mockAiSdk();
    mocks.ai = true;
    for (const provider of SUPPORTED_PROVIDERS) {
      this.mockProviderModule(provider);
      mocks[PROVIDER_MODULES[provider]] = true;
    }
    return mocks;
  }
  /**
   * Set a response for a specific provider.
   *
   * @param provider - The provider
   * @param config - Response configuration
   */
  setProviderResponse(provider, config) {
    this.providerConfigs.set(provider, { ...this.defaultResponse, ...config });
  }
  /**
   * Get the configuration for a provider.
   *
   * @param provider - The provider
   * @returns The configuration or undefined
   */
  getProviderConfig(provider) {
    return this.providerConfigs.get(provider);
  }
  /**
   * Restore all mocked modules.
   */
  restoreAll() {
    this.mockedModules.clear();
    this.providerConfigs.clear();
    this.models.clear();
  }
  /**
   * Reset all mocks.
   */
  resetAll() {
    for (const model of this.models.values()) {
      model.generate.mockClear();
      model.generateStream.mockClear();
    }
  }
};
function createProviderMocker(options = {}) {
  return new ProviderMocker(options);
}
__name(createProviderMocker, "createProviderMocker");
var SandboxMocker = class {
  static {
    __name(this, "SandboxMocker");
  }
  /** Default container configuration */
  defaultConfig;
  /** Default execution result */
  defaultResult;
  /** Track mock containers */
  containers = /* @__PURE__ */ new Map();
  /** Track mocked modules */
  mockedModules = /* @__PURE__ */ new Set();
  /** Queue of results to return from executions */
  resultQueue = [];
  /** Counter for generating container IDs */
  containerIdCounter = 0;
  constructor(options = {}) {
    this.defaultConfig = options.defaultConfig ?? {
      image: "node:22-slim",
      workdir: "/workspace"
    };
    this.defaultResult = options.defaultResult ?? {
      success: true,
      exitCode: 0,
      stdout: "",
      stderr: ""
    };
    if (options.autoMock) {
      this.mockDockerCommands();
    }
  }
  /**
   * Set the default execution result.
   *
   * @param result - The result to return from executions
   */
  mockExecution(result) {
    this.defaultResult = result;
  }
  /**
   * Queue a result to be returned from the next execution.
   *
   * @param result - The result to queue
   */
  queueResult(result) {
    this.resultQueue.push(result);
  }
  /**
   * Queue multiple results to be returned from executions.
   *
   * @param results - The results to queue
   */
  queueResults(results) {
    this.resultQueue.push(...results);
  }
  /**
   * Get the next result from the queue or the default.
   */
  getNextResult() {
    return this.resultQueue.shift() ?? this.defaultResult;
  }
  /**
   * Create a mock container.
   *
   * @param config - Container configuration
   * @returns Mock container instance
   */
  createMockContainer(config = {}) {
    const mergedConfig = {
      ...this.defaultConfig,
      ...config,
      id: config.id ?? `mock-container-${++this.containerIdCounter}`,
      name: config.name ?? `mock-container-${this.containerIdCounter}`
    };
    const containerId = mergedConfig.id || "unknown";
    const container = {
      config: mergedConfig,
      status: "created",
      executions: [],
      start: vi.fn(async () => {
        container.status = "running";
      }),
      stop: vi.fn(async () => {
        container.status = "stopped";
      }),
      exec: vi.fn(async (command) => {
        const result = this.getNextResult();
        container.executions.push({
          command,
          result,
          timestamp: /* @__PURE__ */ new Date()
        });
        if (result.error) {
          throw result.error;
        }
        return result;
      }),
      remove: vi.fn(async () => {
        container.status = "removed";
        this.containers.delete(containerId);
      }),
      copyTo: vi.fn(async () => {
      }),
      copyFrom: vi.fn(async () => {
      })
    };
    this.containers.set(containerId, container);
    return container;
  }
  /**
   * Mock Docker CLI commands.
   */
  mockDockerCommands() {
    vi.doMock("child_process", () => ({
      spawn: vi.fn((command, args) => {
        const isDocker = command === "docker" || command.includes("docker");
        if (isDocker) {
          return this.createMockDockerProcess(args);
        }
        return {
          stdout: { on: vi.fn() },
          stderr: { on: vi.fn() },
          on: vi.fn((event, cb) => {
            if (event === "close") {
              cb(0);
            }
          }),
          kill: vi.fn()
        };
      }),
      spawnSync: vi.fn((command, _args) => {
        const isDocker = command === "docker" || command.includes("docker");
        if (isDocker) {
          const result = this.getNextResult();
          return {
            status: result.exitCode ?? 0,
            stdout: Buffer.from(result.stdout ?? ""),
            stderr: Buffer.from(result.stderr ?? ""),
            error: result.error
          };
        }
        return {
          status: 0,
          stdout: Buffer.from(""),
          stderr: Buffer.from("")
        };
      }),
      execSync: vi.fn((command) => {
        if (command.includes("docker")) {
          return this.defaultResult.stdout ?? "";
        }
        return "";
      })
    }));
    this.mockedModules.add("child_process");
  }
  /**
   * Create a mock Docker process.
   */
  createMockDockerProcess(args) {
    const result = this.getNextResult();
    const subcommand = args[0];
    const process2 = {
      stdout: {
        on: vi.fn((event, cb) => {
          if (event === "data" && result.stdout) {
            cb(Buffer.from(result.stdout));
          }
        })
      },
      stderr: {
        on: vi.fn((event, cb) => {
          if (event === "data" && result.stderr) {
            cb(Buffer.from(result.stderr));
          }
        })
      },
      on: vi.fn((event, cb) => {
        if (event === "close") {
          setTimeout(() => cb(result.exitCode ?? 0), 10);
        }
      }),
      kill: vi.fn()
    };
    if (subcommand === "run") {
      const container = this.createMockContainer({
        image: args.find((_a, i) => args[i - 1] === "--image") ?? "unknown"
      });
      container.status = "running";
    }
    return process2;
  }
  /**
   * Mock the ContainerManager class from agentic-control.
   */
  mockContainerManager() {
    const mockManager = vi.fn().mockImplementation(() => ({
      createContainer: vi.fn(async (config) => {
        return this.createMockContainer(config);
      }),
      startContainer: vi.fn(async (id) => {
        const container = this.containers.get(id);
        if (container) {
          await container.start();
        }
      }),
      stopContainer: vi.fn(async (id) => {
        const container = this.containers.get(id);
        if (container) {
          await container.stop();
        }
      }),
      removeContainer: vi.fn(async (id) => {
        const container = this.containers.get(id);
        if (container) {
          await container.remove();
        }
      }),
      execInContainer: vi.fn(async (id, command) => {
        const container = this.containers.get(id);
        if (container) {
          return container.exec(command);
        }
        throw new Error(`Container not found: ${id}`);
      })
    }));
    vi.doMock("agentic-control/sandbox", () => ({
      ContainerManager: mockManager
    }));
    this.mockedModules.add("agentic-control/sandbox");
    return mockManager;
  }
  /**
   * Mock the SandboxExecutor class from agentic-control.
   */
  mockSandboxExecutor() {
    const mockExecutor = vi.fn().mockImplementation(() => ({
      execute: vi.fn(async () => {
        return this.getNextResult();
      }),
      executeWithTimeout: vi.fn(async () => {
        return this.getNextResult();
      })
    }));
    vi.doMock("agentic-control/sandbox", () => ({
      SandboxExecutor: mockExecutor
    }));
    this.mockedModules.add("agentic-control/sandbox");
    return mockExecutor;
  }
  /**
   * Create a mock runtime adapter.
   *
   * @param name - Runtime name
   * @param command - Command to return from prepareCommand
   */
  createMockRuntime(name, command = ["echo", "mock"]) {
    return {
      name,
      prepareCommand: vi.fn(() => command),
      parseOutput: vi.fn((output) => ({ output }))
    };
  }
  /**
   * Get all container IDs.
   */
  getContainerIds() {
    return Array.from(this.containers.keys());
  }
  /**
   * Get a container by ID.
   */
  getContainer(id) {
    return this.containers.get(id);
  }
  /**
   * Restore all mocked modules.
   */
  restoreAll() {
    this.mockedModules.clear();
    this.containers.clear();
    this.resultQueue = [];
  }
  /**
   * Reset all mocks.
   */
  resetAll() {
    for (const container of this.containers.values()) {
      container.start.mockClear();
      container.stop.mockClear();
      container.exec.mockClear();
      container.remove.mockClear();
      container.copyTo.mockClear();
      container.copyFrom.mockClear();
      container.executions = [];
    }
    this.resultQueue = [];
  }
};
function createSandboxMocker(options = {}) {
  return new SandboxMocker(options);
}
__name(createSandboxMocker, "createSandboxMocker");

// src/mocking.ts
var AgenticMocker = class {
  static {
    __name(this, "AgenticMocker");
  }
  /** MCP mocking utilities */
  mcp;
  /** AI provider mocking utilities */
  providers;
  /** Sandbox execution mocking utilities */
  sandbox;
  /** Track all mocked modules for cleanup */
  mockedModules = /* @__PURE__ */ new Map();
  /** Track original module values */
  originalModules = /* @__PURE__ */ new Map();
  /**
   * Creates a new AgenticMocker instance.
   *
   * @param options - Configuration options for the mocker
   */
  constructor(options = {}) {
    this.mcp = createMcpMocker(options.mcp);
    this.providers = createProviderMocker(options.providers);
    this.sandbox = createSandboxMocker(options.sandbox);
  }
  /**
   * Mock a module by path.
   *
   * @param modulePath - The module path to mock
   * @param mockValue - The mock value to use
   * @returns The mock value for chaining
   */
  mockModule(modulePath, mockValue) {
    vi.doMock(modulePath, () => mockValue);
    this.mockedModules.set(modulePath, mockValue);
    return mockValue;
  }
  /**
   * Create a spy on a function.
   *
   * @param implementation - Optional implementation for the spy
   * @returns The mock function
   */
  createSpy(implementation) {
    return vi.fn(implementation);
  }
  /**
   * Mock all agentic-control framework modules.
   *
   * This mocks the common modules used in agentic-control:
   * - MCP SDK modules
   * - AI SDK modules
   * - GitHub client modules
   *
   * @returns Dictionary of all mocked modules
   */
  mockAllFrameworks() {
    const mocks = {};
    const mcpMocks = this.mcp.mockAllModules();
    Object.assign(mocks, mcpMocks);
    const providerMocks = this.providers.mockAllModules();
    Object.assign(mocks, providerMocks);
    return mocks;
  }
  /**
   * Mock the GitHub client.
   *
   * @param options - Options for the mock
   * @returns The mock GitHub client
   */
  mockGitHubClient(options = {}) {
    const mockClient = {
      issues: {
        list: vi.fn().mockResolvedValue({ data: options.issues ?? [] }),
        get: vi.fn().mockResolvedValue({ data: options.issues?.[0] ?? {} }),
        create: vi.fn().mockResolvedValue({ data: { id: 1, number: 1 } }),
        update: vi.fn().mockResolvedValue({ data: {} })
      },
      pulls: {
        list: vi.fn().mockResolvedValue({ data: options.pullRequests ?? [] }),
        get: vi.fn().mockResolvedValue({ data: options.pullRequests?.[0] ?? {} }),
        create: vi.fn().mockResolvedValue({ data: { id: 1, number: 1 } })
      },
      repos: {
        get: vi.fn().mockResolvedValue({ data: options.repositories?.[0] ?? {} }),
        listForOrg: vi.fn().mockResolvedValue({ data: options.repositories ?? [] })
      }
    };
    this.mockModule("@octokit/rest", { Octokit: vi.fn().mockReturnValue(mockClient) });
    return mockClient;
  }
  /**
   * Mock environment variables temporarily.
   *
   * @param env - Environment variables to set
   * @returns Cleanup function to restore original values
   */
  mockEnv(env) {
    const original = {};
    for (const [key, value] of Object.entries(env)) {
      original[key] = process.env[key];
      process.env[key] = value;
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
  /**
   * Restore all mocked modules to their original values.
   */
  restoreAll() {
    this.mockedModules.clear();
    this.originalModules.clear();
    this.mcp.restoreAll();
    this.providers.restoreAll();
    this.sandbox.restoreAll();
    vi.clearAllMocks();
    vi.resetModules();
  }
  /**
   * Reset all mocks without restoring.
   */
  resetAll() {
    vi.resetAllMocks();
    this.mcp.resetAll();
    this.providers.resetAll();
    this.sandbox.resetAll();
  }
};
function createAgenticMocker(options = {}) {
  return new AgenticMocker(options);
}
__name(createAgenticMocker, "createAgenticMocker");

// src/index.ts
var VERSION = "1.0.0";

export { AgenticMocker, DEFAULT_TEST_ENV, McpMocker, MockMcpServer, ProviderMocker, SUPPORTED_PROVIDERS, SandboxMocker, VERSION, createAgenticMocker, createFleetConfig, createMcpMocker, createMockAgentConfig, createMockCrewConfig, createMockGitHubIssue, createMockGitHubPR, createMockTaskConfig, createProviderMocker, createSandboxConfig, createSandboxMocker, createTestConfig, createTokenConfig, createTriageConfig, withTestEnv };
//# sourceMappingURL=index.js.map
//# sourceMappingURL=index.js.map