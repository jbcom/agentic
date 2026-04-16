# Basic Usage Examples

This guide provides simple configuration examples to get you started with agentic-control.

## Basic Configuration

Create `agentic.config.json` in your project root:

```json
{
  "tokens": {
    "defaultTokenEnvVar": "GITHUB_TOKEN"
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514"
  },
  "fleet": {
    "autoCreatePr": false
  },
  "logLevel": "info"
}
```

## Environment Variables

Set these environment variables:

```bash
# Required: GitHub token for API access
export GITHUB_TOKEN="ghp_your_github_token_here"

# Required: AI provider API key (choose one)
export ANTHROPIC_API_KEY="sk-ant-your_anthropic_key_here"
# OR
export OPENAI_API_KEY="sk-your_openai_key_here"

# Optional: Cursor API key for fleet management
export CURSOR_API_KEY="your_cursor_api_key_here"
```

## Basic Commands

### Fleet Management

```bash
# List all agents
agentic fleet list

# Spawn a new agent
agentic fleet spawn https://github.com/owner/repo "Fix the login bug"

# Get agent status
agentic fleet get <agent-id>
```

### AI Triage

```bash
# Quick triage of text
echo "The login form is broken" | agentic triage quick -

# Review code changes
agentic triage review --base main --head feature-branch

# Analyze agent conversation
agentic triage analyze <agent-id>
```

### Sandbox Execution

```bash
# Run an agent in sandbox
agentic sandbox run "Create a hello world script" --runtime claude

# Run multiple agents in parallel
agentic sandbox fleet "Task 1" "Task 2" "Task 3"
```

## Configuration Generation

Generate a configuration file interactively:

```bash
agentic init
```

Or non-interactively:

```bash
agentic init --non-interactive
```

## Viewing Configuration

Check your current configuration:

```bash
agentic config
```

Check token status:

```bash
agentic tokens status
```