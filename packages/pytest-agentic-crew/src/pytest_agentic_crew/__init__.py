"""Pytest plugin with fixtures for agentic-crew E2E testing.

This package provides pytest fixtures and CLI options for building
E2E tests with agentic-crew framework runners (CrewAI, LangGraph, Strands).

Installation:
    pip install pytest-agentic-crew[all]

Usage:
    Fixtures are automatically available when the package is installed:

    ```python
    import pytest

    @pytest.mark.e2e
    @pytest.mark.crewai
    def test_my_crew(check_api_key, simple_crew_config):
        from agentic_crew.runners.crewai_runner import CrewAIRunner

        runner = CrewAIRunner()
        result = runner.build_and_run(simple_crew_config, {"input": "test"})
        assert result is not None
    ```

For unit testing with mocked frameworks:
    ```python
    def test_runner_behavior(crew_mocker):
        # Mock all frameworks
        crew_mocker.mock_all_frameworks()

        # Or mock specific framework
        crew_mocker.mock_crewai()

        # Patch specific components
        mock_agent = crew_mocker.patch_crewai_agent()
        mock_llm = crew_mocker.patch_get_llm()

        # Test your code
        from agentic_crew.runners.crewai_runner import CrewAIRunner
        runner = CrewAIRunner()
        ...
    ```

Available fixtures:
    E2E Testing:
    - check_api_key: Skips test if ANTHROPIC_API_KEY not set
    - check_aws_credentials: Skips test if AWS credentials not configured
    - simple_agent_config: Basic agent configuration dict
    - simple_task_config: Basic task configuration dict
    - simple_crew_config: Single agent/task crew configuration
    - multi_agent_crew_config: Multi-agent crew configuration
    - crew_with_knowledge: Crew with knowledge sources
    - temp_crew_dir: Temporary crew directory for testing

    Unit Testing (Mocking):
    - crew_mocker: CrewMocker instance for comprehensive mocking
    - mock_frameworks: Mock all AI framework modules
    - mock_crewai: Mock only CrewAI modules
    - mock_langgraph: Mock only LangGraph modules
    - mock_strands: Mock only Strands modules

Command line options:
    --e2e: Enable E2E tests (disabled by default)
    --framework=<name>: Filter tests by framework (crewai, langgraph, strands)
"""

from __future__ import annotations

from pytest_agentic_crew.mocking import (
    ALL_FRAMEWORK_MODULES,
    CREWAI_MODULES,
    LANGGRAPH_MODULES,
    STRANDS_MODULES,
    CrewMocker,
)
from pytest_agentic_crew.models import (
    ANTHROPIC_MODELS,
    BEDROCK_MODELS,
    DEFAULT_BEDROCK_MODEL,
    DEFAULT_MODEL,
    get_anthropic_model,
    get_bedrock_model,
)
from pytest_agentic_crew.vcr import (
    SENSITIVE_BODY_KEYS,
    SENSITIVE_HEADERS,
)

__version__ = "0.1.0"
__all__ = [
    # Mocking
    "CrewMocker",
    "CREWAI_MODULES",
    "LANGGRAPH_MODULES",
    "STRANDS_MODULES",
    "ALL_FRAMEWORK_MODULES",
    # Model helpers
    "DEFAULT_MODEL",
    "DEFAULT_BEDROCK_MODEL",
    "ANTHROPIC_MODELS",
    "BEDROCK_MODELS",
    "get_anthropic_model",
    "get_bedrock_model",
    # VCR configuration
    "SENSITIVE_HEADERS",
    "SENSITIVE_BODY_KEYS",
]
