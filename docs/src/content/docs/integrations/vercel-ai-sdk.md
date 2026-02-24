---
title: Vercel AI SDK Integration
description: Use @agentic/triage tools with the Vercel AI SDK
---

# Vercel AI SDK Integration

`@agentic/triage` provides portable tools designed for the [Vercel AI SDK](https://sdk.vercel.ai/). These tools can be used with any AI provider supported by the SDK.

## Installation

```bash
# Install triage package
npm install @agentic-dev-library/triage

# Install an AI provider
npm install @ai-sdk/anthropic
# or
npm install @ai-sdk/openai
```

## Quick Start

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  maxSteps: 10,
  prompt: 'List all open critical bugs and create a triage plan',
});

console.log(result.text);
```

## Available Tool Sets

### All Tools

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';

// Get all available tools
const allTools = getTriageTools();
```

### Issue Tools

```typescript
import { getIssueTools } from '@agentic-dev-library/triage';

const issueTools = getIssueTools();
// Includes: listIssues, getIssue, createIssue, updateIssue, 
//           closeIssue, searchIssues, addLabels, removeLabels
```

### Review Tools

```typescript
import { getReviewTools } from '@agentic-dev-library/triage';

const reviewTools = getReviewTools();
// Includes: getPRComments, addPRComment, approvePR, requestChanges
```

### Project Tools

```typescript
import { getProjectTools } from '@agentic-dev-library/triage';

const projectTools = getProjectTools();
// Includes: getSprints, getCurrentSprint, getSprintIssues, moveToSprint
```

### Individual Tools

```typescript
import {
  listIssuesTool,
  createIssueTool,
  getIssueTool,
  updateIssueTool,
  closeIssueTool,
  searchIssuesTool,
  addLabelsTool,
  removeLabelsTool,
} from '@agentic-dev-library/triage';

// Use only what you need
const minimalTools = {
  listIssues: listIssuesTool,
  createIssue: createIssueTool,
};
```

## Usage Patterns

### Basic Text Generation

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

async function triageIssues() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: 'Find all open bugs, prioritize them by severity, and create a summary',
  });

  return result.text;
}
```

### Streaming

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { streamText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

async function streamTriage() {
  const result = await streamText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: 'Review all open PRs and provide feedback',
  });

  for await (const chunk of result.textStream) {
    process.stdout.write(chunk);
  }
}
```

### With Tool Results

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';

async function triageWithResults() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: 'List all open issues and categorize them',
  });

  // Access tool call results
  for (const step of result.steps) {
    for (const toolCall of step.toolCalls) {
      console.log(`Tool: ${toolCall.toolName}`);
      console.log(`Args: ${JSON.stringify(toolCall.args)}`);
    }
    for (const toolResult of step.toolResults) {
      console.log(`Result: ${JSON.stringify(toolResult.result)}`);
    }
  }

  return result.text;
}
```

### Combining with Custom Tools

```typescript
import { getIssueTools } from '@agentic-dev-library/triage';
import { generateText, tool } from 'ai';
import { anthropic } from '@ai-sdk/anthropic';
import { z } from 'zod';

// Custom tool
const notifySlackTool = tool({
  description: 'Send a notification to Slack',
  parameters: z.object({
    channel: z.string(),
    message: z.string(),
  }),
  execute: async ({ channel, message }) => {
    // Slack integration logic
    return { sent: true };
  },
});

async function triageAndNotify() {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: {
      ...getIssueTools(),
      notifySlack: notifySlackTool,
    },
    maxSteps: 10,
    prompt: 'Find critical bugs and notify #engineering on Slack',
  });

  return result.text;
}
```

## Provider Examples

### Anthropic

```typescript
import { anthropic } from '@ai-sdk/anthropic';
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'Triage open issues',
});
```

### OpenAI

```typescript
import { openai } from '@ai-sdk/openai';
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';

const result = await generateText({
  model: openai('gpt-4o'),
  tools: getTriageTools(),
  prompt: 'Triage open issues',
});
```

### Google

```typescript
import { google } from '@ai-sdk/google';
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';

const result = await generateText({
  model: google('gemini-2.0-flash'),
  tools: getTriageTools(),
  prompt: 'Triage open issues',
});
```

## Advanced Patterns

### Custom System Prompt

```typescript
const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  system: `You are a senior engineering manager responsible for issue triage.
    
    When triaging issues:
    1. Prioritize security issues as critical
    2. Group related issues together
    3. Estimate effort for each issue
    4. Suggest assignees based on labels`,
  prompt: 'Triage all open issues for the next sprint',
});
```

### Multi-Turn Conversations

```typescript
import { generateText } from 'ai';
import { getTriageTools } from '@agentic-dev-library/triage';

const messages = [
  { role: 'user', content: 'List all open bugs' },
];

// First turn
let result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  messages,
});

messages.push({ role: 'assistant', content: result.text });
messages.push({ role: 'user', content: 'Now prioritize them by severity' });

// Second turn
result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  messages,
});
```

### Error Handling

```typescript
import { generateText } from 'ai';
import { getTriageTools } from '@agentic-dev-library/triage';

try {
  const result = await generateText({
    model: anthropic('claude-sonnet-4-20250514'),
    tools: getTriageTools(),
    maxSteps: 10,
    prompt: 'Triage issues',
  });
  
  return result.text;
} catch (error) {
  if (error.name === 'ToolExecutionError') {
    console.error('Tool failed:', error.message);
    // Handle tool failure
  } else if (error.name === 'RateLimitError') {
    console.error('Rate limited, retrying...');
    // Implement retry logic
  } else {
    throw error;
  }
}
```

## Tool Reference

### listIssues

Lists issues with optional filtering.

```typescript
// Parameters
{
  status?: 'open' | 'closed' | 'all';
  labels?: string[];
  assignee?: string;
  limit?: number;
}
```

### createIssue

Creates a new issue.

```typescript
// Parameters
{
  title: string;
  body?: string;
  labels?: string[];
  assignees?: string[];
  milestone?: string;
}
```

### searchIssues

Searches issues by query.

```typescript
// Parameters
{
  query: string;
  sort?: 'created' | 'updated' | 'comments';
  order?: 'asc' | 'desc';
}
```

### addLabels

Adds labels to an issue.

```typescript
// Parameters
{
  issueId: number;
  labels: string[];
}
```

### getPRComments

Gets comments on a pull request.

```typescript
// Parameters
{
  prNumber: number;
}
```

### approvePR

Approves a pull request.

```typescript
// Parameters
{
  prNumber: number;
  comment?: string;
}
```

## Next Steps

- [MCP Server Integration](/integrations/mcp-server/) - Use with Claude Desktop
- [@agentic/triage Package](/packages/triage/) - Full package reference
- [TypeScript Examples](/examples/typescript/) - More examples
