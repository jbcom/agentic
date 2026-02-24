"""Hierarchical Manager Agent for crew orchestration.

This module provides a manager agent that can orchestrate multiple crews,
enabling complex workflows with smart delegation, sequential and parallel
execution, and human-in-the-loop checkpoints.

Example:
    ```python
    from agentic_crew.core.manager import ManagerAgent

    class GameDevManager(ManagerAgent):
        '''Manager that orchestrates game development crews.'''

        def __init__(self):
            super().__init__(
                crews={
                    "design": "gameplay_design",
                    "implementation": "ecs_implementation",
                    "assets": "asset_pipeline",
                    "qa": "qa_validation",
                }
            )

        async def execute_workflow(self, task: str):
            # Sequential execution
            design_result = await self.delegate_async("design", task)

            # Parallel execution
            impl_result, asset_result = await asyncio.gather(
                self.delegate_async("implementation", design_result),
                self.delegate_async("assets", design_result)
            )

            # Final QA
            return await self.delegate_async("qa", {
                "implementation": impl_result,
                "assets": asset_result,
            })
    ```
"""

from __future__ import annotations

import asyncio
import logging
from pathlib import Path
from typing import Any

from agentic_crew.core.decomposer import run_crew_auto
from agentic_crew.core.discovery import discover_packages, get_crew_config

logger = logging.getLogger(__name__)


class ManagerAgent:
    """Base class for hierarchical manager agents.

    A manager agent orchestrates multiple specialized crews to accomplish
    complex tasks that require coordination between different domains or phases.

    Attributes:
        crews: Dict mapping crew role names to crew names in packages.
        package_name: Optional package name if all crews are in one package.
        workspace_root: Optional workspace root for crew discovery.
    """

    def __init__(
        self,
        crews: dict[str, str],
        package_name: str | None = None,
        workspace_root: Path | None = None,
    ):
        """Initialize the manager agent.

        Args:
            crews: Dict mapping role names to crew names (e.g., {"design": "game_design"}).
            package_name: Optional package name if all crews are in the same package.
            workspace_root: Optional workspace root for discovering packages.
        """
        self.crews = crews
        self.package_name = package_name
        self.workspace_root = workspace_root
        self._packages_cache: dict[str, Path] | None = None
        self._crew_config_cache: dict[str, dict[str, Any]] = {}

    def _get_packages(self) -> dict[str, Path]:
        """Get discovered packages, using cache if available."""
        if self._packages_cache is None:
            self._packages_cache = discover_packages(self.workspace_root)
        return self._packages_cache

    def delegate(
        self,
        crew_role: str,
        inputs: dict[str, Any] | str,
        framework: str | None = None,
    ) -> str:
        """Delegate a task to a specific crew synchronously.

        Args:
            crew_role: Role name from the crews dict (e.g., "design").
            inputs: Input dict or string to pass to the crew.
            framework: Optional framework override.

        Returns:
            Crew output as a string.

        Raises:
            ValueError: If crew_role not found in crews mapping.
        """
        if crew_role not in self.crews:
            raise ValueError(f"Unknown crew role '{crew_role}'. Available: {list(self.crews.keys())}")

        crew_name = self.crews[crew_role]

        # Convert string inputs to dict (create new dict to avoid mutation)
        if isinstance(inputs, str):
            inputs = {"task": inputs}

        # Check crew config cache first for performance
        if crew_name in self._crew_config_cache:
            cached_config = self._crew_config_cache[crew_name]
            return run_crew_auto(cached_config, inputs=inputs, framework=framework)

        # Get crew configuration
        crew_config: dict[str, Any]
        if self.package_name:
            # Use specified package
            packages = self._get_packages()
            if self.package_name not in packages:
                raise ValueError(f"Package '{self.package_name}' not found. Available: {list(packages.keys())}")
            crewai_dir = packages[self.package_name]
            crew_config = get_crew_config(crewai_dir, crew_name)
        else:
            # Auto-discover package containing the crew
            packages = self._get_packages()
            found_config: dict[str, Any] | None = None
            for pkg_dir in packages.values():
                try:
                    found_config = get_crew_config(pkg_dir, crew_name)
                    break
                except ValueError:
                    continue

            if found_config is None:
                raise ValueError(
                    f"Crew '{crew_name}' not found in any package. Available packages: {list(packages.keys())}"
                )
            crew_config = found_config

        # Cache the config for future calls
        self._crew_config_cache[crew_name] = crew_config
        return run_crew_auto(crew_config, inputs=inputs, framework=framework)

    async def delegate_async(
        self,
        crew_role: str,
        inputs: dict[str, Any] | str,
        framework: str | None = None,
    ) -> str:
        """Delegate a task to a specific crew asynchronously.

        This runs the crew in a thread pool to avoid blocking the event loop.

        Args:
            crew_role: Role name from the crews dict (e.g., "design").
            inputs: Input dict or string to pass to the crew.
            framework: Optional framework override.

        Returns:
            Crew output as a string.
        """
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(None, self.delegate, crew_role, inputs, framework)

    async def delegate_parallel(
        self,
        delegations: list[tuple[str, dict[str, Any] | str]],
        framework: str | None = None,
    ) -> list[str]:
        """Delegate tasks to multiple crews in parallel.

        Args:
            delegations: List of (crew_role, inputs) tuples.
            framework: Optional framework override for all crews.

        Returns:
            List of crew outputs in the same order as delegations.

        Example:
            ```python
            results = await manager.delegate_parallel([
                ("design", "Create game concept"),
                ("assets", "Generate placeholder assets"),
            ])
            design_result, assets_result = results
            ```
        """
        tasks = [self.delegate_async(crew_role, inputs, framework) for crew_role, inputs in delegations]
        return await asyncio.gather(*tasks)

    def delegate_sequential(
        self,
        delegations: list[tuple[str, dict[str, Any] | str]],
        framework: str | None = None,
    ) -> list[str]:
        """Delegate tasks to multiple crews sequentially.

        Args:
            delegations: List of (crew_role, inputs) tuples.
            framework: Optional framework override for all crews.

        Returns:
            List of crew outputs in the same order as delegations.

        Example:
            ```python
            results = manager.delegate_sequential([
                ("design", "Create game concept"),
                ("implementation", "Implement the design"),
                ("qa", "Test the implementation"),
            ])
            ```
        """
        results = []
        for crew_role, inputs in delegations:
            result = self.delegate(crew_role, inputs, framework)
            results.append(result)
        return results

    def checkpoint(
        self,
        message: str,
        result: Any,
        auto_approve: bool = False,
    ) -> tuple[bool, Any]:
        """Create a human-in-the-loop checkpoint.

        Args:
            message: Message to display to the human reviewer.
            result: Current result to review.
            auto_approve: If True, automatically approve without waiting.

        Returns:
            Tuple of (approved, result). If not approved, result may be modified.

        Note:
            In this base implementation, checkpoints are logged but auto-approved.
            Subclasses can override to implement actual HITL workflows.
        """
        logger.info("=" * 60)
        logger.info("CHECKPOINT: %s", message)
        logger.info("=" * 60)
        # Log result type only to avoid exposing sensitive data (CWE-532)
        logger.info("Result type: %s", type(result).__name__)
        logger.info("=" * 60)

        if auto_approve:
            logger.info("Auto-approved (auto_approve=True)")
            return True, result

        # Base implementation auto-approves
        logger.info("Auto-approved (base implementation)")
        return True, result

    async def execute_workflow(self, task: str, **kwargs: Any) -> str:
        """Execute the manager's workflow.

        This is the main entry point for the manager agent. Subclasses should
        override this method to implement their specific orchestration logic.

        Args:
            task: The main task to accomplish.
            **kwargs: Additional keyword arguments.

        Returns:
            Final result as a string.

        Raises:
            NotImplementedError: If not overridden by subclass.
        """
        raise NotImplementedError("Subclasses must implement execute_workflow() to define their orchestration logic")
