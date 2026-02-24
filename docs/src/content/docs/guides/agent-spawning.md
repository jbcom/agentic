---
title: Agent Spawning
description: Learn how to spawn and monitor AI agents with @jbcom/agentic
---

# Agent Spawning Guide

This guide demonstrates how to spawn a single Cursor Background Agent and monitor its progress until completion.

## Prerequisites

- `@jbcom/agentic` installed globally
- `CURSOR_API_KEY` environment variable set
- `GITHUB_TOKEN` with repo access

## CLI Usage

### Basic Spawn

```bash
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Review the README and add a Quick Start section"
```

### Spawn with Auto-PR

```bash
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Fix the failing GitHub Actions workflow" \
  --auto-pr
```

### Spawn on Specific Branch

```bash
agentic fleet spawn "https://github.com/my-org/my-repo" \
  "Update dependencies to latest versions" \
  --ref feature-branch \
  --branch agent/update-deps \
  --auto-pr
```

## Programmatic Usage

### Basic Example

```typescript
import { Fleet } from '@jbcom/agentic';

async function spawnAgent() {
  // Initialize Fleet manager
  const fleet = new Fleet();

  // Check if API is available
  if (!fleet.isApiAvailable()) {
    console.error('❌ Cursor API not available. Set CURSOR_API_KEY.');
    process.exit(1);
  }

  // Spawn the agent
  const result = await fleet.spawn({
    repository: 'https://github.com/my-org/my-repo',
    task: `
      Review the codebase and create a summary document.

      Steps:
      1. Read the README.md and package.json
      2. Identify the main entry points
      3. List the key dependencies
      4. Create a SUMMARY.md with your findings
    `,
    ref: 'main',
    target: {
      autoCreatePr: true,
      branchName: 'agent/codebase-review',
    },
  });

  if (!result.success || !result.data) {
    console.error(`❌ Failed to spawn agent: ${result.error}`);
    process.exit(1);
  }

  console.log(`✅ Agent spawned: ${result.data.id}`);
  return result.data;
}
```

### Monitoring Progress

```typescript
import { Fleet } from '@jbcom/agentic';

async function spawnAndMonitor() {
  const fleet = new Fleet();
  
  // Spawn agent
  const spawnResult = await fleet.spawn({
    repository: 'https://github.com/my-org/my-repo',
    task: 'Fix the CI workflow',
    target: { autoCreatePr: true },
  });

  if (!spawnResult.success || !spawnResult.data) {
    throw new Error(spawnResult.error);
  }

  const agent = spawnResult.data;
  console.log(`Agent ${agent.id} spawned, monitoring...`);

  // Wait for completion with timeout
  const finalResult = await fleet.waitFor(agent.id, {
    timeout: 600000, // 10 minutes
    pollInterval: 15000, // Check every 15 seconds
  });

  if (!finalResult.success || !finalResult.data) {
    throw new Error(finalResult.error);
  }

  const finalAgent = finalResult.data;
  console.log(`Agent completed with status: ${finalAgent.status}`);

  if (finalAgent.target?.prUrl) {
    console.log(`PR created: ${finalAgent.target.prUrl}`);
  }

  return finalAgent;
}
```

### Full Example with Error Handling

This is a complete example from the [control repository](https://github.com/jbcom/agentic/blob/main/examples/01-agent-spawn.ts):

```typescript
import { Fleet } from '@jbcom/agentic';

async function main(): Promise<void> {
  console.log('🚀 Agent Spawning Example\n');

  const fleet = new Fleet();

  if (!fleet.isApiAvailable()) {
    console.error('❌ Cursor API not available. Set CURSOR_API_KEY.');
    process.exit(1);
  }

  const spawnOptions = {
    repository: 'https://github.com/your-org/your-repo',
    task: `
      Review the codebase and create a summary document.

      Steps:
      1. Read the README.md and package.json
      2. Identify the main entry points
      3. List the key dependencies
      4. Create a SUMMARY.md with your findings
    `,
    ref: 'main',
    target: {
      autoCreatePr: true,
      branchName: 'agent/codebase-review',
    },
  };

  console.log('📋 Spawn Configuration:');
  console.log(`   Repository: ${spawnOptions.repository}`);
  console.log(`   Branch: ${spawnOptions.ref}`);
  console.log(`   Auto-create PR: ${spawnOptions.target.autoCreatePr}`);

  // Spawn the agent
  console.log('\n⏳ Spawning agent...');
  const result = await fleet.spawn(spawnOptions);

  if (!result.success || !result.data) {
    console.error(`❌ Failed to spawn agent: ${result.error}`);
    process.exit(1);
  }

  const agent = result.data;
  console.log(`✅ Agent spawned successfully!`);
  console.log(`   Agent ID: ${agent.id}`);
  console.log(`   Status: ${agent.status}`);

  // Monitor progress
  console.log('\n📊 Monitoring agent progress...');
  console.log('   (Press Ctrl+C to stop monitoring)\n');

  const finalResult = await fleet.waitFor(agent.id, {
    timeout: 600000,
    pollInterval: 15000,
  });

  if (!finalResult.success || !finalResult.data) {
    console.error(`❌ Error waiting for agent: ${finalResult.error}`);
    process.exit(1);
  }

  const finalAgent = finalResult.data;
  console.log('\n🏁 Agent completed!');
  console.log(`   Final Status: ${finalAgent.status}`);

  if (finalAgent.target?.prUrl) {
    console.log(`   PR URL: ${finalAgent.target.prUrl}`);
  }

  // Archive the conversation
  console.log('\n📁 Archiving conversation...');
  const archiveResult = await fleet.archive(agent.id);

  if (archiveResult.success) {
    console.log(`   Saved to: ${archiveResult.data}`);
  }

  console.log('\n✨ Done!');
}

main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
```

## Spawn Options Reference

| Option | Type | Description |
|--------|------|-------------|
| `repository` | `string` | Full GitHub repository URL |
| `task` | `string` | The task description for the agent |
| `ref` | `string` | Branch, tag, or commit to work from |
| `target.autoCreatePr` | `boolean` | Create PR on completion |
| `target.branchName` | `string` | Custom branch name for changes |
| `target.openAsCursorGithubApp` | `boolean` | Open PR as Cursor app |

## Best Practices

### 1. Write Clear Task Descriptions

Be specific about what you want the agent to do:

```typescript
// ❌ Too vague
task: 'Fix the bug'

// ✅ Clear and actionable
task: `
  Fix the authentication bug in src/auth/login.ts.
  
  The issue: Users with special characters in passwords cannot log in.
  
  Steps:
  1. Find the password validation function
  2. Update the regex to allow special characters
  3. Add unit tests for edge cases
  4. Update the changelog
`
```

### 2. Use Appropriate Timeouts

Set timeouts based on task complexity:

```typescript
// Quick fixes: 5 minutes
await fleet.waitFor(id, { timeout: 300000 });

// Feature development: 30 minutes
await fleet.waitFor(id, { timeout: 1800000 });

// Complex refactoring: 1 hour
await fleet.waitFor(id, { timeout: 3600000 });
```

### 3. Handle Failures Gracefully

```typescript
const result = await fleet.spawn(options);

if (!result.success) {
  if (result.error?.includes('rate limit')) {
    console.log('Rate limited, retrying in 60s...');
    await sleep(60000);
    return fleet.spawn(options);
  }
  throw new Error(result.error);
}
```

## Next Steps

- [Fleet Management](/guides/fleet-management/) - Coordinate multiple agents
- [Orchestration Patterns](/guides/orchestration-patterns/) - Advanced multi-agent workflows
- [Fleet API Reference](/api/fleet-management/) - Complete API documentation
