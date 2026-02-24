"""Tests for the ManagerAgent class."""

from __future__ import annotations

import asyncio
from pathlib import Path
from unittest.mock import patch

import pytest
from agentic_crew.core.manager import ManagerAgent


class TestManagerAgent:
    """Tests for ManagerAgent base class."""

    def test_init_with_crews(self):
        """Test manager initialization with crew mappings."""
        crews = {"design": "game_design", "qa": "quality_assurance"}
        manager = ManagerAgent(crews=crews)

        assert manager.crews == crews
        assert manager.package_name is None
        assert manager.workspace_root is None

    def test_init_with_package_name(self):
        """Test manager initialization with package name."""
        crews = {"design": "game_design"}
        manager = ManagerAgent(crews=crews, package_name="my_package")

        assert manager.package_name == "my_package"

    def test_get_packages_caches_result(self):
        """Test that package discovery is cached."""
        manager = ManagerAgent(crews={"test": "test_crew"})

        with patch("agentic_crew.core.manager.discover_packages") as mock_discover:
            mock_packages = {"pkg1": Path("/path/pkg1")}
            mock_discover.return_value = mock_packages

            # First call
            result1 = manager._get_packages()
            assert result1 == mock_packages
            assert mock_discover.call_count == 1

            # Second call should use cache
            result2 = manager._get_packages()
            assert result2 == mock_packages
            assert mock_discover.call_count == 1  # Not called again

    def test_delegate_with_string_input(self):
        """Test delegation with string input."""
        manager = ManagerAgent(
            crews={"design": "game_design"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}
        mock_config = {
            "name": "game_design",
            "agents": {},
            "tasks": {},
        }

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.return_value = mock_config
            mock_run.return_value = "Design complete"

            result = manager.delegate("design", "Create a game design")

            assert result == "Design complete"
            # Verify string was converted to dict
            mock_run.assert_called_once()
            call_args = mock_run.call_args
            assert call_args[1]["inputs"] == {"task": "Create a game design"}

    def test_delegate_with_dict_input(self):
        """Test delegation with dict input."""
        manager = ManagerAgent(
            crews={"design": "game_design"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}
        mock_config = {"name": "game_design", "agents": {}, "tasks": {}}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.return_value = mock_config
            mock_run.return_value = "Design complete"

            inputs = {"task": "Create design", "theme": "fantasy"}
            result = manager.delegate("design", inputs)

            assert result == "Design complete"
            mock_run.assert_called_once()
            call_args = mock_run.call_args
            assert call_args[1]["inputs"] == inputs

    def test_delegate_unknown_role_raises_error(self):
        """Test that delegating to unknown role raises ValueError."""
        manager = ManagerAgent(crews={"design": "game_design"})

        with pytest.raises(ValueError, match="Unknown crew role 'unknown'"):
            manager.delegate("unknown", "test task")

    def test_delegate_package_not_found_raises_error(self):
        """Test that missing package raises ValueError."""
        manager = ManagerAgent(
            crews={"design": "game_design"},
            package_name="nonexistent",
        )

        with patch("agentic_crew.core.manager.discover_packages") as mock_discover:
            mock_discover.return_value = {"other_pkg": Path("/other")}

            with pytest.raises(ValueError, match="Package 'nonexistent' not found"):
                manager.delegate("design", "test task")

    def test_delegate_auto_discover_crew(self):
        """Test auto-discovery of crew in packages when package_name not specified."""
        manager = ManagerAgent(crews={"design": "game_design"})

        mock_packages = {
            "pkg1": Path("/pkg1/.crewai"),
            "pkg2": Path("/pkg2/.crewai"),
        }
        mock_config = {"name": "game_design", "agents": {}, "tasks": {}}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages

            # First package doesn't have the crew
            def get_config_side_effect(pkg_dir, crew_name):
                if pkg_dir == Path("/pkg1/.crewai"):
                    raise ValueError("Crew not found")
                return mock_config

            mock_get_config.side_effect = get_config_side_effect
            mock_run.return_value = "Success"

            result = manager.delegate("design", "test task")

            assert result == "Success"
            # Should have tried pkg1 (failed), then found in pkg2 (config is cached)
            assert mock_get_config.call_count == 2

    def test_delegate_crew_not_found_raises_error(self):
        """Test that crew not found in any package raises ValueError."""
        manager = ManagerAgent(crews={"design": "nonexistent_crew"})

        mock_packages = {"pkg1": Path("/pkg1/.crewai")}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.side_effect = ValueError("Crew not found")

            with pytest.raises(ValueError, match="Crew 'nonexistent_crew' not found"):
                manager.delegate("design", "test task")

    @pytest.mark.asyncio
    async def test_delegate_async(self):
        """Test async delegation."""
        manager = ManagerAgent(
            crews={"design": "game_design"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}
        mock_config = {"name": "game_design", "agents": {}, "tasks": {}}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.return_value = mock_config
            mock_run.return_value = "Async result"

            result = await manager.delegate_async("design", "test task")

            assert result == "Async result"

    @pytest.mark.asyncio
    async def test_delegate_parallel(self):
        """Test parallel delegation to multiple crews."""
        manager = ManagerAgent(
            crews={"design": "game_design", "assets": "asset_gen"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages

            # Return different configs for different crews to distinguish them
            def get_config_side_effect(pkg_dir, crew_name):
                return {"name": crew_name, "agents": {}, "tasks": {}}

            mock_get_config.side_effect = get_config_side_effect

            # Mock different results based on crew name in config
            def run_side_effect(config, inputs, framework=None):
                crew_name = config["name"]
                if crew_name == "game_design":
                    return "Design done"
                elif crew_name == "asset_gen":
                    return "Assets done"
                return f"Unknown crew: {crew_name}"

            mock_run.side_effect = run_side_effect

            results = await manager.delegate_parallel(
                [
                    ("design", "Create design"),
                    ("assets", "Generate assets"),
                ]
            )

            assert len(results) == 2
            # Verify correct results returned (order matches input order)
            assert "Design done" in results
            assert "Assets done" in results
            # Both should have been executed
            assert mock_run.call_count == 2

    def test_delegate_sequential(self):
        """Test sequential delegation to multiple crews."""
        manager = ManagerAgent(
            crews={"design": "game_design", "impl": "implementation"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}
        mock_config = {"name": "test", "agents": {}, "tasks": {}}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.return_value = mock_config
            mock_run.side_effect = ["Design result", "Implementation result"]

            results = manager.delegate_sequential(
                [
                    ("design", "Create design"),
                    ("impl", "Implement design"),
                ]
            )

            assert results == ["Design result", "Implementation result"]
            assert mock_run.call_count == 2
            # Verify they were called in order
            calls = mock_run.call_args_list
            assert calls[0][1]["inputs"]["task"] == "Create design"
            assert calls[1][1]["inputs"]["task"] == "Implement design"

    def test_checkpoint_auto_approve(self):
        """Test checkpoint with auto_approve=True."""
        manager = ManagerAgent(crews={"test": "test_crew"})

        approved, result = manager.checkpoint(
            "Review design",
            "Design output",
            auto_approve=True,
        )

        assert approved is True
        assert result == "Design output"

    def test_checkpoint_base_implementation_auto_approves(self):
        """Test that base checkpoint implementation auto-approves."""
        manager = ManagerAgent(crews={"test": "test_crew"})

        approved, result = manager.checkpoint(
            "Review design",
            "Design output",
            auto_approve=False,  # Even with False, base impl approves
        )

        assert approved is True
        assert result == "Design output"

    def test_execute_workflow_not_implemented(self):
        """Test that execute_workflow raises NotImplementedError in base class."""
        manager = ManagerAgent(crews={"test": "test_crew"})

        with pytest.raises(NotImplementedError, match="Subclasses must implement"):
            asyncio.run(manager.execute_workflow("test task"))


class TestManagerAgentSubclass:
    """Tests for ManagerAgent subclass implementation."""

    @pytest.mark.asyncio
    async def test_custom_workflow_implementation(self):
        """Test a custom manager implementation."""

        class TestManager(ManagerAgent):
            async def execute_workflow(self, task: str, **kwargs):
                # Simple sequential workflow
                design = await self.delegate_async("design", task)
                impl = await self.delegate_async("impl", design)
                return impl

        manager = TestManager(
            crews={"design": "game_design", "impl": "implementation"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}
        mock_config = {"name": "test", "agents": {}, "tasks": {}}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.return_value = mock_config
            mock_run.side_effect = ["Design done", "Implementation done"]

            result = await manager.execute_workflow("Build a game")

            assert result == "Implementation done"
            assert mock_run.call_count == 2

    @pytest.mark.asyncio
    async def test_custom_workflow_with_parallel_execution(self):
        """Test a custom manager with parallel execution."""

        class ParallelManager(ManagerAgent):
            async def execute_workflow(self, task: str, **kwargs):
                # Parallel execution
                results = await self.delegate_parallel(
                    [
                        ("design", task),
                        ("assets", task),
                    ]
                )
                # Then sequential QA
                qa_result = await self.delegate_async(
                    "qa",
                    {
                        "design": results[0],
                        "assets": results[1],
                    },
                )
                return qa_result

        manager = ParallelManager(
            crews={"design": "design", "assets": "assets", "qa": "qa"},
            package_name="test_pkg",
        )

        mock_packages = {"test_pkg": Path("/test/.crewai")}
        mock_config = {"name": "test", "agents": {}, "tasks": {}}

        with (
            patch("agentic_crew.core.manager.discover_packages") as mock_discover,
            patch("agentic_crew.core.manager.get_crew_config") as mock_get_config,
            patch("agentic_crew.core.manager.run_crew_auto") as mock_run,
        ):
            mock_discover.return_value = mock_packages
            mock_get_config.return_value = mock_config
            mock_run.side_effect = ["Design done", "Assets done", "QA passed"]

            result = await manager.execute_workflow("Build game")

            assert result == "QA passed"
            assert mock_run.call_count == 3
