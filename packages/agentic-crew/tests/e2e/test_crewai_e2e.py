"""End-to-end tests for CrewAI runner.

These tests verify that the CrewAI runner works correctly with real LLM calls.
They require:
- The --e2e flag to run
- ANTHROPIC_API_KEY environment variable
- CrewAI installed (pip install crewai[tools])
"""

from __future__ import annotations

from typing import Any

import pytest

# Check if CrewAI is available
try:
    import crewai  # noqa: F401

    CREWAI_AVAILABLE = True
except ImportError:
    CREWAI_AVAILABLE = False

# Skip all tests if CrewAI is not installed
pytestmark = [
    pytest.mark.e2e,
    pytest.mark.crewai,
    pytest.mark.skipif(not CREWAI_AVAILABLE, reason="crewai not installed"),
]


class TestCrewAISimpleExecution:
    """Test simple crew execution with CrewAI."""

    def test_simple_crew_execution(
        self,
        check_api_key: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test execution of a simple single-agent, single-task crew.

        Args:
            check_api_key: Fixture to check for required API keys.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()

        # Build the crew
        crew = runner.build_crew(simple_crew_config)
        assert crew is not None

        # Run the crew with a simple input
        inputs = {"input": "What is 2 + 2?"}
        result = runner.run(crew, inputs)

        # Verify we got a response
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # The answer should contain '4' somewhere
        assert "4" in result

    def test_crew_build_and_run(
        self,
        check_api_key: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test the convenience build_and_run method.

        Args:
            check_api_key: Fixture to check for required API keys.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()

        # Use build_and_run convenience method
        inputs = {"input": "What is the capital of France?"}
        result = runner.build_and_run(simple_crew_config, inputs)

        # Verify we got a response about Paris
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        assert "Paris" in result or "paris" in result.lower()


class TestCrewAIMultiAgent:
    """Test multi-agent collaboration with CrewAI."""

    def test_multi_agent_crew(
        self,
        check_api_key: None,
        multi_agent_crew_config: dict[str, Any],
    ) -> None:
        """Test multi-agent crew with sequential task execution.

        Args:
            check_api_key: Fixture to check for required API keys.
            multi_agent_crew_config: Multi-agent crew configuration fixture.
        """
        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()

        # Build and run multi-agent crew
        inputs = {"input": "Tell me about Python programming"}
        result = runner.build_and_run(multi_agent_crew_config, inputs)

        # Verify we got a result from the collaborative effort
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should mention Python in some form
        assert "Python" in result or "python" in result.lower()


class TestCrewAIKnowledge:
    """Test knowledge source integration with CrewAI."""

    def test_knowledge_source_integration(
        self,
        check_api_key: None,
        crew_with_knowledge: dict[str, Any],
    ) -> None:
        """Test crew with knowledge sources loaded from files.

        Args:
            check_api_key: Fixture to check for required API keys.
            crew_with_knowledge: Crew config with knowledge sources fixture.
        """
        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()

        # Build and run crew with knowledge
        inputs = {"question": "What color is mentioned in the knowledge base?"}
        result = runner.build_and_run(crew_with_knowledge, inputs)

        # Verify the crew accessed the knowledge
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should mention blue from the knowledge file
        assert "blue" in result.lower() or "Blue" in result


class TestCrewAITools:
    """Test tool usage with CrewAI."""

    def test_tool_usage(
        self,
        check_api_key: None,
        simple_agent_config: dict[str, Any],
    ) -> None:
        """Test crew with tools assigned to agents.

        Args:
            check_api_key: Fixture to check for required API keys.
            simple_agent_config: Simple agent configuration fixture.
        """
        from agentic_crew.runners.crewai_runner import CrewAIRunner
        from crewai_tools import tool

        # Create a simple tool
        @tool("get_test_data")
        def get_test_data(query: str) -> str:
            """Get test data for a query.

            Args:
                query: The query string.

            Returns:
                Test data response.
            """
            # Use the query parameter in the response for completeness
            return f"Test data for '{query}': The answer is 42"

        runner = CrewAIRunner()

        # Build agent with tool
        agent = runner.build_agent(simple_agent_config, tools=[get_test_data])
        assert agent is not None

        # Build a simple task
        task_config = {
            "description": "Use the get_test_data tool to find the answer",
            "expected_output": "The answer from the tool",
        }
        task = runner.build_task(task_config, agent)
        assert task is not None

        # Create a minimal crew
        from crewai import Crew, Process

        crew = Crew(
            agents=[agent],
            tasks=[task],
            process=Process.sequential,
            verbose=True,
        )

        # Run the crew
        result = crew.kickoff()

        # Verify the tool was used
        assert result is not None
        result_str = result.raw if hasattr(result, "raw") else str(result)
        assert "42" in result_str
