---
title: CLI Workflows
description: Command-line examples for common Agentic workflows
---

# CLI Workflows

Practical command-line examples for common Agentic workflows. These can be used directly in your terminal or incorporated into shell scripts.

## Setup & Configuration

### Initialize a New Project

```bash
# Interactive initialization
agentic init

# Non-interactive with defaults
agentic init --non-interactive
```

### Verify Token Configuration

```bash
# Check all tokens
agentic tokens status

# Validate tokens are working
agentic tokens validate

# Get token for specific repo
agentic tokens for-repo my-org/my-repo
```

## Agent Operations

### Spawn a Single Agent

```bash
# Basic spawn
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Fix the failing CI workflow"

# With auto-PR creation
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Update dependencies to latest versions" \
  --auto-pr

# On specific branch with custom target branch
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Implement user authentication" \
  --ref develop \
  --branch feature/auth \
  --auto-pr
```

### Monitor Agents

```bash
# List all agents
agentic fleet list

# List only running agents
agentic fleet list --running

# Get fleet summary
agentic fleet summary

# Watch agent status (refresh every 30s)
watch -n 30 'agentic fleet list --running'
```

### Interact with Running Agents

```bash
# Send a followup message
agentic fleet followup bc-xxx-xxx "Please focus on the authentication module first"

# Request status update
agentic fleet followup bc-xxx-xxx "What's your current progress?"

# Provide additional context
agentic fleet followup bc-xxx-xxx "The config file is at ./config/settings.json"
```

## Triage Operations

### Quick Analysis

```bash
# Analyze an error message
agentic triage quick "TypeError: Cannot read property 'map' of undefined"

# Analyze a log snippet
agentic triage quick "$(tail -50 /var/log/app/error.log)"
```

### Code Review

```bash
# Review current changes vs main
agentic triage review --base main --head HEAD

# Review a specific branch
agentic triage review --base main --head feature/user-auth

# Review with specific model
agentic triage review --base main --head HEAD --model claude-opus-4-20250514
```

### Analyze Agent Conversations

```bash
# Generate markdown report
agentic triage analyze bc-xxx-xxx -o ./reports/agent-analysis.md

# Create GitHub issues from findings
agentic triage analyze bc-xxx-xxx --create-issues

# Both report and issues
agentic triage analyze bc-xxx-xxx -o report.md --create-issues
```

## Sandbox Operations

### Single Sandbox Execution

```bash
# Basic analysis
agentic sandbox run "Analyze this codebase for security issues" \
  --workspace . \
  --output ./security-analysis

# With specific runtime and timeout
agentic sandbox run "Refactor the API layer" \
  --runtime claude \
  --workspace ./src/api \
  --output ./api-refactor \
  --timeout 600

# With resource limits
agentic sandbox run "Process large dataset" \
  --workspace . \
  --output ./results \
  --memory 4096 \
  --timeout 1800
```

### Parallel Fleet Execution

```bash
# Run multiple analyses in parallel
agentic sandbox fleet \
  "Review authentication system" \
  "Analyze database queries for N+1" \
  "Check for XSS vulnerabilities" \
  --workspace . \
  --output ./fleet-analysis
```

## Coordination Workflows

### Coordinate Multiple Agents

```bash
# Start coordination loop
agentic fleet coordinate \
  --pr 123 \
  --repo my-org/control-center \
  --agents agent-1,agent-2,agent-3
```

### Handoff Between Agents

```bash
# Initiate handoff from predecessor
agentic handoff initiate bc-predecessor-xxx \
  --pr 123 \
  --branch feature/my-feature \
  --repo https://github.com/my-org/my-repo

# Confirm as successor
agentic handoff confirm bc-predecessor-xxx

# Take over work
agentic handoff takeover bc-predecessor-xxx 123 feature/continued
```

## Scripted Workflows

### Multi-Repo Dependency Update

```bash
#!/bin/bash
# update-deps.sh - Update dependencies across multiple repos

REPOS=(
  "my-org/frontend"
  "my-org/backend"
  "my-org/shared-lib"
  "my-org/mobile"
)

for repo in "${REPOS[@]}"; do
  echo "📦 Spawning agent for $repo..."
  agentic fleet spawn "https://github.com/$repo" \
    "Update all dependencies to latest compatible versions. Run tests and fix any breaking changes." \
    --auto-pr \
    --branch "chore/update-deps-$(date +%Y%m%d)"
done

echo "✅ All agents spawned. Monitor with: agentic fleet list --running"
```

### Security Audit Script

```bash
#!/bin/bash
# security-audit.sh - Run security audit on codebase

OUTPUT_DIR="./security-audit-$(date +%Y%m%d)"
mkdir -p "$OUTPUT_DIR"

echo "🔍 Running security audit..."

agentic sandbox run "$(cat <<'EOF'
Perform a comprehensive security audit:

1. Check for hardcoded secrets and API keys
2. Identify SQL injection vulnerabilities
3. Find XSS attack vectors
4. Review authentication flows
5. Check for insecure dependencies
6. Identify OWASP Top 10 issues

Create a detailed report with:
- Severity ratings (Critical/High/Medium/Low)
- File paths and line numbers
- Remediation steps
EOF
)" \
  --workspace . \
  --output "$OUTPUT_DIR" \
  --timeout 900 \
  --memory 2048

echo "✅ Audit complete. Results in $OUTPUT_DIR"
```

### Release Preparation

```bash
#!/bin/bash
# prepare-release.sh - Prepare a new release

VERSION=$1
if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

echo "🚀 Preparing release v$VERSION"

# Spawn documentation agent
DOC_AGENT=$(agentic fleet spawn "https://github.com/my-org/docs" \
  "Update documentation for v$VERSION release. Update version numbers, add changelog entries, and ensure all examples work." \
  --auto-pr --branch "release/v$VERSION-docs" 2>&1 | grep -o 'bc-[a-z0-9-]*')

# Spawn changelog agent
CHANGELOG_AGENT=$(agentic fleet spawn "https://github.com/my-org/app" \
  "Generate CHANGELOG entries for v$VERSION. Review all commits since last release and create comprehensive changelog." \
  --auto-pr --branch "release/v$VERSION-changelog" 2>&1 | grep -o 'bc-[a-z0-9-]*')

# Spawn test agent
TEST_AGENT=$(agentic fleet spawn "https://github.com/my-org/app" \
  "Run full test suite and create release readiness report for v$VERSION." \
  --auto-pr --branch "release/v$VERSION-tests" 2>&1 | grep -o 'bc-[a-z0-9-]*')

echo "📋 Agents spawned:"
echo "   Docs: $DOC_AGENT"
echo "   Changelog: $CHANGELOG_AGENT"
echo "   Tests: $TEST_AGENT"

# Broadcast coordination message
agentic fleet broadcast "$DOC_AGENT,$CHANGELOG_AGENT,$TEST_AGENT" \
  "COORDINATION: Working on v$VERSION release. Report any blockers to the release PR."
```

### CI Integration

```bash
#!/bin/bash
# ci-triage.sh - Analyze CI failures

# Get the last failed run
FAILED_RUN=$(gh run list --status failure --limit 1 --json databaseId -q '.[0].databaseId')

if [ -z "$FAILED_RUN" ]; then
  echo "✅ No failed runs found"
  exit 0
fi

# Get logs
LOGS=$(gh run view "$FAILED_RUN" --log 2>&1 | tail -200)

# Analyze with AI
agentic triage quick "CI run $FAILED_RUN failed with these logs:

$LOGS

What's the likely cause and how should we fix it?"
```

## Crew CLI (Python)

### List Available Runners

```bash
agentic-crew list-runners
```

### Run a Crew

```bash
# Run with auto-detected framework
agentic-crew run my-package analyzer --input "Analyze ./src"

# Force specific framework
agentic-crew run my-package analyzer --framework crewai --input "..."
```

### Single-Agent Runners

```bash
# Quick fix with Aider
agentic-crew run --runner aider --input "Fix the typo in README.md"

# Free local execution with Ollama
agentic-crew run --runner ollama --input "Add docstrings" --model deepseek-coder

# Complex task with Claude Code
agentic-crew run --runner claude-code --input "Refactor auth module"
```

## Tips & Tricks

### Use Environment Variables

```bash
# Set defaults
export AGENTIC_REPOSITORY="my-org/my-repo"
export AGENTIC_PROVIDER="anthropic"
export AGENTIC_MODEL="claude-sonnet-4-20250514"
```

### JSON Output for Scripting

```bash
# Get agent list as JSON
agentic fleet list --json | jq '.[] | select(.status == "RUNNING")'

# Get summary as JSON
agentic fleet summary --json
```

### Combine with Standard Tools

```bash
# Find and fix all TODO comments
grep -rn "TODO:" ./src | agentic triage quick "$(cat -)"

# Analyze git diff
git diff main | agentic triage quick "Review these changes"

# Check recent commits
git log --oneline -10 | agentic triage quick "Summarize recent work"
```

## Next Steps

- [TypeScript Examples](/examples/typescript/) - Programmatic TypeScript examples
- [Python Examples](/examples/python/) - Python crew examples
- [API Reference](/api/fleet-management/) - Complete API documentation
