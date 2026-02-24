# Programmatic Integration Examples

This guide shows how to integrate agentic-control into your applications programmatically.

## TypeScript/Node.js Integration

### Basic Fleet Management

```typescript
import { Fleet, getConfig, initConfig } from 'agentic-control';

// Initialize configuration
initConfig({
  triage: {
    provider: 'anthropic',
    model: 'claude-sonnet-4-20250514'
  }
});

const fleet = new Fleet();

// Spawn an agent
const result = await fleet.spawn({
  repository: 'https://github.com/owner/repo',
  task: 'Fix the authentication bug in login.ts',
  ref: 'main',
  target: {
    autoCreatePr: true,
    branchName: 'fix/auth-bug'
  }
});

if (result.success) {
  console.log(`Agent spawned: ${result.data?.id}`);
  
  // Monitor agent progress
  const status = await fleet.status(result.data!.id);
  console.log(`Status: ${status.data?.status}`);
}
```

### AI-Powered Triage

```typescript
import { AIAnalyzer } from 'agentic-control/triage';

const analyzer = new AIAnalyzer({
  model: 'claude-sonnet-4-20250514'
});

// Quick triage of user input
const triage = await analyzer.quickTriage(
  "The login form crashes when I enter my email"
);

console.log(`Priority: ${triage.priority}`);
console.log(`Category: ${triage.category}`);
console.log(`Action: ${triage.suggestedAction}`);

// Code review
const diff = await getDiffFromGit();
const review = await analyzer.reviewCode(diff);

if (review.readyToMerge) {
  console.log('✅ Code is ready to merge');
} else {
  console.log('❌ Issues found:');
  review.issues.forEach(issue => {
    console.log(`  - ${issue.description}`);
  });
}
```

### GitHub Integration

```typescript
import { GitHubClient } from 'agentic-control/github';

const github = new GitHubClient();

// Create an issue with automatic token selection
const issue = await github.createIssue({
  owner: 'my-org',
  repo: 'my-repo',
  title: 'Bug Report',
  body: 'Description of the bug',
  labels: ['bug', 'high-priority']
});

// The client automatically uses the correct token for 'my-org'
console.log(`Created issue: ${issue.html_url}`);
```

### Sandbox Execution

```typescript
import { SandboxExecutor } from 'agentic-control/sandbox';

const executor = new SandboxExecutor();

// Run a single agent in sandbox
const result = await executor.execute({
  runtime: 'claude',
  workspace: './project',
  outputDir: './results',
  prompt: 'Generate unit tests for the API endpoints',
  timeout: 300000, // 5 minutes
  memory: 1024, // 1GB
  env: {
    NODE_ENV: 'test',
    API_URL: 'http://localhost:3000'
  }
});

if (result.success) {
  console.log('✅ Tests generated successfully');
  console.log(result.output);
} else {
  console.error('❌ Generation failed:', result.error);
}

// Run multiple agents in parallel
const tasks = [
  { prompt: 'Generate API tests', outputDir: './tests/api' },
  { prompt: 'Generate UI tests', outputDir: './tests/ui' },
  { prompt: 'Generate integration tests', outputDir: './tests/integration' }
];

const results = await executor.executeFleet(
  tasks.map(task => ({
    runtime: 'claude',
    workspace: './project',
    ...task,
    timeout: 300000,
    memory: 512
  }))
);

console.log(`Completed ${results.filter(r => r.success).length}/${results.length} tasks`);
```

## Express.js API Integration

```typescript
import express from 'express';
import { Fleet, AIAnalyzer } from 'agentic-control';

const app = express();
app.use(express.json());

const fleet = new Fleet();
const analyzer = new AIAnalyzer();

// Webhook endpoint for agent status updates
app.post('/webhook/agent-status', async (req, res) => {
  const { agentId, status } = req.body;
  
  if (status === 'COMPLETED') {
    // Analyze the completed work
    const conversation = await fleet.conversation(agentId);
    if (conversation.success) {
      const analysis = await analyzer.analyzeConversation(conversation.data!);
      
      // Create GitHub issues for outstanding tasks
      await analyzer.createIssuesFromAnalysis(analysis);
    }
  }
  
  res.json({ received: true });
});

// API endpoint to spawn agents
app.post('/api/agents/spawn', async (req, res) => {
  const { repository, task, branch } = req.body;
  
  try {
    const result = await fleet.spawn({
      repository,
      task,
      target: {
        branchName: branch,
        autoCreatePr: true
      }
    });
    
    if (result.success) {
      res.json({ agentId: result.data?.id });
    } else {
      res.status(400).json({ error: result.error });
    }
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3000, () => {
  console.log('Agent control API listening on port 3000');
});
```

## CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/ai-review.yml
name: AI Code Review

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  ai-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install agentic-control
        run: npm install -g agentic-control
      
      - name: Run AI code review
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          # Review the PR diff
          agentic triage review \
            --base ${{ github.event.pull_request.base.sha }} \
            --head ${{ github.event.pull_request.head.sha }} \
            > review.md
          
          # Post review as comment
          gh pr comment ${{ github.event.number }} \
            --body-file review.md
```

### Docker Integration

```dockerfile
# Dockerfile for AI agent runner
FROM node:20-slim

# Install Python for CrewAI integration
RUN apt-get update && apt-get install -y python3 python3-pip

# Install agentic-control
RUN npm install -g agentic-control

# Install Python companion
RUN pip3 install agentic-control-crews

# Set up workspace
WORKDIR /workspace
COPY . .

# Run agent coordination
CMD ["agentic", "fleet", "coordinate", "--pr", "1", "--repo", "owner/repo"]
```

## Custom MCP Server Integration

```typescript
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { Fleet, AIAnalyzer } from 'agentic-control';

const server = new Server(
  {
    name: 'custom-agentic-server',
    version: '1.0.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

const fleet = new Fleet();
const analyzer = new AIAnalyzer();

// Add fleet management tools
server.setRequestHandler('tools/call', async (request) => {
  const { name, arguments: args } = request.params;
  
  switch (name) {
    case 'spawn_agent':
      const result = await fleet.spawn({
        repository: args.repository,
        task: args.task,
        target: { autoCreatePr: args.autoCreatePr }
      });
      return { content: [{ type: 'text', text: JSON.stringify(result) }] };
      
    case 'analyze_conversation':
      const conversation = await fleet.conversation(args.agentId);
      if (conversation.success) {
        const analysis = await analyzer.analyzeConversation(conversation.data!);
        return { content: [{ type: 'text', text: analysis.summary }] };
      }
      break;
  }
});

// Start the server
const transport = new StdioServerTransport();
server.connect(transport);
```

## Testing Integration

```typescript
import { describe, it, expect, beforeEach } from 'vitest';
import { Fleet, AIAnalyzer } from 'agentic-control';

describe('Agent Integration Tests', () => {
  let fleet: Fleet;
  let analyzer: AIAnalyzer;
  
  beforeEach(() => {
    fleet = new Fleet();
    analyzer = new AIAnalyzer({ model: 'claude-sonnet-4-20250514' });
  });
  
  it('should spawn and monitor agent', async () => {
    const spawnResult = await fleet.spawn({
      repository: 'https://github.com/test/repo',
      task: 'Add unit tests',
      target: { autoCreatePr: false }
    });
    
    expect(spawnResult.success).toBe(true);
    expect(spawnResult.data?.id).toBeDefined();
    
    // Monitor status
    const statusResult = await fleet.status(spawnResult.data!.id);
    expect(statusResult.success).toBe(true);
  });
  
  it('should analyze code and provide feedback', async () => {
    const diff = `
      +function add(a, b) {
      +  return a + b;
      +}
    `;
    
    const review = await analyzer.reviewCode(diff);
    expect(review.readyToMerge).toBeDefined();
    expect(review.issues).toBeInstanceOf(Array);
  });
});
```