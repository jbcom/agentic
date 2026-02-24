---
title: GitHub Actions Integration
description: Integrate Agentic into your CI/CD pipelines with GitHub Actions
---

# GitHub Actions Integration

Integrate Agentic into your CI/CD workflows for automated code review, issue triage, and agent spawning.

## Quick Start

### Install and Run Agentic in CI

```yaml
name: Agentic CI

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  agentic-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Install Agentic
        run: npm install -g @agentic-dev-library/control

      - name: Review PR
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          agentic triage review \
            --base ${{ github.base_ref }} \
            --head ${{ github.head_ref }}
```

## Use Cases

### Automated Code Review

Automatically review PRs when they're opened:

```yaml
name: AI Code Review

on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Install dependencies
        run: npm install -g @agentic-dev-library/control @ai-sdk/anthropic

      - name: Run AI Review
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          # Review changes
          agentic triage review \
            --base origin/${{ github.base_ref }} \
            --head ${{ github.sha }} \
            -o review.md

          # Post review as comment
          gh pr comment ${{ github.event.pull_request.number }} \
            --body-file review.md
```

### Issue Triage on Creation

Automatically triage new issues:

```yaml
name: Issue Triage

on:
  issues:
    types: [opened]

jobs:
  triage:
    runs-on: ubuntu-latest
    permissions:
      issues: write

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Install Agentic
        run: npm install -g @agentic-dev-library/control @ai-sdk/anthropic

      - name: Triage Issue
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          ISSUE_BODY="${{ github.event.issue.body }}"
          ISSUE_TITLE="${{ github.event.issue.title }}"

          # Analyze the issue
          ANALYSIS=$(agentic triage quick "$ISSUE_TITLE: $ISSUE_BODY" --json)

          # Extract suggested labels
          LABELS=$(echo "$ANALYSIS" | jq -r '.labels | join(",")')

          # Apply labels
          gh issue edit ${{ github.event.issue.number }} --add-label "$LABELS"

          # Add triage comment
          gh issue comment ${{ github.event.issue.number }} \
            --body "## AI Triage Summary

          $(echo "$ANALYSIS" | jq -r '.summary')

          **Suggested Priority:** $(echo "$ANALYSIS" | jq -r '.priority')
          **Category:** $(echo "$ANALYSIS" | jq -r '.category')"
```

### Spawn Agent on Label

Spawn an agent when a specific label is added:

```yaml
name: Spawn Agent

on:
  issues:
    types: [labeled]

jobs:
  spawn:
    if: github.event.label.name == 'ai-fix'
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Install Agentic
        run: npm install -g @agentic-dev-library/control

      - name: Spawn Agent
        env:
          CURSOR_API_KEY: ${{ secrets.CURSOR_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          ISSUE_TITLE="${{ github.event.issue.title }}"
          ISSUE_BODY="${{ github.event.issue.body }}"
          ISSUE_NUM="${{ github.event.issue.number }}"

          agentic fleet spawn \
            "https://github.com/${{ github.repository }}" \
            "Fix issue #$ISSUE_NUM: $ISSUE_TITLE

            Details:
            $ISSUE_BODY

            Requirements:
            - Create a fix for this issue
            - Add tests if applicable
            - Reference the issue in the commit message" \
            --auto-pr \
            --branch "fix/issue-$ISSUE_NUM"
```

### Nightly Security Audit

Run security analysis on a schedule:

```yaml
name: Nightly Security Audit

on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM daily
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Install Agentic
        run: npm install -g @agentic-dev-library/control @ai-sdk/anthropic

      - name: Run Security Audit
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          agentic sandbox run \
            "Perform comprehensive security audit" \
            --workspace . \
            --output ./security-audit \
            --timeout 900

      - name: Upload Report
        uses: actions/upload-artifact@v4
        with:
          name: security-audit-${{ github.run_id }}
          path: ./security-audit

      - name: Create Issue if Findings
        run: |
          if [ -f ./security-audit/findings.md ]; then
            FINDINGS=$(cat ./security-audit/findings.md)
            gh issue create \
              --title "Security Audit Findings - $(date +%Y-%m-%d)" \
              --body "$FINDINGS" \
              --label "security,automated"
          fi
```

## Using the Agentic Action

For convenience, you can use the official Agentic GitHub Action:

```yaml
name: Agentic Review

on:
  pull_request:

jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: agentic-dev-library/control@v1
        with:
          command: triage review
          args: --base ${{ github.base_ref }} --head ${{ github.sha }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
```

### Action Inputs

| Input | Description | Required |
|-------|-------------|----------|
| `command` | Agentic command to run | Yes |
| `args` | Command arguments | No |
| `config` | Path to config file | No |

## Security Best Practices

### Use GitHub Secrets

Never hardcode tokens:

```yaml
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  CURSOR_API_KEY: ${{ secrets.CURSOR_API_KEY }}
  ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
```

### Limit Permissions

Use minimal permissions:

```yaml
permissions:
  contents: read
  pull-requests: write
  issues: write
```

### Pin Action Versions

Use SHA pinning for third-party actions:

```yaml
- uses: actions/checkout@8ade135a41bc03ea155e62e844d188df1ea18608  # v4.1.0
```

## Complete Example Workflow

```yaml
name: Agentic CI/CD

on:
  push:
    branches: [main]
  pull_request:
  issues:
    types: [opened, labeled]
  schedule:
    - cron: '0 2 * * 1'  # Weekly Monday 2 AM

permissions:
  contents: read
  pull-requests: write
  issues: write

env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}

jobs:
  # Review PRs
  review:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - run: npm install -g @agentic-dev-library/control @ai-sdk/anthropic

      - name: AI Code Review
        run: |
          agentic triage review \
            --base origin/${{ github.base_ref }} \
            --head ${{ github.sha }} \
            -o review.md

          gh pr comment ${{ github.event.pull_request.number }} \
            --body-file review.md

  # Triage new issues
  triage-issue:
    if: github.event_name == 'issues' && github.event.action == 'opened'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - run: npm install -g @agentic-dev-library/control @ai-sdk/anthropic

      - name: Triage Issue
        run: |
          agentic triage quick \
            "${{ github.event.issue.title }}: ${{ github.event.issue.body }}" \
            --json > triage.json

          LABELS=$(jq -r '.labels | join(",")' triage.json)
          gh issue edit ${{ github.event.issue.number }} --add-label "$LABELS"

  # Spawn agent for ai-fix labeled issues
  spawn-agent:
    if: github.event_name == 'issues' && github.event.action == 'labeled' && github.event.label.name == 'ai-fix'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - run: npm install -g @agentic-dev-library/control

      - name: Spawn Agent
        env:
          CURSOR_API_KEY: ${{ secrets.CURSOR_API_KEY }}
        run: |
          agentic fleet spawn \
            "https://github.com/${{ github.repository }}" \
            "Fix issue #${{ github.event.issue.number }}: ${{ github.event.issue.title }}" \
            --auto-pr \
            --branch "fix/issue-${{ github.event.issue.number }}"

  # Weekly security audit
  security-audit:
    if: github.event_name == 'schedule'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - run: npm install -g @agentic-dev-library/control @ai-sdk/anthropic

      - name: Run Audit
        run: |
          agentic sandbox run \
            "Security audit focusing on OWASP Top 10" \
            --workspace . \
            --output ./audit \
            --timeout 900

      - uses: actions/upload-artifact@v4
        with:
          name: security-audit
          path: ./audit
```

## Next Steps

- [Vercel AI SDK Integration](/integrations/vercel-ai-sdk/) - Deep AI integration
- [CLI Workflows](/examples/cli-workflows/) - More CLI examples
- [Fleet API Reference](/api/fleet-management/) - Complete API docs
