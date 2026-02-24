"""Tests for runner implementations.

These tests verify the interface contracts for all runner implementations
without requiring the actual frameworks to be installed.
"""

from __future__ import annotations

import sys
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from pytest_agentic_crew.mocking import CrewMocker


class TestCrewAIRunner:
    """Tests for CrewAI runner implementation."""

    def test_build_agent_creates_crewai_agent(self, crew_mocker: CrewMocker) -> None:
        """Test that build_agent creates a CrewAI Agent."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        MockAgent = crew_mocker.patch_crewai_agent()
        crew_mocker.patch_get_llm()

        runner = CrewAIRunner()

        agent_config = {
            "role": "Test Agent",
            "goal": "Test goal",
            "backstory": "Test backstory",
            "allow_delegation": True,
        }

        tools = [crew_mocker.MagicMock()]
        runner.build_agent(agent_config, tools=tools)

        MockAgent.assert_called_once()
        call_kwargs = MockAgent.call_args[1]
        assert call_kwargs["role"] == "Test Agent"
        assert call_kwargs["goal"] == "Test goal"
        assert call_kwargs["backstory"] == "Test backstory"
        assert call_kwargs["allow_delegation"] is True
        assert call_kwargs["tools"] == tools

    def test_build_task_creates_crewai_task(self, crew_mocker: CrewMocker) -> None:
        """Test that build_task creates a CrewAI Task."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        MockTask = crew_mocker.patch_crewai_task()

        runner = CrewAIRunner()

        task_config = {
            "description": "Test task description",
            "expected_output": "Test output",
        }
        mock_agent = crew_mocker.MagicMock()

        runner.build_task(task_config, mock_agent)

        MockTask.assert_called_once()
        call_kwargs = MockTask.call_args[1]
        assert call_kwargs["description"] == "Test task description"
        assert call_kwargs["expected_output"] == "Test output"
        assert call_kwargs["agent"] == mock_agent

    def test_build_task_includes_context(self, crew_mocker: CrewMocker) -> None:
        """Test that build_task includes context tasks when provided."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        MockTask = crew_mocker.patch_crewai_task()

        runner = CrewAIRunner()

        task_config = {
            "description": "Test task",
            "expected_output": "Test output",
        }
        mock_agent = crew_mocker.MagicMock()
        context_tasks = [crew_mocker.MagicMock(), crew_mocker.MagicMock()]

        runner.build_task(task_config, mock_agent, context=context_tasks)

        call_kwargs = MockTask.call_args[1]
        assert call_kwargs["context"] == context_tasks

    def test_build_crew_creates_crewai_crew(self, crew_mocker: CrewMocker) -> None:
        """Test that build_crew creates a CrewAI Crew."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        MockCrew = crew_mocker.patch_crewai_crew()
        MockAgent = crew_mocker.patch_crewai_agent()
        MockTask = crew_mocker.patch_crewai_task()
        crew_mocker.patch_crewai_process()
        crew_mocker.patch_get_llm()

        runner = CrewAIRunner()

        mock_agent = crew_mocker.MagicMock()
        MockAgent.return_value = mock_agent

        mock_task = crew_mocker.MagicMock()
        MockTask.return_value = mock_task

        crew_config = {
            "name": "test_crew",
            "agents": {
                "agent1": {
                    "role": "Agent 1",
                    "goal": "Goal 1",
                    "backstory": "Backstory 1",
                }
            },
            "tasks": {
                "task1": {
                    "description": "Task 1",
                    "expected_output": "Output 1",
                    "agent": "agent1",
                }
            },
            "knowledge_paths": [],
        }

        runner.build_crew(crew_config)

        MockCrew.assert_called_once()
        call_kwargs = MockCrew.call_args[1]
        assert len(call_kwargs["agents"]) == 1
        assert len(call_kwargs["tasks"]) == 1
        assert call_kwargs["planning"] is True
        assert call_kwargs["memory"] is True

    def test_run_executes_crew_and_returns_result(self, crew_mocker: CrewMocker) -> None:
        """Test that run executes crew and returns string result."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()

        mock_crew = crew_mocker.mock_crewai_crew(result="Test output")

        result = runner.run(mock_crew, {"input": "test input"})

        mock_crew.kickoff.assert_called_once_with(inputs={"input": "test input"})
        assert result == "Test output"

    def test_run_handles_result_without_raw(self, crew_mocker: CrewMocker) -> None:
        """Test that run handles results without raw attribute."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()

        mock_crew = crew_mocker.MagicMock()
        mock_crew.kickoff.return_value = "Direct string result"

        result = runner.run(mock_crew, {})

        assert result == "Direct string result"

    def test_handles_knowledge_sources(self, crew_mocker: CrewMocker, tmp_path) -> None:
        """Test that knowledge sources are loaded from paths."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        MockKnowledge = crew_mocker.patch_knowledge_source()

        runner = CrewAIRunner()

        # Create test knowledge file
        knowledge_dir = tmp_path / "knowledge"
        knowledge_dir.mkdir()
        (knowledge_dir / "test.md").write_text("# Test Knowledge\nContent here")

        sources = runner._load_knowledge([knowledge_dir])

        # Verify TextFileKnowledgeSource was called for the .md file
        assert MockKnowledge.called
        assert len(sources) > 0

    def test_handles_task_context_dependencies(self, crew_mocker: CrewMocker) -> None:
        """Test that tasks with context dependencies are handled correctly."""
        crew_mocker.mock_crewai()

        from agentic_crew.runners.crewai_runner import CrewAIRunner

        crew_mocker.patch_crewai_crew()
        MockAgent = crew_mocker.patch_crewai_agent()
        MockTask = crew_mocker.patch_crewai_task()
        crew_mocker.patch_crewai_process()
        crew_mocker.patch_get_llm()

        runner = CrewAIRunner()

        mock_agent = crew_mocker.MagicMock()
        MockAgent.return_value = mock_agent

        mock_task1 = crew_mocker.MagicMock()
        mock_task2 = crew_mocker.MagicMock()
        MockTask.side_effect = [mock_task1, mock_task2]

        crew_config = {
            "agents": {
                "agent1": {
                    "role": "Agent",
                    "goal": "Goal",
                    "backstory": "Story",
                }
            },
            "tasks": {
                "task1": {
                    "description": "Task 1",
                    "expected_output": "Output 1",
                    "agent": "agent1",
                },
                "task2": {
                    "description": "Task 2",
                    "expected_output": "Output 2",
                    "agent": "agent1",
                    "context": ["task1"],
                },
            },
            "knowledge_paths": [],
        }

        runner.build_crew(crew_config)

        # Verify second task was created with context
        assert MockTask.call_count == 2
        second_task_kwargs = MockTask.call_args_list[1][1]
        assert "context" in second_task_kwargs
        assert second_task_kwargs["context"] == [mock_task1]


class TestLangGraphRunner:
    """Tests for LangGraph runner implementation."""

    def test_build_agent_creates_react_agent(self, crew_mocker: CrewMocker) -> None:
        """Test that build_agent creates a LangGraph ReAct agent."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        mock_create = crew_mocker.patch_create_react_agent()
        MockLLM = crew_mocker.patch_chat_anthropic()
        mock_llm = crew_mocker.MagicMock()
        MockLLM.return_value = mock_llm

        runner = LangGraphRunner()

        agent_config = {
            "role": "Test Agent",
            "goal": "Test goal",
            "llm": "claude-sonnet-4-20250514",
        }
        tools = [crew_mocker.MagicMock()]

        runner.build_agent(agent_config, tools=tools)

        mock_create.assert_called_once()
        args = mock_create.call_args[0]
        assert args[0] == mock_llm
        assert args[1] == tools

    def test_build_crew_creates_graph(self, crew_mocker: CrewMocker) -> None:
        """Test that build_crew creates a LangGraph workflow."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        mock_create = crew_mocker.patch_create_react_agent()
        crew_mocker.patch_chat_anthropic()

        runner = LangGraphRunner()

        crew_config = {
            "llm": {"model": "claude-sonnet-4-20250514"},
            "agents": {},
            "tasks": {},
        }

        runner.build_crew(crew_config)

        mock_create.assert_called_once()

    def test_run_invokes_graph(self, crew_mocker: CrewMocker) -> None:
        """Test that run invokes the LangGraph workflow."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        runner = LangGraphRunner()

        mock_graph = crew_mocker.mock_langgraph_graph(result="Test response")

        result = runner.run(mock_graph, {"input": "test prompt"})

        mock_graph.invoke.assert_called_once()
        invoke_args = mock_graph.invoke.call_args[0][0]
        assert "messages" in invoke_args
        assert result == "Test response"

    def test_run_handles_dict_messages(self, crew_mocker: CrewMocker) -> None:
        """Test that run handles dictionary message format."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        runner = LangGraphRunner()

        mock_graph = crew_mocker.MagicMock()
        mock_graph.invoke.return_value = {"messages": [("assistant", "Test response")]}

        runner.run(mock_graph, {"task": "test task"})

        mock_graph.invoke.assert_called_once()
        invoke_args = mock_graph.invoke.call_args[0][0]
        assert "messages" in invoke_args
        assert any("test task" in str(msg) for msg in invoke_args["messages"])

    def test_get_llm_returns_chat_anthropic(self, crew_mocker: CrewMocker) -> None:
        """Test that get_llm returns ChatAnthropic instance."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        MockLLM = crew_mocker.patch_chat_anthropic()

        runner = LangGraphRunner()
        runner.get_llm("claude-sonnet-4-20250514")

        MockLLM.assert_called_once_with(model="claude-sonnet-4-20250514")

    def test_get_llm_uses_default_model(self, crew_mocker: CrewMocker) -> None:
        """Test that get_llm uses default model when none specified."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        MockLLM = crew_mocker.patch_chat_anthropic()

        runner = LangGraphRunner()
        runner.get_llm()

        MockLLM.assert_called_once()
        assert MockLLM.call_args[1]["model"] == "claude-haiku-4-5-20251001"

    def test_build_task_returns_dict(self, crew_mocker: CrewMocker) -> None:
        """Test that build_task returns task configuration dict."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        runner = LangGraphRunner()

        task_config = {
            "description": "Test task",
            "expected_output": "Test output",
        }
        mock_agent = crew_mocker.MagicMock()

        result = runner.build_task(task_config, mock_agent)

        assert isinstance(result, dict)
        assert result["description"] == "Test task"
        assert result["expected_output"] == "Test output"
        assert result["agent"] == mock_agent

    def test_handles_multi_agent_flows(self, crew_mocker: CrewMocker) -> None:
        """Test that multiple agents can be created for complex flows."""
        crew_mocker.mock_langgraph()

        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        mock_create = crew_mocker.patch_create_react_agent()
        crew_mocker.patch_chat_anthropic()

        runner = LangGraphRunner()

        # Create multiple agents
        agent1_config = {"role": "Agent 1", "goal": "Goal 1"}
        agent2_config = {"role": "Agent 2", "goal": "Goal 2"}

        runner.build_agent(agent1_config)
        runner.build_agent(agent2_config)

        # Verify create_react_agent was called twice
        assert mock_create.call_count == 2


class TestStrandsRunner:
    """Tests for Strands runner implementation."""

    def test_build_agent_creates_strands_agent(self, crew_mocker: CrewMocker) -> None:
        """Test that build_agent creates a Strands Agent."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        MockAgent = crew_mocker.patch_strands_agent()

        runner = StrandsRunner()

        agent_config = {
            "role": "Test Agent",
            "goal": "Test goal",
            "backstory": "Test backstory",
        }
        tools = [crew_mocker.MagicMock()]

        runner.build_agent(agent_config, tools=tools)

        MockAgent.assert_called_once()
        call_kwargs = MockAgent.call_args[1]
        assert "Test Agent" in call_kwargs["system_prompt"]
        assert "Test goal" in call_kwargs["system_prompt"]
        assert "Test backstory" in call_kwargs["system_prompt"]
        assert call_kwargs["tools"] == tools

    def test_build_system_prompt_combines_agent_prompts(self, crew_mocker: CrewMocker) -> None:
        """Test that _build_system_prompt combines multiple agent roles."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        crew_config = {
            "description": "Test crew description",
            "agents": {
                "agent1": {
                    "role": "Agent One",
                    "goal": "Goal one",
                },
                "agent2": {
                    "role": "Agent Two",
                    "goal": "Goal two",
                },
            },
            "tasks": {
                "task1": {
                    "description": "Task one description",
                }
            },
        }

        prompt = runner._build_system_prompt(crew_config)

        assert "Test crew description" in prompt
        assert "Agent One" in prompt
        assert "Agent Two" in prompt
        assert "Goal one" in prompt
        assert "Goal two" in prompt
        assert "task1" in prompt

    def test_build_system_prompt_truncates_long_descriptions(self, crew_mocker: CrewMocker) -> None:
        """Test that long task descriptions are truncated."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        long_desc = "A" * 250
        crew_config = {
            "tasks": {
                "task1": {
                    "description": long_desc,
                }
            },
        }

        prompt = runner._build_system_prompt(crew_config)

        # Should be truncated to 200 chars + ellipsis
        assert "..." in prompt
        assert prompt.count("A") == 200

    def test_build_system_prompt_keeps_short_descriptions(self, crew_mocker: CrewMocker) -> None:
        """Test that short task descriptions are not truncated."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        short_desc = "Short description"
        crew_config = {
            "tasks": {
                "task1": {
                    "description": short_desc,
                }
            },
        }

        prompt = runner._build_system_prompt(crew_config)

        # Should not have ellipsis for short descriptions
        assert "..." not in prompt
        assert short_desc in prompt

    def test_run_returns_string_result(self, crew_mocker: CrewMocker) -> None:
        """Test that run executes agent and returns string result."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        mock_agent = crew_mocker.mock_strands_agent(result="Test agent response")

        result = runner.run(mock_agent, {"input": "test prompt"})

        mock_agent.assert_called_once_with("test prompt")
        assert result == "Test agent response"

    def test_run_handles_task_input(self, crew_mocker: CrewMocker) -> None:
        """Test that run handles 'task' key in inputs."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        mock_agent = crew_mocker.mock_strands_agent(result="Response")

        runner.run(mock_agent, {"task": "do something"})

        mock_agent.assert_called_once_with("do something")

    def test_get_model_provider_from_string(self, crew_mocker: CrewMocker) -> None:
        """Test extracting model provider from string config."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        result = runner._get_model_provider("claude-3-5-sonnet")

        assert result == "claude-3-5-sonnet"

    def test_get_model_provider_from_dict(self, crew_mocker: CrewMocker) -> None:
        """Test extracting model provider from dict config."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        result = runner._get_model_provider({"model": "claude-3-5-sonnet", "provider": "anthropic"})

        assert result == "claude-3-5-sonnet"

    def test_get_model_provider_returns_none_for_empty(self, crew_mocker: CrewMocker) -> None:
        """Test that None is returned when no config provided."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        result = runner._get_model_provider(None)

        assert result is None

    def test_build_crew_creates_strands_agent(self, crew_mocker: CrewMocker) -> None:
        """Test that build_crew creates a Strands agent."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        MockAgent = crew_mocker.patch_strands_agent()

        runner = StrandsRunner()

        crew_config = {
            "description": "Test crew",
            "llm": {"model": "claude-3-5-sonnet"},
            "agents": {},
            "tasks": {},
        }

        runner.build_crew(crew_config)

        MockAgent.assert_called_once()
        call_kwargs = MockAgent.call_args[1]
        assert "system_prompt" in call_kwargs
        assert call_kwargs["model_id"] == "claude-3-5-sonnet"

    def test_build_task_returns_dict(self, crew_mocker: CrewMocker) -> None:
        """Test that build_task returns task configuration dict."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        task_config = {
            "description": "Test task",
            "expected_output": "Test output",
        }
        mock_agent = crew_mocker.MagicMock()

        result = runner.build_task(task_config, mock_agent)

        assert isinstance(result, dict)
        assert result["description"] == "Test task"
        assert result["expected_output"] == "Test output"
        assert result["agent"] == mock_agent

    def test_combines_agent_prompts(self, crew_mocker: CrewMocker) -> None:
        """Test that multiple agent capabilities are combined into system prompt."""
        crew_mocker.mock_strands()

        from agentic_crew.runners.strands_runner import StrandsRunner

        MockAgent = crew_mocker.patch_strands_agent()

        runner = StrandsRunner()

        crew_config = {
            "description": "Multi-capability crew",
            "agents": {
                "researcher": {"role": "Researcher", "goal": "Research topics"},
                "writer": {"role": "Writer", "goal": "Write content"},
            },
            "tasks": {},
        }

        runner.build_crew(crew_config)

        call_kwargs = MockAgent.call_args[1]
        system_prompt = call_kwargs["system_prompt"]

        # Verify both agent capabilities are in the prompt
        assert "Researcher" in system_prompt
        assert "Writer" in system_prompt
        assert "Research topics" in system_prompt
        assert "Write content" in system_prompt


class TestBaseRunner:
    """Tests for base runner interface."""

    def test_build_and_run_convenience_method(self, crew_mocker: CrewMocker) -> None:
        """Test that build_and_run calls build_crew and run with correct args."""
        from agentic_crew.runners.base import BaseRunner

        class TestRunner(BaseRunner):
            # Define abstract methods so the class can be instantiated
            def build_crew(self, crew_config: dict[str, Any]) -> Any:
                pass

            def run(self, crew: Any, inputs: dict[str, Any]) -> str:
                pass

            def build_agent(self, agent_config: dict[str, Any], tools: list | None = None) -> Any:
                return crew_mocker.MagicMock()

            def build_task(self, task_config: dict[str, Any], agent: Any) -> Any:
                return crew_mocker.MagicMock()

        runner = TestRunner()

        # Mock the methods to verify they're called correctly
        mock_crew_obj = crew_mocker.MagicMock()
        runner.build_crew = crew_mocker.MagicMock(return_value=mock_crew_obj)
        runner.run = crew_mocker.MagicMock(return_value="test result")

        crew_config = {"test": "config"}
        inputs = {"test": "input"}

        result = runner.build_and_run(crew_config, inputs)

        assert result == "test result"
        runner.build_crew.assert_called_once_with(crew_config)
        runner.run.assert_called_once_with(mock_crew_obj, inputs)

    def test_build_and_run_uses_empty_inputs_when_none(self, crew_mocker: CrewMocker) -> None:
        """Test that build_and_run uses empty dict when inputs is None."""
        from agentic_crew.runners.base import BaseRunner

        class TestRunner(BaseRunner):
            def build_crew(self, crew_config: dict[str, Any]) -> Any:
                return crew_mocker.MagicMock()

            def run(self, crew: Any, inputs: dict[str, Any]) -> str:
                assert inputs == {}
                return "result"

            def build_agent(self, agent_config: dict[str, Any], tools: list | None = None) -> Any:
                return crew_mocker.MagicMock()

            def build_task(self, task_config: dict[str, Any], agent: Any) -> Any:
                return crew_mocker.MagicMock()

        runner = TestRunner()
        result = runner.build_and_run({}, None)

        assert result == "result"

    def test_get_llm_default_implementation(self, crew_mocker: CrewMocker) -> None:
        """Test that get_llm has a default implementation."""
        mock_get_llm = crew_mocker.patch_get_llm()
        crew_mocker.patch("agentic_crew.config.llm.DEFAULT_MODEL", "claude-sonnet-4-20250514")

        from agentic_crew.runners.base import BaseRunner

        class TestRunner(BaseRunner):
            def build_crew(self, crew_config: dict[str, Any]) -> Any:
                return crew_mocker.MagicMock()

            def run(self, crew: Any, inputs: dict[str, Any]) -> str:
                return "result"

            def build_agent(self, agent_config: dict[str, Any], tools: list | None = None) -> Any:
                return crew_mocker.MagicMock()

            def build_task(self, task_config: dict[str, Any], agent: Any) -> Any:
                return crew_mocker.MagicMock()

        runner = TestRunner()
        llm = runner.get_llm()

        assert llm == mock_get_llm.return_value

    def test_get_llm_returns_none_when_module_unavailable(self, crew_mocker: CrewMocker) -> None:
        """Test that get_llm returns None when llm module not available."""
        from agentic_crew.runners.base import BaseRunner

        class TestRunner(BaseRunner):
            def build_crew(self, crew_config: dict[str, Any]) -> Any:
                return crew_mocker.MagicMock()

            def run(self, crew: Any, inputs: dict[str, Any]) -> str:
                return "result"

            def build_agent(self, agent_config: dict[str, Any], tools: list | None = None) -> Any:
                return crew_mocker.MagicMock()

            def build_task(self, task_config: dict[str, Any], agent: Any) -> Any:
                return crew_mocker.MagicMock()

        runner = TestRunner()

        # Simulate that the agentic_crew.config.llm module is not available
        crew_mocker.patch.dict(sys.modules, {"agentic_crew.config.llm": None})
        llm = runner.get_llm()

        assert llm is None
