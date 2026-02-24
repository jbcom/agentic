---
title: "agentic-crew"
description: Framework-agnostic AI crew orchestration — define once, run on CrewAI, LangGraph, or Strands
---

# agentic-crew

<div class="polyglot-bar">
  <span class="lang-badge lang-badge--py">Python</span>
</div>

> Define AI crews in YAML. Run them on CrewAI, LangGraph, or Strands — whichever is installed. One format, any framework.

[![PyPI version](https://badge.fury.io/py/agentic-crew.svg)](https://pypi.org/project/agentic-crew/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## The Problem

The AI agent ecosystem is fragmented. CrewAI, LangGraph, and Strands each have different APIs for the same concepts — agents, tasks, and orchestration. Switching frameworks means rewriting your crew definitions. And single-agent CLI tools (aider, claude-code, ollama) each have their own invocation patterns.

## The Solution

agentic-crew gives you:

1. **Universal crew format** — YAML definitions that work across all three frameworks
2. **Runtime auto-detection** — Automatically uses whichever framework is installed
3. **Universal CLI runner** — One interface for aider, claude-code, ollama, codex, kiro, and goose
4. **Custom runner support** — Define your own CLI tools in YAML

## Installation

```bash
# Core (auto-detects framework at runtime)
pip install agentic-crew

# With a specific framework
pip install agentic-crew[crewai]      # CrewAI (recommended)
pip install agentic-crew[langgraph]   # LangGraph
pip install agentic-crew[strands]     # AWS Strands

# All frameworks
pip install agentic-crew[ai]
```

## Quick Start

### 1. Define a Crew

```yaml
# .crewai/manifest.yaml
name: my-package
version: "1.0"
crews:
  analyzer:
    description: Analyze codebases
    agents: crews/analyzer/agents.yaml
    tasks: crews/analyzer/tasks.yaml
```

```yaml
# crews/analyzer/agents.yaml
code_reviewer:
  role: Senior Code Reviewer
  goal: Find bugs and improvements
  backstory: Expert at code analysis
```

```yaml
# crews/analyzer/tasks.yaml
review_code:
  description: Review the provided code for issues
  expected_output: List of findings with severity
  agent: code_reviewer
```

### 2. Run It

```python
from agentic_crew import run_crew

# Auto-detects best framework (CrewAI > LangGraph > Strands)
result = run_crew("my-package", "analyzer", inputs={"code": "..."})
```

```bash
# Or from the CLI
agentic-crew run my-package analyzer --input "Review this code"
```

## Single-Agent CLI Runners

For simpler tasks, run CLI-based coding tools through a unified interface:

| Runner | Description | Local? |
|--------|-------------|--------|
| **aider** | AI pair programming | No |
| **claude-code** | Anthropic's coding agent | No |
| **codex** | OpenAI's coding agent | No |
| **ollama** | Free local LLM execution | Yes |
| **kiro** | AWS AI development CLI | No |
| **goose** | Block's extensible agent | No |

```bash
# Aider for quick fixes
agentic-crew run --runner aider --input "Add error handling to auth.py"

# Claude Code for refactoring
agentic-crew run --runner claude-code --input "Refactor the database module"

# Ollama for free local execution
agentic-crew run --runner ollama --input "Fix the bug" --model deepseek-coder
```

### When to Use What

| Scenario | Use |
|----------|-----|
| Complex multi-step tasks needing collaboration | Multi-agent crew |
| Quick edits, simple file changes | Single-agent CLI runner |
| Local development without API keys | Ollama runner |
| Tasks requiring planning and delegation | Multi-agent crew |

## Framework Auto-Detection

agentic-crew checks what's installed and picks the best option:

```python
from agentic_crew.core.decomposer import detect_framework, get_runner

framework = detect_framework()  # "crewai", "langgraph", or "strands"
runner = get_runner()           # Auto-selects best available
runner = get_runner("langgraph") # Force a specific framework
```

**Priority order:**
1. **CrewAI** — Most features, best for complex crews
2. **LangGraph** — Good for flow-based orchestration
3. **Strands** — Lightweight, minimal dependencies

## Programmatic API

```python
from agentic_crew import CrewConfig, AgentConfig, TaskConfig, run_crew_auto

config = CrewConfig(
    name="review-crew",
    agents=[
        AgentConfig(
            name="security_reviewer",
            role="Security Expert",
            goal="Identify security vulnerabilities",
            backstory="10+ years of security auditing",
        ),
        AgentConfig(
            name="performance_reviewer",
            role="Performance Engineer",
            goal="Identify performance bottlenecks",
            backstory="Expert in optimization and profiling",
        ),
    ],
    tasks=[
        TaskConfig(
            name="security_review",
            description="Review code for security issues",
            agent="security_reviewer",
            expected_output="Vulnerabilities with severity ratings",
        ),
        TaskConfig(
            name="performance_review",
            description="Analyze code for performance issues",
            agent="performance_reviewer",
            expected_output="Bottlenecks with recommendations",
        ),
    ],
)

result = run_crew_auto(config, inputs={"code_path": "./src"})
```

## Package Structure

Any package can include crew definitions:

```
my-package/
├── .crewai/
│   ├── manifest.yaml
│   ├── knowledge/
│   │   └── domain_docs/
│   └── crews/
│       └── my_crew/
│           ├── agents.yaml
│           └── tasks.yaml
└── src/
```

## Development

```bash
# Install with dev deps
uv sync --extra dev --extra tests --extra crewai

# Run tests
uv run pytest tests/ -v

# Lint
uvx ruff check src/ tests/ --fix
```

## Related

- **[CrewAI](https://github.com/crewAIInc/crewAI)** — Original crew framework
- **[LangGraph](https://github.com/langchain-ai/langgraph)** — Graph-based agents
- **[Strands](https://github.com/strands-agents/strands-agents-python)** — AWS agent framework
- **[@jbcom/agentic](/packages/control/)** — Fleet management and orchestration
- **[@jbcom/agentic (Triage)](/packages/triage/)** — Triage primitives

## Links

- [GitHub](https://github.com/jbcom/agentic/tree/main/packages/agentic-crew)
- [PyPI](https://pypi.org/project/agentic-crew/)
