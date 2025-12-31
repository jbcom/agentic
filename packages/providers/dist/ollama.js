var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

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

export { createOllamaAgent, createOllamaEvaluator };
//# sourceMappingURL=ollama.js.map
//# sourceMappingURL=ollama.js.map