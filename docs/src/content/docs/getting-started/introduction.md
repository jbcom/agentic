---
title: Introduction to Agentic
description: The polyglot AI agent toolkit — what it is, what it solves, and how to start
---

# Introduction to Agentic

**Agentic** is a polyglot toolkit for AI agent orchestration. It gives you the primitives to spawn, coordinate, and manage AI agents across TypeScript, Python, and Rust — without locking you into a single framework or provider.

## The Problem

AI agent tooling is fragmented:

- **Framework lock-in** — CrewAI, LangGraph, and Strands each have different APIs for the same concepts. Switching frameworks means rewriting everything.
- **Token complexity** — Managing GitHub tokens across multiple organizations is manual and error-prone. One wrong token breaks CI, PRs, or agent spawns.
- **No fleet coordination** — Spawning one agent is easy. Coordinating ten across different repos, monitoring their progress, and handling failures? That requires infrastructure.
- **Scattered tools** — Your triage system is separate from your fleet manager which is separate from your 3D pipeline which is separate from your game generator. No shared patterns.

## The Solution

Agentic is five packages that solve these problems independently or together:

### @jbcom/agentic (Control)

<span class="lang-badge lang-badge--ts">TypeScript</span>

The orchestration layer. Spawn AI agent fleets, route tokens per organization, execute tasks in sandboxed Docker containers, and hand off work between agents with full context preservation.

```bash
agentic fleet spawn "my-org/api" "Fix failing CI" --auto-pr
agentic sandbox run "Security audit" --workspace . --timeout 900
```

### @jbcom/agentic (Triage)

<span class="lang-badge lang-badge--ts">TypeScript</span>

Portable triage primitives built on the Vercel AI SDK. Issue management, PR review, sprint planning — as composable tools that work with any AI provider and any project tracker (GitHub, Jira, Linear, Beads).

```typescript
const result = await generateText({
  model: anthropic('claude-sonnet-4-20250514'),
  tools: getTriageTools(),
  prompt: 'Triage all open critical bugs',
});
```

### agentic-crew

<span class="lang-badge lang-badge--py">Python</span>

Framework-agnostic crew orchestration. Define agents and tasks in YAML, run them on CrewAI, LangGraph, or Strands — whichever is installed. Includes a universal CLI runner for aider, claude-code, ollama, and other single-agent tools.

```python
from agentic_crew import run_crew

# Auto-detects best framework at runtime
result = run_crew("my-app", "reviewer", inputs={"pr": 42})
```

### @agentic/meshy

<span class="lang-badge lang-badge--ts">TypeScript</span>

Declarative 3D asset pipelines for Meshy AI. Define text-to-3D, rigging, and animation pipelines in JSON. Built-in 3D preview, VCR recording for CI, and manifest-based state that survives failures.

```bash
content-gen run character ./assets/characters/hero
```

### game-generator

<span class="lang-badge lang-badge--rs">Rust</span>

AI-powered retro RPG generation with Bevy. Procedural worlds, characters, quests, and assets. Multi-provider AI with intelligent caching. Part of the [Strata](https://strata.game) gaming platform.

```rust
let game = QuickGenerator::new()
    .genre(Genre::Jrpg)
    .theme(Theme::Fantasy)
    .generate_and_run();
```

## Key Principles

**Provider agnostic** — Every package works with multiple AI providers. Anthropic, OpenAI, Google, Mistral, Ollama, Azure. Swap providers by changing a config value.

**Declarative over imperative** — Crews in YAML. Pipelines in JSON. Fleet config in JSON. Token routing from environment variables. Configuration lives in version control.

**Security first** — Token sanitization everywhere. SHA-pinned GitHub Actions. OIDC publishing. Sandboxed execution with Docker resource limits. Zero hardcoded credentials.

**Polyglot** — TypeScript for orchestration and web tooling. Python for AI framework integration. Rust for performance-critical game generation. Each language where it's strongest.

## Who Is This For?

| You are... | Agentic gives you... |
|------------|---------------------|
| A developer managing multiple GitHub orgs | Automatic token routing — one config, all orgs |
| A team standardizing AI workflows | Framework-agnostic crews — define once, run anywhere |
| A platform engineer coordinating agents | Fleet management — spawn, monitor, coordinate, hand off |
| A game developer using AI generation | Procedural RPG content with Bevy integration |
| A 3D artist building asset pipelines | Declarative Meshy pipelines with manifest state |

## Next Steps

1. **[Quick Start](/getting-started/quick-start/)** — Install and spawn your first agent in 5 minutes
2. **[Configuration](/getting-started/configuration/)** — Set up tokens, providers, and fleet defaults
3. **[Packages](/packages/control/)** — Deep dive into each package
4. **[Guides](/guides/agent-spawning/)** — Learn orchestration patterns and best practices
