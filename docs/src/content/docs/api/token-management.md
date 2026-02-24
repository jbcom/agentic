---
title: Token Management API
description: API reference for @jbcom/agentic token management
---

# Token Management API Reference

Complete reference for token management functions from `@jbcom/agentic`.

## Overview

The token management system automatically routes GitHub operations to the correct tokens based on repository organization. This eliminates manual token switching when working across multiple organizations.

## Import

```typescript
import {
  getTokenForRepo,
  setTokenConfig,
  addOrganization,
  getTokenConfig,
  validateTokens,
} from '@jbcom/agentic';
```

## Functions

### getTokenForRepo()

Get the appropriate token for a repository.

```typescript
getTokenForRepo(repoOrFullName: string): string | undefined
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `repoOrFullName` | `string` | Repo in `owner/name` format or full URL |

**Returns:** Token value or `undefined` if not found.

**Example:**
```typescript
// By owner/name
const token = getTokenForRepo('my-company/my-repo');
// Returns value of GITHUB_MY_COMPANY_TOKEN if configured

// By full URL
const token = getTokenForRepo('https://github.com/my-company/my-repo');
// Same result

// Unconfigured org falls back to default
const token = getTokenForRepo('unknown-org/repo');
// Returns value of GITHUB_TOKEN
```

**Token Resolution:**
1. Check if org is in `tokens.organizations` config
2. If found, return value of configured `tokenEnvVar`
3. If not found, return value of `defaultTokenEnvVar`
4. If no default, return `undefined`

---

### setTokenConfig()

Set the complete token configuration.

```typescript
setTokenConfig(config: TokenConfig): void
```

**Parameters:**

```typescript
interface TokenConfig {
  organizations: Record<string, OrganizationConfig>;
  defaultTokenEnvVar?: string;
  prReviewTokenEnvVar?: string;
}

interface OrganizationConfig {
  name: string;
  tokenEnvVar: string;
}
```

**Example:**
```typescript
setTokenConfig({
  organizations: {
    'my-company': {
      name: 'my-company',
      tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
    },
    'partner-org': {
      name: 'partner-org',
      tokenEnvVar: 'PARTNER_GH_PAT',
    },
  },
  defaultTokenEnvVar: 'GITHUB_TOKEN',
  prReviewTokenEnvVar: 'GITHUB_BOT_TOKEN',
});
```

---

### addOrganization()

Add a single organization configuration.

```typescript
addOrganization(config: OrganizationConfig): void
```

**Parameters:**

```typescript
interface OrganizationConfig {
  name: string;
  tokenEnvVar: string;
}
```

**Example:**
```typescript
addOrganization({
  name: 'new-org',
  tokenEnvVar: 'GITHUB_NEW_ORG_TOKEN',
});

// Token is now available
const token = getTokenForRepo('new-org/my-repo');
```

---

### getTokenConfig()

Get the current token configuration.

```typescript
getTokenConfig(): TokenConfig
```

**Returns:** Current `TokenConfig` object.

**Example:**
```typescript
const config = getTokenConfig();
console.log('Configured orgs:', Object.keys(config.organizations));
console.log('Default token:', config.defaultTokenEnvVar);
```

---

### validateTokens()

Validate that all configured tokens are available.

```typescript
validateTokens(): TokenValidationResult
```

**Returns:**

```typescript
interface TokenValidationResult {
  valid: boolean;
  missing: string[];
  available: string[];
}
```

**Example:**
```typescript
const result = validateTokens();

if (!result.valid) {
  console.error('Missing tokens:', result.missing.join(', '));
  process.exit(1);
}

console.log('All tokens available:', result.available.join(', '));
```

---

### getPRReviewToken()

Get the token configured for PR review operations.

```typescript
getPRReviewToken(): string | undefined
```

**Returns:** Value of `prReviewTokenEnvVar` or default token.

**Example:**
```typescript
const token = getPRReviewToken();
// Use this token for all PR reviews for consistent identity
```

## Configuration

### Config File

Token configuration can be set in `agentic.config.json`:

```json
{
  "tokens": {
    "organizations": {
      "my-company": {
        "name": "my-company",
        "tokenEnvVar": "GITHUB_COMPANY_TOKEN"
      },
      "open-source": {
        "name": "open-source",
        "tokenEnvVar": "GITHUB_OSS_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_TOKEN"
  }
}
```

### Environment Variables

Add organizations dynamically via environment variables:

```bash
# Pattern: AGENTIC_ORG_<NAME>_TOKEN=<ENV_VAR_NAME>
export AGENTIC_ORG_MYCOMPANY_TOKEN=GITHUB_MYCOMPANY_TOKEN
export AGENTIC_ORG_PARTNER_TOKEN=PARTNER_GH_PAT

# Set the default
export AGENTIC_DEFAULT_TOKEN=GITHUB_TOKEN

# Set the PR review token
export AGENTIC_PR_REVIEW_TOKEN=GITHUB_BOT_TOKEN
```

## Token Resolution Flow

```
Repository: my-company/my-repo
                    │
                    ▼
    ┌───────────────────────────────┐
    │ Check organizations config     │
    │ for "my-company"              │
    └───────────────┬───────────────┘
                    │
            ┌───────┴───────┐
            │               │
        Found           Not Found
            │               │
            ▼               ▼
    ┌───────────────┐ ┌───────────────┐
    │ Return value  │ │ Return value  │
    │ of configured │ │ of default    │
    │ tokenEnvVar   │ │ tokenEnvVar   │
    └───────────────┘ └───────────────┘
            │               │
            ▼               ▼
    GITHUB_COMPANY_TOKEN   GITHUB_TOKEN
```

## Usage with Fleet

The Fleet class automatically uses token management:

```typescript
import { Fleet, addOrganization } from '@jbcom/agentic';

// Configure organizations
addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
});

// Fleet automatically uses correct token
const fleet = new Fleet();

// This uses GITHUB_COMPANY_TOKEN automatically
await fleet.spawn({
  repository: 'https://github.com/my-company/my-repo',
  task: 'Fix the bug',
});
```

## Usage with GitHubClient

```typescript
import { GitHubClient, getTokenForRepo } from '@jbcom/agentic';

// Token is automatically selected
const client = new GitHubClient('my-company/my-repo');

// Or explicitly
const token = getTokenForRepo('my-company/my-repo');
const client = new GitHubClient('my-company/my-repo', token);
```

## Best Practices

### 1. Use Fine-Grained Tokens

Create separate tokens with minimal permissions:

```bash
# Personal repos - full access
export GITHUB_TOKEN="ghp_personal..."

# Work repos - only repo scope
export GITHUB_WORK_TOKEN="ghp_work..."

# Open source - only public repos
export GITHUB_OSS_TOKEN="ghp_oss..."
```

### 2. Use Bot Account for PR Reviews

Set up a dedicated bot for consistent identity:

```json
{
  "tokens": {
    "prReviewTokenEnvVar": "GITHUB_BOT_TOKEN"
  }
}
```

### 3. Validate on Startup

```typescript
import { validateTokens } from '@jbcom/agentic';

const result = validateTokens();
if (!result.valid) {
  console.error('Missing tokens:', result.missing);
  console.error('Please set the required environment variables');
  process.exit(1);
}
```

### 4. Never Hardcode Tokens

```typescript
// ❌ Bad
const token = 'ghp_xxx...';

// ✅ Good
const token = getTokenForRepo('my-org/my-repo');
```

## CLI Commands

```bash
# Check all token status
agentic tokens status

# Validate tokens are working
agentic tokens validate

# Get token for specific repo
agentic tokens for-repo my-org/my-repo
```

## Next Steps

- [Configuration Guide](/getting-started/configuration/) - Complete config reference
- [Fleet API Reference](/api/fleet-management/) - Fleet management
- [GitHub Actions Integration](/integrations/github-actions/) - CI/CD setup
