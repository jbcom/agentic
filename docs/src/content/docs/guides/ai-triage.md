---
title: AI Triage
description: Analyze conversations, review code, and extract actionable insights with AI
---

# AI Triage Guide

This guide covers using AI-powered triage for code review, conversation analysis, and task extraction using multiple AI providers.

## Prerequisites

- `@agentic-dev-library/control` installed
- At least one AI provider SDK installed
- API key for your chosen provider

## Installing AI Providers

```bash
# Anthropic (recommended)
pnpm add @ai-sdk/anthropic

# OpenAI
pnpm add @ai-sdk/openai

# Google AI
pnpm add @ai-sdk/google

# Mistral
pnpm add @ai-sdk/mistral
```

## CLI Usage

### Quick Triage

Quickly analyze text for issues:

```bash
agentic triage quick "Error: Cannot read property 'map' of undefined at UserList.tsx:45"
```

### Code Review

Review changes between branches:

```bash
# Review current changes
agentic triage review --base main --head HEAD

# Review a specific PR
agentic triage review --base main --head feature/user-auth
```

### Analyze Agent Conversation

Analyze a completed agent's conversation:

```bash
# Generate a report
agentic triage analyze bc-xxx-xxx -o report.md

# Create GitHub issues from findings
agentic triage analyze bc-xxx-xxx --create-issues

# Use a specific model
agentic triage analyze bc-xxx-xxx --model claude-opus-4-20250514
```

## Programmatic Usage

### Basic Analysis

```typescript
import { AIAnalyzer } from '@agentic-dev-library/control';

async function analyzeError() {
  const analyzer = new AIAnalyzer({
    repo: 'my-org/my-repo',
    // Uses config from agentic.config.json by default
  });

  const result = await analyzer.quickTriage(
    'Error: ENOENT: no such file or directory, open "config.json"'
  );

  console.log('Analysis:', result.data?.summary);
  console.log('Suggested fixes:', result.data?.suggestions);
}
```

### With Specific Provider

```typescript
import { AIAnalyzer } from '@agentic-dev-library/control';

async function analyzeWithOpenAI() {
  const analyzer = new AIAnalyzer({
    repo: 'my-org/my-repo',
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: process.env.OPENAI_API_KEY,
  });

  const result = await analyzer.quickTriage('Memory leak in production');
  return result;
}
```

### Analyze Agent Conversation

```typescript
import { Fleet, AIAnalyzer } from '@agentic-dev-library/control';

async function analyzeAgentWork(agentId: string) {
  const fleet = new Fleet();
  const analyzer = new AIAnalyzer({ repo: 'my-org/my-repo' });

  // Get the conversation
  const convResult = await fleet.conversation(agentId);
  
  if (!convResult.success || !convResult.data) {
    throw new Error('Could not fetch conversation');
  }

  // Analyze it
  const analysis = await analyzer.analyzeConversation(convResult.data);

  if (analysis.success && analysis.data) {
    console.log('Summary:', analysis.data.summary);
    console.log('Completed Tasks:', analysis.data.completedTasks);
    console.log('Outstanding:', analysis.data.outstandingTasks);
    console.log('Blockers:', analysis.data.blockers);
  }

  return analysis;
}
```

### Code Review

```typescript
import { AIAnalyzer } from '@agentic-dev-library/control';

async function reviewPR() {
  const analyzer = new AIAnalyzer({ repo: 'my-org/my-repo' });

  const review = await analyzer.reviewCode({
    base: 'main',
    head: 'feature/auth-refactor',
  });

  if (review.success && review.data) {
    console.log('Overall:', review.data.overallAssessment);
    console.log('Issues:', review.data.issues);
    console.log('Suggestions:', review.data.suggestions);
    console.log('Security concerns:', review.data.securityConcerns);
  }

  return review;
}
```

## Using @agentic/triage Directly

For more control, use the triage package directly with the Vercel AI SDK:

### All Triage Tools

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

async function triageAgent() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: 'List all open critical bugs and create a triage plan',
  });

  console.log(result.text);
}
```

### Selective Tools

Import only what you need:

```typescript
import { 
  listIssuesTool, 
  createIssueTool, 
  searchIssuesTool 
} from '@agentic-dev-library/triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

async function minimalTriageAgent() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: {
      listIssues: listIssuesTool,
      createIssue: createIssueTool,
      searchIssues: searchIssuesTool,
    },
    maxSteps: 5,
    prompt: 'List open issues and create a summary report',
  });

  console.log(result.text);
}
```

### Tool Categories

```typescript
import { 
  getIssueTools,    // Issue CRUD, search, labels
  getReviewTools,   // PR review, comments, approval
  getProjectTools,  // Sprints, project management
} from '@agentic-dev-library/triage';

// Combine what you need
const myAgentTools = {
  ...getIssueTools(),
  ...getReviewTools(),
};
```

## Provider Configuration

### In Config File

```json
{
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514",
    "apiKeyEnvVar": "ANTHROPIC_API_KEY"
  }
}
```

### Supported Providers

| Provider | Package | Models |
|----------|---------|--------|
| Anthropic | `@ai-sdk/anthropic` | `claude-sonnet-4-20250514`, `claude-opus-4-20250514` |
| OpenAI | `@ai-sdk/openai` | `gpt-4o`, `gpt-4-turbo` |
| Google | `@ai-sdk/google` | `gemini-2.0-flash`, `gemini-pro` |
| Mistral | `@ai-sdk/mistral` | `mistral-large`, `mistral-medium` |
| Azure | `@ai-sdk/azure` | Deployed model names |

## Analysis Output Format

### Quick Triage Response

```typescript
interface QuickTriageResult {
  summary: string;           // Brief description of the issue
  severity: 'low' | 'medium' | 'high' | 'critical';
  category: string;          // bug, security, performance, etc.
  suggestions: string[];     // Suggested fixes
  relatedDocs?: string[];    // Links to relevant documentation
}
```

### Conversation Analysis Response

```typescript
interface ConversationAnalysis {
  summary: string;           // What the agent did
  completedTasks: string[];  // Tasks that were finished
  outstandingTasks: string[]; // Tasks still pending
  blockers: string[];        // Issues preventing progress
  recommendations: string[]; // Suggested next steps
  codeChanges: {
    files: string[];
    additions: number;
    deletions: number;
  };
}
```

### Code Review Response

```typescript
interface CodeReviewResult {
  overallAssessment: 'approve' | 'request-changes' | 'needs-discussion';
  issues: Array<{
    file: string;
    line: number;
    severity: 'error' | 'warning' | 'info';
    message: string;
  }>;
  suggestions: string[];
  securityConcerns: string[];
  testCoverage: 'adequate' | 'needs-improvement' | 'missing';
}
```

## Best Practices

### 1. Use Appropriate Models

```typescript
// Quick analysis: Use faster, cheaper models
const quickAnalyzer = new AIAnalyzer({
  model: 'claude-3-haiku-20240307', // Fast and cheap
});

// Deep analysis: Use more capable models
const deepAnalyzer = new AIAnalyzer({
  model: 'claude-opus-4-20250514', // Most capable
});
```

### 2. Batch Related Operations

```typescript
// Instead of multiple quick triages, batch them
const issues = ['issue1', 'issue2', 'issue3'];
const prompt = `Analyze these issues and prioritize:\n${issues.join('\n')}`;
const result = await analyzer.quickTriage(prompt);
```

### 3. Cache Results for Development

```typescript
import { AIAnalyzer } from '@agentic-dev-library/control';

const analyzer = new AIAnalyzer({
  repo: 'my-org/my-repo',
  cache: true, // Enable caching during development
  cacheDir: '.agentic-cache',
});
```

## Next Steps

- [Sandbox Execution](/guides/sandbox-execution/) - Run AI in isolated environments
- [Triage Tools API](/api/triage-tools/) - Complete tool reference
- [Vercel AI SDK Integration](/integrations/vercel-ai-sdk/) - Deep integration guide
