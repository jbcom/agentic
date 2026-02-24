---
title: "Crew API Reference"
description: "Complete API reference for the agentic-crew Python package, covering YAML manifests, framework-agnostic runners, CLI commands, and the manager agent API."
---

# agentic-crew API Reference

The `agentic-crew` package is a framework-agnostic crew runner for AI agent orchestration. It discovers and executes crews defined in YAML manifests, supporting CrewAI, LangGraph, and Strands as execution backends.

## Installation

```bash
# Core package
pip install agentic-crew

# With framework extras
pip install agentic-crew[crewai]
pip install agentic-crew[langgraph]
pip install agentic-crew[strands]
```

Or with `uv`:

```bash
uv add agentic-crew
uv add agentic-crew --extra crewai
```

## Import

```python
from agentic_crew.core.runner import run_crew, run_crew_from_path
from agentic_crew.core.discovery import (
    discover_packages,
    discover_all_framework_configs,
    load_manifest,
    get_crew_config,
    list_crews,
)
from agentic_crew.core.loader import (
    create_agent_from_config,
    create_task_from_config,
    load_crew_from_config,
)
from agentic_crew.core.manager import ManagerAgent
from agentic_crew.runners.base import BaseRunner
```

---

## YAML Manifest Format

Each package with crew configurations contains a `manifest.yaml` file in a framework directory. The directory name determines which framework is used (or allowed):

| Directory | Framework | Behavior |
|-----------|-----------|----------|
| `.crew/` | Auto-detect | Framework-agnostic; uses the best available framework at runtime |
| `.crewai/` | CrewAI | Enforces CrewAI as the execution framework |
| `.langgraph/` | LangGraph | Enforces LangGraph as the execution framework |
| `.strands/` | Strands | Enforces Strands as the execution framework |

### manifest.yaml

The manifest declares the crews available in a package and their configuration file references:

```yaml
# packages/my-project/.crew/manifest.yaml

# Optional: LLM configuration shared across all crews
llm:
  model: "claude-sonnet-4-20250514"
  temperature: 0.7

crews:
  game_builder:
    description: "Build game components from specifications"
    agents: "agents.yaml"
    tasks: "tasks.yaml"
    knowledge:
      - "../docs"
      - "../src"
    preferred_framework: "auto"   # or "crewai", "langgraph", "strands"

  qa_validator:
    description: "Validate code quality and test coverage"
    agents: "qa_agents.yaml"
    tasks: "qa_tasks.yaml"
    llm:
      model: "claude-haiku-4-5-20251001"  # Per-crew LLM override
```

### agents.yaml

Defines the AI agents with their roles, goals, backstories, and tool access:

```yaml
# packages/my-project/.crew/agents.yaml

technical_director:
  role: "Technical Director & Workflow Coordinator"
  goal: "Orchestrate workflow, delegate to specialists, validate deliverables"
  backstory: |
    You are an experienced technical director who understands how to
    decompose complex tasks into vertical slices. You validate each
    slice before proceeding and maintain architectural coherence.
  llm: "claude-sonnet-4-20250514"    # Optional model override
  max_iter: 40                        # Maximum iterations
  max_reasoning_attempts: 5           # Maximum reasoning retries
  allow_delegation: true              # Can delegate to other agents
  tools:
    - "mcp://git/execute_command"
    - "mcp://filesystem/read_file"
    - "mcp://filesystem/write_file"

ecs_architect:
  role: "ECS & Data Architecture Lead"
  goal: "Design type-safe schemas, validate data contracts"
  backstory: |
    You specialize in Entity Component System design with TypeScript.
    You create minimal, focused schemas that solve ONE problem well.
  llm: "claude-3-5-sonnet-20241022"
  max_iter: 30
  tools:
    - "mcp://filesystem/read_file"
    - "mcp://filesystem/write_file"

qa_validator:
  role: "Quality Assurance Specialist"
  goal: "Validate deliverables, run tests, verify performance"
  backstory: |
    You validate that deliverables meet all acceptance criteria.
    You run unit tests, check compilation, and create validation reports.
  llm: "claude-3-5-sonnet-20241022"
  max_iter: 25
  tools:
    - "mcp://filesystem/read_file"
    - "mcp://git/execute_command"
```

**Agent Configuration Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | `string` | Yes | Agent's role title |
| `goal` | `string` | Yes | What the agent aims to achieve |
| `backstory` | `string` | Yes | Agent's background and expertise |
| `llm` | `string` | No | Model override (e.g., `"claude-sonnet-4-20250514"`) |
| `max_iter` | `int` | No | Maximum iterations per task |
| `max_reasoning_attempts` | `int` | No | Maximum reasoning retries |
| `allow_delegation` | `bool` | No | Whether the agent can delegate to others (default: `false`) |
| `tools` | `list[str]` | No | Tool URIs (MCP protocol format) |

### tasks.yaml

Defines the tasks with descriptions, expected outputs, agent assignments, and dependencies:

```yaml
# packages/my-project/.crew/tasks.yaml

alpha_context_load:
  description: |
    ALPHA TASK: Load complete project context before any work begins.

    Actions:
    1. Read projectbrief.md for project goals and constraints
    2. Query ConPort schema to understand current state
    3. Identify any missing prerequisites or blockers
  expected_output: |
    Markdown report with:
    ## Context Loaded
    - List of files read
    ## Prerequisites
    - [ ] Validated prerequisites checklist
    ## Ready to Proceed
    - Yes/No with reasoning
  agent: technical_director
  human_input: false

validate_schemas:
  description: |
    Validate existing component schemas for TypeScript compilation
    and pattern compliance. DO NOT create new components.
  expected_output: |
    ## Validation Report
    ### TypeScript Compilation: PASS/FAIL
    ### Pattern Compliance: per-component results
  agent: ecs_architect
  context:
    - alpha_context_load    # Depends on alpha task
  async_execution: false

omega_record_progress:
  description: |
    OMEGA TASK: Record all decisions, progress, and learnings.
  expected_output: |
    ## Work Completed
    ## Architectural Decisions
    ## Handoff Notes
  agent: qa_validator
  context:
    - validate_schemas
```

**Task Configuration Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | `string` | Yes | Detailed task instructions (supports `{placeholders}`) |
| `expected_output` | `string` | Yes | Description of expected output format |
| `agent` | `string` | Yes | Name of the agent to execute this task (must match agents.yaml key) |
| `context` | `list[str]` | No | Names of prerequisite tasks whose output feeds into this task |
| `human_input` | `bool` | No | Whether to request human feedback (default: `false`) |
| `async_execution` | `bool` | No | Run asynchronously (default: `false`) |

---

## Python API

### run_crew()

Run a crew from a discovered package.

```python
def run_crew(
    package_name: str,
    crew_name: str,
    inputs: dict | None = None,
    workspace_root: Path | None = None,
) -> str
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `package_name` | `str` | Yes | Package name (e.g., `"otterfall"`) |
| `crew_name` | `str` | Yes | Crew name from manifest (e.g., `"game_builder"`) |
| `inputs` | `dict` | No | Input variables for task templates |
| `workspace_root` | `Path` | No | Workspace root (auto-detected if not provided) |

**Returns:** Crew output as a string.

```python
from agentic_crew.core.runner import run_crew

result = run_crew(
    package_name="otterfall",
    crew_name="game_builder",
    inputs={
        "spec": "Create a BiomeComponent with temperature and humidity",
        "component_spec": "BiomeComponent for procedural terrain generation",
    },
)
print(result)
```

### run_crew_from_path()

Run a crew directly from a configuration directory path, bypassing package discovery.

```python
def run_crew_from_path(
    crewai_dir: Path,
    crew_name: str,
    inputs: dict | None = None,
) -> str
```

```python
from pathlib import Path
from agentic_crew.core.runner import run_crew_from_path

result = run_crew_from_path(
    crewai_dir=Path("packages/otterfall/.crewai"),
    crew_name="game_builder",
    inputs={"spec": "Create a new QuestComponent"},
)
```

---

## Discovery API

### discover_packages()

Discover all packages with crew configuration directories.

```python
def discover_packages(
    workspace_root: Path | None = None,
    framework: str | None = None,
) -> dict[str, Path]
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `workspace_root` | `Path` | Auto-detected | Root directory to search |
| `framework` | `str` | `None` | Filter by framework (`"crewai"`, `"langgraph"`, `"strands"`) |

**Returns:** Dict mapping package name to its config directory path.

```python
from agentic_crew.core.discovery import discover_packages

packages = discover_packages()
# {"otterfall": Path("packages/otterfall/.crewai")}

# Filter by framework
crewai_only = discover_packages(framework="crewai")
```

### discover_all_framework_configs()

Discover all framework-specific configuration directories for all packages (returns all frameworks per package, not just the first match).

```python
def discover_all_framework_configs(
    workspace_root: Path | None = None,
) -> dict[str, dict[str | None, Path]]
```

```python
from agentic_crew.core.discovery import discover_all_framework_configs

configs = discover_all_framework_configs()
# {
#   "otterfall": {
#     "crewai": Path("packages/otterfall/.crewai"),
#     "strands": Path("packages/otterfall/.strands"),
#   }
# }
```

### list_crews()

List all available crews with metadata, optionally filtered by package or framework.

```python
def list_crews(
    package_name: str | None = None,
    framework: str | None = None,
) -> dict[str, list[dict]]
```

**Returns:** Dict mapping package name to list of crew info dicts:

```python
from agentic_crew.core.discovery import list_crews

crews = list_crews()
# {
#   "otterfall": [
#     {
#       "name": "game_builder",
#       "description": "Build game components",
#       "required_framework": "crewai",
#       "preferred_framework": None,
#     }
#   ]
# }
```

### get_crew_config()

Load a specific crew's full configuration from a config directory.

```python
def get_crew_config(config_dir: Path, crew_name: str) -> dict
```

**Returns:** Dict containing:

| Key | Type | Description |
|-----|------|-------------|
| `name` | `str` | Crew name |
| `description` | `str` | Crew description |
| `agents` | `dict` | Parsed agents YAML |
| `tasks` | `dict` | Parsed tasks YAML |
| `knowledge_paths` | `list[Path]` | Resolved knowledge directory paths |
| `manifest` | `dict` | Full manifest data |
| `config_dir` | `Path` | Config directory path |
| `required_framework` | `str \| None` | Framework enforced by directory name |
| `preferred_framework` | `str \| None` | Framework preferred in manifest |
| `llm` | `dict` | LLM configuration |

---

## Manager Agent API

The `ManagerAgent` class provides hierarchical orchestration of multiple crews with delegation, parallel execution, and human-in-the-loop checkpoints.

```python
from agentic_crew.core.manager import ManagerAgent
```

### Constructor

```python
class ManagerAgent:
    def __init__(
        self,
        crews: dict[str, str],
        package_name: str | None = None,
        workspace_root: Path | None = None,
    )
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `crews` | `dict[str, str]` | Mapping of role names to crew names |
| `package_name` | `str` | Package to search in (optional -- auto-discovers if not set) |
| `workspace_root` | `Path` | Workspace root for discovery |

### delegate()

Delegate a task to a specific crew synchronously.

```python
def delegate(
    self,
    crew_role: str,
    inputs: dict[str, Any] | str,
    framework: str | None = None,
) -> str
```

### delegate_async()

Delegate a task asynchronously (runs in a thread pool).

```python
async def delegate_async(
    self,
    crew_role: str,
    inputs: dict[str, Any] | str,
    framework: str | None = None,
) -> str
```

### delegate_parallel()

Delegate tasks to multiple crews in parallel.

```python
async def delegate_parallel(
    self,
    delegations: list[tuple[str, dict[str, Any] | str]],
    framework: str | None = None,
) -> list[str]
```

### delegate_sequential()

Delegate tasks to multiple crews sequentially.

```python
def delegate_sequential(
    self,
    delegations: list[tuple[str, dict[str, Any] | str]],
    framework: str | None = None,
) -> list[str]
```

### checkpoint()

Create a human-in-the-loop checkpoint. The base implementation auto-approves; subclasses can override for interactive workflows.

```python
def checkpoint(
    self,
    message: str,
    result: Any,
    auto_approve: bool = False,
) -> tuple[bool, Any]
```

### execute_workflow()

Main entry point for the manager. Subclasses must override this to define orchestration logic.

```python
async def execute_workflow(self, task: str, **kwargs) -> str
```

### Example: Game Development Manager

```python
import asyncio
from agentic_crew.core.manager import ManagerAgent


class GameDevManager(ManagerAgent):
    """Manager that orchestrates game development crews."""

    def __init__(self):
        super().__init__(
            crews={
                "design": "gameplay_design",
                "implementation": "ecs_implementation",
                "assets": "asset_pipeline",
                "qa": "qa_validation",
            }
        )

    async def execute_workflow(self, task: str) -> str:
        # Phase 1: Design (sequential)
        design_result = await self.delegate_async("design", task)

        # Checkpoint before implementation
        approved, design_result = self.checkpoint(
            "Review the design before proceeding",
            design_result,
        )

        # Phase 2: Implementation + Assets (parallel)
        impl_result, asset_result = await self.delegate_parallel([
            ("implementation", {"spec": design_result}),
            ("assets", {"spec": design_result}),
        ])

        # Phase 3: QA (sequential)
        qa_result = await self.delegate_async("qa", {
            "implementation": impl_result,
            "assets": asset_result,
        })

        return qa_result


# Usage
manager = GameDevManager()
result = asyncio.run(manager.execute_workflow("Create a BiomeComponent"))
```

---

## Runner Interface

### BaseRunner

All framework runners extend `BaseRunner`:

```python
from agentic_crew.runners.base import BaseRunner

class MyRunner(BaseRunner):
    framework_name = "myframework"

    def build_crew(self, crew_config: dict) -> Any:
        """Convert universal config to framework-specific crew."""
        ...

    def run(self, crew: Any, inputs: dict) -> str:
        """Execute the crew and return string output."""
        ...

    def build_agent(self, agent_config: dict, tools=None) -> Any:
        """Create a framework-specific agent."""
        ...

    def build_task(self, task_config: dict, agent: Any) -> Any:
        """Create a framework-specific task."""
        ...
```

**Available runners:**

| Runner | Framework | Import |
|--------|-----------|--------|
| `CrewAIRunner` | CrewAI | `from agentic_crew.runners.crewai_runner import CrewAIRunner` |
| `LangGraphRunner` | LangGraph | `from agentic_crew.runners.langgraph_runner import LangGraphRunner` |
| `StrandsRunner` | Strands | `from agentic_crew.runners.strands_runner import StrandsRunner` |
| `LocalCLIRunner` | CLI tools | `from agentic_crew.runners.local_cli_runner import LocalCLIRunner` |
| `SingleAgentRunner` | Direct LLM | `from agentic_crew.runners.single_agent_runner import SingleAgentRunner` |

---

## CLI Commands

```bash
# List all available packages with crews
agentic-crew list
agentic-crew list --json

# List crews in a specific package
agentic-crew list otterfall

# List crews filtered by framework
agentic-crew list --framework crewai

# Run a multi-agent crew
agentic-crew run otterfall game_builder --input "Create a QuestComponent"
agentic-crew run otterfall game_builder --file tasks.md
agentic-crew run otterfall game_builder --input "..." --json

# Specify framework explicitly
agentic-crew run otterfall game_builder --input "..." --framework crewai

# Run with single-agent CLI runner
agentic-crew run --runner aider --input "Add error handling to auth.py"
agentic-crew run --runner claude-code --input "Refactor the database module"
agentic-crew run --runner ollama --input "Fix the bug" --model deepseek-coder

# Show crew details
agentic-crew info otterfall game_builder
agentic-crew info otterfall game_builder --json

# List available single-agent runners
agentic-crew list-runners
agentic-crew list-runners --json
```

**Exit codes:**

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Crew execution failed |
| `2` | Configuration error (package/crew not found) |

---

## Related Pages

- [Configuration API](/api/configuration/) -- Config schema and loading
- [Fleet API Reference](/api/fleet-management/) -- Agent fleet management
- [Game Generator API](/api/game-gen/) -- Rust game generator
