---
title: TypeScript Examples
description: Real-world TypeScript examples from the Agentic repositories
---

# TypeScript Examples

These examples are taken directly from the Agentic repositories. They demonstrate real-world usage patterns for agent spawning, fleet management, and AI triage.

## Agent Spawning

This example from [`control/examples/01-agent-spawn.ts`](https://github.com/jbcom/agentic/blob/main/examples/01-agent-spawn.ts) shows how to spawn a Cursor Background Agent and monitor it until completion.

```typescript
import { Fleet } from '@jbcom/agentic';

async function main(): Promise<void> {
  console.log('🚀 Agent Spawning Example\n');

  // Initialize Fleet manager
  const fleet = new Fleet();

  // Check if API is available
  if (!fleet.isApiAvailable()) {
    console.error('❌ Cursor API not available. Set CURSOR_API_KEY.');
    process.exit(1);
  }

  // Define spawn options
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

  // Monitor agent progress
  console.log('\n📊 Monitoring agent progress...');

  const finalResult = await fleet.waitFor(agent.id, {
    timeout: 600000, // 10 minutes
    pollInterval: 15000, // 15 seconds
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

main().catch(console.error);
```

## Fleet Management

From [`control/examples/02-fleet-management.ts`](https://github.com/jbcom/agentic/blob/main/examples/02-fleet-management.ts), this shows fleet-level operations.

```typescript
import type { Agent, AgentStatus } from '@jbcom/agentic';
import { Fleet } from '@jbcom/agentic';

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
      .slice(0, 23).padEnd(23);
    console.log(`   │ ${id} │ ${status} │ ${repo} │`);
  }

  console.log('   └────────────────────┴────────────┴─────────────────────────┘');
}

async function main(): Promise<void> {
  console.log('🚢 Fleet Management Example\n');

  const fleet = new Fleet();

  // Get fleet summary
  console.log('📊 Fleet Summary');
  console.log('─'.repeat(50));

  const summaryResult = await fleet.summary();

  if (summaryResult.success && summaryResult.data) {
    const summary = summaryResult.data;
    console.log(`   Total Agents:    ${summary.total}`);
    console.log(`   🏃 Running:      ${summary.running}`);
    console.log(`   ✅ Completed:    ${summary.completed}`);
    console.log(`   ❌ Failed:       ${summary.failed}`);
  }

  // List all agents
  console.log('\n📋 All Agents');
  console.log('─'.repeat(50));
  displayAgents(summaryResult.data?.agents ?? []);

  // Filter running agents
  console.log('\n🏃 Running Agents');
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

  // List available repositories
  console.log('\n📁 Available Repositories');
  console.log('─'.repeat(50));

  const reposResult = await fleet.repositories();

  if (reposResult.success && reposResult.data) {
    for (const repo of reposResult.data.slice(0, 10)) {
      const visibility = repo.isPrivate ? '🔒' : '🌐';
      console.log(`   ${visibility} ${repo.fullName}`);
    }
  }

  console.log('\n✨ Done!');
}

main().catch(console.error);
```

## Triage with Vercel AI SDK

From [`triage/examples/basic-agent.ts`](https://github.com/jbcom/agentic/blob/main/examples/basic-agent.ts):

```typescript
import { anthropic } from '@ai-sdk/anthropic';
import { generateText } from 'ai';
import { getTriageTools } from '@jbcom/agentic';

async function main() {
  try {
    const result = await generateText({
      model: anthropic('claude-sonnet-4-20250514'),
      tools: getTriageTools(),
      maxSteps: 10,
      prompt: 'Find all open critical bugs and summarize them.',
    });

    console.log(result.text);
  } catch (error) {
    console.error('Failed to generate text:', error);
    process.exit(1);
  }
}

main().catch(console.error);
```

## Selective Triage Tools

From [`triage/examples/selective-tools.ts`](https://github.com/jbcom/agentic/blob/main/examples/selective-tools.ts), using only a subset of tools:

```typescript
import { anthropic } from '@ai-sdk/anthropic';
import { generateText } from 'ai';
import { 
  createIssueTool, 
  listIssuesTool, 
  searchIssuesTool 
} from '@jbcom/agentic';

async function main() {
  try {
    // Use only the tools needed for this specific task
    const minimalTools = {
      listIssues: listIssuesTool,
      createIssue: createIssueTool,
      searchIssues: searchIssuesTool,
    };

    console.log('Available minimal tools:', Object.keys(minimalTools));

    const result = await generateText({
      model: anthropic('claude-sonnet-4-20250514'),
      tools: minimalTools,
      maxSteps: 5,
      prompt: 'List the open issues and create a summary report.',
    });

    console.log(result.text);
  } catch (error) {
    console.error('Failed to run agent:', error);
    process.exit(1);
  }
}

main().catch(console.error);
```

## Orchestration Patterns

From [`control/examples/03-orchestration-patterns.ts`](https://github.com/jbcom/agentic/blob/main/examples/03-orchestration-patterns.ts), advanced multi-agent coordination:

```typescript
import type { DiamondConfig, SpawnOptions } from '@jbcom/agentic';
import { Fleet } from '@jbcom/agentic';

async function diamondPatternExample(fleet: Fleet): Promise<void> {
  console.log('💎 Diamond Pattern Orchestration');
  console.log('─'.repeat(50));

  // Define target repositories for parallel work
  const targetRepos: SpawnOptions[] = [
    {
      repository: 'https://github.com/your-org/frontend-app',
      task: `
        Update the UI components to use the new design tokens.
        Report progress via PR comments using format:
        ✅ DONE: [agent-id] [summary]
      `,
      ref: 'main',
      target: { autoCreatePr: true },
    },
    {
      repository: 'https://github.com/your-org/backend-api',
      task: `
        Update API response formats for v2.
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
      1. Update integration tests
      2. Verify type compatibility
      3. Create summary of all changes
    `,
    ref: 'main',
    target: { autoCreatePr: true },
  };

  const diamondConfig: DiamondConfig = {
    targetRepos,
    counterparty,
    controlCenter: 'https://github.com/your-org/control-center',
  };

  console.log('📋 Diamond Configuration:');
  console.log(`   Target Repos: ${targetRepos.length}`);
  console.log(`   Counterparty: ${counterparty.repository}`);

  // Create the diamond
  const result = await fleet.createDiamond(diamondConfig);

  if (result.success && result.data) {
    console.log('✅ Diamond pattern created!');
    console.log(`   Target Agents: ${result.data.targetAgents.length}`);
    console.log(`   Counterparty: ${result.data.counterpartyAgent.id}`);
  }
}

async function main(): Promise<void> {
  console.log('🎭 Orchestration Patterns Example\n');

  const fleet = new Fleet();

  if (!fleet.isApiAvailable()) {
    console.error('❌ Cursor API not available.');
    process.exit(1);
  }

  await diamondPatternExample(fleet);

  console.log('\n✨ Done!');
}

main().catch(console.error);
```

## Token Management

Programmatic token configuration:

```typescript
import { 
  Fleet, 
  getTokenForRepo,
  setTokenConfig,
  addOrganization,
} from '@jbcom/agentic';

// Configure organizations
addOrganization({
  name: 'my-company',
  tokenEnvVar: 'GITHUB_COMPANY_TOKEN',
});

addOrganization({
  name: 'partner-org',
  tokenEnvVar: 'PARTNER_GH_PAT',
});

// Or configure everything at once
setTokenConfig({
  organizations: {
    'my-company': { name: 'my-company', tokenEnvVar: 'GITHUB_COMPANY_TOKEN' },
    'partner-org': { name: 'partner-org', tokenEnvVar: 'PARTNER_GH_PAT' },
  },
  defaultTokenEnvVar: 'GITHUB_TOKEN',
  prReviewTokenEnvVar: 'GITHUB_BOT_TOKEN',
});

// Token is automatically routed based on repo
const token = getTokenForRepo('my-company/my-repo');
// Returns value of GITHUB_COMPANY_TOKEN
```

## Running the Examples

Clone the repository and run the examples:

```bash
git clone https://github.com/jbcom/agentic.git
cd control
pnpm install

# Set required environment variables
export CURSOR_API_KEY="your-cursor-api-key"
export GITHUB_TOKEN="your-github-token"

# Run an example
pnpm tsx examples/01-agent-spawn.ts
```

## Next Steps

- [Python Examples](/examples/python/) - Python crew examples
- [CLI Workflows](/examples/cli-workflows/) - Command-line examples
- [API Reference](/api/fleet-management/) - Complete API documentation
