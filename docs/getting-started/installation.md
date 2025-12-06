# Installation

## TypeScript Core

The TypeScript core provides fleet management, triage, and orchestration capabilities.

### Prerequisites

- Node.js 20.0.0 or higher
- pnpm (recommended) or npm

### Install via npm

```bash
npm install agentic-control
```

### Install via pnpm

```bash
pnpm add agentic-control
```

### Optional AI Provider Dependencies

Install the AI provider(s) you want to use:

```bash
# Anthropic (Claude)
pnpm add @ai-sdk/anthropic

# OpenAI
pnpm add @ai-sdk/openai

# Google (Gemini)
pnpm add @ai-sdk/google

# Azure OpenAI
pnpm add @ai-sdk/azure

# Mistral
pnpm add @ai-sdk/mistral
```

## Python CrewAI

The Python component provides CrewAI-powered autonomous agents for development tasks.

### Prerequisites

- Python 3.11 or higher
- uv (recommended) or pip

### Install with uv

```bash
cd python
uv sync
```

### Install with pip

```bash
pip install -e python/
```

### Optional Dependencies

```bash
# For running tests
uv sync --extra tests

# For MCP server
uv sync --extra mcp

# For documentation
uv sync --extra docs
```
