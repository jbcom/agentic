---
title: Sandbox Execution
description: Run AI agents in isolated Docker containers for safe development
---

# Sandbox Execution Guide

Run AI agents in isolated Docker containers with resource limits, workspace mounting, and parallel execution support. This is ideal for safe local development and testing.

## Prerequisites

- Docker installed and running
- `@agentic-dev-library/control` installed
- AI provider API key (for AI-powered sandboxes)

## CLI Usage

### Basic Sandbox Execution

```bash
agentic sandbox run "Analyze this codebase for security vulnerabilities" \
  --workspace . \
  --output ./analysis-results
```

### With Runtime and Timeout

```bash
agentic sandbox run "Refactor the authentication module" \
  --runtime claude \
  --workspace ./src \
  --output ./refactor-results \
  --timeout 300
```

### Parallel Fleet Execution

Run multiple tasks in parallel:

```bash
agentic sandbox fleet \
  "Review authentication system" \
  "Analyze database queries" \
  "Check for security vulnerabilities" \
  --runtime claude \
  --workspace . \
  --output ./fleet-results
```

### With Resource Limits

```bash
agentic sandbox run "Analyze large codebase" \
  --workspace . \
  --output ./analysis \
  --memory 2048 \
  --timeout 600 \
  --env "NODE_ENV=development,LOG_LEVEL=debug"
```

## Programmatic Usage

### Basic Execution

```typescript
import { SandboxExecutor } from '@agentic-dev-library/control';

async function runSandbox() {
  const sandbox = new SandboxExecutor();

  const result = await sandbox.execute({
    runtime: 'claude',
    workspace: './src',
    outputDir: './analysis',
    prompt: 'Analyze this code for performance bottlenecks',
    timeout: 300000, // 5 minutes
  });

  if (result.success) {
    console.log('Analysis complete:', result.output);
  } else {
    console.error('Sandbox failed:', result.error);
  }

  return result;
}
```

### Parallel Fleet Execution

```typescript
import { SandboxExecutor } from '@agentic-dev-library/control';

async function runFleet() {
  const sandbox = new SandboxExecutor();

  const results = await sandbox.executeFleet([
    {
      runtime: 'claude',
      workspace: './frontend',
      outputDir: './frontend-analysis',
      prompt: 'Review React components for accessibility issues',
    },
    {
      runtime: 'claude',
      workspace: './backend',
      outputDir: './backend-analysis',
      prompt: 'Analyze API endpoints for security vulnerabilities',
    },
    {
      runtime: 'claude',
      workspace: './shared',
      outputDir: './shared-analysis',
      prompt: 'Check shared utilities for performance issues',
    },
  ]);

  // Process results
  for (const [index, result] of results.entries()) {
    console.log(`Task ${index + 1}: ${result.success ? '✅' : '❌'}`);
    if (result.success) {
      console.log(`  Output: ${result.outputDir}`);
    } else {
      console.error(`  Error: ${result.error}`);
    }
  }

  return results;
}
```

### With Custom Configuration

```typescript
import { SandboxExecutor, SandboxConfig } from '@agentic-dev-library/control';

async function customSandbox() {
  const sandbox = new SandboxExecutor();

  const config: SandboxConfig = {
    runtime: 'claude',
    workspace: './src/api',
    outputDir: './api-refactor',
    prompt: `
      Refactor the API layer to use async/await consistently.
      
      Requirements:
      1. Convert all callback-based code to async/await
      2. Add proper error handling with try/catch
      3. Ensure backwards compatibility
      4. Add JSDoc comments to all public functions
    `,
    timeout: 600000, // 10 minutes
    memory: 2048, // 2GB RAM limit
    env: {
      NODE_ENV: 'development',
      LOG_LEVEL: 'debug',
      DEBUG: 'api:*',
    },
    volumes: [
      // Mount additional directories read-only
      { source: './config', target: '/config', readonly: true },
      { source: './fixtures', target: '/fixtures', readonly: true },
    ],
  };

  const result = await sandbox.execute(config);
  return result;
}
```

## Available Runtimes

| Runtime | Description | Use Case |
|---------|-------------|----------|
| `claude` | Anthropic Claude in container | General analysis, refactoring |
| `cursor` | Cursor agent runtime | Code generation, complex tasks |
| `aider` | Aider CLI in container | Quick code fixes |
| `custom` | Custom Docker image | Specialized environments |

### Using Custom Runtime

```typescript
const result = await sandbox.execute({
  runtime: 'custom',
  image: 'my-registry/my-ai-agent:latest',
  workspace: './src',
  outputDir: './output',
  prompt: 'Custom analysis task',
});
```

## Configuration Options

### SandboxConfig Reference

```typescript
interface SandboxConfig {
  // Required
  workspace: string;      // Path to mount as workspace
  outputDir: string;      // Path for output files
  prompt: string;         // Task description

  // Runtime
  runtime: 'claude' | 'cursor' | 'aider' | 'custom';
  image?: string;         // Custom Docker image (for runtime: 'custom')

  // Resource limits
  timeout?: number;       // Milliseconds (default: 300000 / 5 min)
  memory?: number;        // MB (default: 1024)
  cpus?: number;          // CPU cores (default: 2)

  // Environment
  env?: Record<string, string>;  // Environment variables
  
  // Volumes
  volumes?: Array<{
    source: string;       // Host path
    target: string;       // Container path
    readonly?: boolean;   // Mount as read-only
  }>;

  // Callbacks
  onProgress?: (message: string) => void;
  onOutput?: (chunk: string) => void;
}
```

## Real-World Examples

### Security Audit

```typescript
import { SandboxExecutor } from '@agentic-dev-library/control';

async function securityAudit() {
  const sandbox = new SandboxExecutor();

  const result = await sandbox.execute({
    runtime: 'claude',
    workspace: '.',
    outputDir: './security-audit',
    prompt: `
      Perform a comprehensive security audit:
      
      1. Check for hardcoded secrets and credentials
      2. Identify SQL injection vulnerabilities
      3. Find XSS attack vectors
      4. Review authentication/authorization logic
      5. Check for insecure dependencies
      6. Identify OWASP Top 10 issues
      
      Create a detailed report with severity ratings.
    `,
    timeout: 900000, // 15 minutes
    memory: 2048,
  });

  return result;
}
```

### Dependency Update

```typescript
import { SandboxExecutor } from '@agentic-dev-library/control';

async function updateDependencies() {
  const sandbox = new SandboxExecutor();

  const result = await sandbox.execute({
    runtime: 'claude',
    workspace: '.',
    outputDir: './dependency-update',
    prompt: `
      Update all npm dependencies to their latest compatible versions:
      
      1. Run npm outdated to identify updates
      2. Update each dependency one at a time
      3. Run tests after each update
      4. Document any breaking changes
      5. Create a summary of all updates
      
      If tests fail, revert that specific update.
    `,
    timeout: 600000,
    env: {
      NPM_CONFIG_REGISTRY: 'https://registry.npmjs.org',
    },
  });

  return result;
}
```

### Code Migration

```typescript
import { SandboxExecutor } from '@agentic-dev-library/control';

async function migrateToTypeScript() {
  const sandbox = new SandboxExecutor();

  const results = await sandbox.executeFleet([
    {
      runtime: 'claude',
      workspace: './src/utils',
      outputDir: './migration/utils',
      prompt: 'Convert all JavaScript files to TypeScript with proper types',
    },
    {
      runtime: 'claude',
      workspace: './src/components',
      outputDir: './migration/components',
      prompt: 'Convert React components to TypeScript with proper prop types',
    },
    {
      runtime: 'claude',
      workspace: './src/api',
      outputDir: './migration/api',
      prompt: 'Convert API layer to TypeScript with proper request/response types',
    },
  ]);

  return results;
}
```

## Best Practices

### 1. Set Appropriate Timeouts

```typescript
// Quick analysis: 5 minutes
timeout: 300000

// Complex refactoring: 15 minutes
timeout: 900000

// Large codebase analysis: 30 minutes
timeout: 1800000
```

### 2. Use Memory Limits

```typescript
// Small tasks
memory: 512  // 512MB

// Medium tasks
memory: 1024 // 1GB (default)

// Large codebases
memory: 4096 // 4GB
```

### 3. Mount Read-Only When Possible

```typescript
volumes: [
  { source: './config', target: '/config', readonly: true },
  { source: './node_modules', target: '/node_modules', readonly: true },
]
```

### 4. Use Progress Callbacks

```typescript
const result = await sandbox.execute({
  // ...config
  onProgress: (message) => {
    console.log(`[${new Date().toISOString()}] ${message}`);
  },
  onOutput: (chunk) => {
    process.stdout.write(chunk);
  },
});
```

### 5. Handle Failures Gracefully

```typescript
const result = await sandbox.execute(config);

if (!result.success) {
  if (result.error?.includes('timeout')) {
    console.log('Task timed out, retrying with longer timeout...');
    return sandbox.execute({ ...config, timeout: config.timeout * 2 });
  }
  
  if (result.error?.includes('memory')) {
    console.log('Out of memory, retrying with more RAM...');
    return sandbox.execute({ ...config, memory: config.memory * 2 });
  }
  
  throw new Error(result.error);
}
```

## Security Considerations

### Workspace Isolation

- The sandbox mounts your workspace into the container
- Changes made inside the container affect your actual files
- Use `outputDir` to separate outputs from source code

### Network Access

- By default, containers have network access for API calls
- Use `--network none` for fully isolated execution (custom Docker only)

### Secret Management

- Pass API keys via environment variables, not in prompts
- Use `.dockerignore` to prevent sensitive files from being copied

```typescript
// Good: API key via env
env: {
  ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY,
}

// Bad: API key in prompt
prompt: `Use API key sk-xxx to call...`
```

## Next Steps

- [Agent Spawning](/guides/agent-spawning/) - Spawn cloud agents
- [Fleet Management](/guides/fleet-management/) - Coordinate multiple agents
- [Fleet API Reference](/api/fleet-management/) - Complete API documentation
