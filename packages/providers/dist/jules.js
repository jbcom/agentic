var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// src/jules.ts
var DEFAULT_BASE_URL = "https://jules.googleapis.com/v1alpha";
function createJulesAgent(id, config, options = {}) {
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL;
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
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL;
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
  const baseUrl = config.baseUrl ?? DEFAULT_BASE_URL;
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

export { createJulesAgent, pollJulesSession, sendJulesFollowUp };
//# sourceMappingURL=jules.js.map
//# sourceMappingURL=jules.js.map