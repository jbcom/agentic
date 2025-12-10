# Product Overview

agentic-control is a unified AI agent fleet management, triage, and orchestration toolkit for control centers.

## Core Capabilities

- **Fleet Management**: Spawn, monitor, and coordinate Cursor Background Agents
- **AI-Powered Triage**: Analyze conversations, review code, extract tasks using pluggable AI providers
- **Token Switching**: Automatically selects the correct GitHub token based on organization
- **Multi-Org Support**: Manage agents across multiple GitHub organizations
- **Station-to-Station Handoff**: Seamless agent continuity across sessions
- **CrewAI Integration**: Python-based autonomous development crews

## Dual-Language Architecture

- **TypeScript (npm: agentic-control)**: CLI, fleet management, triage, GitHub integration, MCP server
- **Python (PyPI: agentic-control-crews)**: CrewAI engine for autonomous AI crews

## Key Features

- Security-first design (no hardcoded values, all user-configured)
- Pluggable AI providers (Anthropic, OpenAI, Google, Mistral, Azure)
- MCP (Model Context Protocol) integration
- Configuration via cosmiconfig (agentic.config.json)
- Intelligent token management for multi-org workflows
