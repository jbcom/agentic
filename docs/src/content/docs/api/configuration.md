---
title: "Configuration API"
description: "Complete API reference for the configuration system in @jbcom/agentic, including cosmiconfig loading, schema validation, and environment variable overrides."
---

# Configuration API Reference

The configuration system uses [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig) for file discovery and [Zod](https://zod.dev/) for schema validation. Configuration is loaded from multiple sources with a clear priority order: CLI arguments > environment variables > config file > defaults.

## Import

```typescript
import {
  // Loading
  initConfig,
  loadConfigFromPath,

  // Getters
  getConfig,
  getConfigPath,
  getConfigValue,

  // Setters
  setConfig,
  resetConfig,

  // Convenience
  getTriageConfig,
  getFleetDefaults,
  getDefaultModel,
  getLogLevel,
  getCursorApiKey,
  getTriageApiKey,
  getDefaultApiKeyEnvVar,
  isVerbose,

  // Logging
  log,
} from '@jbcom/agentic';
```

---

## Config File Discovery

cosmiconfig searches for configuration in these locations (first match wins):

1. `package.json` -- `"agentic"` key
2. `agentic.config.json`
3. `.agenticrc`
4. `.agenticrc.json`

**Security note:** JavaScript config files (`agentic.config.js`, `.agenticrc.cjs`) are deliberately disabled to prevent arbitrary code execution during `require()`. Only JSON-based formats are supported.

---

## Full Configuration Schema

### AgenticConfig

```typescript
interface AgenticConfig {
  /** Token configuration for multi-org GitHub access */
  tokens?: Partial<TokenConfig>;

  /** Default repository for fleet operations (owner/repo format) */
  defaultRepository?: string;

  /** PR number for fleet coordination */
  coordinationPr?: number;

  /** Log level (default: "info") */
  logLevel?: 'debug' | 'info' | 'warn' | 'error';

  /** Enable verbose output */
  verbose?: boolean;

  /** Cursor API configuration */
  cursor?: {
    /** API key environment variable name (default: "CURSOR_API_KEY") */
    apiKeyEnvVar?: string;
    /** Base URL for Cursor API (only via programmatic config, not env) */
    baseUrl?: string;
  };

  /** Fleet default options */
  fleet?: FleetConfig;

  /** Triage (AI analysis) configuration */
  triage?: TriageConfig;

  /** MCP server configuration */
  mcp?: MCPConfig;
}
```

### FleetConfig

```typescript
interface FleetConfig {
  /** Auto-create PR when agent completes */
  autoCreatePr?: boolean;
  /** Open PR as Cursor GitHub App */
  openAsCursorGithubApp?: boolean;
  /** Skip adding user as reviewer */
  skipReviewerRequest?: boolean;
}
```

### TriageConfig

```typescript
interface TriageConfig {
  /** AI provider: anthropic, openai, google, mistral, azure */
  provider?: string;
  /** Model ID for the provider (default: "claude-sonnet-4-20250514") */
  model?: string;
  /** API key environment variable name */
  apiKeyEnvVar?: string;
}
```

### MCPConfig

```typescript
interface MCPConfig {
  /** Cursor Background Agent MCP */
  cursor?: MCPServerConfig;
  /** GitHub MCP */
  github?: MCPServerConfig;
  /** Context7 documentation MCP */
  context7?: MCPServerConfig;
  /** 21st.dev Magic MCP */
  '21st-magic'?: MCPServerConfig;
  /** Custom MCP servers (any key) */
  [key: string]: MCPServerConfig | undefined;
}

interface MCPServerConfig {
  /** Whether this MCP server is enabled */
  enabled?: boolean;
  /** Environment variable name for the API key/token */
  tokenEnvVar?: string;
  /** Fallback env vars to try if primary not found */
  tokenEnvVarFallbacks?: string[];
  /** Transport mode: stdio or proxy */
  mode?: 'stdio' | 'proxy';
  /** Command to run for stdio transport */
  command?: string;
  /** Arguments for the command */
  args?: string[];
  /** Proxy URL for proxy mode (must be a valid URL) */
  proxyUrl?: string;
}
```

### TokenConfig

See [Token Management API](/api/token-management/) for the complete `TokenConfig` schema.

---

## Loading Functions

### initConfig()

Initialize configuration from all sources. This is the primary loading function, typically called once at startup.

```typescript
initConfig(overrides?: Partial<AgenticConfig>): AgenticConfig
```

**Priority order:**

1. Search for config file via cosmiconfig
2. Validate file config against Zod schema
3. Merge environment variable overrides
4. Apply programmatic `overrides` parameter
5. Validate final merged configuration
6. Apply token configuration to the token module

```typescript
import { initConfig } from '@jbcom/agentic';

// Basic initialization
const config = initConfig();

// With overrides
const config = initConfig({
  logLevel: 'debug',
  fleet: { autoCreatePr: true },
});
```

### loadConfigFromPath()

Load configuration from a specific file path instead of searching.

```typescript
loadConfigFromPath(filepath: string): AgenticConfig
```

```typescript
import { loadConfigFromPath } from '@jbcom/agentic';

const config = loadConfigFromPath('/path/to/agentic.config.json');
```

**Throws:** `Error` if the file cannot be loaded or is empty.

---

## Getter Functions

### getConfig()

Get the current configuration. Automatically calls `initConfig()` on first access if not already loaded.

```typescript
getConfig(): AgenticConfig
```

```typescript
const config = getConfig();
console.log('Default repo:', config.defaultRepository);
console.log('Log level:', config.logLevel);
console.log('Provider:', config.triage?.provider);
```

### getConfigPath()

Get the filesystem path of the loaded config file.

```typescript
getConfigPath(): string | null
```

Returns `null` if no config file was found (using defaults only).

### getConfigValue()

Get a specific top-level configuration value. Triggers `initConfig()` if needed.

```typescript
getConfigValue<K extends keyof AgenticConfig>(key: K): AgenticConfig[K]
```

```typescript
const logLevel = getConfigValue('logLevel');
const repo = getConfigValue('defaultRepository');
```

### getTriageConfig()

Get the triage configuration with defaults applied.

```typescript
getTriageConfig(): TriageConfig
```

**Default values:**

```typescript
{
  provider: 'anthropic',
  model: 'claude-sonnet-4-20250514',
  apiKeyEnvVar: 'ANTHROPIC_API_KEY',
}
```

### getFleetDefaults()

Get fleet default configuration.

```typescript
getFleetDefaults(): FleetConfig
```

### getDefaultModel()

Get the default AI model name. Deprecated in favor of `getTriageConfig().model`.

```typescript
getDefaultModel(): string
// Returns triage config model or "claude-sonnet-4-20250514"
```

### getLogLevel()

Get the current log level.

```typescript
getLogLevel(): string
// Returns config.logLevel or "info"
```

### isVerbose()

Check if verbose mode is enabled.

```typescript
isVerbose(): boolean
```

### getCursorApiKey()

Get the Cursor API key from the configured environment variable.

```typescript
getCursorApiKey(): string | undefined
```

Reads from `config.cursor.apiKeyEnvVar` (default: `CURSOR_API_KEY`).

### getTriageApiKey()

Get the triage API key for the configured or specified provider.

```typescript
getTriageApiKey(providerOverride?: string): string | undefined
```

```typescript
// Use configured provider
const key = getTriageApiKey();

// Override provider
const openaiKey = getTriageApiKey('openai');
```

### getDefaultApiKeyEnvVar()

Get the default API key environment variable name for a given provider.

```typescript
getDefaultApiKeyEnvVar(provider?: string): string
```

| Provider | Default Env Var |
|----------|----------------|
| `anthropic` (default) | `ANTHROPIC_API_KEY` |
| `openai` | `OPENAI_API_KEY` |
| `google` | `GOOGLE_API_KEY` |
| `mistral` | `MISTRAL_API_KEY` |
| `azure` | `AZURE_API_KEY` |
| `ollama` | `OLLAMA_API_KEY` |

---

## Setter Functions

### setConfig()

Update configuration at runtime. Merges with the existing configuration (deep merge for nested objects).

```typescript
setConfig(updates: Partial<AgenticConfig>): void
```

```typescript
import { setConfig } from '@jbcom/agentic';

setConfig({
  logLevel: 'debug',
  triage: { model: 'claude-opus-4-20250514' },
});
```

If `updates` includes `tokens`, the token module is also updated via `setTokenConfig()`.

### resetConfig()

Reset all configuration to unloaded state. The next call to `getConfig()` will re-initialize.

```typescript
resetConfig(): void
```

---

## Logging

The `log` object provides level-aware logging that respects `config.logLevel`:

```typescript
import { log } from '@jbcom/agentic';

log.debug('Detailed debugging information');
log.info('Normal operational messages');
log.warn('Warning conditions');
log.error('Error conditions');
```

Each method only outputs if the configured log level permits it. Levels in ascending severity order: `debug` (0) < `info` (1) < `warn` (2) < `error` (3).

Output format: `[agentic:<level>] <message>`

---

## Environment Variable Overrides

Environment variables take precedence over config file values:

| Variable | Config Field | Description |
|----------|-------------|-------------|
| `AGENTIC_REPOSITORY` | `defaultRepository` | Default repository |
| `AGENTIC_PROVIDER` | `triage.provider` | AI provider name |
| `AGENTIC_MODEL` | `triage.model` | AI model name |
| `AGENTIC_LOG_LEVEL` | `logLevel` | Log level (debug/info/warn/error) |
| `AGENTIC_VERBOSE` | `verbose` | Enable verbose (`true` or `1`) |
| `AGENTIC_COORDINATION_PR` | `coordinationPr` | Coordination PR number (positive int) |

```bash
# Override provider and model via environment
export AGENTIC_PROVIDER=openai
export AGENTIC_MODEL=gpt-4o
export AGENTIC_LOG_LEVEL=debug
export AGENTIC_VERBOSE=true
```

---

## Validation

Configuration is validated at load time using Zod schemas. The `AgenticConfigSchema` is exported for custom validation:

```typescript
import { AgenticConfigSchema } from '@jbcom/agentic';
import { validateConfig } from '@jbcom/agentic';

// Validate an arbitrary object
try {
  validateConfig(myConfig);
} catch (error) {
  // ConfigurationError with:
  //   - message: "Invalid configuration at 'triage.provider': Invalid enum value..."
  //   - code: ConfigErrorCode.INVALID_SCHEMA
  //   - field: "triage.provider"
}
```

Additional validation utilities:

```typescript
import {
  validateEnvVar,
  validateEnvVarWithMessage,
  validateRepository,
  validateGitRef,
  validatePositiveInt,
} from '@jbcom/agentic';

// Validate required environment variable
const apiKey = validateEnvVar('ANTHROPIC_API_KEY', 'Anthropic API key');

// Validate with custom error message
const token = validateEnvVarWithMessage('GITHUB_TOKEN', 'Fleet operations');

// Validate repository format
validateRepository('my-org/my-repo');  // OK
validateRepository('not-valid');        // Throws ConfigurationError

// Validate git ref
validateGitRef('main');               // OK
validateGitRef('feature/my-branch');  // OK

// Validate positive integer
const prNumber = validatePositiveInt('42', 'PR number');
```

---

## Complete Configuration Example

```json
{
  "defaultRepository": "my-org/my-repo",
  "logLevel": "info",
  "verbose": false,
  "coordinationPr": 42,
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
  },
  "cursor": {
    "apiKeyEnvVar": "CURSOR_API_KEY"
  },
  "fleet": {
    "autoCreatePr": true,
    "openAsCursorGithubApp": false,
    "skipReviewerRequest": false
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  },
  "mcp": {
    "cursor": {
      "enabled": true,
      "tokenEnvVar": "CURSOR_API_KEY",
      "mode": "stdio",
      "command": "npx",
      "args": ["-y", "@anthropic-ai/claude-code", "--mcp"]
    },
    "github": {
      "enabled": true,
      "tokenEnvVar": "GITHUB_TOKEN",
      "tokenEnvVarFallbacks": ["GH_TOKEN"],
      "mode": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"]
    }
  }
}
```

### In package.json

```json
{
  "name": "my-project",
  "agentic": {
    "defaultRepository": "my-org/my-repo",
    "logLevel": "info",
    "triage": {
      "provider": "anthropic",
      "model": "claude-sonnet-4-20250514"
    }
  }
}
```

---

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

Available models: `claude-opus-4-20250514`, `claude-sonnet-4-20250514`, `claude-3-5-sonnet-20241022`, `claude-3-haiku-20240307`

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

---

## CLI Configuration

Initialize configuration interactively:

```bash
# Interactive mode
agentic init

# Non-interactive with defaults
agentic init --non-interactive
```

The `init` command:

1. Detects the Git repository from remotes
2. Scans for existing tokens in the environment
3. Prompts for missing configuration
4. Generates `agentic.config.json`

---

## Related Pages

- [Token Management API](/api/token-management/) -- Token functions reference
- [Fleet API Reference](/api/fleet-management/) -- Fleet management
- [Getting Started: Configuration](/getting-started/configuration/) -- Setup guide
