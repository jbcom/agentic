var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// src/cursor.ts
var DEFAULT_BASE_URL = "https://api.cursor.com/v0";
function createCursorAgent(id, config, options = {}) {
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL;
  return {
    id,
    name: options.name ?? "Cursor Cloud Agent",
    cost: options.cost ?? 100,
    // Expensive!
    priority: options.priority ?? 100,
    // Very low priority (last resort)
    enabled: true,
    requiresApproval: options.requiresApproval ?? true,
    // Require approval by default
    capabilities: {
      tiers: ["expert"],
      // Only for expert-level tasks
      maxContext: 2e5,
      canCreatePR: true,
      canExecute: true,
      async: true,
      ...options.capabilities
    },
    execute: /* @__PURE__ */ __name(async (task) => {
      try {
        const response = await fetch(`${baseUrl}/agents`, {
          method: "POST",
          headers: {
            Authorization: `Bearer ${config.apiKey}`,
            "Content-Type": "application/json"
          },
          body: JSON.stringify({
            messages: [
              {
                role: "user",
                content: formatCursorPrompt(task)
              }
            ],
            workspacePath: config.workspacePath
          })
        });
        if (!response.ok) {
          const error = await response.text();
          return {
            success: false,
            error: `Cursor API error: ${response.status} - ${error}`,
            escalate: false,
            // No more tiers to escalate to
            cost: 0
            // Don't charge for failed requests
          };
        }
        const result = await response.json();
        return {
          success: true,
          data: {
            agentId: result.id ?? result.agentId,
            status: result.status ?? "running",
            messages: result.messages
          },
          jobId: result.id,
          // Caller should poll
          cost: 100
          // Charge for successful spawn
        };
      } catch (error) {
        return {
          success: false,
          error: error instanceof Error ? error.message : "Unknown error",
          escalate: false,
          cost: 0
        };
      }
    }, "execute")
  };
}
__name(createCursorAgent, "createCursorAgent");
async function pollCursorAgent(config, agentId) {
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL;
  const response = await fetch(`${baseUrl}/agents/${agentId}`, {
    headers: {
      Authorization: `Bearer ${config.apiKey}`
    }
  });
  if (!response.ok) {
    throw new Error(`Failed to poll agent: ${response.status}`);
  }
  const result = await response.json();
  return {
    agentId: result.id,
    status: result.status,
    messages: result.messages
  };
}
__name(pollCursorAgent, "pollCursorAgent");
function formatCursorPrompt(task) {
  return `You are an expert developer. Complete this complex task:

TASK: ${task.description}

REPOSITORY: ${task.repo ?? "Unknown"}

CONTEXT:
${task.context}

This task was escalated to you because simpler approaches failed.
Take your time, analyze carefully, and provide a complete solution.`;
}
__name(formatCursorPrompt, "formatCursorPrompt");

// src/jules.ts
var DEFAULT_BASE_URL2 = "https://jules.googleapis.com/v1alpha";
function createJulesAgent(id, config, options = {}) {
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL2;
  const automationMode = config.automationMode ?? "AUTO_CREATE_PR";
  return {
    id,
    name: options.name ?? "Google Jules",
    cost: options.cost ?? 0,
    // Free tier
    priority: options.priority ?? 10,
    // Lower priority than Ollama
    enabled: true,
    requiresApproval: false,
    capabilities: {
      tiers: ["moderate", "complex", "expert"],
      maxContext: 1e5,
      canCreatePR: true,
      canExecute: true,
      async: true,
      // Jules is async - returns job ID
      ...options.capabilities
    },
    execute: /* @__PURE__ */ __name(async (task) => {
      try {
        const response = await fetch(`${baseUrl}/sessions`, {
          method: "POST",
          headers: {
            "X-Goog-Api-Key": config.apiKey,
            "Content-Type": "application/json"
          },
          body: JSON.stringify({
            prompt: formatJulesPrompt(task),
            sourceContext: task.repo ? {
              source: `sources/github/${task.repo}`,
              githubRepoContext: {
                startingBranch: "main"
              }
            } : void 0,
            automationMode
          })
        });
        if (!response.ok) {
          const error = await response.text();
          return {
            success: false,
            error: `Jules API error: ${response.status} - ${error}`,
            escalate: true,
            cost: 0
          };
        }
        const result = await response.json();
        return {
          success: true,
          data: {
            sessionId: result.name?.split("/").pop() ?? result.name,
            name: result.name,
            state: result.state ?? "PENDING"
          },
          jobId: result.name,
          // Caller should poll this
          cost: 0
        };
      } catch (error) {
        return {
          success: false,
          error: error instanceof Error ? error.message : "Unknown error",
          escalate: true,
          cost: 0
        };
      }
    }, "execute")
  };
}
__name(createJulesAgent, "createJulesAgent");
async function pollJulesSession(config, sessionName) {
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL2;
  const response = await fetch(`${baseUrl}/${sessionName}`, {
    headers: {
      "X-Goog-Api-Key": config.apiKey
    }
  });
  if (!response.ok) {
    throw new Error(`Failed to poll session: ${response.status}`);
  }
  const result = await response.json();
  return {
    state: result.state,
    prUrl: result.pullRequestUrl,
    error: result.error?.message
  };
}
__name(pollJulesSession, "pollJulesSession");
async function sendJulesFollowUp(config, sessionName, message) {
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL2;
  const response = await fetch(`${baseUrl}/${sessionName}:addUserResponse`, {
    method: "POST",
    headers: {
      "X-Goog-Api-Key": config.apiKey,
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ userResponse: message })
  });
  if (!response.ok) {
    throw new Error(`Failed to send follow-up: ${response.status}`);
  }
}
__name(sendJulesFollowUp, "sendJulesFollowUp");
function formatJulesPrompt(task) {
  return `${task.description}

Context:
${task.context}

Requirements:
- Create a PR with your changes
- Ensure all tests pass
- Follow existing code style
- Add appropriate documentation`;
}
__name(formatJulesPrompt, "formatJulesPrompt");

// src/ollama.ts
var DEFAULT_URL = "http://localhost:11434";
var DEFAULT_MODEL = "qwen2.5-coder:32b";
var DEFAULT_TIMEOUT = 6e4;
function createOllamaEvaluator(config = {}) {
  const url = config.url ?? DEFAULT_URL;
  const model = config.model ?? DEFAULT_MODEL;
  const timeout = config.timeout ?? DEFAULT_TIMEOUT;
  return async (prompt) => {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);
    try {
      const response = await fetch(`${url}/api/generate`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          model,
          prompt,
          stream: false,
          format: "json"
        }),
        signal: controller.signal
      });
      if (!response.ok) {
        throw new Error(`Ollama API error: ${response.status}`);
      }
      const result = await response.json();
      return result.response;
    } finally {
      clearTimeout(timeoutId);
    }
  };
}
__name(createOllamaEvaluator, "createOllamaEvaluator");
function createOllamaAgent(id, config = {}, options = {}) {
  const url = config.url ?? DEFAULT_URL;
  const model = config.model ?? DEFAULT_MODEL;
  const timeout = config.timeout ?? DEFAULT_TIMEOUT;
  return {
    id,
    name: options.name ?? `Ollama (${model})`,
    cost: options.cost ?? 0,
    // Free
    priority: options.priority ?? 1,
    // High priority (try first)
    enabled: true,
    requiresApproval: false,
    capabilities: {
      tiers: ["trivial", "simple"],
      maxContext: 8e3,
      canCreatePR: false,
      canExecute: false,
      async: false,
      ...options.capabilities
    },
    execute: /* @__PURE__ */ __name(async (task) => {
      try {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), timeout);
        const response = await fetch(`${url}/api/generate`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            model,
            prompt: formatTaskPrompt(task),
            stream: false
          }),
          signal: controller.signal
        });
        clearTimeout(timeoutId);
        if (!response.ok) {
          return {
            success: false,
            error: `Ollama error: ${response.status}`,
            escalate: true,
            cost: 0
          };
        }
        const result = await response.json();
        const output = result.response?.trim();
        if (output?.toUpperCase().startsWith("ESCALATE:")) {
          return {
            success: false,
            error: output,
            escalate: true,
            cost: 0
          };
        }
        return {
          success: true,
          data: output,
          cost: 0
        };
      } catch (error) {
        return {
          success: false,
          error: error instanceof Error ? error.message : "Unknown error",
          escalate: true,
          cost: 0
        };
      }
    }, "execute")
  };
}
__name(createOllamaAgent, "createOllamaAgent");
function formatTaskPrompt(task) {
  return `You are a code assistant. Complete this task:

TASK: ${task.description}

CONTEXT:
${task.context.slice(0, 8e3)}

If you can complete this task, provide the solution.
If this task is too complex, respond with: ESCALATE: <reason>

Provide your response:`;
}
__name(formatTaskPrompt, "formatTaskPrompt");

export { createCursorAgent, createJulesAgent, createOllamaAgent, createOllamaEvaluator, pollCursorAgent, pollJulesSession, sendJulesFollowUp };
//# sourceMappingURL=index.js.map
//# sourceMappingURL=index.js.map