---
title: Orchestration Patterns
description: Advanced multi-agent orchestration with diamond patterns and coordination
---

# Orchestration Patterns

This guide covers advanced multi-agent orchestration patterns including the diamond pattern, coordination loops, and agent monitoring with callbacks.

## The Diamond Pattern

The diamond pattern enables coordinated work across multiple repositories with a fan-out/fan-in structure:

```
              ┌──────────────┐
              │ Control      │
              │ Manager      │
              └──────┬───────┘
                     │
          ┌──────────┼──────────┐
          │          │          │
          ▼          ▼          ▼
     ┌────────┐ ┌────────┐ ┌────────┐
     │ Target │ │ Target │ │ Target │
     │ Repo 1 │ │ Repo 2 │ │ Repo 3 │
     └────┬───┘ └────┬───┘ └────┬───┘
          │          │          │
          └──────────┼──────────┘
                     │
                     ▼
              ┌──────────────┐
              │ Counterparty │
              │ (Aggregator) │
              └──────────────┘
```

### Use Case: Cross-Repository Refactoring

When you need to update multiple repositories in coordination:

```typescript
import type { DiamondConfig, SpawnOptions } from '@agentic-dev-library/control';
import { Fleet } from '@agentic-dev-library/control';

async function diamondPattern() {
  const fleet = new Fleet();

  // Define target repositories for parallel work
  const targetRepos: SpawnOptions[] = [
    {
      repository: 'https://github.com/your-org/frontend-app',
      task: `
        Update the UI components to use the new design tokens.
        Focus on:
        1. Color palette updates
        2. Typography changes
        3. Spacing adjustments

        Report progress via PR comments using format:
        ✅ DONE: [agent-id] [summary]
        ⚠️ BLOCKED: [agent-id] [issue]
      `,
      ref: 'main',
      target: { autoCreatePr: true },
    },
    {
      repository: 'https://github.com/your-org/backend-api',
      task: `
        Update API response formats for v2.
        Focus on:
        1. New pagination format
        2. Error response structure
        3. Rate limiting headers

        Report progress via PR comments.
      `,
      ref: 'main',
      target: { autoCreatePr: true },
    },
    {
      repository: 'https://github.com/your-org/shared-types',
      task: `
        Generate TypeScript types from OpenAPI spec.
        Focus on:
        1. Request/response types
        2. Error types
        3. Utility types

        Report progress via PR comments.
      `,
      ref: 'main',
      target: { autoCreatePr: true },
    },
  ];

  // Define counterparty (aggregator) agent
  const counterparty: SpawnOptions = {
    repository: 'https://github.com/your-org/integration-tests',
    task: `
      You are the integration coordinator.

      Wait for updates from target repository agents, then:
      1. Update integration tests for new APIs
      2. Verify type compatibility
      3. Create summary of all changes

      Monitor the coordination PR for status updates from other agents.
    `,
    ref: 'main',
    target: { autoCreatePr: true },
  };

  // Create the diamond
  const diamondConfig: DiamondConfig = {
    targetRepos,
    counterparty,
    controlCenter: 'https://github.com/your-org/control-center',
  };

  const result = await fleet.createDiamond(diamondConfig);

  if (result.success && result.data) {
    console.log('✅ Diamond pattern created!');
    console.log(`   Target Agents: ${result.data.targetAgents.length}`);
    console.log(`   Counterparty: ${result.data.counterpartyAgent.id}`);
  }

  return result;
}
```

## Agent Monitoring with Callbacks

Monitor multiple agents with progress callbacks:

```typescript
import type { AgentStatus } from '@agentic-dev-library/control';
import { Fleet } from '@agentic-dev-library/control';

async function monitorWithCallbacks() {
  const fleet = new Fleet();

  // Get running agents
  const runningResult = await fleet.running();

  if (!runningResult.success || !runningResult.data?.length) {
    console.log('No running agents to monitor.');
    return;
  }

  const agentIds = runningResult.data.map((a) => a.id);
  console.log(`Monitoring ${agentIds.length} agents...`);

  // Monitor with progress callback
  const results = await fleet.monitorAgents(agentIds, {
    pollInterval: 30000, // 30 seconds
    onProgress: (statusMap: Map<string, AgentStatus>) => {
      console.log(`[${new Date().toISOString()}] Status Update:`);
      for (const [id, status] of statusMap) {
        const emoji = status === 'RUNNING' ? '🏃' 
          : status === 'COMPLETED' ? '✅' 
          : '❓';
        console.log(`  ${emoji} ${id.slice(0, 12)}: ${status}`);
      }
    },
  });

  // Process final results
  console.log('\nFinal Results:');
  for (const [id, agent] of results) {
    console.log(`  ${id.slice(0, 12)}: ${agent.status}`);
    if (agent.target?.prUrl) {
      console.log(`    PR: ${agent.target.prUrl}`);
    }
  }

  return results;
}
```

## Coordination Loop

Set up bidirectional communication between a control center and agents via GitHub PR comments:

```typescript
import { Fleet } from '@agentic-dev-library/control';

async function coordinationLoop() {
  const fleet = new Fleet();

  const coordinationConfig = {
    coordinationPr: 123, // PR number for coordination
    repo: 'your-org/control-center',
    outboundInterval: 60000, // Check agents every 60s
    inboundInterval: 15000, // Check PR comments every 15s
    agentIds: ['bc-agent-1', 'bc-agent-2'],
  };

  console.log('Starting coordination loop...');
  console.log(`PR: ${coordinationConfig.repo}#${coordinationConfig.coordinationPr}`);
  console.log(`Agents: ${coordinationConfig.agentIds.length}`);

  // This runs indefinitely
  await fleet.coordinate(coordinationConfig);
}
```

### CLI Coordination

```bash
agentic fleet coordinate \
  --pr 123 \
  --repo my-org/control-center \
  --agents agent-1,agent-2,agent-3
```

## Handoff Protocol

Transfer work between agents with full context preservation:

```typescript
import { HandoffManager } from '@agentic-dev-library/control';

async function agentHandoff() {
  const handoff = new HandoffManager();

  // Predecessor agent initiates handoff
  const initiateResult = await handoff.initiate({
    predecessorId: 'bc-predecessor-xxx',
    pr: 123,
    branch: 'feature/my-branch',
    repo: 'https://github.com/my-org/my-repo',
    context: {
      completedTasks: ['Task 1', 'Task 2'],
      pendingTasks: ['Task 3', 'Task 4'],
      blockers: ['Need review on auth changes'],
    },
  });

  console.log(`Handoff initiated: ${initiateResult.data?.handoffId}`);

  // Successor agent confirms health
  const confirmResult = await handoff.confirm({
    predecessorId: 'bc-predecessor-xxx',
    successorId: 'bc-successor-xxx',
  });

  // Successor takes over
  const takeoverResult = await handoff.takeover({
    predecessorId: 'bc-predecessor-xxx',
    pr: 123,
    newBranch: 'feature/my-branch-continued',
  });

  return takeoverResult;
}
```

### CLI Handoff Commands

```bash
# Initiate handoff
agentic handoff initiate bc-predecessor-xxx \
  --pr 123 \
  --branch feature/my-branch \
  --repo https://github.com/my-org/my-repo

# Confirm as successor
agentic handoff confirm bc-predecessor-xxx

# Take over
agentic handoff takeover bc-predecessor-xxx 123 feature/continued
```

## Real-World Patterns

### Automated Code Maintenance Fleet

Spawn agents across multiple repos for coordinated updates:

```typescript
import { Fleet } from '@agentic-dev-library/control';

async function maintenanceFleet() {
  const fleet = new Fleet();

  const repos = [
    { repo: 'my-org/frontend', task: 'Update React to v18' },
    { repo: 'my-org/backend', task: 'Update Node.js dependencies' },
    { repo: 'my-org/mobile', task: 'Update React Native' },
    { repo: 'my-org/shared', task: 'Update shared utilities' },
  ];

  const agents = [];

  for (const { repo, task } of repos) {
    const result = await fleet.spawn({
      repository: `https://github.com/${repo}`,
      task: `${task}. Fix any breaking changes and update tests.`,
      target: { autoCreatePr: true },
    });

    if (result.success && result.data) {
      agents.push(result.data);
      console.log(`✅ Spawned agent for ${repo}: ${result.data.id}`);
    }
  }

  // Monitor all agents
  const agentIds = agents.map((a) => a.id);
  const results = await fleet.monitorAgents(agentIds, {
    pollInterval: 60000,
    onProgress: (statusMap) => {
      const running = [...statusMap.values()].filter((s) => s === 'RUNNING').length;
      const completed = [...statusMap.values()].filter((s) => s === 'COMPLETED').length;
      console.log(`Progress: ${completed}/${agents.length} completed, ${running} running`);
    },
  });

  return results;
}
```

### Release Preparation Coordinator

```typescript
import { Fleet } from '@agentic-dev-library/control';

async function releasePreparation(version: string) {
  const fleet = new Fleet();

  // Spawn specialized agents
  const docsAgent = await fleet.spawn({
    repository: 'https://github.com/my-org/docs',
    task: `Update documentation for v${version} release`,
    target: { autoCreatePr: true },
  });

  const changelogAgent = await fleet.spawn({
    repository: 'https://github.com/my-org/app',
    task: `Generate CHANGELOG entries for v${version}`,
    target: { autoCreatePr: true },
  });

  const testAgent = await fleet.spawn({
    repository: 'https://github.com/my-org/app',
    task: `Run full test suite and create release readiness report`,
    target: { autoCreatePr: true },
  });

  // Coordinate via broadcast
  const agentIds = [docsAgent.data?.id, changelogAgent.data?.id, testAgent.data?.id]
    .filter(Boolean) as string[];

  await fleet.broadcast(
    agentIds,
    `COORDINATION: Working on v${version} release. Report any blockers.`
  );

  return { docsAgent, changelogAgent, testAgent };
}
```

## Best Practices

### 1. Use Clear Status Reporting

Instruct agents to use consistent status formats:

```typescript
const task = `
  Your task: Update authentication module

  Report progress using these formats in PR comments:
  - ✅ DONE: [your-agent-id] [what you completed]
  - 🏃 WORKING: [your-agent-id] [current task]
  - ⚠️ BLOCKED: [your-agent-id] [what's blocking you]
  - ❓ QUESTION: [your-agent-id] [what you need clarification on]
`;
```

### 2. Set Appropriate Poll Intervals

```typescript
// Fast-moving coordination: 15s
await fleet.coordinate({ inboundInterval: 15000 });

// Long-running tasks: 60s
await fleet.monitorAgents(ids, { pollInterval: 60000 });

// Overnight jobs: 5 minutes
await fleet.monitorAgents(ids, { pollInterval: 300000 });
```

### 3. Handle Partial Failures

```typescript
const results = await fleet.monitorAgents(agentIds);

const succeeded = [...results.entries()].filter(([_, a]) => a.status === 'COMPLETED');
const failed = [...results.entries()].filter(([_, a]) => a.status === 'FAILED');

if (failed.length > 0) {
  console.log(`${failed.length} agents failed. Analyzing...`);
  for (const [id] of failed) {
    const conv = await fleet.conversation(id);
    // Analyze failure
  }
}
```

## Next Steps

- [Agent Spawning](/guides/agent-spawning/) - Basic agent creation
- [Fleet Management](/guides/fleet-management/) - Fleet operations
- [AI Triage](/guides/ai-triage/) - Analyze agent conversations
