---
title: Fleet Management
description: Coordinate multiple AI agents across your repositories
---

# Fleet Management Guide

This guide demonstrates fleet-level operations: listing agents, filtering by status, broadcasting messages, and getting summary statistics.

## Prerequisites

- `@agentic-dev-library/control` installed globally
- `CURSOR_API_KEY` environment variable set

## CLI Commands

### List All Agents

```bash
agentic fleet list
```

### List Only Running Agents

```bash
agentic fleet list --running
```

### Get Fleet Summary

```bash
agentic fleet summary
```

Example output:
```
Fleet Summary
─────────────────────────────────────
Total Agents:    15
🏃 Running:      3
✅ Completed:    10
❌ Failed:       2
```

### List Available Models

```bash
agentic fleet models
```

### Send Followup Message

```bash
agentic fleet followup bc-xxx-xxx "Please provide a status update"
```

## Programmatic Usage

### Fleet Summary

```typescript
import { Fleet } from '@agentic-dev-library/control';

async function getFleetStatus() {
  const fleet = new Fleet();

  const summaryResult = await fleet.summary();

  if (!summaryResult.success || !summaryResult.data) {
    throw new Error(summaryResult.error);
  }

  const summary = summaryResult.data;
  
  console.log(`Total Agents:    ${summary.total}`);
  console.log(`🏃 Running:      ${summary.running}`);
  console.log(`✅ Completed:    ${summary.completed}`);
  console.log(`❌ Failed:       ${summary.failed}`);

  return summary;
}
```

### Display Agents Table

```typescript
import type { Agent } from '@agentic-dev-library/control';
import { Fleet } from '@agentic-dev-library/control';

function displayAgents(agents: Agent[]): void {
  if (agents.length === 0) {
    console.log('No agents found.');
    return;
  }

  console.log('┌────────────────────┬────────────┬─────────────────────────┐');
  console.log('│ Agent ID           │ Status     │ Repository              │');
  console.log('├────────────────────┼────────────┼─────────────────────────┤');

  for (const agent of agents) {
    const id = agent.id.slice(0, 16).padEnd(18);
    const status = agent.status.padEnd(10);
    const repo = (agent.source.repository.split('/').pop() ?? 'unknown')
      .slice(0, 23)
      .padEnd(23);
    console.log(`│ ${id} │ ${status} │ ${repo} │`);
  }

  console.log('└────────────────────┴────────────┴─────────────────────────┘');
}

async function listAllAgents() {
  const fleet = new Fleet();
  const listResult = await fleet.list();

  if (listResult.success && listResult.data) {
    displayAgents(listResult.data);
  }
}
```

### Broadcasting to Running Agents

Send a message to all running agents simultaneously:

```typescript
import { Fleet } from '@agentic-dev-library/control';

async function broadcastStatusCheck() {
  const fleet = new Fleet();

  // Get running agents
  const runningResult = await fleet.running();

  if (!runningResult.success || !runningResult.data) {
    console.log('No running agents.');
    return;
  }

  const runningAgents = runningResult.data;
  console.log(`Broadcasting to ${runningAgents.length} agents...`);

  // Send message to all
  const agentIds = runningAgents.map((a) => a.id);
  const broadcastResults = await fleet.broadcast(
    agentIds,
    'STATUS CHECK: Please provide a brief progress update.'
  );

  // Display results
  for (const [id, result] of broadcastResults) {
    const emoji = result.success ? '✅' : '❌';
    const message = result.success ? 'Message sent' : result.error;
    console.log(`${emoji} ${id.slice(0, 12)}: ${message}`);
  }
}
```

### List Available Repositories

```typescript
import { Fleet } from '@agentic-dev-library/control';

async function listRepositories() {
  const fleet = new Fleet();
  const reposResult = await fleet.repositories();

  if (!reposResult.success || !reposResult.data) {
    console.log('Could not fetch repositories.');
    return;
  }

  for (const repo of reposResult.data) {
    const visibility = repo.isPrivate ? '🔒' : '🌐';
    console.log(`${visibility} ${repo.fullName} (${repo.defaultBranch})`);
  }
}
```

### Full Fleet Management Example

From the [control repository examples](https://github.com/agentic-dev-library/control/blob/main/examples/02-fleet-management.ts):

```typescript
import type { Agent, AgentStatus } from '@agentic-dev-library/control';
import { Fleet } from '@agentic-dev-library/control';

function displayAgents(agents: Agent[]): void {
  if (agents.length === 0) {
    console.log('   No agents found.');
    return;
  }

  console.log('   ┌────────────────────┬────────────┬─────────────────────────┐');
  console.log('   │ Agent ID           │ Status     │ Repository              │');
  console.log('   ├────────────────────┼────────────┼─────────────────────────┤');

  for (const agent of agents) {
    const id = agent.id.slice(0, 16).padEnd(18);
    const status = agent.status.padEnd(10);
    const repo = (agent.source.repository.split('/').pop() ?? 'unknown')
      .slice(0, 23)
      .padEnd(23);
    console.log(`   │ ${id} │ ${status} │ ${repo} │`);
  }

  console.log('   └────────────────────┴────────────┴─────────────────────────┘');
}

async function main(): Promise<void> {
  console.log('🚢 Fleet Management Example\n');

  const fleet = new Fleet();

  if (!fleet.isApiAvailable()) {
    console.error('❌ Cursor API not available.');
    process.exit(1);
  }

  // Get fleet summary
  console.log('📊 Fleet Summary');
  console.log('─'.repeat(50));

  const summaryResult = await fleet.summary();

  if (!summaryResult.success || !summaryResult.data) {
    console.error(`❌ Failed to get summary: ${summaryResult.error}`);
    process.exit(1);
  }

  const summary = summaryResult.data;
  console.log(`   Total Agents:    ${summary.total}`);
  console.log(`   🏃 Running:      ${summary.running}`);
  console.log(`   ✅ Completed:    ${summary.completed}`);
  console.log(`   ❌ Failed:       ${summary.failed}`);
  console.log('');

  // List all agents
  console.log('📋 All Agents');
  console.log('─'.repeat(50));
  displayAgents(summary.agents);
  console.log('');

  // Filter running agents
  console.log('🏃 Running Agents');
  console.log('─'.repeat(50));

  const runningResult = await fleet.running();

  if (runningResult.success && runningResult.data) {
    displayAgents(runningResult.data);

    // Broadcast to running agents
    if (runningResult.data.length > 0) {
      console.log('\n📢 Broadcasting status check...');

      const agentIds = runningResult.data.map((a) => a.id);
      const broadcastResults = await fleet.broadcast(
        agentIds,
        'STATUS CHECK: Please provide a brief progress update.'
      );

      for (const [id, result] of broadcastResults) {
        const emoji = result.success ? '✅' : '❌';
        console.log(`   ${emoji} ${id.slice(0, 12)}: ${result.success ? 'Sent' : result.error}`);
      }
    }
  }

  // List repositories
  console.log('\n📁 Available Repositories');
  console.log('─'.repeat(50));

  const reposResult = await fleet.repositories();

  if (reposResult.success && reposResult.data) {
    for (const repo of reposResult.data.slice(0, 10)) {
      const visibility = repo.isPrivate ? '🔒' : '🌐';
      console.log(`   ${visibility} ${repo.fullName} (${repo.defaultBranch})`);
    }

    if (reposResult.data.length > 10) {
      console.log(`   ... and ${reposResult.data.length - 10} more`);
    }
  }

  // List models
  console.log('\n🤖 Available Models');
  console.log('─'.repeat(50));

  const modelsResult = await fleet.listModels();

  if (modelsResult.success && modelsResult.data) {
    for (const model of modelsResult.data) {
      console.log(`   • ${model}`);
    }
  }

  console.log('\n✨ Done!');
}

main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
```

## Agent Status Types

| Status | Description |
|--------|-------------|
| `PENDING` | Agent is queued but not yet started |
| `RUNNING` | Agent is actively working |
| `COMPLETED` | Agent finished successfully |
| `FINISHED` | Alias for COMPLETED |
| `FAILED` | Agent encountered an error |
| `CANCELLED` | Agent was manually cancelled |
| `UNKNOWN` | Status could not be determined |

## Fleet API Reference

| Method | Description |
|--------|-------------|
| `fleet.list()` | List all agents |
| `fleet.running()` | List only running agents |
| `fleet.summary()` | Get fleet statistics |
| `fleet.spawn(options)` | Spawn a new agent |
| `fleet.broadcast(ids, message)` | Send message to multiple agents |
| `fleet.conversation(id)` | Get agent conversation |
| `fleet.archive(id)` | Archive agent conversation |
| `fleet.repositories()` | List available repositories |
| `fleet.listModels()` | List available AI models |

## Next Steps

- [Orchestration Patterns](/guides/orchestration-patterns/) - Diamond pattern and coordination
- [Agent Spawning](/guides/agent-spawning/) - Spawn individual agents
- [Fleet API Reference](/api/fleet-management/) - Complete API documentation
