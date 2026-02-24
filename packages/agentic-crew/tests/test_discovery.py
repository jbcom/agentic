"""Tests for the discovery module."""

from __future__ import annotations

from pathlib import Path
from unittest.mock import patch

import pytest


class TestDiscovery:
    """Tests for package discovery functionality."""

    def test_discover_packages_finds_crewai_directories(self, temp_workspace: Path) -> None:
        """Test that discover_packages finds packages with .crewai directories."""
        from agentic_crew.core.discovery import discover_packages

        packages = discover_packages(workspace_root=temp_workspace)

        assert "otterfall" in packages
        assert packages["otterfall"].exists()

    def test_discover_packages_finds_crew_directories(self, tmp_path: Path) -> None:
        """Test that discover_packages finds framework-agnostic .crew directories."""
        from agentic_crew.core.discovery import discover_packages

        # Create packages with .crew directory
        pkg_dir = tmp_path / "packages" / "strata"
        crew_dir = pkg_dir / ".crew"
        crew_dir.mkdir(parents=True)
        (crew_dir / "manifest.yaml").write_text("name: strata\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path)

        assert "strata" in packages
        assert packages["strata"].name == ".crew"

    def test_discover_packages_prefers_crew_over_crewai(self, tmp_path: Path) -> None:
        """Test that .crew takes priority over .crewai when both exist."""
        from agentic_crew.core.discovery import discover_packages

        # Create package with both .crew and .crewai
        pkg_dir = tmp_path / "packages" / "hybrid"
        (pkg_dir / ".crew").mkdir(parents=True)
        (pkg_dir / ".crew" / "manifest.yaml").write_text("name: hybrid\ncrews: {}")
        (pkg_dir / ".crewai").mkdir(parents=True)
        (pkg_dir / ".crewai" / "manifest.yaml").write_text("name: hybrid\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path)

        assert "hybrid" in packages
        # .crew should be preferred (framework-agnostic first)
        assert packages["hybrid"].name == ".crew"

    def test_discover_packages_returns_empty_when_no_packages(self, tmp_path: Path) -> None:
        """Test that discover_packages returns empty dict when no config dirs exist."""
        from agentic_crew.core.discovery import discover_packages

        # Create empty packages directory
        packages_dir = tmp_path / "packages"
        packages_dir.mkdir()
        (packages_dir / "some_package").mkdir()

        packages = discover_packages(workspace_root=tmp_path)

        assert packages == {}

    def test_discover_all_framework_configs(self, tmp_path: Path) -> None:
        """Test discovering all framework configs for a package."""
        from agentic_crew.core.discovery import discover_all_framework_configs

        # Create package with multiple framework configs
        pkg_dir = tmp_path / "packages" / "multi"
        (pkg_dir / ".crew").mkdir(parents=True)
        (pkg_dir / ".crew" / "manifest.yaml").write_text("name: multi\ncrews: {}")
        (pkg_dir / ".crewai").mkdir(parents=True)
        (pkg_dir / ".crewai" / "manifest.yaml").write_text("name: multi\ncrews: {}")

        configs = discover_all_framework_configs(workspace_root=tmp_path)

        assert "multi" in configs
        assert None in configs["multi"]  # .crew -> None (agnostic)
        assert "crewai" in configs["multi"]

    def test_list_crews_returns_crews_from_manifest(self, temp_workspace: Path) -> None:
        """Test that list_crews returns crew definitions from manifest."""
        from agentic_crew.core.discovery import list_crews

        with patch(
            "agentic_crew.core.discovery.discover_packages",
            return_value={"otterfall": temp_workspace / "packages" / "otterfall" / ".crewai"},
        ):
            crews_by_package = list_crews()

        assert "otterfall" in crews_by_package
        crews = crews_by_package["otterfall"]
        assert len(crews) == 1
        assert crews[0]["name"] == "test_crew"

    def test_list_crews_filters_by_package_name(self, temp_workspace: Path) -> None:
        """Test that list_crews can filter to a specific package."""
        from agentic_crew.core.discovery import list_crews

        with patch(
            "agentic_crew.core.discovery.discover_packages",
            return_value={"otterfall": temp_workspace / "packages" / "otterfall" / ".crewai"},
        ):
            crews_by_package = list_crews(package_name="otterfall")

        assert "otterfall" in crews_by_package
        assert len(crews_by_package) == 1

    def test_list_crews_returns_empty_for_nonexistent_package(self, temp_workspace: Path) -> None:
        """Test that list_crews returns empty for non-existent package."""
        from agentic_crew.core.discovery import list_crews

        with patch(
            "agentic_crew.core.discovery.discover_packages",
            return_value={"otterfall": temp_workspace / "packages" / "otterfall" / ".crewai"},
        ):
            crews_by_package = list_crews(package_name="nonexistent")

        assert crews_by_package == {}

    def test_load_manifest_parses_yaml(self, temp_workspace: Path) -> None:
        """Test that load_manifest parses YAML correctly."""
        from agentic_crew.core.discovery import load_manifest

        crewai_dir = temp_workspace / "packages" / "otterfall" / ".crewai"
        manifest = load_manifest(crewai_dir)

        assert manifest is not None
        assert manifest.get("name") == "otterfall"
        assert "crews" in manifest

    def test_get_workspace_root_finds_root(self) -> None:
        """Test that get_workspace_root finds the workspace root."""
        from agentic_crew.core.discovery import get_workspace_root

        # This should find the actual workspace root
        root = get_workspace_root()

        # Verify it looks like a workspace root
        assert (root / "packages").exists() or root == Path.cwd()

    def test_get_framework_from_config_dir(self) -> None:
        """Test framework detection from directory name."""
        from agentic_crew.core.discovery import get_framework_from_config_dir

        assert get_framework_from_config_dir(Path("/some/path/.crew")) is None
        assert get_framework_from_config_dir(Path("/some/path/.crewai")) == "crewai"
        assert get_framework_from_config_dir(Path("/some/path/.langgraph")) == "langgraph"
        assert get_framework_from_config_dir(Path("/some/path/.strands")) == "strands"

    def test_get_crew_config_includes_required_framework(self, temp_workspace: Path) -> None:
        """Test that get_crew_config includes required_framework field."""
        from agentic_crew.core.discovery import get_crew_config

        crewai_dir = temp_workspace / "packages" / "otterfall" / ".crewai"
        config = get_crew_config(crewai_dir, "test_crew")

        assert config["required_framework"] == "crewai"


class TestDecomposer:
    """Tests for the decomposer module."""

    def test_is_framework_available_caches_results(self) -> None:
        """Test that framework availability is cached."""
        from agentic_crew.core.decomposer import _framework_cache, is_framework_available

        # Clear cache first
        _framework_cache.clear()

        # Check availability (will cache result)
        result1 = is_framework_available("nonexistent_framework")
        result2 = is_framework_available("nonexistent_framework")

        assert result1 is False
        assert result2 is False
        assert "nonexistent_framework" in _framework_cache

    def test_detect_framework_raises_when_none_available(self) -> None:
        """Test that detect_framework raises when no frameworks are available."""
        from agentic_crew.core.decomposer import detect_framework

        with (
            patch("agentic_crew.core.decomposer.is_framework_available", return_value=False),
            pytest.raises(RuntimeError, match="No AI frameworks installed"),
        ):
            detect_framework()

    def test_detect_framework_respects_priority(self) -> None:
        """Test that frameworks are detected in priority order."""
        from agentic_crew.core.decomposer import detect_framework

        def mock_available(framework):
            return framework in ["langgraph", "strands"]

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = detect_framework()

        # langgraph should be preferred over strands
        assert result == "langgraph"

    def test_detect_framework_with_preferred_returns_preferred(self) -> None:
        """Test that detect_framework returns preferred if available."""
        from agentic_crew.core.decomposer import detect_framework

        def mock_available(framework):
            return framework in ["crewai", "langgraph", "strands"]

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = detect_framework(preferred="strands")

        assert result == "strands"

    def test_detect_framework_falls_back_when_preferred_unavailable(self) -> None:
        """Test that detect_framework falls back when preferred not available."""
        from agentic_crew.core.decomposer import detect_framework

        def mock_available(framework):
            return framework in ["langgraph"]

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = detect_framework(preferred="crewai")

        assert result == "langgraph"

    def test_get_available_frameworks_returns_list(self) -> None:
        """Test that get_available_frameworks returns installed frameworks."""
        from agentic_crew.core.decomposer import get_available_frameworks

        def mock_available(framework):
            return framework in ["crewai", "strands"]

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = get_available_frameworks()

        assert isinstance(result, list)
        assert "crewai" in result
        assert "strands" in result
        assert "langgraph" not in result

    def test_get_available_frameworks_returns_empty_when_none_installed(self) -> None:
        """Test get_available_frameworks returns empty list when none installed."""
        from agentic_crew.core.decomposer import get_available_frameworks

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            return_value=False,
        ):
            result = get_available_frameworks()

        assert result == []

    def test_get_runner_returns_crewai_runner(self) -> None:
        """Test that get_runner returns CrewAIRunner for crewai."""
        from unittest.mock import MagicMock

        from agentic_crew.core.decomposer import get_runner

        mock_runner = MagicMock()
        mock_runner.framework_name = "crewai"

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                return_value=True,
            ),
            patch(
                "agentic_crew.runners.crewai_runner.CrewAIRunner",
                return_value=mock_runner,
            ),
        ):
            runner = get_runner("crewai")

        assert runner is not None
        assert runner.framework_name == "crewai"

    def test_get_runner_returns_langgraph_runner(self) -> None:
        """Test that get_runner returns LangGraphRunner for langgraph."""
        from unittest.mock import MagicMock

        from agentic_crew.core.decomposer import get_runner

        mock_runner = MagicMock()
        mock_runner.framework_name = "langgraph"

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                return_value=True,
            ),
            patch(
                "agentic_crew.runners.langgraph_runner.LangGraphRunner",
                return_value=mock_runner,
            ),
        ):
            runner = get_runner("langgraph")

        assert runner is not None
        assert runner.framework_name == "langgraph"

    def test_get_runner_returns_strands_runner(self) -> None:
        """Test that get_runner returns StrandsRunner for strands."""
        from unittest.mock import MagicMock

        from agentic_crew.core.decomposer import get_runner

        mock_runner = MagicMock()
        mock_runner.framework_name = "strands"

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                return_value=True,
            ),
            patch(
                "agentic_crew.runners.strands_runner.StrandsRunner",
                return_value=mock_runner,
            ),
        ):
            runner = get_runner("strands")

        assert runner is not None
        assert runner.framework_name == "strands"

    def test_get_runner_auto_detects_framework(self) -> None:
        """Test that get_runner with no args auto-detects framework."""
        from unittest.mock import MagicMock

        from agentic_crew.core.decomposer import get_runner

        mock_runner = MagicMock()
        mock_runner.framework_name = "strands"

        def mock_available(framework):
            return framework == "strands"

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                side_effect=mock_available,
            ),
            patch(
                "agentic_crew.runners.strands_runner.StrandsRunner",
                return_value=mock_runner,
            ),
        ):
            runner = get_runner()

        assert runner.framework_name == "strands"

    def test_get_runner_raises_for_unknown_framework(self) -> None:
        """Test that get_runner raises for unknown framework."""
        from agentic_crew.core.decomposer import get_runner

        with pytest.raises(ValueError, match="Unknown framework"):
            get_runner("unknown_framework")

    def test_decompose_crew_uses_required_framework(self, tmp_path: Path) -> None:
        """Test that decompose_crew respects required_framework from config."""
        from unittest.mock import MagicMock

        from agentic_crew.core.decomposer import decompose_crew

        crew_config = {
            "name": "test_crew",
            "required_framework": "strands",
            "agents": {},
            "tasks": {},
        }

        mock_runner = MagicMock()
        mock_runner.framework_name = "strands"
        mock_runner.build_crew.return_value = MagicMock()

        def mock_available(framework):
            return framework in ["crewai", "strands"]

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                side_effect=mock_available,
            ),
            patch(
                "agentic_crew.runners.strands_runner.StrandsRunner",
                return_value=mock_runner,
            ),
        ):
            # decompose_crew returns the result of runner.build_crew()
            decompose_crew(crew_config)

        # Verify the runner's build_crew was called
        mock_runner.build_crew.assert_called_once_with(crew_config)

    def test_decompose_crew_raises_when_required_unavailable(self) -> None:
        """Test decompose_crew raises when required framework not available."""
        from agentic_crew.core.decomposer import decompose_crew

        crew_config = {
            "name": "test_crew",
            "required_framework": "langgraph",
            "agents": {},
            "tasks": {},
        }

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                return_value=False,
            ),
            pytest.raises(RuntimeError, match="requires langgraph.*not installed"),
        ):
            decompose_crew(crew_config)

    def test_decompose_crew_validates_framework_conflict(self) -> None:
        """Test decompose_crew validates requested vs required conflict."""
        from agentic_crew.core.decomposer import decompose_crew

        crew_config = {
            "name": "test_crew",
            "required_framework": "crewai",
            "agents": {},
            "tasks": {},
        }

        with (
            patch(
                "agentic_crew.core.decomposer.is_framework_available",
                return_value=True,
            ),
            pytest.raises(ValueError, match="requires crewai.*langgraph was requested"),
        ):
            decompose_crew(crew_config, framework="langgraph")

    def test_get_install_command_returns_pip_install(self) -> None:
        """Test that _get_install_command returns correct pip command."""
        from agentic_crew.core.decomposer import _get_install_command

        result = _get_install_command("crewai")
        assert "crewai" in result

    def test_get_install_command_maps_langgraph(self) -> None:
        """Test that _get_install_command maps langgraph correctly."""
        from agentic_crew.core.decomposer import _get_install_command

        result = _get_install_command("langgraph")
        assert "langgraph" in result
