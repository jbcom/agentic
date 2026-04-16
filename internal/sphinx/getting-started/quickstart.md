# Quick Start Guide

Get up and running with agentic-control in minutes. This guide covers the essential workflows for fleet management, AI triage, and sandbox execution.

## Prerequisites

Before starting, ensure you have:

- [Installed agentic-control](installation.md)
- Set up your [environment variables](installation.md#environment-setup)
- A GitHub repository you want to work with

## Initial Setup

### 1. Initialize Configuration

```bash
agentic init
```

This interactive setup will:
- Detect your environment variables
- Configure AI providers and models
- Set up organization-specific tokens
- Create your `agentic.config.json` file

### 2. Verify Token Setup

```bash
agentic tokens status
```

You should see something like:

```
=== Token Status ===

✅ _default        GITHUB_TOKEN
✅ my-org          GITHUB_MY_ORG_TOKEN
✅ _pr_review      GITHUB_TOKEN
```

## Fleet Management

Fleet management lets you spawn, monitor, and coordinate Cursor Background Agents across your repositories.

### List Available Repositories

```bash
agentic fleet repos
```

### Spawn an Agent

```bash
agentic fleet spawn \
  "https://github.com/my-org/my-repo" \
  "Add TypeScript strict mode and fix any type errors" \
  --auto-pr \
  --branch "feature/typescript-strict"
```

### Monitor Fleet Status

```bash
# List all agents
agentic fleet list

# Get specific agent details
agentic fleet get <agent-id>

# View agent conversation
agentic fleet conversation <agent-id>
```

### Send Follow-up Instructions

```bash
agentic fleet followup <agent-id> "Also update the README with the new TypeScript requirements"
```

## AI-Powered Triage

Use AI to analyze conversations, review code, and extract actionable insights.

### Quick Text Analysis

```bash
# Analyze text directly
agentic triage quick "The build is failing on CI, users are reporting login issues, and we need to deploy the hotfix by EOD"

# Analyze from stdin
cat issue-description.txt | agentic triage quick -
```

### Code Review

```bash
# Review current changes
agentic triage review

# Review specific diff
agentic triage review --base main --head feature-branch
```

### Analyze Agent Conversations

```bash
# Analyze and generate report
agentic triage analyze <agent-id> --output report.md

# Create GitHub issues from outstanding tasks
agentic triage analyze <agent-id> --create-issues
```

## Sandbox Execution

Run AI agents in isolated Docker containers for safe local development.

### Run a Single Agent

```bash
agentic sandbox run \
  "Analyze this codebase and suggest performance improvements" \
  --runtime claude \
  --workspace . \
  --output ./sandbox-results \
  --timeout 300
```

### Run Multiple Agents in Parallel

```bash
agentic sandbox fleet \
  "Review the authentication system" \
  "Analyze database queries for optimization" \
  "Check for security vulnerabilities" \
  --runtime claude \
  --workspace . \
  --output ./fleet-results
```

## Station-to-Station Handoff

Coordinate seamless transitions between agents working on the same codebase.

### Initiate Handoff (Predecessor Agent)

```bash
agentic handoff initiate <your-agent-id> \
  --pr 123 \
  --branch "feature/user-auth" \
  --repo "https://github.com/my-org/my-repo" \
  --tasks "Complete OAuth integration,Add rate limiting,Update documentation"
```

### Confirm Health (Successor Agent)

```bash
agentic handoff confirm <predecessor-agent-id>
```

### Take Over (Successor Agent)

```bash
agentic handoff takeover <predecessor-agent-id> 123 "feature/user-auth-v2" \
  --merge-method squash \
  --auto
```

## Configuration Examples

### Multi-Organization Setup

For teams working across multiple GitHub organizations:

```json
{
  "tokens": {
    "organizations": {
      "company": {
        "name": "company",
        "tokenEnvVar": "GITHUB_COMPANY_TOKEN"
      },
      "open-source": {
        "name": "open-source", 
        "tokenEnvVar": "GITHUB_OSS_TOKEN"
      }
    },
    "prReviewTokenEnvVar": "PR_REVIEW_TOKEN"
  },
  "defaultRepository": "company/main-product",
  "coordinationPr": 42
}
```

### AI Provider Configuration

```json
{
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514"
  }
}
```

Available providers and models:

- **Anthropic**: `claude-sonnet-4-20250514`, `claude-opus-4-20250514`, `claude-haiku-4-5-20251001`
- **OpenAI**: `gpt-4o`, `gpt-4-turbo`, `gpt-3.5-turbo`, `o1`, `o1-mini`
- **Google**: `gemini-1.5-pro`, `gemini-1.5-flash`, `gemini-1.0-pro`
- **Mistral**: `mistral-large-latest`, `mistral-medium-latest`, `codestral-latest`

## Common Workflows

### 1. Feature Development with Agent Fleet

```bash
# Spawn agent for feature work
AGENT_ID=$(agentic fleet spawn \
  "https://github.com/my-org/app" \
  "Implement user profile management with avatar upload" \
  --auto-pr --json | jq -r '.id')

# Monitor progress
agentic fleet get $AGENT_ID

# Provide feedback
agentic fleet followup $AGENT_ID "Make sure to add proper error handling for file uploads"

# Analyze the work
agentic triage analyze $AGENT_ID --output profile-feature-report.md
```

### 2. Code Review Workflow

```bash
# Review PR changes
agentic triage review --base main --head pr-branch

# If issues found, create follow-up agent
agentic fleet spawn \
  "https://github.com/my-org/app" \
  "Address the code review feedback: fix error handling and add unit tests" \
  --ref pr-branch \
  --branch pr-branch-fixes
```

### 3. Multi-Agent Coordination

```bash
# Start coordination for complex feature
agentic fleet coordinate \
  --repo my-org/app \
  --pr 156 \
  --agents agent-1,agent-2,agent-3 \
  --outbound 60000 \
  --inbound 15000
```

### 4. Sandbox Development

```bash
# Test changes safely
agentic sandbox run \
  "Refactor the authentication module to use dependency injection" \
  --workspace ./src/auth \
  --output ./refactor-results \
  --env "NODE_ENV=development,LOG_LEVEL=debug"

# Review results
ls -la ./refactor-results/
cat ./refactor-results/summary.json
```

## Best Practices

### Security

- Use dedicated tokens for different organizations
- Rotate tokens regularly
- Use read-only tokens where possible
- Never commit tokens to version control

### Fleet Management

- Use descriptive task descriptions
- Monitor agent progress regularly
- Provide clear follow-up instructions
- Use auto-PR for routine tasks

### AI Triage

- Use specific models for different tasks
- Review AI suggestions before acting
- Combine with human judgment
- Keep conversation context relevant

### Sandbox Execution

- Use appropriate resource limits
- Mount only necessary directories
- Clean up output directories regularly
- Test with different runtime adapters

## Next Steps

- Explore the [API Reference](../api/index.rst) for programmatic usage
- Read the [Architecture Guide](../development/architecture.md) to understand the system design
- Check out [Advanced Patterns](../examples/advanced-patterns.md) for complex workflows
- Join the [Community Discussions](https://github.com/jbcom/agentic-control/discussions) for tips and support

## Troubleshooting

### Agent Not Spawning

1. Check token permissions: `agentic tokens validate`
2. Verify repository access: `agentic fleet repos`
3. Check Cursor API connectivity
4. Review error logs with `--json` flag for details

### Triage Analysis Failing

1. Verify AI provider API key: `echo $ANTHROPIC_API_KEY`
2. Check model availability: `agentic triage models`
3. Try a different model or provider
4. Reduce input size if hitting token limits

### Sandbox Execution Issues

1. Ensure Docker is running: `docker ps`
2. Check image availability: `docker pull jbcom/agentic-control:latest`
3. Verify workspace permissions
4. Check resource limits and available memory

For more help, see the [Installation Troubleshooting](installation.md#troubleshooting) section.