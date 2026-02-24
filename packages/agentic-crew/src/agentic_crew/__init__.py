"""agentic-crew: Framework-agnostic AI crew orchestration.

Declare crews once, run on CrewAI, LangGraph, or Strands.

Usage:
    from agentic_crew.core.decomposer import run_crew_auto, get_runner, detect_framework
    from agentic_crew.core.discovery import discover_packages, get_crew_config
    from agentic_crew.core.manager import ManagerAgent

    # Auto-detect framework and run a crew
    packages = discover_packages()
    config = get_crew_config(packages["my-package"], "my_crew")
    result = run_crew_auto(config, inputs={"task": "..."})

    # Or get a specific runner
    runner = get_runner("crewai")  # or "langgraph", "strands"
    crew = runner.build_crew(config)
    result = runner.run(crew, inputs)

    # Or use a hierarchical manager agent
    class MyManager(ManagerAgent):
        def __init__(self):
            super().__init__(crews={
                "design": "design_crew",
                "implementation": "impl_crew"
            })

        async def execute_workflow(self, task):
            design = await self.delegate_async("design", task)
            return await self.delegate_async("implementation", design)
"""

from __future__ import annotations

__version__ = "1.0.0"

# Core exports - framework-agnostic functionality
from agentic_crew.core.decomposer import (
    decompose_crew,
    detect_framework,
    get_available_frameworks,
    get_runner,
    is_framework_available,
    run_crew_auto,
)
from agentic_crew.core.discovery import (
    discover_all_framework_configs,
    discover_packages,
    get_crew_config,
    list_crews,
)
from agentic_crew.core.manager import ManagerAgent

__all__ = [
    # Version
    "__version__",
    # Decomposer - framework detection and selection
    "detect_framework",
    "get_available_frameworks",
    "is_framework_available",
    "get_runner",
    "decompose_crew",
    "run_crew_auto",
    # Discovery - find and load crew configs
    "discover_packages",
    "discover_all_framework_configs",
    "get_crew_config",
    "list_crews",
    # Manager - hierarchical orchestration
    "ManagerAgent",
]
