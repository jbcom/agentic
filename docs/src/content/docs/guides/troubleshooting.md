---
title: Troubleshooting
description: Common issues and solutions for the Agentic toolkit -- token configuration, agent spawning, build errors, CI/CD integration, and provider-specific problems.
---

This page covers common issues encountered when working with the Agentic toolkit and how to resolve them.

## Token Configuration Errors

### "No token found for organization"

The fleet or triage command cannot find a GitHub token for the target repository's organization.

**Cause:** The organization is not mapped in `agentic.config.json` or via environment variables.

**Fix:**

1. Run `agentic init` to regenerate configuration with detected tokens.
2. Or add the organization manually:

```json
{
  "tokens": {
    "organizations": {
      "my-org": {
        "name": "my-org",
        "tokenEnvVar": "GITHUB_MY_ORG_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN"
  }
}
```

3. Or set a dynamic environment variable:

```bash
export AGENTIC_ORG_MYORG_TOKEN=GITHUB_MYORG_TOKEN
export GITHUB_MYORG_TOKEN=ghp_xxxxxxxxxxxx
```

### "Token validation failed"

The token exists but is rejected by the GitHub API.

**Cause:** The token is expired, revoked, or missing required scopes.

**Fix:**

1. Check the token status: `agentic tokens status`
2. Verify the token has the `repo` scope (for private repositories) or `public_repo` scope.
3. Regenerate the token on GitHub and update your environment.

### "ANTHROPIC_API_KEY not set"

AI triage features require a provider API key.

**Fix:**

```bash
export ANTHROPIC_API_KEY=sk-ant-xxxxxxxxxxxx
```

Or configure a different provider in `agentic.config.json`:

```json
{
  "triage": {
    "provider": "openai",
    "model": "gpt-4o",
    "apiKeyEnvVar": "OPENAI_API_KEY"
  }
}
```

## Agent Spawn Failures

### Agent stays in PENDING state

**Symptom:** After calling `fleet.spawn()`, the agent never transitions to `RUNNING`.

**Possible causes:**

1. **Invalid API key** -- Verify `CURSOR_API_KEY` is set and valid.
2. **Repository access** -- The Cursor account must have access to the target repository.
3. **Rate limits** -- The Cursor API may throttle spawn requests. Wait 30 seconds and retry.

```bash
# Quick check
echo $CURSOR_API_KEY | head -c 8
# Should print the first 8 characters of your key
```

### "Failed to spawn agent: Cursor API error"

The Cursor Background Agent API rejected the spawn request.

**Possible causes:**

- `CURSOR_API_KEY` is not set or is invalid.
- The target repository URL is malformed.
- The Cursor service is temporarily unavailable.

**Fix:**

1. Verify the API key: `echo $CURSOR_API_KEY | head -c 10`
2. Ensure the repository URL is a full HTTPS URL: `https://github.com/owner/repo`
3. Check Cursor service status.

### "Agent spawned but no PR created"

The agent completed its work but did not open a pull request.

**Cause:** The `--auto-pr` flag was not passed, or the agent determined no changes were needed.

**Fix:**

```bash
agentic fleet spawn <repo> <task> --auto-pr
```

Or set the default in configuration:

```json
{
  "fleet": {
    "autoCreatePr": true
  }
}
```

### `fleet.list()` returns empty

The API key may be scoped to a different workspace. Confirm you are using the correct key for your organization.

### Coordination PR not receiving updates

- Ensure `GITHUB_TOKEN` (or `GITHUB_JBCOM_TOKEN` in this organization) has `repo` and `pull_request` scopes.
- Check that the `repo` parameter uses `owner/name` format, not a full URL.

### Handoff failed: predecessor not found

The handoff protocol could not locate the predecessor agent.

**Cause:** The predecessor agent ID is incorrect or the agent has already been cleaned up.

**Fix:**

1. List agents to find the correct ID: `agentic fleet list`
2. Ensure the predecessor is still active before initiating handoff.

## Build and Install Issues

### TypeScript: "Cannot find module '@jbcom/agentic'"

**Cause:** The package is not installed, or the project is not using ES modules.

**Fix:**

1. Install the package: `pnpm add @jbcom/agentic`
2. Ensure `"type": "module"` is set in your `package.json`.
3. Ensure your `tsconfig.json` uses `"moduleResolution": "bundler"` or `"node16"`.

### TypeScript: "No AI provider SDK found"

**Cause:** `@jbcom/agentic` requires at least one AI provider peer dependency installed.

**Fix:**

```bash
pnpm add @ai-sdk/anthropic    # or @ai-sdk/openai, @ai-sdk/google, etc.
```

### Python: "ModuleNotFoundError: No module named 'crewai'"

**Cause:** `agentic-crew` was installed without a framework extra.

**Fix:**

```bash
pip install agentic-crew[crewai]
# or
pip install agentic-crew[langgraph]
# or
pip install agentic-crew[strands]
```

### Python: "No framework detected"

**Cause:** None of the supported frameworks (CrewAI, LangGraph, Strands) are installed.

**Fix:**

```bash
pip install agentic-crew[ai]   # Install all frameworks
```

### Python: `uv run` fails with "No such module: agentic_crew"

```bash
# Re-sync dependencies
cd packages/agentic-crew
uv sync --all-extras
```

### CrewTool bridge timeout

The TypeScript-to-Python bridge defaults to a 60-second timeout. For long-running crews, increase it:

```typescript
const tool = new CrewTool({
  crewPath: './my-crew',
  timeout: 300000, // 5 minutes
});
```

### Rust: "error[E0433]: failed to resolve: use of undeclared crate"

**Cause:** Rust dependencies are not downloaded.

**Fix:**

```bash
cargo build -p game-generator
```

Cargo will fetch all dependencies automatically. Ensure you have Rust >= 1.85 installed.

### Rust: `cargo build` fails on macOS

Ensure you have the latest Xcode Command Line Tools:

```bash
xcode-select --install
```

### Rust: Missing system dependencies on Linux

```bash
# Ubuntu/Debian
sudo apt-get install -y pkg-config libssl-dev
```

### Rust: Bevy requires a GPU

The `game-generator` uses Bevy for rendering and requires GPU support. Ensure you have up-to-date graphics drivers. On headless servers, Bevy cannot run the GUI wizard -- use the library API instead.

## CI/CD Integration

### GitHub Actions: "Resource not accessible by integration"

**Cause:** The GitHub Actions workflow token does not have sufficient permissions.

**Fix:** Add the required permissions to your workflow:

```yaml
permissions:
  issues: write
  pull-requests: write
  contents: read
```

### GitHub Actions: "Action timed out"

**Cause:** The AI triage or fleet operation exceeded the step timeout.

**Fix:** Increase the timeout for the step:

```yaml
- uses: jbcom/agentic/actions/agentic-issue-triage@main
  timeout-minutes: 10
```

### npm publish: "ENEEDAUTH"

**Cause:** npm authentication is not configured for publishing.

**Fix:** If using OIDC trusted publishing (recommended), ensure the GitHub Actions workflow has `id-token: write` permission:

```yaml
permissions:
  id-token: write
  contents: read
```

If using a token, set the `NPM_TOKEN` secret in your repository settings.

### pnpm install fails in CI

Make sure `pnpm-lock.yaml` is committed and up-to-date:

```bash
pnpm install --frozen-lockfile
```

### Docs build fails with TypeDoc errors

The docs site uses `starlight-typedoc` to generate API reference pages. If the build fails:

1. Ensure all TypeScript packages compile: `pnpm run check` from the monorepo root.
2. The TypeDoc plugins use `skipErrorChecking: true`, but missing entry points still cause failures. Verify the paths in `docs/astro.config.mjs`.

### pytest: "E2E tests skipped"

**Cause:** E2E tests are disabled by default in `pytest-agentic-crew`.

**Fix:**

```bash
pytest --e2e                          # Enable E2E tests
pytest --e2e --framework=crewai       # Run only CrewAI tests
```

Ensure the required API keys are set:

```bash
export ANTHROPIC_API_KEY=sk-ant-xxxxxxxxxxxx
```

## Provider-Specific Issues

### Triage: `TriageConnectors` throws "provider not found"

Make sure the `provider` field matches one of: `github`, `jira`, `linear`, `beads`.

### GitHub: Rate limiting (403)

The triage tools make multiple API calls. If you hit rate limits:

1. Use a GitHub App token instead of a personal access token.
2. Reduce `pollInterval` frequency in coordination config.
3. Add retry logic with exponential backoff (built-in when using `getTriageTools()`).

### Ollama: "Connection refused"

**Cause:** The Ollama server is not running or is on a different port.

**Fix:**

1. Start Ollama: `ollama serve`
2. Verify the URL matches your configuration (default: `http://localhost:11434`).
3. If using Ollama Cloud, ensure `OLLAMA_API_KEY` is set.

### Ollama: "Model not found"

**Cause:** The requested model is not pulled locally.

**Fix:**

```bash
ollama pull qwen2.5-coder:32b
```

### Anthropic: "Rate limit exceeded"

**Cause:** Too many API calls in a short period.

**Fix:**

1. Reduce concurrency in fleet operations.
2. Use the escalation ladder to route simple tasks to cheaper providers first.
3. Wait and retry -- Anthropic rate limits reset on a per-minute basis.

### Jules: "Session timed out"

**Cause:** Jules sessions are asynchronous and may take longer than expected.

**Fix:** Use polling to wait for completion:

```typescript
import { createJulesAgent, pollJulesSession } from '@jbcom/agentic-providers/jules';

const agent = createJulesAgent('jules', { apiKey });
const session = await agent.start(task);
const result = await pollJulesSession(session.id, { timeout: 600000 }); // 10 min
```

### Cursor: "Budget exceeded"

**Cause:** The daily cost budget for the task router has been reached.

**Fix:** Increase the budget or wait for the next day:

```typescript
const router = new TaskRouter({
  registry,
  dailyBudget: 100, // Increase from default
});
```

## Sandbox Issues

### "Docker daemon not running"

**Cause:** The sandbox executor requires Docker to be running.

**Fix:**

1. Start Docker Desktop (macOS/Windows) or the Docker daemon (Linux).
2. Verify: `docker info`

### "Container OOM killed"

**Cause:** The sandbox container exceeded its memory limit.

**Fix:** Increase the memory allocation:

```bash
agentic sandbox run "..." --memory 4096   # 4GB
```

Or programmatically:

```typescript
const result = await sandbox.execute({
  prompt: '...',
  memory: 4096,
});
```

### "Permission denied: workspace mount"

**Cause:** Docker does not have permission to mount the workspace directory.

**Fix:**

1. On macOS, ensure Docker Desktop has file sharing enabled for the workspace path.
2. On Linux, ensure the current user is in the `docker` group: `sudo usermod -aG docker $USER`

## Environment Variables Reference

| Variable | Purpose | Required |
|----------|---------|----------|
| `CURSOR_API_KEY` | Fleet API authentication | For fleet operations |
| `GITHUB_TOKEN` | GitHub API access | For triage and coordination |
| `GITHUB_JBCOM_TOKEN` | Org-scoped GitHub token | For this organization |
| `OPENAI_API_KEY` | OpenAI provider | If using OpenAI models |
| `ANTHROPIC_API_KEY` | Anthropic provider | If using Claude models |
| `OLLAMA_API_KEY` | Ollama Cloud API | If using Ollama Cloud |
| `MESHY_API_KEY` | Meshy 3D API | For meshy content generation |
| `JULES_API_KEY` | Jules provider | If using Jules agent |

## Getting Help

If your issue is not covered here:

1. Search [existing issues](https://github.com/jbcom/agentic/issues) on GitHub.
2. Open a [new issue](https://github.com/jbcom/agentic/issues/new) with reproduction steps.
3. For urgent fleet issues, use the `/jules` command in an issue comment.
4. Visit the [full documentation](https://agentic.coach) for detailed guides.

## Next Steps

- [Architecture Overview](/guides/architecture/) -- understand the system design
- [Configuration](/getting-started/configuration/) -- review all config options
- [Fleet Management](/guides/fleet-management/) -- fleet setup and management
