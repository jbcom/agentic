---
title: "Token Management API"
description: "Complete API reference for multi-organization GitHub token management in @jbcom/agentic."
---

# Token Management API Reference

The token management module provides automatic token switching based on repository organization. It eliminates manual token management when working across multiple GitHub organizations by routing API calls to the correct credentials.

## Import

```typescript
import {
  // Configuration
  getTokenConfig,
  setTokenConfig,
  resetTokenConfig,
  addOrganization,
  removeOrganization,

  // Token resolution
  getTokenForRepo,
  getTokenForOrg,
  getTokenEnvVar,
  getPRReviewToken,
  getPRReviewTokenEnvVar,

  // Organization helpers
  extractOrg,
  getOrgConfig,
  getConfiguredOrgs,
  isOrgConfigured,

  // Validation
  validateTokens,
  hasTokenForOrg,
  hasTokenForRepo,

  // Subprocess helpers
  getEnvForRepo,
  getEnvForPRReview,

  // Debugging
  getTokenSummary,
} from '@jbcom/agentic';
```

## Configuration

Token configuration can be set through three methods, listed in priority order:

1. **Programmatic**: `setTokenConfig()` / `addOrganization()`
2. **Config file**: `agentic.config.json` with `"tokens"` section
3. **Environment variables**: `AGENTIC_ORG_<NAME>_TOKEN` pattern

### TokenConfig Schema

```typescript
interface TokenConfig {
  /** Mapping of org name to configuration */
  organizations: Record<string, OrganizationConfig>;
  /** Default token env var when org is unknown (default: "GITHUB_TOKEN") */
  defaultTokenEnvVar: string;
  /** Token env var for PR reviews -- ensures consistent identity (default: "GITHUB_TOKEN") */
  prReviewTokenEnvVar: string;
}

interface OrganizationConfig {
  /** Organization name (e.g., "my-org") */
  name: string;
  /** Environment variable name for the token */
  tokenEnvVar: string;
  /** Default branch for repos in this org */
  defaultBranch?: string;
  /** Whether this is a GitHub Enterprise org */
  isEnterprise?: boolean;
}
```

---

## Configuration Functions

### setTokenConfig()

Set or merge token configuration. Organizations are merged (not replaced) with existing config.

```typescript
setTokenConfig(config: Partial<TokenConfig>): void
```

```typescript
import { setTokenConfig } from '@jbcom/agentic';

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

### getTokenConfig()

Get the current token configuration. Returns a shallow copy.

```typescript
getTokenConfig(): TokenConfig
```

```typescript
const config = getTokenConfig();
console.log('Configured orgs:', Object.keys(config.organizations));
console.log('Default token:', config.defaultTokenEnvVar);
console.log('PR review token:', config.prReviewTokenEnvVar);
```

### resetTokenConfig()

Reset configuration to defaults and reload from environment variables. Useful for testing.

```typescript
resetTokenConfig(): void
```

### addOrganization()

Add or update a single organization configuration.

```typescript
addOrganization(org: OrganizationConfig): void
```

```typescript
import { addOrganization } from '@jbcom/agentic';

addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_MYCOMPANY_TOKEN',
  defaultBranch: 'main',
  isEnterprise: true,
});

// Token is now available for repos in this org
const token = getTokenForRepo('my-company/my-repo');
```

### removeOrganization()

Remove an organization from the configuration.

```typescript
removeOrganization(orgName: string): void
```

---

## Token Resolution

### getTokenForRepo()

Get the actual token value for a repository URL. This is the primary resolution function.

```typescript
getTokenForRepo(repoUrl: string): string | undefined
```

**Resolution order:**

1. Extract organization from the URL using `extractOrg()`
2. Look up the organization in `config.organizations` (case-insensitive)
3. If found, return `process.env[org.tokenEnvVar]`
4. If not found, return `process.env[config.defaultTokenEnvVar]`

```typescript
import { getTokenForRepo, addOrganization } from '@jbcom/agentic';

addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
});

// By owner/name format
const token1 = getTokenForRepo('my-company/my-repo');
// Returns value of GITHUB_COMPANY_TOKEN

// By full URL
const token2 = getTokenForRepo('https://github.com/my-company/my-repo');
// Same result

// By SSH URL
const token3 = getTokenForRepo('git@github.com:my-company/my-repo.git');
// Same result

// Unknown org falls back to default
const token4 = getTokenForRepo('unknown-org/repo');
// Returns value of GITHUB_TOKEN
```

### getTokenForOrg()

Get the token value for an organization name directly (without URL parsing).

```typescript
getTokenForOrg(org: string): string | undefined
```

```typescript
const token = getTokenForOrg('my-company');
// Returns process.env[configured_env_var] or process.env[default]
```

### getTokenEnvVar()

Get the environment variable name (not the value) for an organization. Case-insensitive lookup with exact match priority.

```typescript
getTokenEnvVar(org: string): string
```

```typescript
const envVar = getTokenEnvVar('my-company');
// Returns "GITHUB_COMPANY_TOKEN" (or defaultTokenEnvVar if not configured)
```

### getPRReviewToken()

Get the token designated for PR review operations. This ensures all PR reviews use a consistent identity (e.g., a bot account).

```typescript
getPRReviewToken(): string | undefined
```

```typescript
const token = getPRReviewToken();
// Returns process.env[prReviewTokenEnvVar]
```

### getPRReviewTokenEnvVar()

Get the environment variable name for the PR review token.

```typescript
getPRReviewTokenEnvVar(): string
```

---

## Organization Helpers

### extractOrg()

Extract the organization name from a repository URL or `owner/repo` string. Uses safe regex patterns to prevent ReDoS attacks.

```typescript
extractOrg(repoUrl: string): string | null
```

**Supported formats:**

```typescript
extractOrg('https://github.com/my-org/my-repo');     // "my-org"
extractOrg('my-org/my-repo');                          // "my-org"
extractOrg('git@github.com:my-org/my-repo.git');       // "my-org"
extractOrg('not-a-repo');                              // null
```

### getOrgConfig()

Get the full organization configuration object. Case-insensitive lookup with exact match priority.

```typescript
getOrgConfig(org: string): OrganizationConfig | undefined
```

### getConfiguredOrgs()

List all configured organization names.

```typescript
getConfiguredOrgs(): string[]
```

```typescript
const orgs = getConfiguredOrgs();
console.log('Configured orgs:', orgs);
// ["my-company", "partner-org"]
```

### isOrgConfigured()

Check if an organization has been configured (case-insensitive).

```typescript
isOrgConfigured(org: string): boolean
```

---

## Validation

### validateTokens()

Validate that all configured tokens are actually available as environment variables.

```typescript
validateTokens(orgs?: string[]): Result<string[]>
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `orgs` | `string[]` | All configured orgs | Specific organizations to validate |

**Returns:** `Result<string[]>` where `data` contains the list of missing tokens and `success` is `true` if no tokens are missing.

Validates:
- All specified organization tokens
- The PR review token
- The default token

```typescript
import { validateTokens } from '@jbcom/agentic';

const result = validateTokens();
if (!result.success) {
  console.error('Missing tokens:');
  for (const missing of result.data ?? []) {
    console.error(`  - ${missing}`);
  }
  // e.g., "my-company: GITHUB_COMPANY_TOKEN not set"
  // e.g., "PR Review: GITHUB_TOKEN not set"
  process.exit(1);
}
```

### hasTokenForOrg()

Quick check if a valid (non-empty) token is available for an organization.

```typescript
hasTokenForOrg(org: string): boolean
```

### hasTokenForRepo()

Quick check if a valid token is available for a repository.

```typescript
hasTokenForRepo(repoUrl: string): boolean
```

---

## Subprocess Helpers

### getEnvForRepo()

Create an environment variables object for child processes targeting a specific repository. Sets both `GH_TOKEN` and `GITHUB_TOKEN`.

```typescript
getEnvForRepo(repoUrl: string): Record<string, string>
```

```typescript
import { spawnSync } from 'node:child_process';
import { getEnvForRepo } from '@jbcom/agentic';

const proc = spawnSync('gh', ['pr', 'list'], {
  env: { ...process.env, ...getEnvForRepo('my-company/my-repo') },
});
// Uses GITHUB_COMPANY_TOKEN for GH_TOKEN and GITHUB_TOKEN
```

### getEnvForPRReview()

Create environment variables for PR review operations using the PR review token.

```typescript
getEnvForPRReview(): Record<string, string>
```

```typescript
const proc = spawnSync('gh', ['pr', 'review', '42', '--approve'], {
  env: { ...process.env, ...getEnvForPRReview() },
});
// Uses the PR review token identity
```

---

## Debugging

### getTokenSummary()

Get a summary of all configured tokens and their availability. Useful for diagnostic output.

```typescript
getTokenSummary(): Record<string, {
  envVar: string;
  available: boolean;
  configured: boolean;
}>
```

```typescript
import { getTokenSummary } from '@jbcom/agentic';

const summary = getTokenSummary();
for (const [name, info] of Object.entries(summary)) {
  const status = info.available ? 'OK' : 'MISSING';
  console.log(`${name}: ${info.envVar} [${status}]`);
}
// my-company: GITHUB_COMPANY_TOKEN [OK]
// partner-org: PARTNER_GH_PAT [MISSING]
// _default: GITHUB_TOKEN [OK]
// _pr_review: GITHUB_BOT_TOKEN [OK]
```

---

## Environment Variable Patterns

### Automatic Organization Loading

On module initialization, the token system scans `process.env` for variables matching the pattern `AGENTIC_ORG_<NAME>_TOKEN` and automatically registers them.

```bash
# Pattern: AGENTIC_ORG_<UPPERCASE_NAME>_TOKEN=<TOKEN_ENV_VAR_NAME>
export AGENTIC_ORG_MYCOMPANY_TOKEN=GITHUB_MYCOMPANY_TOKEN
export AGENTIC_ORG_PARTNER_TOKEN=PARTNER_GH_PAT
```

The `<NAME>` portion is converted to lowercase and underscores become hyphens:
- `AGENTIC_ORG_MYCOMPANY_TOKEN` registers org `mycompany`
- `AGENTIC_ORG_MY_COMPANY_TOKEN` registers org `my-company`

### Override Variables

```bash
# Override the default token env var
export AGENTIC_DEFAULT_TOKEN=GITHUB_TOKEN

# Override the PR review token env var
export AGENTIC_PR_REVIEW_TOKEN=GITHUB_BOT_TOKEN
```

---

## Config File Format

Token configuration in `agentic.config.json`:

```json
{
  "tokens": {
    "organizations": {
      "my-company": {
        "name": "my-company",
        "tokenEnvVar": "GITHUB_COMPANY_TOKEN",
        "defaultBranch": "main",
        "isEnterprise": false
      },
      "open-source": {
        "name": "open-source",
        "tokenEnvVar": "GITHUB_OSS_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_BOT_TOKEN"
  }
}
```

---

## Token Resolution Flow

```
Repository: "https://github.com/my-company/my-repo"
                        |
                        v
        +-------------------------------+
        | extractOrg() -> "my-company"  |
        +---------------+---------------+
                        |
                +-------+-------+
                |               |
           Configured       Not Found
                |               |
                v               v
        +---------------+ +-------------------+
        | Return value  | | Return value      |
        | of configured | | of default        |
        | tokenEnvVar   | | tokenEnvVar       |
        +---------------+ +-------------------+
                |               |
                v               v
        GITHUB_COMPANY_TOKEN   GITHUB_TOKEN
```

---

## Usage with Fleet

The Fleet class automatically uses token management for all GitHub operations:

```typescript
import { Fleet, addOrganization } from '@jbcom/agentic';

// Configure organizations
addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
});

const fleet = new Fleet();

// Fleet uses GITHUB_COMPANY_TOKEN automatically for this repo
await fleet.spawn({
  repository: 'https://github.com/my-company/my-repo',
  task: 'Fix the authentication bug',
  target: { autoCreatePr: true },
});
```

---

## CLI Commands

```bash
# Show token status for all configured organizations
agentic tokens status

# Validate that all configured tokens are set
agentic tokens validate

# Get the token env var for a specific repo
agentic tokens for-repo my-org/my-repo
```

---

## Best Practices

### Use Fine-Grained Tokens

Create separate GitHub Personal Access Tokens with minimal scopes:

```bash
# Personal repos -- full access
export GITHUB_TOKEN="ghp_personal..."

# Work repos -- only repo scope
export GITHUB_WORK_TOKEN="ghp_work..."

# Open source contributions -- only public repos
export GITHUB_OSS_TOKEN="ghp_oss..."
```

### Use a Bot Account for PR Reviews

Configure a dedicated bot identity so all automated PR reviews appear consistently:

```json
{
  "tokens": {
    "prReviewTokenEnvVar": "GITHUB_BOT_TOKEN"
  }
}
```

### Validate on Startup

Check token availability early to fail fast:

```typescript
import { validateTokens } from '@jbcom/agentic';

const result = validateTokens();
if (!result.success) {
  console.error('Missing tokens:', result.data);
  process.exit(1);
}
```

### Never Hardcode Tokens

```typescript
// Bad -- hardcoded token
const token = 'ghp_xxx...';

// Good -- resolved from configuration
const token = getTokenForRepo('my-org/my-repo');
```

---

## Related Pages

- [Configuration API](/api/configuration/) -- Full config schema reference
- [Fleet API Reference](/api/fleet-management/) -- Fleet management
- [Getting Started: Configuration](/getting-started/configuration/) -- Setup guide
