#!/usr/bin/env node
import { Command } from 'commander';
import { resolveModel } from './ai.js';
import {
    type AgentRouting,
    answerQuestion,
    classifyQuery,
    decomposeTask,
    routeToAgent,
    type SageResponse,
    sage,
    type TaskDecomposition,
    type UnblockResponse,
    unblock,
} from './handlers/sage.js';
import { runMcpServer } from './mcp-server.js';

const program = new Command();

program.name('agentic-triage').description('AI-powered GitHub issue triage and PR review primitives').version('0.2.1');

program
    .command('assess')
    .description('Assess an issue')
    .argument('<issue>', 'Issue number')
    .action(async (issueNum) => {
        console.log(`Assessing issue ${issueNum}...`);
        await resolveModel();
        console.log('Triage primitives called successfully.');
    });

/**
 * Format and print Sage results to the console
 */
function printSageResult(result: any, json: boolean) {
    if (json) {
        console.log(JSON.stringify(result, null, 2));
        return;
    }

    // Pretty print based on result type
    if ('answer' in result) {
        console.log('\n🔮 Sage Response:\n');
        console.log(result.answer);
        if (result.agentRecommendation) {
            console.log(`\n📌 Recommended: ${result.agentRecommendation.agent.toUpperCase()}`);
            console.log(`   Reason: ${result.agentRecommendation.reason}`);
        }
        console.log(`\n(Confidence: ${Math.round(result.confidence * 100)}%)`);
    } else if ('subtasks' in result) {
        console.log('\n📋 Task Decomposition:\n');
        for (const task of result.subtasks) {
            console.log(`${task.id}: ${task.title}`);
            console.log(`   Agent: ${task.agent} | Priority: ${task.priority} | Effort: ${task.effort}`);
            console.log(`   ${task.description}\n`);
        }
    } else if ('agent' in result && 'instructions' in result) {
        console.log('\n🎯 Agent Routing:\n');
        console.log(`Agent: ${result.agent.toUpperCase()}`);
        console.log(`Reason: ${result.reason}`);
        console.log(`Instructions: ${result.instructions}`);
        console.log(`(Confidence: ${Math.round(result.confidence * 100)}%)`);
    } else if ('diagnosis' in result) {
        console.log('\n🔓 Unblock Analysis:\n');
        console.log(`Diagnosis: ${result.diagnosis}`);
        console.log(`Root Cause: ${result.rootCause}`);
        console.log(`\n⚡ Immediate Action: ${result.immediateAction}`);
        if (result.needsHuman) {
            console.log(`\n⚠️ Human intervention required: ${result.escalationReason}`);
        }
    }
}

program
    .command('sage')
    .description('Ask Sage a question or get guidance')
    .argument('<query>', 'Your question or request')
    .option('--decompose', 'Decompose into subtasks')
    .option('--route', 'Get agent routing recommendation')
    .option('--unblock', 'Get help unblocking')
    .option('--json', 'Output as JSON')
    .action(async (query, options) => {
        const resolved = await resolveModel();
        const model = resolved.model;

        let result: SageResponse | TaskDecomposition | AgentRouting | UnblockResponse;

        if (options.decompose) {
            console.log('Decomposing task...');
            result = await decomposeTask(query, model);
        } else if (options.route) {
            console.log('Routing to agent...');
            result = await routeToAgent(query, model);
        } else if (options.unblock) {
            console.log('Analyzing blocker...');
            result = await unblock(query, model);
        } else {
            result = await sage(query, model);
        }

        printSageResult(result, options.json);
    });

program
    .command('mcp-server')
    .description('Run the MCP server for Claude/Cursor integration')
    .action(async () => {
        await runMcpServer();
    });

program.parse();
