---
title: CrewAI Integration
description: Use agentic-crew with CrewAI for multi-agent orchestration
---

# CrewAI Integration

`agentic-crew` provides first-class support for [CrewAI](https://github.com/crewAIInc/crewAI), the popular multi-agent orchestration framework. Define crews once in YAML and run them on CrewAI with full feature support.

## Installation

```bash
# Install with CrewAI support
pip install agentic-crew[crewai]

# Or install everything
pip install agentic-crew[ai]
```

## Quick Start

### 1. Define Your Crew

```yaml
# .crewai/manifest.yaml
name: my-project
version: "1.0"

crews:
  code_reviewer:
    description: Review code for issues and improvements
    agents: crews/code_reviewer/agents.yaml
    tasks: crews/code_reviewer/tasks.yaml
```

```yaml
# crews/code_reviewer/agents.yaml
security_expert:
  role: Security Analyst
  goal: Identify security vulnerabilities in code
  backstory: |
    You are a senior security engineer with 15 years of experience
    in secure coding practices and penetration testing.
  verbose: true
  allow_delegation: false

code_quality_expert:
  role: Code Quality Specialist
  goal: Ensure code follows best practices
  backstory: |
    You are a clean code advocate with deep expertise in
    design patterns, SOLID principles, and maintainability.
  verbose: true
  allow_delegation: true
```

```yaml
# crews/code_reviewer/tasks.yaml
security_review:
  description: |
    Analyze the provided code for security vulnerabilities:
    - SQL injection
    - XSS attacks
    - Authentication issues
    - Hardcoded secrets
  agent: security_expert
  expected_output: Security findings with severity ratings

quality_review:
  description: |
    Review code for quality issues:
    - Code duplication
    - Complex functions
    - Naming conventions
    - Error handling
  agent: code_quality_expert
  expected_output: Quality improvement suggestions
```

### 2. Run Your Crew

```python
from agentic_crew import run_crew

result = run_crew(
    package="my-project",
    crew="code_reviewer",
    inputs={"code_path": "./src"}
)

print(result)
```

Or from CLI:

```bash
agentic-crew run my-project code_reviewer --input "Review ./src"
```

## CrewAI-Specific Features

### Using CrewAI Tools

```yaml
# agents.yaml
researcher:
  role: Research Analyst
  goal: Research topics thoroughly
  backstory: Expert researcher
  tools:
    - search_internet
    - read_website
    - analyze_data
```

### Agent Delegation

Enable agents to delegate tasks:

```yaml
manager:
  role: Project Manager
  goal: Coordinate the team
  backstory: Experienced tech lead
  allow_delegation: true  # Can delegate to other agents

developer:
  role: Developer
  goal: Write code
  backstory: Senior developer
  allow_delegation: false  # Executes tasks directly
```

### Sequential vs Hierarchical

```python
from agentic_crew import run_crew

# Sequential process (default)
result = run_crew(
    package="my-project",
    crew="my_crew",
    inputs={},
    process="sequential"  # Tasks run in order
)

# Hierarchical process
result = run_crew(
    package="my-project",
    crew="my_crew",
    inputs={},
    process="hierarchical"  # Manager delegates
)
```

### Memory and Context

```yaml
# agents.yaml
analyst:
  role: Data Analyst
  goal: Analyze data patterns
  backstory: Expert in data analysis
  memory: true  # Enable memory
  cache: true   # Cache responses
```

### Task Dependencies

```yaml
# tasks.yaml
gather_requirements:
  description: Gather project requirements
  agent: analyst
  expected_output: Requirements document

design_solution:
  description: Design a solution based on requirements
  agent: architect
  expected_output: Architecture document
  context:
    - gather_requirements  # Depends on previous task

implement_solution:
  description: Implement the designed solution
  agent: developer
  expected_output: Working code
  context:
    - design_solution  # Gets context from design
```

## Advanced Configuration

### Custom LLM Configuration

```python
from agentic_crew import get_crew_config, run_crew_auto
from crewai import LLM

# Get crew config
config = get_crew_config("./my-project", "code_reviewer")

# Configure custom LLM
llm = LLM(
    model="anthropic/claude-sonnet-4-20250514",
    temperature=0.7,
    max_tokens=4096
)

# Run with custom LLM
result = run_crew_auto(config, llm=llm)
```

### Using CrewAI Directly

For maximum control, build the CrewAI objects directly:

```python
from agentic_crew.core.decomposer import get_runner

# Get the CrewAI runner
runner = get_runner("crewai")

# Load configuration
config = runner.load_config("./my-project", "code_reviewer")

# Build CrewAI objects
crew = runner.build_crew(config)

# Customize before running
crew.verbose = True
crew.max_rpm = 10

# Run
result = crew.kickoff(inputs={"code_path": "./src"})
```

### Custom Agents in Code

Mix YAML and code definitions:

```python
from crewai import Agent, Task, Crew
from agentic_crew import get_crew_config

# Load base config
config = get_crew_config("./my-project", "code_reviewer")

# Create custom agent
custom_agent = Agent(
    role="Performance Specialist",
    goal="Optimize code performance",
    backstory="Expert in algorithms and optimization",
    verbose=True,
    llm="anthropic/claude-sonnet-4-20250514"
)

# Create custom task
custom_task = Task(
    description="Analyze code for performance bottlenecks",
    agent=custom_agent,
    expected_output="Performance optimization report"
)

# Add to crew
from agentic_crew.core.decomposer import get_runner
runner = get_runner("crewai")
crew = runner.build_crew(config)

crew.agents.append(custom_agent)
crew.tasks.append(custom_task)

result = crew.kickoff()
```

## Best Practices

### 1. Clear Role Definitions

```yaml
# Good: Specific, focused role
security_expert:
  role: API Security Specialist
  goal: Find authentication and authorization vulnerabilities
  backstory: 10 years of API security experience

# Bad: Vague role
helper:
  role: Helper
  goal: Help with things
  backstory: General assistant
```

### 2. Detailed Task Descriptions

```yaml
# Good: Detailed with expected format
security_scan:
  description: |
    Scan the codebase for security issues:
    
    1. Check all API endpoints for authentication
    2. Verify input validation on user-facing forms
    3. Look for SQL injection in database queries
    4. Check for XSS in template rendering
    
    Format findings as:
    - [SEVERITY] File:Line - Description
  expected_output: |
    Markdown table with columns:
    | Severity | Location | Issue | Recommendation |

# Bad: Vague task
scan:
  description: Check for security issues
  expected_output: Report
```

### 3. Appropriate Delegation

```yaml
# Manager can delegate
project_lead:
  allow_delegation: true

# Specialists should execute
security_analyst:
  allow_delegation: false

developer:
  allow_delegation: false
```

## Troubleshooting

### Common Issues

**Crew not finding agents:**
```bash
# Ensure paths are correct in manifest
# agents: crews/my_crew/agents.yaml  ✓
# agents: ./agents.yaml  ✗
```

**Tasks running out of order:**
```yaml
# Use context to define dependencies
task_b:
  context:
    - task_a  # task_a must complete first
```

**LLM rate limits:**
```python
from crewai import Crew

crew = Crew(
    agents=[...],
    tasks=[...],
    max_rpm=10,  # Limit requests per minute
    share_crew=False
)
```

## Next Steps

- [LangGraph Integration](/integrations/langgraph/) - Alternative framework
- [Python Examples](/examples/python/) - More Python examples
- [@agentic/crew Package](/packages/crew/) - Full package reference
