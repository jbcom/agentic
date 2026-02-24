---
title: "@agentic/crew"
description: Framework-agnostic AI crew orchestration - declare once, run on CrewAI, LangGraph, or Strands
---

# @agentic/crew

**Framework-agnostic AI crew orchestration** - define once, run on CrewAI, LangGraph, Strands, or single-agent CLI tools.

## Why @agentic/crew?

The AI agent ecosystem is fragmented:
- **Multi-agent frameworks** (CrewAI, LangGraph, Strands) - Different APIs for the same concepts
- **Single-agent CLI tools** (aider, claude-code, ollama) - Each with different invocation patterns

@agentic/crew solves this with:
1. **Universal multi-agent format** - Define once, run on any framework
2. **Universal CLI runner** - Single interface for all coding CLI tools
3. **Smart orchestration** - Use the right tool for the job automatically

## Installation

```bash
# Core (no framework - auto-detects at runtime)
pip install agentic-crew

# With specific framework
pip install agentic-crew[crewai]      # CrewAI (recommended)
pip install agentic-crew[langgraph]   # LangGraph
pip install agentic-crew[strands]     # AWS Strands

# All frameworks
pip install agentic-crew[ai]
```

## Quick Start

### 1. Define a Crew (YAML)

Create a manifest file:

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

Define your agents:

```yaml
# crews/analyzer/agents.yaml
code_reviewer:
  role: Senior Code Reviewer
  goal: Find bugs and improvements
  backstory: Expert at code analysis
```

Define tasks:

```yaml
# crews/analyzer/tasks.yaml
review_code:
  description: Review the provided code for issues
  expected_output: List of findings with severity
  agent: code_reviewer
```

### 2. Run It

**Python API:**

```python
from agentic_crew import run_crew

# Auto-detects best framework
result = run_crew("my-package", "analyzer", inputs={"code": "..."})
```

**CLI:**

```bash
agentic-crew run my-package analyzer --input "Review this code: ..."
```

## Single-Agent CLI Runners

For simpler tasks that don't need multi-agent collaboration, @agentic/crew supports direct execution via CLI-based coding tools.

### Available Runners

```bash
# List available single-agent runners
agentic-crew list-runners
```

| Runner | Description | API Key Required |
|--------|-------------|------------------|
| **aider** | AI pair programming in your terminal | Yes |
| **claude-code** | Anthropic's AI coding agent | Yes |
| **codex** | OpenAI's local coding agent | Yes |
| **ollama** | Free local LLM execution | No |
| **kiro** | AWS AI-assisted development CLI | Yes |
| **goose** | Block's extensible AI coding agent | Yes |

### Usage Examples

```bash
# Quick code fixes with Aider
agentic-crew run --runner aider --input "Add error handling to auth.py"

# Code generation with Claude Code
agentic-crew run --runner claude-code --input "Refactor the database module"

# Free local execution with Ollama
agentic-crew run --runner ollama --input "Fix the bug in utils.py" --model deepseek-coder

# With specific model
agentic-crew run --runner aider --input "Add tests" --model gpt-4o
```

### When to Use Single-Agent vs Multi-Agent

| Use Case | Best Choice |
|----------|-------------|
| Complex task with multiple steps requiring collaboration | Multi-agent crew |
| Sequential coding tasks, quick edits | Single-agent CLI |
| Simple file generation or code fixes | Single-agent CLI |
| Local development and testing | Single-agent CLI (especially ollama) |
| Tasks requiring planning and delegation | Multi-agent crew |

### Custom CLI Runners

Define your own CLI tool runner:

```yaml
# my-custom-runner.yaml
command: "my-coding-assistant"
task_flag: "--task"
auth_env:
  - "MY_API_KEY"
auto_approve: "--yes"
structured_output: "--json"
timeout: 600
```

Use it:

```python
import yaml
from agentic_crew.core.decomposer import get_cli_runner

with open("my-custom-runner.yaml") as f:
    custom_config = yaml.safe_load(f)

runner = get_cli_runner(custom_config)
result = runner.run("Fix the bug")
```

## Framework Decomposition

The magic happens in `core/decomposer.py`:

```python
from agentic_crew.core.decomposer import detect_framework, get_runner

# See what's available
framework = detect_framework()  # "crewai", "langgraph", or "strands"

# Get a runner
runner = get_runner()  # Auto-selects best
runner = get_runner("langgraph")  # Force specific

# Build and run
crew = runner.build_crew(config)
result = runner.run(crew, inputs)
```

### Framework Priority

1. **CrewAI** (if installed) - Most features, best for complex crews
2. **LangGraph** (if CrewAI unavailable) - Good for flow-based logic
3. **Strands** (fallback) - Lightweight, minimal deps

## Package Integration

Any package can define crews in a `.crewai/` directory:

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

Then run:

```bash
agentic-crew run my-package my_crew --input "..."
```

## Programmatic Usage

### Minimal Example

```python
from agentic_crew import discover_packages, get_crew_config, run_crew_auto

# Discover packages with crew configurations
packages = discover_packages()
print(f"Discovered packages: {list(packages.keys())}")

# Pick a package and crew
package_name = list(packages.keys())[0]
package_path = packages[package_name]

# List crews in package
from agentic_crew import list_crews
package_crews = list_crews(package_name)
crew_name = package_crews[package_name][0]["name"]

# Load configuration
config = get_crew_config(package_path, crew_name)

# Run the crew (framework auto-detected)
inputs = {"input": "Create a simple HTTP connector for a weather API"}
result = run_crew_auto(config, inputs=inputs)

print(result)
```

### Advanced Configuration

```python
from agentic_crew import CrewConfig, AgentConfig, TaskConfig

# Programmatic crew definition
config = CrewConfig(
    name="code-review-crew",
    agents=[
        AgentConfig(
            name="security_reviewer",
            role="Security Expert",
            goal="Identify security vulnerabilities",
            backstory="10+ years of security auditing experience",
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
            expected_output="List of vulnerabilities with severity",
        ),
        TaskConfig(
            name="performance_review",
            description="Analyze code for performance issues",
            agent="performance_reviewer",
            expected_output="List of bottlenecks with recommendations",
        ),
    ],
)

result = run_crew_auto(config, inputs={"code_path": "./src"})
```

## Use Cases

### Connector Builder (vendor-connectors)

A crew that scrapes API docs and generates HTTP connectors:

```bash
agentic-crew run vendor-connectors connector_builder \
  --input '{"api_docs_url": "https://docs.meshy.ai/en", "vendor_name": "meshy"}'
```

### Code Review Crew

Define specialized reviewers for comprehensive code review:

```yaml
# crews/review/agents.yaml
security_expert:
  role: Security Reviewer
  goal: Find security vulnerabilities
  backstory: Expert in OWASP Top 10 and secure coding

performance_expert:
  role: Performance Reviewer
  goal: Identify performance issues
  backstory: Expert in algorithms and optimization

style_expert:
  role: Style Reviewer
  goal: Ensure code consistency
  backstory: Expert in clean code principles
```

```yaml
# crews/review/tasks.yaml
security_review:
  description: Review for security vulnerabilities
  agent: security_expert
  expected_output: Security findings report

performance_review:
  description: Analyze for performance issues
  agent: performance_expert
  expected_output: Performance analysis report

style_review:
  description: Check code style and consistency
  agent: style_expert
  expected_output: Style improvement suggestions
```

### Documentation Generator

```yaml
# crews/docs/agents.yaml
technical_writer:
  role: Technical Writer
  goal: Create clear documentation
  backstory: Expert in API documentation

example_creator:
  role: Example Author
  goal: Write practical code examples
  backstory: Developer advocate with teaching experience
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

## Related Projects

- **[CrewAI](https://github.com/crewAIInc/crewAI)** - Original crew framework
- **[LangGraph](https://github.com/langchain-ai/langgraph)** - Graph-based agents
- **[Strands](https://github.com/strands-agents/strands-agents-python)** - AWS agent framework
- **[@agentic/control](/packages/control/)** - Fleet management and orchestration
- **[@agentic/triage](/packages/triage/)** - Triage primitives

## Links

- [GitHub Repository](https://github.com/agentic-dev-library/crew)
- [PyPI Package](https://pypi.org/project/agentic-crew/)
