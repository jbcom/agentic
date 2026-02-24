"""Framework decomposition - auto-detect and select AI framework.

This module provides the core capability of agentic-crew: declaring crews
once and running them on CrewAI, LangGraph, or Strands depending on what's
installed. It also supports single-agent CLI runners for simpler tasks.

Usage:
    from agentic_crew.core.decomposer import get_runner, detect_framework

    # Auto-detect best framework for multi-agent
    framework = detect_framework()

    # Get runner for that framework
    runner = get_runner(framework)

    # Or let it auto-select
    runner = get_runner()  # Uses best available

    # Get single-agent CLI runner
    from agentic_crew.core.decomposer import get_cli_runner
    runner = get_cli_runner("aider")
"""

from __future__ import annotations

import importlib
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from agentic_crew.runners.base import BaseRunner
    from agentic_crew.runners.single_agent_runner import SingleAgentRunner

# Framework detection cache
_framework_cache: dict[str, bool] = {}

# Framework priority (first available wins)
FRAMEWORK_PRIORITY = ["crewai", "langgraph", "strands"]


def is_framework_available(framework: str) -> bool:
    """Check if a framework is installed and importable.

    Args:
        framework: Framework name (crewai, langgraph, strands)

    Returns:
        True if framework is available
    """
    if framework in _framework_cache:
        return _framework_cache[framework]

    # Check if framework is in our supported list
    if framework not in FRAMEWORK_PRIORITY:
        _framework_cache[framework] = False
        return False

    try:
        importlib.import_module(framework)
        _framework_cache[framework] = True
        return True
    except ImportError:
        _framework_cache[framework] = False
        return False


def detect_framework(preferred: str | None = None) -> str:
    """Detect the best available AI framework.

    Args:
        preferred: Optional preferred framework. If available, use it.

    Returns:
        Name of the best available framework.

    Raises:
        RuntimeError: If no frameworks are installed.
    """
    # Check preferred first
    if preferred and preferred != "auto" and is_framework_available(preferred):
        return preferred
        # Fall through to auto-detect if preferred not available

    # Auto-detect based on priority
    for framework in FRAMEWORK_PRIORITY:
        if is_framework_available(framework):
            return framework

    raise RuntimeError(
        "No AI frameworks installed. Install one of:\n"
        "  pip install crewai[tools]\n"
        "  pip install langgraph\n"
        "  pip install strands-agents"
    )


def get_available_frameworks() -> list[str]:
    """Get list of all available frameworks.

    Returns:
        List of framework names that are installed.
    """
    return [f for f in FRAMEWORK_PRIORITY if is_framework_available(f)]


def get_runner(framework: str | None = None) -> BaseRunner:
    """Get a runner for the specified or auto-detected framework.

    Args:
        framework: Framework name or None for auto-detect.

    Returns:
        Runner instance for the framework.

    Raises:
        RuntimeError: If framework not available.
        ValueError: If unknown framework specified.
    """
    if framework is None or framework == "auto":
        framework = detect_framework()

    if framework == "crewai":
        from agentic_crew.runners.crewai_runner import CrewAIRunner

        return CrewAIRunner()

    if framework == "langgraph":
        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        return LangGraphRunner()

    if framework == "strands":
        from agentic_crew.runners.strands_runner import StrandsRunner

        return StrandsRunner()

    raise ValueError(f"Unknown framework: {framework}. Options: {FRAMEWORK_PRIORITY}")


def get_cli_runner(
    profile: str | dict[str, Any],
    model: str | None = None,
) -> SingleAgentRunner:
    """Get a single-agent CLI runner for the specified profile.

    Args:
        profile: Profile name (e.g., "aider", "claude-code", "ollama") or
                custom config dict.
        model: Optional model override.

    Returns:
        LocalCLIRunner instance for the profile.

    Raises:
        ValueError: If profile not found.
        FileNotFoundError: If profiles file missing.

    Examples:
        # Use built-in profile
        runner = get_cli_runner("aider")
        result = runner.run("Add error handling to auth.py")

        # Use with model override
        runner = get_cli_runner("ollama", model="deepseek-coder")
        result = runner.run("Fix the bug")

        # Use custom config
        runner = get_cli_runner({
            "command": "my-tool",
            "task_flag": "--task",
            "auto_approve": "--yes",
        })
    """
    from agentic_crew.runners.local_cli_runner import LocalCLIRunner

    return LocalCLIRunner(profile, model=model)


def get_available_cli_runners() -> list[str]:
    """Get list of available CLI runner profiles.

    Returns:
        List of profile names (e.g., ["aider", "claude-code", "ollama"]).
    """
    from agentic_crew.runners.local_cli_runner import LocalCLIRunner

    return LocalCLIRunner.get_available_profiles()


def is_cli_runner_available(profile: str) -> bool:
    """Check if a CLI runner profile is available (tool installed).

    Args:
        profile: Profile name to check.

    Returns:
        True if the tool is installed and accessible.
    """
    try:
        runner = get_cli_runner(profile)
        return runner.is_available()
    except (ValueError, FileNotFoundError):
        return False


def decompose_crew(
    crew_config: dict[str, Any],
    framework: str | None = None,
) -> Any:
    """Decompose a crew configuration to a framework-specific crew.

    This is the core function that converts a framework-agnostic crew
    definition into a runnable crew for the target framework.

    Args:
        crew_config: Crew configuration from loader.
        framework: Target framework or None for auto-detect.
                   If crew_config has required_framework, that takes precedence.

    Returns:
        Framework-specific crew object ready to run.

    Raises:
        RuntimeError: If required framework is not available.
    """
    # Check if crew config requires a specific framework
    required_framework = crew_config.get("required_framework")

    if required_framework:
        # Framework is enforced by config directory (.crewai, .strands, etc.)
        if framework and framework != required_framework and framework != "auto":
            raise ValueError(
                f"Crew requires {required_framework} (defined in .{required_framework}/ directory) "
                f"but {framework} was requested"
            )

        if not is_framework_available(required_framework):
            raise RuntimeError(
                f"Crew requires {required_framework} but it's not installed. "
                f"Install with: pip install {_get_install_command(required_framework)}"
            )
        framework = required_framework

    runner = get_runner(framework)
    return runner.build_crew(crew_config)


def _get_install_command(framework: str) -> str:
    """Get the pip install command for a framework."""
    install_commands = {
        "crewai": "crewai[tools]",
        "langgraph": "langgraph langchain-anthropic",
        "strands": "strands-agents",
    }
    return install_commands.get(framework, framework)


# Convenience function for simple use cases
def run_crew_auto(
    crew_config: dict[str, Any],
    inputs: dict[str, Any] | None = None,
    framework: str | None = None,
) -> str:
    """Run a crew using the best available framework.

    Args:
        crew_config: Crew configuration from loader.
        inputs: Optional inputs for the crew.
        framework: Optional framework override. If crew_config has
                   required_framework (from .crewai/.strands/.langgraph dir),
                   that takes precedence.

    Returns:
        Crew output as string.

    Raises:
        RuntimeError: If required framework is not available.
        ValueError: If requested framework conflicts with required framework.
    """
    # Check if crew config requires a specific framework
    required_framework = crew_config.get("required_framework")

    if required_framework:
        if framework and framework != required_framework and framework != "auto":
            raise ValueError(
                f"Crew requires {required_framework} (defined in .{required_framework}/ directory) "
                f"but {framework} was requested"
            )

        if not is_framework_available(required_framework):
            raise RuntimeError(
                f"Crew requires {required_framework} but it's not installed. "
                f"Install with: pip install {_get_install_command(required_framework)}"
            )
        framework = required_framework

    runner = get_runner(framework)
    crew = runner.build_crew(crew_config)
    return runner.run(crew, inputs or {})
