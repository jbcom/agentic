"""Tests for pytest-agentic-crew plugin fixture registration and functionality."""

from __future__ import annotations

from pathlib import Path
from typing import Any


class TestPluginRegistration:
    """Verify the plugin registers correctly via entry points."""

    def test_plugin_loads(self, pytestconfig) -> None:
        """The agentic_crew plugin should be registered."""
        plugin_manager = pytestconfig.pluginmanager
        # The plugin module should be loaded (registered via pytest11 entry point)
        plugin = plugin_manager.get_plugin("agentic_crew")
        assert plugin is not None

    def test_crew_mocker_fixture_available(self, request) -> None:
        """crew_mocker fixture should be registered and resolvable."""
        # Check fixture is available in the fixture manager
        fixturedefs = request.session._fixturemanager.getfixturedefs("crew_mocker", request.node)
        assert fixturedefs is not None
        assert len(fixturedefs) > 0

    def test_mock_frameworks_fixture_available(self, request) -> None:
        """mock_frameworks fixture should be available."""
        fixturedefs = request.session._fixturemanager.getfixturedefs("mock_frameworks", request.node)
        assert fixturedefs is not None

    def test_mock_crewai_fixture_available(self, request) -> None:
        """mock_crewai fixture should be available."""
        fixturedefs = request.session._fixturemanager.getfixturedefs("mock_crewai", request.node)
        assert fixturedefs is not None

    def test_mock_langgraph_fixture_available(self, request) -> None:
        """mock_langgraph fixture should be available."""
        fixturedefs = request.session._fixturemanager.getfixturedefs("mock_langgraph", request.node)
        assert fixturedefs is not None

    def test_mock_strands_fixture_available(self, request) -> None:
        """mock_strands fixture should be available."""
        fixturedefs = request.session._fixturemanager.getfixturedefs("mock_strands", request.node)
        assert fixturedefs is not None


class TestConfigFixtures:
    """Test the configuration fixtures provided by the plugin."""

    def test_check_api_key_fixture_available(self, request) -> None:
        """check_api_key fixture should be registered."""
        fixturedefs = request.session._fixturemanager.getfixturedefs("check_api_key", request.node)
        assert fixturedefs is not None

    def test_temp_crew_dir_fixture(self, temp_crew_dir: Path) -> None:
        """temp_crew_dir should create a valid directory structure."""
        assert temp_crew_dir.exists()
        assert temp_crew_dir.name == ".crewai"
        assert (temp_crew_dir / "crews").exists()

    def test_simple_agent_config_fixture(self, simple_agent_config: dict[str, Any]) -> None:
        """simple_agent_config should have required agent fields."""
        assert "role" in simple_agent_config
        assert "goal" in simple_agent_config
        assert "backstory" in simple_agent_config
        assert isinstance(simple_agent_config["role"], str)

    def test_simple_task_config_fixture(self, simple_task_config: dict[str, Any]) -> None:
        """simple_task_config should have required task fields."""
        assert "description" in simple_task_config
        assert "expected_output" in simple_task_config

    def test_simple_crew_config_fixture(self, simple_crew_config: dict[str, Any]) -> None:
        """simple_crew_config should have agents, tasks, and name."""
        assert "name" in simple_crew_config
        assert "agents" in simple_crew_config
        assert "tasks" in simple_crew_config
        assert len(simple_crew_config["agents"]) >= 1
        assert len(simple_crew_config["tasks"]) >= 1

    def test_multi_agent_crew_config_fixture(self, multi_agent_crew_config: dict[str, Any]) -> None:
        """multi_agent_crew_config should have multiple agents."""
        assert len(multi_agent_crew_config["agents"]) >= 2
        assert len(multi_agent_crew_config["tasks"]) >= 2

    def test_crew_with_knowledge_fixture(self, crew_with_knowledge: dict[str, Any]) -> None:
        """crew_with_knowledge should have knowledge_paths."""
        assert "knowledge_paths" in crew_with_knowledge
        assert len(crew_with_knowledge["knowledge_paths"]) >= 1
        # Knowledge directory should exist
        for kp in crew_with_knowledge["knowledge_paths"]:
            assert kp.exists()


class TestCommandLineOptions:
    """Test that custom CLI options are registered."""

    def test_e2e_option_registered(self, pytestconfig) -> None:
        """--e2e option should be registered."""
        # If registered, getoption should not raise
        result = pytestconfig.getoption("--e2e")
        assert isinstance(result, bool)

    def test_framework_option_registered(self, pytestconfig) -> None:
        """--framework option should be registered."""
        result = pytestconfig.getoption("--framework")
        # Default is None
        assert result is None
