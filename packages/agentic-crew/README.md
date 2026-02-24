# agentic-crew

**Framework-agnostic AI crew orchestration** - declare once, run on CrewAI, LangGraph, or Strands.

## Why agentic-crew?

The AI agent ecosystem is fragmented:
- **CrewAI** - Full-featured but heavyweight
- **LangGraph** - Great for flows but different API
- **Strands** - Lightweight but AWS-specific

agentic-crew solves this with a **universal crew format** that decomposes to any framework.

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

# Auto-detects best framework
result = run_crew("my-package", "analyzer", inputs={"code": "..."})
```

Or from CLI:

```bash
agentic-crew run my-package analyzer --input "Review this code: ..."
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

## Use Cases

### 1. Connector Builder (vendor-connectors)

A crew that scrapes API docs and generates HTTP connectors:

```bash
agentic-crew run vendor-connectors connector_builder \
  --input '{"api_docs": "https://docs.meshy.ai/en"}'
```

### 2. Code Generation (any project)

Define crews for your specific domain and run them on any framework.

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

- [vendor-connectors](https://github.com/jbcom/vendor-connectors) - HTTP connector library
- [CrewAI](https://github.com/crewAIInc/crewAI) - Original crew framework
- [LangGraph](https://github.com/langchain-ai/langgraph) - Graph-based agents
- [Strands](https://github.com/strands-agents/strands-agents-python) - AWS agent framework

## License

MIT
