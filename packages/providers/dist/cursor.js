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

export { createCursorAgent, pollCursorAgent };
//# sourceMappingURL=cursor.js.map
//# sourceMappingURL=cursor.js.map