---
title: Configuration API
description: API reference for @jbcom/agentic configuration
---

# Configuration API Reference

Complete reference for configuration functions and types from `@jbcom/agentic`.

## Overview

Agentic uses [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig) for configuration management, supporting multiple file formats and locations.

## Import

```typescript
import {
  loadConfig,
  getConfig,
  setConfig,
  mergeConfig,
} from '@jbcom/agentic';
```

## Functions

### loadConfig()

Load configuration from file.

```typescript
loadConfig(searchPath?: string): Promise<AgenticConfig>
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `searchPath` | `string` | Directory to search (default: `process.cwd()`) |

**Returns:** `AgenticConfig` object.

**Search Locations (in order):**
1. `agentic.config.json`
2. `agentic.config.js`
3. `.agenticrc`
4. `.agenticrc.json`
5. `package.json` → `"agentic"` key

**Example:**
```typescript
const config = await loadConfig();
console.log('Default repo:', config.defaultRepository);

// Search specific directory
const config = await loadConfig('/path/to/project');
```

---

### getConfig()

Get the current in-memory configuration.

```typescript
getConfig(): AgenticConfig
```

**Returns:** Current `AgenticConfig` object.

**Example:**
```typescript
const config = getConfig();
console.log('Log level:', config.logLevel);
console.log('Triage provider:', config.triage?.provider);
```

---

### setConfig()

Set the complete configuration.

```typescript
setConfig(config: AgenticConfig): void
```

**Example:**
```typescript
setConfig({
  defaultRepository: 'my-org/my-repo',
  logLevel: 'debug',
  tokens: {
    organizations: {},
    defaultTokenEnvVar: 'GITHUB_TOKEN',
  },
  fleet: {
    autoCreatePr: true,
  },
  triage: {
    provider: 'anthropic',
    model: 'claude-sonnet-4-20250514',
    apiKeyEnvVar: 'ANTHROPIC_API_KEY',
  },
});
```

---

### mergeConfig()

Merge partial configuration with current config.

```typescript
mergeConfig(partial: Partial<AgenticConfig>): AgenticConfig
```

**Returns:** Merged `AgenticConfig` object.

**Example:**
```typescript
// Only update specific fields
mergeConfig({
  logLevel: 'debug',
  triage: {
    model: 'claude-opus-4-20250514',
  },
});
```

## Types

### AgenticConfig

Complete configuration object.

```typescript
interface AgenticConfig {
  // Default repository for operations
  defaultRepository?: string;
  
  // Logging level
  logLevel?: 'debug' | 'info' | 'warn' | 'error';
  
  // Token configuration
  tokens?: TokenConfig;
  
  // Fleet configuration
  fleet?: FleetConfig;
  
  // Triage configuration
  triage?: TriageConfig;
}
```

---

### TokenConfig

Token management configuration.

```typescript
interface TokenConfig {
  // Organization-specific tokens
  organizations: Record<string, OrganizationConfig>;
  
  // Default token for unconfigured orgs
  defaultTokenEnvVar?: string;
  
  // Token for PR review operations
  prReviewTokenEnvVar?: string;
}

interface OrganizationConfig {
  name: string;
  tokenEnvVar: string;
}
```

---

### FleetConfig

Fleet management configuration.

```typescript
interface FleetConfig {
  // Automatically create PR when agent completes
  autoCreatePr?: boolean;
  
  // Open PR as Cursor GitHub App
  openAsCursorGithubApp?: boolean;
  
  // Don't add user as reviewer
  skipReviewerRequest?: boolean;
}
```

---

### TriageConfig

AI triage configuration.

```typescript
interface TriageConfig {
  // AI provider
  provider?: 'anthropic' | 'openai' | 'google' | 'mistral' | 'azure';
  
  // Model name
  model?: string;
  
  // Environment variable containing API key
  apiKeyEnvVar?: string;
}
```

## Configuration File Examples

### JSON Format

```json
{
  "defaultRepository": "my-org/my-repo",
  "logLevel": "info",
  "tokens": {
    "organizations": {
      "my-company": {
        "name": "my-company",
        "tokenEnvVar": "GITHUB_COMPANY_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_TOKEN"
  },
  "fleet": {
    "autoCreatePr": true,
    "openAsCursorGithubApp": false
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

### JavaScript Format

```javascript
// agentic.config.js
module.exports = {
  defaultRepository: process.env.AGENTIC_REPOSITORY || 'my-org/my-repo',
  logLevel: process.env.NODE_ENV === 'development' ? 'debug' : 'info',
  tokens: {
    organizations: {
      'my-company': {
        name: 'my-company',
        tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
      },
    },
    defaultTokenEnvVar: 'GITHUB_TOKEN',
  },
  fleet: {
    autoCreatePr: true,
  },
  triage: {
    provider: 'anthropic',
    model: 'claude-sonnet-4-20250514',
    apiKeyEnvVar: 'ANTHROPIC_API_KEY',
  },
};
```

### In package.json

```json
{
  "name": "my-project",
  "agentic": {
    "defaultRepository": "my-org/my-repo",
    "triage": {
      "provider": "anthropic"
    }
  }
}
```

## Environment Variable Overrides

Environment variables take precedence over config files:

| Variable | Overrides |
|----------|-----------|
| `AGENTIC_REPOSITORY` | `defaultRepository` |
| `AGENTIC_PROVIDER` | `triage.provider` |
| `AGENTIC_MODEL` | `triage.model` |
| `AGENTIC_LOG_LEVEL` | `logLevel` |

```bash
# Override via environment
export AGENTIC_REPOSITORY="other-org/other-repo"
export AGENTIC_MODEL="gpt-4o"
```

## Provider Configuration

### Anthropic (Default)

```json
{
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

Available models:
- `claude-opus-4-20250514`
- `claude-sonnet-4-20250514`
- `claude-3-5-sonnet-20241022`
- `claude-3-haiku-20240307`

### OpenAI

```json
{
  "triage": {
    "provider": "openai",
    "model": "gpt-4o",
    "apiKeyEnvVar": "OPENAI_API_KEY"
  }
}
```

Available models:
- `gpt-4o`
- `gpt-4-turbo`
- `gpt-4`
- `gpt-3.5-turbo`

### Google

```json
{
  "triage": {
    "provider": "google",
    "model": "gemini-2.0-flash",
    "apiKeyEnvVar": "GOOGLE_API_KEY"
  }
}
```

### Mistral

```json
{
  "triage": {
    "provider": "mistral",
    "model": "mistral-large-latest",
    "apiKeyEnvVar": "MISTRAL_API_KEY"
  }
}
```

### Azure OpenAI

```json
{
  "triage": {
    "provider": "azure",
    "model": "your-deployment-name",
    "apiKeyEnvVar": "AZURE_API_KEY"
  }
}
```

## CLI Configuration

Initialize configuration interactively:

```bash
# Interactive mode
agentic init

# Non-interactive with defaults
agentic init --non-interactive
```

The `init` command:
1. Detects Git repository from remotes
2. Scans for existing tokens in environment
3. Prompts for missing configuration
4. Generates `agentic.config.json`

## Validation

Validate configuration at runtime:

```typescript
import { loadConfig, validateConfig } from '@jbcom/agentic';

const config = await loadConfig();
const result = validateConfig(config);

if (!result.valid) {
  console.error('Configuration errors:', result.errors);
  process.exit(1);
}
```

## Next Steps

- [Token Management API](/api/token-management/) - Token functions
- [Fleet API Reference](/api/fleet-management/) - Fleet management
- [Configuration Guide](/getting-started/configuration/) - User guide
