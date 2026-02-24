---
title: Introduction to Agentic
description: Learn about the Agentic ecosystem and how it transforms AI agent development
---

# Introduction to Agentic

**Agentic** is the AI & Agents division of the [jbcom enterprise](https://jbcom.github.io), providing a unified toolkit for AI agent development, deployment, and orchestration. Whether you're building AI-powered development workflows, coordinating multi-agent systems, or triaging issues at scale, Agentic has you covered.

## What is Agentic?

Agentic is a comprehensive ecosystem of tools designed to make working with AI agents as seamless as possible:

### 🎯 **Intelligent Multi-Org Token Management**
Automatically routes GitHub operations to the correct tokens based on repository organization. No more manual token switching!

```typescript
import { getTokenForRepo } from '@agentic-dev-library/control';

// Automatically uses GITHUB_MYCOMPANY_TOKEN for my-company repos
const token = getTokenForRepo('my-company/my-repo');
```

### 🚀 **AI Agent Fleet Management**
Spawn, monitor, and coordinate multiple Cursor Background Agents working simultaneously across your repositories.

```bash
# Spawn agents across multiple repos
agentic fleet spawn "my-org/frontend" "Update React to v18" --auto-pr
agentic fleet spawn "my-org/backend" "Update Node.js dependencies" --auto-pr
agentic fleet spawn "my-org/mobile" "Update React Native" --auto-pr
```

### 🏗️ **Secure Sandbox Execution**
Run AI agents in isolated Docker containers with resource limits, workspace mounting, and parallel execution support.

```bash
agentic sandbox run "Perform security audit" \
  --workspace . --output ./security-audit --timeout 900
```

### 🔍 **Advanced AI Triage & Analysis**
Leverage multiple AI providers (Anthropic, OpenAI, Google, Mistral) for code review, conversation analysis, and task extraction.

```typescript
import { getTriageTools } from '@agentic-dev-library/triage';
import { generateText } from 'ai';

const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'List all open critical bugs and create a triage plan',
});
```

### 🤝 **Framework-Agnostic Crew Orchestration**
Define AI crews once, run on CrewAI, LangGraph, or Strands. Universal CLI runner for all major coding tools.

```python
from agentic_crew import run_crew

# Auto-detects best framework (CrewAI > LangGraph > Strands)
result = run_crew("my-package", "analyzer", inputs={"code": "..."})
```

## The Agentic Ecosystem

| Package | Language | Purpose |
|---------|----------|---------|
| **[@agentic/control](/packages/control/)** | TypeScript | Fleet management, token switching, sandbox execution |
| **[@agentic/crew](/packages/crew/)** | Python | Framework-agnostic crew orchestration |
| **[@agentic/triage](/packages/triage/)** | TypeScript | Portable triage primitives for AI agents |
| **[game-generator](/packages/game-generator/)** | Rust | AI-powered retro RPG generation |

## Enterprise Context

Agentic is part of a coherent suite of specialized tools within the jbcom enterprise:

- **[jbcom Hub](https://jbcom.github.io)** - Central documentation and portfolio
- **[Strata](https://strata.game)** - AI-powered gaming platform
- **[Extended Data](https://extendeddata.dev)** - Enterprise infrastructure tools

All tools share a unified design system and are interconnected for seamless workflows.

## Key Benefits

### For Individual Developers
- **Reduced Context Switching**: Smart token management handles multi-org authentication automatically
- **Parallel Development**: Spawn multiple AI agents to work on different tasks simultaneously
- **AI-Assisted Code Review**: Get intelligent feedback on your PRs before human review

### For Teams
- **Standardized Workflows**: Define crews once, run consistently across all team members
- **Secure Execution**: Sandbox mode ensures AI operations don't affect production systems
- **Full Audit Trail**: Every agent action is logged and can be analyzed

### For Enterprises
- **Multi-Org Support**: Manage multiple GitHub organizations with different access tokens
- **OIDC Publishing**: Secure npm publishing without long-lived tokens
- **Compliance Ready**: Token sanitization and audit logs for security requirements

## Next Steps

Ready to get started? Here's your path:

1. **[Quick Start](/getting-started/quick-start/)** - Get up and running in 5 minutes
2. **[Configuration](/getting-started/configuration/)** - Customize Agentic for your workflow
3. **[TypeScript Examples](/examples/typescript/)** - Learn from real-world code
4. **[API Reference](/api/fleet-management/)** - Deep dive into the API
