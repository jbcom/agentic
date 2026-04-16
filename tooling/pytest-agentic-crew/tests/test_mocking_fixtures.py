"""Tests for the mocking fixtures (CrewMocker) functionality."""

from __future__ import annotations

import sys
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from pytest_agentic_crew.mocking import CrewMocker


class TestCrewMockerBasics:
    """Test basic CrewMocker functionality."""

    def test_crew_mocker_has_mocker(self, crew_mocker: CrewMocker) -> None:
        """CrewMocker should have a mocker attribute."""
        assert crew_mocker.mocker is not None

    def test_magic_mock_property(self, crew_mocker: CrewMocker) -> None:
        """MagicMock property should work."""
        mock = crew_mocker.MagicMock()
        assert mock is not None

    def test_mock_property(self, crew_mocker: CrewMocker) -> None:
        """Mock property should work."""
        mock = crew_mocker.Mock()
        assert mock is not None

    def test_patch_property(self, crew_mocker: CrewMocker) -> None:
        """patch property should expose mocker.patch."""
        assert crew_mocker.patch is not None


class TestModuleMocking:
    """Test module mocking functionality."""

    def test_mock_module_adds_to_sys_modules(self, crew_mocker: CrewMocker) -> None:
        """mock_module should add a mock to sys.modules."""
        mock = crew_mocker.mock_module("test_fake_module_xyz")
        assert "test_fake_module_xyz" in sys.modules
        assert sys.modules["test_fake_module_xyz"] is mock

    def test_mock_module_returns_same_mock_on_repeat(self, crew_mocker: CrewMocker) -> None:
        """Calling mock_module twice with same name should return same mock."""
        mock1 = crew_mocker.mock_module("test_fake_repeat")
        mock2 = crew_mocker.mock_module("test_fake_repeat")
        assert mock1 is mock2

    def test_mock_modules_batch(self, crew_mocker: CrewMocker) -> None:
        """mock_modules should mock multiple modules at once."""
        mocks = crew_mocker.mock_modules(["test_mod_a", "test_mod_b"])
        assert "test_mod_a" in mocks
        assert "test_mod_b" in mocks
        assert "test_mod_a" in sys.modules
        assert "test_mod_b" in sys.modules

    def test_restore_modules_cleans_up(self, crew_mocker: CrewMocker) -> None:
        """restore_modules should remove mocked modules from sys.modules."""
        crew_mocker.mock_module("test_restore_target")
        assert "test_restore_target" in sys.modules

        crew_mocker.restore_modules()
        assert "test_restore_target" not in sys.modules
        assert crew_mocker.mocked_modules == {}


class TestCrewAIMocking:
    """Test CrewAI-specific mocking helpers."""

    def test_mock_crewai_mocks_all_modules(self, crew_mocker: CrewMocker) -> None:
        """mock_crewai should mock all CrewAI framework modules."""
        from pytest_agentic_crew.mocking import CREWAI_MODULES

        mocks = crew_mocker.mock_crewai()

        for module in CREWAI_MODULES:
            assert module in mocks
            assert module in sys.modules

    def test_mock_crewai_agent_creates_mock(self, crew_mocker: CrewMocker) -> None:
        """mock_crewai_agent should return a MagicMock."""
        agent = crew_mocker.mock_crewai_agent(role="Test Agent")
        assert agent.role == "Test Agent"

    def test_mock_crewai_task_creates_mock(self, crew_mocker: CrewMocker) -> None:
        """mock_crewai_task should return a MagicMock."""
        task = crew_mocker.mock_crewai_task(description="Test task")
        assert task.description == "Test task"

    def test_mock_crewai_crew_has_kickoff(self, crew_mocker: CrewMocker) -> None:
        """mock_crewai_crew should return a mock with kickoff behavior."""
        crew = crew_mocker.mock_crewai_crew(result="Test output")
        result = crew.kickoff()
        assert result.raw == "Test output"

    def test_patch_crewai_agent(self, crew_mocker: CrewMocker) -> None:
        """patch_crewai_agent should return a patcher mock."""
        crew_mocker.mock_crewai()
        mock = crew_mocker.patch_crewai_agent()
        assert mock is not None

    def test_patch_crewai_crew(self, crew_mocker: CrewMocker) -> None:
        """patch_crewai_crew should return a patcher mock."""
        crew_mocker.mock_crewai()
        mock = crew_mocker.patch_crewai_crew()
        assert mock is not None

    def test_patch_knowledge_source(self, crew_mocker: CrewMocker) -> None:
        """patch_knowledge_source should return the TextFileKnowledgeSource mock."""
        crew_mocker.mock_crewai()
        mock = crew_mocker.patch_knowledge_source()
        assert mock is not None


class TestLangGraphMocking:
    """Test LangGraph-specific mocking helpers."""

    def test_mock_langgraph_mocks_all_modules(self, crew_mocker: CrewMocker) -> None:
        """mock_langgraph should mock all LangGraph framework modules."""
        from pytest_agentic_crew.mocking import LANGGRAPH_MODULES

        mocks = crew_mocker.mock_langgraph()

        for module in LANGGRAPH_MODULES:
            assert module in mocks

    def test_patch_create_react_agent(self, crew_mocker: CrewMocker) -> None:
        """patch_create_react_agent should return a mock."""
        crew_mocker.mock_langgraph()
        mock = crew_mocker.patch_create_react_agent()
        assert mock is not None

    def test_patch_chat_anthropic(self, crew_mocker: CrewMocker) -> None:
        """patch_chat_anthropic should return a mock."""
        crew_mocker.mock_langgraph()
        mock = crew_mocker.patch_chat_anthropic()
        assert mock is not None

    def test_mock_langgraph_graph(self, crew_mocker: CrewMocker) -> None:
        """mock_langgraph_graph should return a graph with invoke behavior."""
        graph = crew_mocker.mock_langgraph_graph(result="Test response")
        result = graph.invoke({"messages": []})
        assert "messages" in result
        messages = result["messages"]
        assert len(messages) == 1
        assert messages[0].content == "Test response"


class TestStrandsMocking:
    """Test Strands-specific mocking helpers."""

    def test_mock_strands_mocks_all_modules(self, crew_mocker: CrewMocker) -> None:
        """mock_strands should mock all Strands framework modules."""
        from pytest_agentic_crew.mocking import STRANDS_MODULES

        mocks = crew_mocker.mock_strands()

        for module in STRANDS_MODULES:
            assert module in mocks

    def test_patch_strands_agent(self, crew_mocker: CrewMocker) -> None:
        """patch_strands_agent should return a mock."""
        crew_mocker.mock_strands()
        mock = crew_mocker.patch_strands_agent()
        assert mock is not None

    def test_mock_strands_agent_callable(self, crew_mocker: CrewMocker) -> None:
        """mock_strands_agent should return a callable mock with the result."""
        agent = crew_mocker.mock_strands_agent(result="Strands response")
        result = agent("test prompt")
        assert result == "Strands response"


class TestAllFrameworksMocking:
    """Test mocking all frameworks at once."""

    def test_mock_all_frameworks(self, crew_mocker: CrewMocker) -> None:
        """mock_all_frameworks should mock CrewAI, LangGraph, and Strands."""
        from pytest_agentic_crew.mocking import ALL_FRAMEWORK_MODULES

        mocks = crew_mocker.mock_all_frameworks()

        for module in ALL_FRAMEWORK_MODULES:
            assert module in mocks


class TestInternalMocking:
    """Test agentic-crew internal mocking helpers."""

    def test_patch_get_llm(self, crew_mocker: CrewMocker) -> None:
        """patch_get_llm should mock agentic_crew.config.llm.get_llm."""
        mock = crew_mocker.patch_get_llm()
        assert mock is not None
        # Should have a return value set
        assert mock.return_value is not None

    def test_patch_get_llm_custom_return(self, crew_mocker: CrewMocker) -> None:
        """patch_get_llm should accept custom return value."""
        custom_llm = crew_mocker.MagicMock(name="custom_llm")
        mock = crew_mocker.patch_get_llm(return_value=custom_llm)
        assert mock.return_value is custom_llm

    def test_patch_discover_packages(self, crew_mocker: CrewMocker) -> None:
        """patch_discover_packages should mock discover_packages."""
        mock = crew_mocker.patch_discover_packages()
        assert mock is not None
        assert mock.return_value == {}

    def test_patch_discover_packages_with_data(self, crew_mocker: CrewMocker) -> None:
        """patch_discover_packages should accept custom return data."""
        from pathlib import Path

        packages = {"my_pkg": Path("/test/.crewai")}
        mock = crew_mocker.patch_discover_packages(packages=packages)
        assert mock.return_value == packages

    def test_patch_get_crew_config(self, crew_mocker: CrewMocker) -> None:
        """patch_get_crew_config should mock get_crew_config."""
        mock = crew_mocker.patch_get_crew_config()
        assert mock is not None
        assert mock.return_value["name"] == "test"

    def test_patch_run_crew_auto(self, crew_mocker: CrewMocker) -> None:
        """patch_run_crew_auto should mock run_crew_auto."""
        mock = crew_mocker.patch_run_crew_auto(result="Custom result")
        assert mock.return_value == "Custom result"


class TestMockFrameworksFixture:
    """Test the convenience mock_frameworks fixture."""

    def test_mock_frameworks_returns_dict(self, mock_frameworks: dict[str, Any]) -> None:
        """mock_frameworks fixture should return a dict of mocks."""
        assert isinstance(mock_frameworks, dict)
        assert len(mock_frameworks) > 0

    def test_mock_frameworks_includes_crewai(self, mock_frameworks: dict[str, Any]) -> None:
        """mock_frameworks should include crewai modules."""
        assert "crewai" in mock_frameworks

    def test_mock_frameworks_includes_langgraph(self, mock_frameworks: dict[str, Any]) -> None:
        """mock_frameworks should include langgraph modules."""
        assert "langgraph" in mock_frameworks

    def test_mock_frameworks_includes_strands(self, mock_frameworks: dict[str, Any]) -> None:
        """mock_frameworks should include strands modules."""
        assert "strands" in mock_frameworks
