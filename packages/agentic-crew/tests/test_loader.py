"""Tests for the loader module.

Note: These tests require crewai to be installed.
"""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pytest_agentic_crew.mocking import CrewMocker

# Skip all tests in this module if crewai is not installed
pytestmark = pytest.mark.skipif(
    not pytest.importorskip("crewai", reason="crewai not installed"),
    reason="crewai not installed",
)


class TestCreateAgentFromConfig:
    """Tests for create_agent_from_config function."""

    def test_creates_agent_with_config(self, crew_mocker: CrewMocker) -> None:
        """Test that create_agent_from_config creates an Agent."""
        from agentic_crew.core.loader import create_agent_from_config

        config = {
            "role": "Test Agent Role",
            "goal": "Test goal",
            "backstory": "Test backstory",
        }

        MockAgent = crew_mocker.patch("crewai.Agent")
        crew_mocker.patch_get_llm()

        create_agent_from_config("test_agent", config)

        MockAgent.assert_called_once()
        call_kwargs = MockAgent.call_args[1]
        assert call_kwargs["role"] == "Test Agent Role"
        assert call_kwargs["goal"] == "Test goal"
        assert call_kwargs["backstory"] == "Test backstory"

    def test_uses_agent_name_as_default_role(self, crew_mocker: CrewMocker) -> None:
        """Test that agent name is used as default role."""
        from agentic_crew.core.loader import create_agent_from_config

        config = {"goal": "Test goal", "backstory": "Test backstory"}

        MockAgent = crew_mocker.patch("crewai.Agent")
        crew_mocker.patch_get_llm()

        create_agent_from_config("custom_agent_name", config)

        call_kwargs = MockAgent.call_args[1]
        assert call_kwargs["role"] == "custom_agent_name"


class TestCreateTaskFromConfig:
    """Tests for create_task_from_config function."""

    def test_creates_task_with_config(self, crew_mocker: CrewMocker) -> None:
        """Test that create_task_from_config creates a Task."""
        from agentic_crew.core.loader import create_task_from_config

        config = {
            "description": "Test task description",
            "expected_output": "Test output",
        }
        mock_agent = crew_mocker.MagicMock()

        MockTask = crew_mocker.patch("crewai.Task")

        create_task_from_config("test_task", config, mock_agent)

        MockTask.assert_called_once()
        call_kwargs = MockTask.call_args[1]
        assert call_kwargs["description"] == "Test task description"
        assert call_kwargs["expected_output"] == "Test output"
        assert call_kwargs["agent"] == mock_agent


class TestLoadKnowledgeSources:
    """Tests for load_knowledge_sources function."""

    def test_loads_md_files(self, crew_mocker: CrewMocker, tmp_path: Path) -> None:
        """Test that .md files are loaded as knowledge sources."""
        from agentic_crew.core.loader import load_knowledge_sources

        # Create test knowledge directory with .md file
        knowledge_dir = tmp_path / "knowledge"
        knowledge_dir.mkdir()
        (knowledge_dir / "test.md").write_text("# Test Knowledge\nSome content")

        MockKnowledgeSource = crew_mocker.patch_knowledge_source()

        load_knowledge_sources([knowledge_dir])

        # Verify TextFileKnowledgeSource was called
        assert MockKnowledgeSource.called

    def test_skips_nonexistent_paths(self, tmp_path: Path) -> None:
        """Test that nonexistent paths are skipped."""
        from agentic_crew.core.loader import load_knowledge_sources

        sources = load_knowledge_sources([tmp_path / "nonexistent"])

        assert sources == []


class TestLoadCrewFromConfig:
    """Tests for load_crew_from_config function."""

    def test_creates_crew_with_agents_and_tasks(self, crew_mocker: CrewMocker) -> None:
        """Test that load_crew_from_config creates a complete Crew."""
        from agentic_crew.core.loader import load_crew_from_config

        config = {
            "name": "test_crew",
            "agents": {
                "test_agent": {
                    "role": "Test Agent",
                    "goal": "Test goal",
                    "backstory": "Test backstory",
                }
            },
            "tasks": {
                "test_task": {
                    "description": "Test description",
                    "expected_output": "Test output",
                    "agent": "test_agent",
                }
            },
            "knowledge_paths": [],
        }

        MockCrew = crew_mocker.patch_crewai_crew()
        MockAgent = crew_mocker.patch_crewai_agent()
        MockTask = crew_mocker.patch_crewai_task()
        crew_mocker.patch_crewai_process()
        crew_mocker.patch_get_llm()

        mock_agent = crew_mocker.MagicMock()
        MockAgent.return_value = mock_agent

        mock_task = crew_mocker.MagicMock()
        MockTask.return_value = mock_task

        load_crew_from_config(config)

        # Verify Agent was created
        MockAgent.assert_called()
        # Verify Task was created
        MockTask.assert_called()
        # Verify Crew was created with agents and tasks
        MockCrew.assert_called_once()
        call_kwargs = MockCrew.call_args[1]
        assert len(call_kwargs["agents"]) == 1
        assert len(call_kwargs["tasks"]) == 1
