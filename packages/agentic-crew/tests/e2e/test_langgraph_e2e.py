"""End-to-end tests for LangGraph runner.

These tests verify that the LangGraph runner works correctly with real LLM calls.
They require:
- The --e2e flag to run
- ANTHROPIC_API_KEY environment variable
- LangGraph installed (pip install langgraph langchain-anthropic)
"""

from __future__ import annotations

from typing import Any

import pytest

# Check if LangGraph is available
try:
    import langgraph  # noqa: F401

    LANGGRAPH_AVAILABLE = True
except ImportError:
    LANGGRAPH_AVAILABLE = False

# Skip all tests if LangGraph is not installed
pytestmark = [
    pytest.mark.e2e,
    pytest.mark.langgraph,
    pytest.mark.skipif(not LANGGRAPH_AVAILABLE, reason="langgraph not installed"),
]


class TestLangGraphReActAgent:
    """Test ReAct agent execution with LangGraph."""

    def test_react_agent_execution(
        self,
        check_api_key: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test simple ReAct agent execution.

        Args:
            check_api_key: Fixture to check for required API keys.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        runner = LangGraphRunner()

        # Build the agent/crew
        agent = runner.build_crew(simple_crew_config)
        assert agent is not None

        # Run with simple input
        inputs = {"input": "What is 3 + 5?"}
        result = runner.run(agent, inputs)

        # Verify we got a response
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain '8' in the answer
        assert "8" in result

    def test_build_and_run(
        self,
        check_api_key: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test the convenience build_and_run method.

        Args:
            check_api_key: Fixture to check for required API keys.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        runner = LangGraphRunner()

        # Use build_and_run convenience method
        inputs = {"input": "What is the largest ocean on Earth?"}
        result = runner.build_and_run(simple_crew_config, inputs)

        # Verify we got a response about Pacific Ocean
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        assert "Pacific" in result or "pacific" in result.lower()


class TestLangGraphFlow:
    """Test graph flow execution with LangGraph."""

    def test_graph_flow(
        self,
        check_api_key: None,
        simple_agent_config: dict[str, Any],
    ) -> None:
        """Test multi-step graph execution with state passing.

        Args:
            check_api_key: Fixture to check for required API keys.
            simple_agent_config: Simple agent configuration fixture.
        """
        from agentic_crew.runners.langgraph_runner import LangGraphRunner
        from langgraph.prebuilt import create_react_agent

        runner = LangGraphRunner()

        # Build agent with tools
        llm = runner.get_llm()
        agent = create_react_agent(llm, [])
        assert agent is not None

        # Run the agent with a task that requires reasoning
        result = agent.invoke(
            {
                "messages": [
                    (
                        "user",
                        "If a train travels 60 miles in 1 hour, how far does it travel in 30 minutes?",
                    )
                ]
            }
        )

        # Verify we got a result with messages
        assert result is not None
        assert "messages" in result
        assert len(result["messages"]) > 0

        # Extract the final message
        final_message = result["messages"][-1]
        content = final_message.content if hasattr(final_message, "content") else str(final_message)

        # Should contain '30' miles
        assert "30" in content


class TestLangGraphMultiStepPrompt:
    """Test multi-step prompt handling with LangGraph."""

    def test_agent_handles_multi_step_prompt(
        self,
        check_api_key: None,
        multi_agent_crew_config: dict[str, Any],
    ) -> None:
        """Test that the LangGraph agent can handle a multi-step prompt.

        This test verifies that the created ReAct agent can process a prompt
        with multiple sequential instructions.

        Args:
            check_api_key: Fixture to check for required API keys.
            multi_agent_crew_config: Multi-agent crew configuration fixture.
        """
        from agentic_crew.runners.langgraph_runner import LangGraphRunner

        runner = LangGraphRunner()

        # Build a crew from config
        crew = runner.build_crew(multi_agent_crew_config)
        assert crew is not None

        # Execute with input that requires multi-step reasoning with distinct answers
        inputs = {"input": "First, tell me what 5 + 3 equals, then tell me what 10 - 3 equals."}
        result = runner.run(crew, inputs)

        # Verify the runner properly managed state through the workflow
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain both answers
        assert "8" in result
        assert "7" in result


class TestLangGraphTools:
    """Test tool usage with LangGraph."""

    def test_agent_with_tools(
        self,
        check_api_key: None,
    ) -> None:
        """Test LangGraph agent with custom tools.

        Args:
            check_api_key: Fixture to check for required API keys.
        """
        from agentic_crew.runners.langgraph_runner import LangGraphRunner
        from langchain_core.tools import tool
        from langgraph.prebuilt import create_react_agent

        # Create a test tool
        @tool
        def get_magic_number() -> int:
            """Get the magic number.

            Returns:
                The magic number (always 42).
            """
            return 42

        runner = LangGraphRunner()
        llm = runner.get_llm()

        # Create agent with tool
        agent = create_react_agent(llm, [get_magic_number])

        # Run agent that should use the tool
        result = agent.invoke({"messages": [("user", "What is the magic number? Use the get_magic_number tool.")]})

        # Verify we got a result
        assert result is not None
        assert "messages" in result

        # Extract final message
        final_message = result["messages"][-1]
        content = final_message.content if hasattr(final_message, "content") else str(final_message)

        # Should mention 42
        assert "42" in content
