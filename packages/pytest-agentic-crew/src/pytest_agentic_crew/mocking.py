"""Mocking utilities for agentic-crew testing.

This module provides specialized mocking fixtures that build on top of pytest-mock
to make it easy to test agentic-crew components without needing the actual
AI frameworks installed.

The fixtures automatically mock framework modules and provide convenient methods
for patching agentic-crew internals.
"""

from __future__ import annotations

import sys
from collections.abc import Generator
from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Any

import pytest

if TYPE_CHECKING:
    from pytest_mock import MockerFixture


# =============================================================================
# Framework Module Lists
# =============================================================================

CREWAI_MODULES = [
    "crewai",
    "crewai.knowledge",
    "crewai.knowledge.source",
    "crewai.knowledge.source.text_file_knowledge_source",
]

LANGGRAPH_MODULES = [
    "langgraph",
    "langgraph.prebuilt",
    "langchain_anthropic",
]

STRANDS_MODULES = [
    "strands",
]

ALL_FRAMEWORK_MODULES = CREWAI_MODULES + LANGGRAPH_MODULES + STRANDS_MODULES


# =============================================================================
# CrewMocker Class - Enhanced mocker for agentic-crew testing
# =============================================================================


@dataclass
class CrewMocker:
    """Enhanced mocker specifically for agentic-crew testing.

    This class wraps pytest-mock's MockerFixture and provides convenience
    methods for mocking AI framework components commonly used in agentic-crew.

    Attributes:
        mocker: The underlying pytest-mock MockerFixture.
        mocked_modules: Dictionary of currently mocked framework modules.

    Example:
        def test_crewai_runner(crew_mocker):
            # Mock CrewAI components
            mock_agent = crew_mocker.mock_crewai_agent()
            mock_crew = crew_mocker.mock_crewai_crew()

            # Test runner behavior
            from agentic_crew.runners.crewai_runner import CrewAIRunner
            runner = CrewAIRunner()
            ...
    """

    mocker: MockerFixture
    mocked_modules: dict[str, Any] = field(default_factory=dict)
    _original_modules: dict[str, Any] = field(default_factory=dict)

    # Expose common pytest-mock attributes
    @property
    def MagicMock(self) -> Any:
        """Access to mocker.MagicMock."""
        return self.mocker.MagicMock

    @property
    def Mock(self) -> Any:
        """Access to mocker.Mock."""
        return self.mocker.Mock

    @property
    def patch(self) -> Any:
        """Access to mocker.patch."""
        return self.mocker.patch

    @property
    def spy(self) -> Any:
        """Access to mocker.spy."""
        return self.mocker.spy

    @property
    def stub(self) -> Any:
        """Access to mocker.stub."""
        return self.mocker.stub

    def mock_module(self, module_name: str) -> Any:
        """Mock a single module in sys.modules.

        Args:
            module_name: Full module path (e.g., 'crewai.Agent').

        Returns:
            The mock object for the module.
        """
        if module_name in self.mocked_modules:
            return self.mocked_modules[module_name]

        mock = self.mocker.MagicMock()
        if module_name in sys.modules:
            self._original_modules[module_name] = sys.modules[module_name]
        sys.modules[module_name] = mock
        self.mocked_modules[module_name] = mock
        return mock

    def mock_modules(self, module_names: list[str]) -> dict[str, Any]:
        """Mock multiple modules in sys.modules.

        Args:
            module_names: List of module paths to mock.

        Returns:
            Dictionary mapping module names to their mock objects.
        """
        return {name: self.mock_module(name) for name in module_names}

    def restore_modules(self) -> None:
        """Restore all mocked modules to their original state."""
        for module_name in list(self.mocked_modules.keys()):
            if module_name in self._original_modules:
                sys.modules[module_name] = self._original_modules[module_name]
            else:
                sys.modules.pop(module_name, None)
        self.mocked_modules.clear()
        self._original_modules.clear()

    # =========================================================================
    # CrewAI Mocking Helpers
    # =========================================================================

    def mock_crewai(self) -> dict[str, Any]:
        """Mock all CrewAI framework modules.

        Returns:
            Dictionary of mocked CrewAI modules.
        """
        mocks = self.mock_modules(CREWAI_MODULES)
        # Set up nested module attributes
        if "crewai.knowledge.source.text_file_knowledge_source" in mocks:
            mocks[
                "crewai.knowledge.source.text_file_knowledge_source"
            ].TextFileKnowledgeSource = self.mocker.MagicMock()
        return mocks

    def mock_crewai_agent(self, **kwargs: Any) -> Any:
        """Create a mock for crewai.Agent.

        Args:
            **kwargs: Additional attributes to set on the mock.

        Returns:
            Mock Agent object.
        """
        mock_agent = self.mocker.MagicMock()
        for key, value in kwargs.items():
            setattr(mock_agent, key, value)
        return mock_agent

    def mock_crewai_task(self, **kwargs: Any) -> Any:
        """Create a mock for crewai.Task.

        Args:
            **kwargs: Additional attributes to set on the mock.

        Returns:
            Mock Task object.
        """
        mock_task = self.mocker.MagicMock()
        for key, value in kwargs.items():
            setattr(mock_task, key, value)
        return mock_task

    def mock_crewai_crew(self, result: str = "Test result", **kwargs: Any) -> Any:
        """Create a mock for crewai.Crew with kickoff behavior.

        Args:
            result: The result to return from kickoff().
            **kwargs: Additional attributes to set on the mock.

        Returns:
            Mock Crew object with kickoff configured.
        """
        mock_crew = self.mocker.MagicMock()
        mock_result = self.mocker.MagicMock()
        mock_result.raw = result
        mock_crew.kickoff.return_value = mock_result
        for key, value in kwargs.items():
            setattr(mock_crew, key, value)
        return mock_crew

    def patch_crewai_agent(self) -> Any:
        """Patch crewai.Agent and return the mock.

        Returns:
            The mock for crewai.Agent.
        """
        return self.mocker.patch("crewai.Agent")

    def patch_crewai_task(self) -> Any:
        """Patch crewai.Task and return the mock.

        Returns:
            The mock for crewai.Task.
        """
        return self.mocker.patch("crewai.Task")

    def patch_crewai_crew(self) -> Any:
        """Patch crewai.Crew and return the mock.

        Returns:
            The mock for crewai.Crew.
        """
        return self.mocker.patch("crewai.Crew")

    def patch_crewai_process(self) -> Any:
        """Patch crewai.Process and return the mock.

        Returns:
            The mock for crewai.Process.
        """
        return self.mocker.patch("crewai.Process")

    def patch_knowledge_source(self) -> Any:
        """Get the TextFileKnowledgeSource mock from the mocked crewai module.

        Note: Must call mock_crewai() first.

        Returns:
            The mock for TextFileKnowledgeSource.
        """
        # If the module is already mocked, return its TextFileKnowledgeSource
        if "crewai.knowledge.source.text_file_knowledge_source" in self.mocked_modules:
            return self.mocked_modules["crewai.knowledge.source.text_file_knowledge_source"].TextFileKnowledgeSource
        # Otherwise, fall back to patching
        return self.mocker.patch("crewai.knowledge.source.text_file_knowledge_source.TextFileKnowledgeSource")

    # =========================================================================
    # LangGraph Mocking Helpers
    # =========================================================================

    def mock_langgraph(self) -> dict[str, Any]:
        """Mock all LangGraph framework modules.

        Returns:
            Dictionary of mocked LangGraph modules.
        """
        mocks = self.mock_modules(LANGGRAPH_MODULES)
        # Set up the prebuilt module with create_react_agent
        if "langgraph.prebuilt" in mocks:
            mocks["langgraph.prebuilt"].create_react_agent = self.mocker.MagicMock()
        return mocks

    def patch_create_react_agent(self) -> Any:
        """Get the create_react_agent mock from the mocked langgraph module.

        Note: Must call mock_langgraph() first.

        Returns:
            The mock for create_react_agent.
        """
        # If langgraph.prebuilt is already mocked, return its create_react_agent
        if "langgraph.prebuilt" in self.mocked_modules:
            return self.mocked_modules["langgraph.prebuilt"].create_react_agent
        # Otherwise, fall back to patching
        return self.mocker.patch("langgraph.prebuilt.create_react_agent")

    def patch_chat_anthropic(self) -> Any:
        """Get the ChatAnthropic mock from the mocked langchain_anthropic module.

        Note: Must call mock_langgraph() first.

        Returns:
            The mock for ChatAnthropic.
        """
        # If langchain_anthropic is already mocked, return its ChatAnthropic
        if "langchain_anthropic" in self.mocked_modules:
            return self.mocked_modules["langchain_anthropic"].ChatAnthropic
        # Otherwise, fall back to patching
        return self.mocker.patch("langchain_anthropic.ChatAnthropic")

    def mock_langgraph_graph(self, result: str = "Test response") -> Any:
        """Create a mock LangGraph graph with invoke behavior.

        Args:
            result: The result content to return from invoke().

        Returns:
            Mock graph object with invoke configured.
        """
        mock_graph = self.mocker.MagicMock()
        mock_message = self.mocker.MagicMock()
        mock_message.content = result
        mock_graph.invoke.return_value = {"messages": [mock_message]}
        return mock_graph

    # =========================================================================
    # Strands Mocking Helpers
    # =========================================================================

    def mock_strands(self) -> dict[str, Any]:
        """Mock all Strands framework modules.

        Returns:
            Dictionary of mocked Strands modules.
        """
        return self.mock_modules(STRANDS_MODULES)

    def patch_strands_agent(self) -> Any:
        """Patch strands.Agent and return the mock.

        Returns:
            The mock for strands.Agent.
        """
        return self.mocker.patch("strands.Agent")

    def mock_strands_agent(self, result: str = "Test response") -> Any:
        """Create a mock Strands agent with call behavior.

        Args:
            result: The result to return when agent is called.

        Returns:
            Mock agent object.
        """
        mock_agent = self.mocker.MagicMock()
        mock_agent.return_value = result
        return mock_agent

    # =========================================================================
    # All Frameworks
    # =========================================================================

    def mock_all_frameworks(self) -> dict[str, Any]:
        """Mock all AI framework modules (CrewAI, LangGraph, Strands).

        Returns:
            Dictionary of all mocked framework modules.
        """
        return self.mock_modules(ALL_FRAMEWORK_MODULES)

    # =========================================================================
    # agentic-crew Internal Mocking
    # =========================================================================

    def patch_get_llm(self, return_value: Any = None) -> Any:
        """Patch agentic_crew.config.llm.get_llm.

        Args:
            return_value: Value to return from get_llm. If None, creates a MagicMock.

        Returns:
            The mock for get_llm.
        """
        mock = self.mocker.patch("agentic_crew.config.llm.get_llm")
        mock.return_value = return_value or self.mocker.MagicMock()
        return mock

    def patch_discover_packages(self, packages: dict[str, Any] | None = None) -> Any:
        """Patch agentic_crew.core.discovery.discover_packages.

        Args:
            packages: Dictionary of packages to return. If None, returns empty dict.

        Returns:
            The mock for discover_packages.
        """
        mock = self.mocker.patch("agentic_crew.core.discovery.discover_packages")
        mock.return_value = packages or {}
        return mock

    def patch_get_crew_config(self, config: dict[str, Any] | None = None) -> Any:
        """Patch agentic_crew.core.discovery.get_crew_config.

        Args:
            config: Config to return. If None, returns minimal valid config.

        Returns:
            The mock for get_crew_config.
        """
        mock = self.mocker.patch("agentic_crew.core.discovery.get_crew_config")
        mock.return_value = config or {"name": "test", "agents": {}, "tasks": {}}
        return mock

    def patch_run_crew_auto(self, result: str = "Test result") -> Any:
        """Patch agentic_crew.core.decomposer.run_crew_auto.

        Args:
            result: Result string to return from run_crew_auto.

        Returns:
            The mock for run_crew_auto.
        """
        mock = self.mocker.patch("agentic_crew.core.decomposer.run_crew_auto")
        mock.return_value = result
        return mock


# =============================================================================
# Fixtures
# =============================================================================


@pytest.fixture
def crew_mocker(mocker: MockerFixture) -> Generator[CrewMocker, None, None]:
    """Fixture providing CrewMocker for agentic-crew testing.

    This fixture builds on pytest-mock's mocker fixture and provides
    specialized methods for mocking AI framework components.

    Yields:
        CrewMocker instance with access to all mocking utilities.

    Example:
        def test_my_runner(crew_mocker):
            # Mock CrewAI
            crew_mocker.mock_crewai()
            mock_llm = crew_mocker.patch_get_llm()

            # Test code
            from agentic_crew.runners.crewai_runner import CrewAIRunner
            runner = CrewAIRunner()
            ...
    """
    cm = CrewMocker(mocker=mocker)
    yield cm
    cm.restore_modules()


@pytest.fixture
def mock_frameworks(mocker: MockerFixture) -> Generator[dict[str, Any], None, None]:
    """Fixture to mock all AI framework modules for unit testing.

    This is a convenience fixture that mocks all framework modules
    (crewai, langgraph, strands) so tests can run without the actual
    frameworks installed.

    Yields:
        Dictionary mapping module names to their mock objects.

    Example:
        def test_crewai_runner(mock_frameworks):
            from agentic_crew.runners.crewai_runner import CrewAIRunner
            runner = CrewAIRunner()
            # Test runner behavior with mocked framework
    """
    cm = CrewMocker(mocker=mocker)
    mocks = cm.mock_all_frameworks()
    yield mocks
    cm.restore_modules()


@pytest.fixture
def mock_crewai(mocker: MockerFixture) -> Generator[dict[str, Any], None, None]:
    """Fixture to mock only CrewAI framework modules.

    Yields:
        Dictionary with CrewAI mock objects.
    """
    cm = CrewMocker(mocker=mocker)
    mocks = cm.mock_crewai()
    yield mocks
    cm.restore_modules()


@pytest.fixture
def mock_langgraph(mocker: MockerFixture) -> Generator[dict[str, Any], None, None]:
    """Fixture to mock only LangGraph framework modules.

    Yields:
        Dictionary with LangGraph mock objects.
    """
    cm = CrewMocker(mocker=mocker)
    mocks = cm.mock_langgraph()
    yield mocks
    cm.restore_modules()


@pytest.fixture
def mock_strands(mocker: MockerFixture) -> Generator[dict[str, Any], None, None]:
    """Fixture to mock only Strands framework modules.

    Yields:
        Dictionary with Strands mock objects.
    """
    cm = CrewMocker(mocker=mocker)
    mocks = cm.mock_strands()
    yield mocks
    cm.restore_modules()
