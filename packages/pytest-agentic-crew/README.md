# pytest-agentic-crew

[![PyPI version](https://img.shields.io/pypi/v/pytest-agentic-crew.svg)](https://pypi.org/project/pytest-agentic-crew/)
[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python](https://img.shields.io/pypi/pyversions/pytest-agentic-crew.svg)](https://pypi.org/project/pytest-agentic-crew/)

Pytest plugin with fixtures for end-to-end testing of `agentic-crew` projects. Provides VCR.py integration for recording and replaying LLM API calls, framework mocking fixtures for CrewAI/LangGraph/Strands, crew configuration helpers, and CLI options for filtering tests by framework.

[Full Documentation](https://agentic.coach) | [Package Docs](https://agentic.coach/packages/crew/)

## Installation

```bash
# Basic installation
pip install pytest-agentic-crew

# With framework support
pip install pytest-agentic-crew[crewai]
pip install pytest-agentic-crew[langgraph]
pip install pytest-agentic-crew[strands]
pip install pytest-agentic-crew[all]
```

## Quick Start

Fixtures are automatically available when the package is installed (registered via `pytest11` entry point):

```python
import pytest

@pytest.mark.e2e
@pytest.mark.crewai
def test_my_crew(check_api_key, simple_crew_config):
    from agentic_crew.runners.crewai_runner import CrewAIRunner

    runner = CrewAIRunner()
    crew = runner.build_crew(simple_crew_config)
    result = runner.run(crew, {"input": "Hello"})
    assert result is not None
```

### VCR Cassette Recording

Record and replay LLM API calls for deterministic, offline tests:

```python
@pytest.mark.vcr
def test_llm_call():
    # First run: records HTTP interactions to cassettes/test_llm_call.yaml
    # Subsequent runs: replays from cassette (no real API calls)
    response = call_anthropic_api("Hello")
    assert "Hello" in response
```

API keys are automatically filtered from recordings.

## CLI Options

```bash
pytest --e2e                        # Enable E2E tests (disabled by default)
pytest --e2e --framework=crewai     # Run only CrewAI tests
pytest --vcr-record=once            # Record once, replay thereafter (default)
pytest --vcr-record=none            # Playback only
pytest --disable-vcr                # Disable VCR, make real API calls
```

## Available Fixtures

| Fixture | Description |
|---------|-------------|
| `check_api_key` | Skips test if `ANTHROPIC_API_KEY` not set |
| `check_aws_credentials` | Skips test if AWS credentials not configured |
| `simple_crew_config` | Single agent/task crew configuration |
| `multi_agent_crew_config` | Multi-agent crew with researcher and writer |
| `crew_with_knowledge` | Crew with knowledge sources |
| `temp_crew_dir` | Temporary `.crewai` directory |
| `simple_agent_config` | Basic agent configuration dict |
| `simple_task_config` | Basic task configuration dict |

## Model Helpers

```python
from pytest_agentic_crew import get_anthropic_model, get_bedrock_model

model = get_anthropic_model("sonnet-4")   # claude-sonnet-4-20250514
bedrock = get_bedrock_model("opus-4")     # anthropic.claude-opus-4-20250514-v1:0
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `ANTHROPIC_API_KEY` | For direct API | Claude API key |
| `AWS_ACCESS_KEY_ID` | For Bedrock | AWS credentials |
| `AWS_PROFILE` | For Bedrock | Alternative to access keys |

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation.

## License

MIT
