/**
 * Example: Fleet Management
 *
 * This example demonstrates fleet-level operations:
 * - Listing all agents
 * - Filtering by status
 * - Broadcasting messages
 * - Fleet summary statistics
 *
 * @example
 * ```bash
 * export CURSOR_API_KEY="your-cursor-api-key"
 * pnpm tsx examples/02-fleet-management.ts
 * ```
 */

import type { Agent, AgentStatus } from '@jbcom/agentic';
import { Fleet } from '@jbcom/agentic';

/**
 * Display agent information in a formatted table
 */
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
    const repo = (agent.source.repository.split('/').pop() ?? 'unknown').slice(0, 23).padEnd(23);
    console.log(`   │ ${id} │ ${status} │ ${repo} │`);
  }

  console.log('   └────────────────────┴────────────┴─────────────────────────┘');
}

/**
 * Get status emoji for display
 */
function _statusEmoji(status: AgentStatus): string {
  const emojis: Record<AgentStatus, string> = {
    RUNNING: '🏃',
    FINISHED: '✅',
    COMPLETED: '✅',
    FAILED: '❌',
    CANCELLED: '🚫',
    PENDING: '⏳',
    UNKNOWN: '❓',
  };
  return emojis[status] ?? '❓';
}

/**
 * Main function demonstrating fleet management
 */
async function main(): Promise<void> {
  console.log('🚢 Fleet Management Example\n');

  const fleet = new Fleet();

  if (!fleet.isApiAvailable()) {
    console.error('❌ Cursor API not available. Set CURSOR_API_KEY environment variable.');
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

    // Demonstrate broadcasting to running agents
    if (runningResult.data.length > 0) {
      console.log('\n📢 Broadcasting status check to running agents...');

      const agentIds = runningResult.data.map((a) => a.id);
      const broadcastResults = await fleet.broadcast(
        agentIds,
        'STATUS CHECK: Please provide a brief progress update.'
      );

      for (const [id, result] of broadcastResults) {
        const emoji = result.success ? '✅' : '❌';
        console.log(
          `   ${emoji} ${id.slice(0, 12)}: ${result.success ? 'Message sent' : result.error}`
        );
      }
    }
  }
  console.log('');

  // List available repositories
  console.log('📁 Available Repositories');
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
  } else {
    console.log(`   ⚠️ Could not fetch repositories: ${reposResult.error}`);
  }
  console.log('');

  // List available models
  console.log('🤖 Available Models');
  console.log('─'.repeat(50));

  const modelsResult = await fleet.listModels();

  if (modelsResult.success && modelsResult.data) {
    for (const model of modelsResult.data) {
      console.log(`   • ${model}`);
    }
  } else {
    console.log(`   ⚠️ Could not fetch models: ${modelsResult.error}`);
  }

  console.log('\n✨ Done!');
}

// Run the example
main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
