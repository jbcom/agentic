---
title: Configuration
description: Complete guide to configuring Agentic for your workflow
---

# Configuration Guide

Agentic uses a flexible configuration system based on [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig). This guide covers all configuration options and best practices.

## Configuration File Locations

Agentic searches for configuration in these locations (in order of priority):

1. `agentic.config.json`
2. `agentic.config.js`
3. `.agenticrc`
4. `.agenticrc.json`
5. `package.json` → `"agentic"` key

## Complete Configuration Reference

```json
{
  "tokens": {
    "organizations": {
      "my-company": {
        "name": "my-company",
        "tokenEnvVar": "GITHUB_COMPANY_TOKEN"
      },
      "open-source-org": {
        "name": "open-source-org",
        "tokenEnvVar": "GITHUB_OSS_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_TOKEN"
  },
  "defaultRepository": "my-company/my-repo",
  "logLevel": "info",
  "fleet": {
    "autoCreatePr": true,
    "openAsCursorGithubApp": false,
    "skipReviewerRequest": false
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

## Token Configuration

### Multi-Organization Setup

The most powerful feature of Agentic is automatic token routing based on repository organization:

```json
{
  "tokens": {
    "organizations": {
      "my-company": {
        "name": "my-company",
        "tokenEnvVar": "GITHUB_COMPANY_TOKEN"
      },
      "partner-org": {
        "name": "partner-org",
        "tokenEnvVar": "PARTNER_GH_PAT"
      },
      "agentic-dev-library": {
        "name": "agentic-dev-library",
        "tokenEnvVar": "GITHUB_AGENTIC_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_TOKEN"
  }
}
```

**How it works:**
```
Repository                    → Token Used
────────────────────────────────────────────
my-company/repo-1             → GITHUB_COMPANY_TOKEN
my-company/repo-2             → GITHUB_COMPANY_TOKEN
partner-org/shared-lib        → PARTNER_GH_PAT
unknown-org/repo              → GITHUB_TOKEN (default)
```

### Dynamic Organization Configuration

Add organizations via environment variables without editing the config file:

```bash
# Pattern: AGENTIC_ORG_<NAME>_TOKEN=<ENV_VAR_NAME>
export AGENTIC_ORG_MYCOMPANY_TOKEN=GITHUB_MYCOMPANY_TOKEN
export AGENTIC_ORG_PARTNER_TOKEN=PARTNER_GH_PAT
```

### PR Review Token

Configure a consistent identity for all PR review operations:

```json
{
  "tokens": {
    "prReviewTokenEnvVar": "GITHUB_BOT_TOKEN"
  }
}
```

This ensures all PR reviews appear from the same account, regardless of which repository is being reviewed.

## Fleet Configuration

```json
{
  "fleet": {
    "autoCreatePr": false,
    "openAsCursorGithubApp": false,
    "skipReviewerRequest": false
  }
}
```

| Option | Description | Default |
|--------|-------------|---------|
| `autoCreatePr` | Auto-create PR when agent completes | `false` |
| `openAsCursorGithubApp` | Open PR as Cursor GitHub App | `false` |
| `skipReviewerRequest` | Don't add user as reviewer | `false` |

**CLI overrides:**
```bash
agentic fleet spawn <repo> <task> --auto-pr        # Override autoCreatePr
agentic fleet spawn <repo> <task> --as-app         # Override openAsCursorGithubApp
agentic fleet spawn <repo> <task> --branch my-fix  # Custom branch name
```

## AI Provider Configuration

### Supported Providers

| Provider | Package | Config Value | Default API Key |
|----------|---------|--------------|-----------------|
| **Anthropic** | `@ai-sdk/anthropic` | `anthropic` | `ANTHROPIC_API_KEY` |
| **OpenAI** | `@ai-sdk/openai` | `openai` | `OPENAI_API_KEY` |
| **Google AI** | `@ai-sdk/google` | `google` | `GOOGLE_API_KEY` |
| **Mistral** | `@ai-sdk/mistral` | `mistral` | `MISTRAL_API_KEY` |
| **Azure** | `@ai-sdk/azure` | `azure` | `AZURE_API_KEY` |

### Anthropic Configuration (Recommended)

```json
{
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

### OpenAI Configuration

```json
{
  "triage": {
    "provider": "openai",
    "model": "gpt-4o",
    "apiKeyEnvVar": "OPENAI_API_KEY"
  }
}
```

### Google AI Configuration

```json
{
  "triage": {
    "provider": "google",
    "model": "gemini-2.0-flash",
    "apiKeyEnvVar": "GOOGLE_API_KEY"
  }
}
```

## Environment Variables Reference

### Required

| Variable | Description |
|----------|-------------|
| `GITHUB_TOKEN` | Default GitHub token for unconfigured orgs |

### Optional

| Variable | Description |
|----------|-------------|
| `GITHUB_<ORG>_TOKEN` | Organization-specific GitHub tokens |
| `ANTHROPIC_API_KEY` | Anthropic API key for Claude models |
| `OPENAI_API_KEY` | OpenAI API key |
| `GOOGLE_API_KEY` | Google AI API key |
| `MISTRAL_API_KEY` | Mistral API key |
| `CURSOR_API_KEY` | Cursor API key for fleet operations |
| `AGENTIC_MODEL` | Default AI model override |
| `AGENTIC_PROVIDER` | Default AI provider override |
| `AGENTIC_REPOSITORY` | Default repository override |
| `AGENTIC_LOG_LEVEL` | Log level (debug/info/warn/error) |

## Programmatic Configuration

Configure Agentic programmatically in your TypeScript/JavaScript code:

```typescript
import { 
  setTokenConfig,
  addOrganization,
} from '@agentic-dev-library/control';

// Configure everything at once
setTokenConfig({
  organizations: {
    "my-company": { 
      name: "my-company", 
      tokenEnvVar: "GITHUB_COMPANY_TOKEN" 
    },
    "partner-org": {
      name: "partner-org",
      tokenEnvVar: "PARTNER_GH_PAT"
    }
  },
  defaultTokenEnvVar: "GITHUB_TOKEN",
  prReviewTokenEnvVar: "GITHUB_BOT_TOKEN",
});

// Or add organizations one at a time
addOrganization({
  name: "new-org",
  tokenEnvVar: "GITHUB_NEW_ORG_TOKEN",
});
```

## Configuration Validation

Validate your configuration before running:

```bash
# Check all token status
agentic tokens status

# Validate required tokens
agentic tokens validate

# Show token for a specific repo
agentic tokens for-repo my-company/my-repo
```

## Best Practices

### 1. Use Environment Variables for Secrets

Never commit tokens to your config file. Always use environment variable references:

```json
{
  "tokens": {
    "organizations": {
      "my-org": {
        "tokenEnvVar": "GITHUB_MY_ORG_TOKEN"  // ✅ Good
      }
    }
  }
}
```

### 2. Create Separate Tokens per Organization

Use fine-grained personal access tokens with minimal scopes for each organization:

```bash
# Token for your personal repos
export GITHUB_TOKEN="ghp_personal..."

# Token for work repos (repo scope only)
export GITHUB_WORK_TOKEN="ghp_work..."

# Token for open source contributions
export GITHUB_OSS_TOKEN="ghp_oss..."
```

### 3. Use PR Review Bot Token

Set up a dedicated bot account for PR reviews to have consistent identity:

```json
{
  "tokens": {
    "prReviewTokenEnvVar": "GITHUB_BOT_TOKEN"
  }
}
```

### 4. Configure Defaults Per Project

Create project-specific configs with sensible defaults:

```json
{
  "defaultRepository": "my-company/main-app",
  "fleet": {
    "autoCreatePr": true
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514"
  }
}
```

## Next Steps

- **[Agent Spawning Guide](/guides/agent-spawning/)** - Put your config to use
- **[Fleet Management](/guides/fleet-management/)** - Scale up operations
- **[Token Management API](/api/token-management/)** - Programmatic token control
