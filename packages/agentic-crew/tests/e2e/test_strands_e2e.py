"""End-to-end tests for Strands runner.

These tests verify that the Strands runner works correctly with real LLM calls.
They require:
- The --e2e flag to run
- AWS credentials configured (for Bedrock integration)
- Strands installed (pip install strands-agents)

Note: Some tests may be skipped if AWS Bedrock is not configured.
"""

from __future__ import annotations

import os
from typing import Any

import pytest

# Check if Strands is available
try:
    import strands  # noqa: F401

    STRANDS_AVAILABLE = True
except ImportError:
    STRANDS_AVAILABLE = False

# Skip all tests if Strands is not installed
pytestmark = [
    pytest.mark.e2e,
    pytest.mark.strands,
    pytest.mark.skipif(not STRANDS_AVAILABLE, reason="strands not installed"),
]


class TestStrandsAgentExecution:
    """Test basic agent execution with Strands."""

    def test_strands_agent_execution(
        self,
        check_aws_credentials: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test simple Strands agent execution.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        # Build the agent from crew config
        agent = runner.build_crew(simple_crew_config)
        assert agent is not None

        # Run with simple input
        inputs = {"input": "What is 7 + 3?"}
        result = runner.run(agent, inputs)

        # Verify we got a response
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain '10' in the answer
        assert "10" in result

    def test_build_and_run(
        self,
        check_aws_credentials: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test the convenience build_and_run method.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        # Use build_and_run convenience method
        inputs = {"input": "What is the smallest prime number?"}
        result = runner.build_and_run(simple_crew_config, inputs)

        # Verify we got a response
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should mention 2 as the smallest prime
        assert "2" in result


class TestStrandsAgentBuilder:
    """Test Strands agent building from configuration."""

    def test_build_agent(
        self,
        check_aws_credentials: None,
        simple_agent_config: dict[str, Any],
    ) -> None:
        """Test building a Strands agent from configuration.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            simple_agent_config: Simple agent configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        # Build agent
        agent = runner.build_agent(simple_agent_config)
        assert agent is not None

        # Test the agent with a simple query
        result = agent("What is the capital of Italy?")
        assert result is not None
        result_str = str(result)
        assert "Rome" in result_str or "rome" in result_str.lower()

    def test_build_task(
        self,
        check_aws_credentials: None,
        simple_agent_config: dict[str, Any],
        simple_task_config: dict[str, Any],
    ) -> None:
        """Test building a task for Strands.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            simple_agent_config: Simple agent configuration fixture.
            simple_task_config: Simple task configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        # Build agent first
        agent = runner.build_agent(simple_agent_config)

        # Build task
        task = runner.build_task(simple_task_config, agent)
        assert task is not None
        assert isinstance(task, dict)
        assert "description" in task
        assert "expected_output" in task
        assert "agent" in task


class TestStrandsSystemPrompt:
    """Test Strands system prompt generation."""

    def test_system_prompt_generation(
        self,
        check_aws_credentials: None,
        multi_agent_crew_config: dict[str, Any],
    ) -> None:
        """Test that system prompts are generated correctly from crew config.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            multi_agent_crew_config: Multi-agent crew configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        runner = StrandsRunner()

        # Build system prompt (internal method, testing through crew build)
        agent = runner.build_crew(multi_agent_crew_config)
        assert agent is not None

        # Run the agent to verify the system prompt is working
        result = agent("Tell me about Python programming")
        assert result is not None
        result_str = str(result)
        assert len(result_str) > 0


class TestStrandsBedrockIntegration:
    """Test Strands with AWS Bedrock (optional)."""

    @pytest.mark.skipif(
        not os.environ.get("AWS_ACCESS_KEY_ID"),
        reason="AWS credentials not configured",
    )
    def test_bedrock_integration(
        self,
        check_aws_credentials: None,
        simple_crew_config: dict[str, Any],
    ) -> None:
        """Test Strands with AWS Bedrock model provider.

        This test is skipped if AWS credentials are not available.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            simple_crew_config: Simple crew configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        # Add Bedrock model configuration
        bedrock_config = {
            **simple_crew_config,
            "llm": {
                "provider": "bedrock",
                "model": "anthropic.claude-haiku-4-5-20251001-v1:0",
            },
        }

        runner = StrandsRunner()

        # Build and run with Bedrock
        inputs = {"input": "What is 5 * 6?"}
        result = runner.build_and_run(bedrock_config, inputs)

        # Verify we got a response
        assert result is not None
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain '30' in the answer
        assert "30" in result


class TestStrandsTools:
    """Test tool usage with Strands."""

    def test_agent_with_tools(
        self,
        check_aws_credentials: None,
        simple_agent_config: dict[str, Any],
    ) -> None:
        """Test Strands agent with custom tools.

        Args:
            check_aws_credentials: Fixture to check for required AWS credentials.
            simple_agent_config: Simple agent configuration fixture.
        """
        from agentic_crew.runners.strands_runner import StrandsRunner

        # Create a simple Python function as a tool
        def get_secret_number() -> int:
            """Get the secret number.

            Returns:
                The secret number (always 99).
            """
            return 99

        runner = StrandsRunner()

        # Build agent with tool
        agent = runner.build_agent(simple_agent_config, tools=[get_secret_number])
        assert agent is not None

        # Test execution with a prompt that encourages tool use
        result = agent("Use the get_secret_number tool to find the secret number.")
        assert result is not None
        result_str = str(result)
        # Should mention 99 if the tool was invoked
        assert "99" in result_str
