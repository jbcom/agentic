---
title: Python Examples
description: Real-world Python examples using agentic-crew
---

# Python Examples

These examples demonstrate using `agentic-crew` for framework-agnostic AI crew orchestration.

## Minimal Crew Run

From [`crew/examples/python/minimal_run.py`](https://github.com/agentic-dev-library/crew/blob/main/examples/python/minimal_run.py):

```python
from __future__ import annotations

import os

from agentic_crew import discover_packages, get_crew_config, run_crew_auto


def main():
    """
    Minimal example of running a crew programmatically.
    """
    # 1. Discover all packages with .crew/ directories
    packages = discover_packages()
    
    if not packages:
        print("No crew packages found.")
        return

    print(f"Discovered packages: {list(packages.keys())}")
    
    # 2. Pick a package and a crew
    package_name = list(packages.keys())[0]
    package_path = packages[package_name]
    
    # List crews in this package
    from agentic_crew import list_crews
    package_crews = list_crews(package_name)
    
    if not package_crews.get(package_name):
        print(f"No crews found in package {package_name}")
        return
        
    crew_name = package_crews[package_name][0]["name"]
    print(f"Running crew: {package_name}/{crew_name}")
    
    # 3. Load the crew configuration
    config = get_crew_config(package_path, crew_name)
    
    # 4. Run the crew (framework auto-detected)
    inputs = {"input": "Create a simple HTTP connector for a weather API"}
    
    try:
        if not os.environ.get("ANTHROPIC_API_KEY"):
            print("Warning: ANTHROPIC_API_KEY not set.")
            
        result = run_crew_auto(config, inputs=inputs)
        
        print("\n--- Result ---")
        print(result)
        
    except Exception as e:
        print(f"Error running crew: {e}")


if __name__ == "__main__":
    main()
```

## Defining a Crew in YAML

### Manifest File

```yaml
# .crewai/manifest.yaml
name: code-analyzer
version: "1.0"

crews:
  analyzer:
    description: Analyze codebases for issues
    agents: crews/analyzer/agents.yaml
    tasks: crews/analyzer/tasks.yaml
```

### Agents Definition

```yaml
# crews/analyzer/agents.yaml
security_reviewer:
  role: Security Expert
  goal: Identify security vulnerabilities in code
  backstory: |
    You are a senior security engineer with 15 years of experience.
    You specialize in finding OWASP Top 10 vulnerabilities and
    secure coding practices.
  verbose: true
  allow_delegation: false

performance_reviewer:
  role: Performance Engineer
  goal: Find performance bottlenecks and optimization opportunities
  backstory: |
    You are a performance optimization specialist who has worked
    on high-scale systems. You excel at identifying O(n²) algorithms,
    memory leaks, and inefficient database queries.
  verbose: true
  allow_delegation: false

style_reviewer:
  role: Code Quality Expert
  goal: Ensure code follows best practices and is maintainable
  backstory: |
    You are a clean code advocate with deep knowledge of design
    patterns, SOLID principles, and language-specific idioms.
  verbose: true
  allow_delegation: false
```

### Tasks Definition

```yaml
# crews/analyzer/tasks.yaml
security_scan:
  description: |
    Scan the provided code for security vulnerabilities:
    1. Check for SQL injection risks
    2. Look for XSS vulnerabilities
    3. Identify authentication/authorization issues
    4. Find hardcoded secrets or credentials
    5. Check for insecure dependencies
  agent: security_reviewer
  expected_output: |
    A detailed security report with:
    - List of vulnerabilities found
    - Severity rating (critical/high/medium/low)
    - Line numbers and file paths
    - Remediation suggestions

performance_analysis:
  description: |
    Analyze the code for performance issues:
    1. Identify slow algorithms (O(n²) or worse)
    2. Find potential memory leaks
    3. Look for N+1 query patterns
    4. Check for unnecessary computations
    5. Identify caching opportunities
  agent: performance_reviewer
  expected_output: |
    A performance report with:
    - List of bottlenecks found
    - Impact assessment
    - Optimization recommendations
    - Before/after complexity analysis

style_review:
  description: |
    Review code for style and maintainability:
    1. Check naming conventions
    2. Identify code duplication
    3. Find overly complex functions
    4. Check for proper error handling
    5. Verify documentation coverage
  agent: style_reviewer
  expected_output: |
    A code quality report with:
    - Style violations
    - Refactoring suggestions
    - Documentation gaps
    - Complexity metrics
```

## Running a Crew

### From CLI

```bash
# Run with auto-detected framework
agentic-crew run code-analyzer analyzer --input "Analyze ./src"

# Force a specific framework
agentic-crew run code-analyzer analyzer --framework crewai --input "..."

# Use a specific single-agent runner
agentic-crew run --runner aider --input "Fix the bug in auth.py"
```

### From Python

```python
from agentic_crew import run_crew

# Auto-detect framework (CrewAI > LangGraph > Strands)
result = run_crew(
    package="code-analyzer",
    crew="analyzer",
    inputs={"code_path": "./src"}
)

print(result)
```

## Single-Agent Runners

For simpler tasks, use single-agent CLI runners:

```python
from agentic_crew.core.decomposer import get_cli_runner

# Use Aider for quick fixes
aider = get_cli_runner("aider")
result = aider.run("Add error handling to the login function")

# Use Ollama for free local execution
ollama = get_cli_runner("ollama")
result = ollama.run("Fix the typo in README.md", model="deepseek-coder")

# Use Claude Code for complex refactoring
claude = get_cli_runner("claude-code")
result = claude.run("Refactor the authentication module to use JWT")
```

### Available Runners

```bash
# List all available runners
agentic-crew list-runners

# Output:
# ✅ aider: AI pair programming in your terminal
# ✅ claude-code: Anthropic's AI coding agent
# ✅ codex: OpenAI's local coding agent
# ✅ ollama: Free local LLM execution
# ✅ kiro: AWS AI-assisted development CLI
# ✅ goose: Block's extensible AI coding agent
```

## Custom CLI Runner

Define your own CLI tool:

```yaml
# my-runner.yaml
command: "my-coding-assistant"
task_flag: "--task"
auth_env:
  - "MY_API_KEY"
auto_approve: "--yes"
structured_output: "--json"
timeout: 600
```

```python
import yaml
from agentic_crew.core.decomposer import get_cli_runner

with open("my-runner.yaml") as f:
    config = yaml.safe_load(f)

runner = get_cli_runner(config)
result = runner.run("Implement the new feature")
```

## Framework Detection

The framework is auto-detected based on installed packages:

```python
from agentic_crew.core.decomposer import detect_framework, get_runner

# See what's installed
framework = detect_framework()
print(f"Using framework: {framework}")
# "crewai", "langgraph", or "strands"

# Get a runner for the detected framework
runner = get_runner()

# Or force a specific framework
crewai_runner = get_runner("crewai")
langgraph_runner = get_runner("langgraph")
strands_runner = get_runner("strands")
```

## Programmatic Crew Definition

Define crews in code instead of YAML:

```python
from agentic_crew import CrewConfig, AgentConfig, TaskConfig, run_crew_auto

config = CrewConfig(
    name="quick-review",
    agents=[
        AgentConfig(
            name="reviewer",
            role="Code Reviewer",
            goal="Review code for issues",
            backstory="Senior developer with code review expertise",
        ),
    ],
    tasks=[
        TaskConfig(
            name="review",
            description="Review the provided code and list any issues",
            agent="reviewer",
            expected_output="List of issues with severity",
        ),
    ],
)

result = run_crew_auto(config, inputs={"code": "def foo(): pass"})
print(result)
```

## Connector Builder Example

From the vendor-connectors integration:

```python
from agentic_crew import run_crew

# Use the connector builder crew to generate an HTTP connector
result = run_crew(
    package="vendor-connectors",
    crew="connector_builder",
    inputs={
        "api_docs_url": "https://docs.meshy.ai/en",
        "vendor_name": "meshy",
        "output_dir": "./generated/meshy"
    }
)

# The crew will:
# 1. Scrape the API documentation
# 2. Extract endpoints, parameters, and response types
# 3. Generate TypeScript/Python connector code
# 4. Write tests for the connector
```

## Environment Variables

```bash
# For AI providers
export ANTHROPIC_API_KEY="sk-xxx"
export OPENAI_API_KEY="sk-xxx"

# For single-agent runners
export AIDER_MODEL="claude-sonnet-4-20250514"
export OLLAMA_HOST="http://localhost:11434"
```

## Running Tests

```bash
# Install with test dependencies
pip install agentic-crew[tests]

# Run tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=agentic_crew
```

## Next Steps

- [TypeScript Examples](/examples/typescript/) - TypeScript fleet examples
- [CLI Workflows](/examples/cli-workflows/) - Command-line examples
- [CrewAI Integration](/integrations/crewai/) - Deep CrewAI integration
