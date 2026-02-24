---
title: Architecture Overview
description: System design, package relationships, data flow, event system, and extension points for the Agentic polyglot AI agent orchestration toolkit.
---

The Agentic toolkit is a polyglot system spanning TypeScript, Python, and Rust. Each language targets the domain where it excels, and the packages communicate through well-defined interfaces.

## System Design Diagram

```
                              +---------------------+
                              |    GitHub / CI/CD    |
                              |  (Issues, PRs, Actions)|
                              +----------+----------+
                                         |
                          GitHub API / Webhooks / MCP
                                         |
              +--------------------------+-------------------------+
              |                          |                         |
   +----------v----------+   +----------v----------+   +----------v----------+
   |   @jbcom/agentic    |   | @jbcom/agentic-triage|   |   agentic-crew      |
   |   (Fleet Control)   |   |   (Triage Engine)   |   |  (Crew Orchestration)|
   |                     |   |                     |   |                     |
   | - Fleet management  |   | - AI SDK tools      |   | - YAML crew defs    |
   | - Agent spawning    |   | - MCP server        |   | - Framework runners |
   | - Sandbox execution |   | - Issue/PR/Review   |   | - Auto-detection    |
   | - Handoff protocols |   | - Scoring & queues  |   | - Manager agents    |
   | - CLI               |   | - Escalation ladder |   | - CLI               |
   +-----+----+----------+   +-----+----+----------+   +----------+----------+
         |    |                     |    |                         |
         |    |    consumes         |    |                         |
         |    +---------------------+    |                         |
         |                               |                         |
   +-----v-----------+   +--------------v-------+   +-------------v--------+
   | @jbcom/agentic-  |   | @jbcom/vitest-agentic|   | pytest-agentic-crew  |
   |    providers     |   |   (TS Test Plugin)   |   |  (Py Test Plugin)    |
   |                  |   +----------------------+   +----------------------+
   | - Ollama agent   |
   | - Jules agent    |   +----------------------+
   | - Cursor agent   |   | @jbcom/agentic-meshy |
   +---------+--------+   |  (3D Asset Pipelines)|
             |             +----------------------+
             |
   +---------v--------+   +----------------------+
   |   LLM Services   |   |   game-generator     |
   | Ollama/Jules/     |   |  (Rust Game Engine)  |
   | Cursor/Anthropic  |   +----------------------+
   +-------------------+
```

## Package Responsibilities

### @jbcom/agentic (TypeScript)

The control-plane package. It owns:

- **Fleet** -- spawn, monitor, coordinate, and archive Cursor-based AI agents.
- **Triage** -- multi-provider issue and PR management (GitHub, Jira, Linear, Beads).
- **Providers** -- unified model factory for OpenAI, Anthropic, Google, and more.
- **Sandbox** -- Docker-based isolated execution for untrusted code.
- **Handoff** -- agent-to-agent task delegation with context preservation.

### @jbcom/agentic-triage (TypeScript)

The foundational triage engine. It defines:

- Vercel AI SDK tool schemas for issue triage, PR review, and sprint planning.
- An MCP server exposing those tools to Claude Desktop, Cursor, and other MCP clients.
- Triage provider abstractions (GitHub, Linear, Jira, Beads).
- Scoring, queuing, escalation, and storage subsystems.

### @jbcom/agentic-providers (TypeScript)

Concrete LLM agent implementations:

- **Ollama** -- free, self-hosted inference for simple tasks.
- **Jules** -- free-tier async agent for complex multi-file work.
- **Cursor** -- premium last-resort agent for expert-level tasks.

### agentic-crew (Python)

Framework-agnostic crew orchestration:

- Define crews of specialized agents with distinct roles and tools.
- Run crews on CrewAI, LangGraph, or AWS Strands without code changes.
- Integrates with the TypeScript layer through the `CrewTool` subprocess bridge.

### game-generator (Rust)

Visual-first vintage game generator:

- Bevy-powered desktop UI with guided wizard and freeform conversation modes.
- AI-assisted content generation (text, image, audio, code).
- Game mechanic blending via graph-based similarity scoring.

## Package Dependency Graph

### Core Dependency Chain

```
@jbcom/agentic  --->  @jbcom/agentic-triage  (workspace dependency)
@jbcom/agentic-providers  --->  @jbcom/agentic-triage  (peer dependency)
@jbcom/vitest-agentic  (standalone, mocks agentic components)
pytest-agentic-crew  (standalone, fixtures for agentic-crew)
@jbcom/agentic-meshy  (standalone)
game-generator  (standalone Rust crate)
agentic-crew  (standalone Python package)
```

### Cross-Language Bridge

The TypeScript and Python ecosystems are connected through:

1. **CLI invocation** -- `@jbcom/agentic` spawns `agentic-crew` CLI commands via subprocess.
2. **YAML configuration** -- Both ecosystems read the same `.crewai/` crew definitions.
3. **GitHub Actions** -- CI workflows orchestrate both TypeScript and Python packages.

## Data Flow for Fleet Operations

A typical fleet operation follows this path:

```
User CLI command
    |
    v
agentic CLI (Commander.js)
    |
    v
Fleet class (fleet.ts)
    |
    +---> CursorAPI.spawn()  -----> Cursor Background Agent API
    |         |
    |         v
    |     Agent runs on Cursor infrastructure
    |         |
    |         v
    +---> Fleet.list()  -----> Cursor API (poll agent status)
    |
    v
HandoffManager (if transferring work)
    |
    +---> GitHubClient (token-aware)
    |         |
    |         +---> Token resolution (org -> env var -> value)
    |         |
    |         v
    |     GitHub API (create PR, post comments, manage labels)
    |
    v
AIAnalyzer (if triage requested)
    |
    +---> Provider resolution (config -> SDK -> API call)
    |
    v
Triage result (structured analysis, issues, review comments)
```

### Token Resolution Flow

```
Repository URL "github.com/my-org/my-repo"
    |
    v
Extract organization: "my-org"
    |
    v
Check config.tokens.organizations["my-org"]
    |
    +---> Found: read tokenEnvVar -> read env var -> return token
    |
    +---> Not found: check AGENTIC_ORG_MY_ORG_TOKEN env var
    |         |
    |         +---> Found: read referenced env var -> return token
    |         |
    |         +---> Not found: fall back to defaultTokenEnvVar
    |
    v
Token value (used for all GitHub API calls for this org)
```

## Event System

### Triage Scoring and Routing

The triage package uses a scoring pipeline to evaluate and route tasks:

```
Incoming task (issue, PR, or free-text)
    |
    v
Scoring Evaluator
    |
    +---> Complexity score (code changes, file count, etc.)
    +---> Priority score (labels, severity, staleness)
    +---> Cost estimate (based on provider pricing)
    |
    v
Scoring Router
    |
    +---> Route to cheapest capable agent
    |     (Ollama for trivial, Jules for medium, Cursor for complex)
    |
    v
Escalation Ladder (if initial agent fails)
    |
    +---> Cost tracker enforces daily budget
    +---> Escalate to next tier if within budget
    +---> Block and notify if budget exceeded
    |
    v
Queue Manager
    |
    +---> Priority queue with locking
    +---> Persistent storage (file, memory, or GitHub issue)
    |
    v
Agent execution (via provider implementation)
```

### Sandbox Execution Pipeline

```
SandboxExecutor.execute(options)
    |
    v
ContainerManager.createContainer(config)
    |
    +---> Build Docker image (if custom Dockerfile)
    +---> Configure resource limits (memory, CPU, timeout)
    +---> Mount workspace volume (read-only by default)
    |
    v
Runtime selection (Claude or Cursor)
    |
    +---> claude runtime: mount Claude CLI into container
    +---> cursor runtime: mount Cursor agent into container
    |
    v
Container execution
    |
    +---> Agent runs in isolated environment
    +---> Output written to mounted output directory
    +---> Timeout enforcement (kill after deadline)
    |
    v
ContainerResult { success, stdout, stderr, exitCode, outputDir }
```

## Extension Points

### Adding a New Triage Provider

To add a new issue tracking provider (beyond GitHub, Linear, Jira, Beads):

1. Implement the `TriageProvider` interface in `packages/triage/src/providers/`.
2. Register the provider in `packages/triage/src/providers/index.ts`.
3. Add auto-detection logic if applicable.

### Adding a New LLM Provider

To add a new agent provider (beyond Ollama, Jules, Cursor):

1. Create a new file in `packages/providers/src/`.
2. Export a factory function that returns an agent object with `id`, `capabilities`, `costPerTask`, and `execute`.
3. Export from `packages/providers/src/index.ts`.

### Adding a New Crew Runner

To add support for a new AI framework in `agentic-crew`:

1. Create a runner in `packages/agentic-crew/src/agentic_crew/runners/` extending `BaseRunner`.
2. Register the runner in `packages/agentic-crew/src/agentic_crew/runners/__init__.py`.
3. Add it to the detection priority in `core/decomposer.py`.

### Adding a New Sandbox Runtime

To add a new container runtime (beyond Claude and Cursor):

1. Create a runtime module in `packages/agentic-control/src/sandbox/runtime/`.
2. Register the runtime in `packages/agentic-control/src/sandbox/runtime/index.ts`.

### Adding a New CLI Command

Add the command in `packages/agentic-control/src/cli.ts` using Commander.js. The CLI loads configuration via cosmiconfig and resolves tokens before dispatching to the appropriate subsystem.

## Technology Stack

| Layer | Technology |
|-------|-----------|
| TypeScript runtime | Node.js >= 22, ES modules |
| TypeScript build | `tsc` (agentic), `tsup` (triage, meshy) |
| TypeScript lint/format | Biome |
| TypeScript test | Vitest |
| AI SDK | Vercel AI SDK (`ai` package) |
| MCP | `@modelcontextprotocol/sdk` |
| GitHub API | Octokit |
| Python runtime | Python >= 3.10 |
| Python build | Hatchling |
| Python lint/format | Ruff |
| Python test | Pytest |
| Rust runtime | Rust >= 1.85 |
| Rust UI | Bevy + egui |
| Documentation | Astro + Starlight |
| Package management | pnpm (TS), uv (Python), cargo (Rust) |
| CI/CD | GitHub Actions |

## Configuration

All packages read from a shared `agentic.config.json` (or `.agenticrc`, or `package.json` "agentic" key) at the repository root, loaded via [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig). See the [Configuration guide](/getting-started/configuration/) for the full schema.

## Next Steps

- [Fleet Management guide](/guides/fleet-management/) -- hands-on fleet orchestration
- [Orchestration Patterns](/guides/orchestration-patterns/) -- diamond, fan-out, pipeline
- [Troubleshooting](/guides/troubleshooting/) -- common issues and solutions
