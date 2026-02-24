# agentic-crew

[![PyPI version](https://img.shields.io/pypi/v/agentic-crew.svg)](https://pypi.org/project/agentic-crew/)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python](https://img.shields.io/pypi/pyversions/agentic-crew.svg)](https://pypi.org/project/agentic-crew/)

Framework-agnostic AI crew orchestration. Declare crews once in YAML, then run them on CrewAI, LangGraph, or AWS Strands without changing a single line of crew configuration. The framework is auto-detected at runtime based on what is installed, or you can force a specific runner.

[Full Documentation](https://agentic.coach) | [API Reference](https://agentic.coach/api/crew/) | [Package Docs](https://agentic.coach/packages/crew/)

## Installation

```bash
# Core (no framework -- auto-detects at runtime)
pip install agentic-crew

# With a specific framework
pip install agentic-crew[crewai]      # CrewAI (recommended)
pip install agentic-crew[langgraph]   # LangGraph
pip install agentic-crew[strands]     # AWS Strands

# All frameworks
pip install agentic-crew[ai]
```

## Quick Start

### 1. Define a Crew (YAML)

```yaml
# .crewai/crews/analyzer/agents.yaml
code_reviewer:
  role: Senior Code Reviewer
  goal: Find bugs and improvements
  backstory: Expert at code analysis
```

```yaml
# .crewai/crews/analyzer/tasks.yaml
review_code:
  description: Review the provided code for issues
  expected_output: List of findings with severity
  agent: code_reviewer
```

### 2. Run It

```python
from agentic_crew import run_crew_auto, detect_framework

# See what framework is available
framework = detect_framework()  # "crewai", "langgraph", or "strands"

# Auto-detect best framework and run
result = run_crew_auto(config, inputs={"code": "..."})
```

Or from the CLI:

```bash
agentic-crew run my-package analyzer --input "Review this code: ..."
```

### 3. Use a Specific Runner

```python
from agentic_crew import get_runner

runner = get_runner("langgraph")  # Force LangGraph
crew = runner.build_crew(config)
result = runner.run(crew, inputs)
```

## Key Features

- **Framework agnostic** -- one crew definition, multiple runtime backends
- **Auto-detection** -- picks the best installed framework automatically
- **YAML-first** -- crew configuration in YAML, not Python boilerplate
- **Hierarchical orchestration** -- `ManagerAgent` for delegating across crews
- **Package discovery** -- finds `.crewai/` directories in any project tree
- **CLI and library** -- use from the command line or import as a module

## Framework Priority

1. **CrewAI** (if installed) -- most features, best for complex crews
2. **LangGraph** (if CrewAI unavailable) -- good for flow-based logic
3. **Strands** (fallback) -- lightweight, minimal dependencies

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation including:

- [CrewAI Integration](https://agentic.coach/integrations/crewai/)
- [LangGraph Integration](https://agentic.coach/integrations/langgraph/)
- [Python Examples](https://agentic.coach/examples/python/)

## License

MIT
