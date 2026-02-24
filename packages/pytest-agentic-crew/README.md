# pytest-agentic-crew

Pytest plugin with fixtures for agentic-crew E2E testing.

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

## Usage

Fixtures are automatically available when the package is installed:

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

Record and replay LLM API calls for deterministic tests:

```python
@pytest.mark.vcr
def test_llm_call():
    # First run: records HTTP interactions to cassettes/test_llm_call.yaml
    # Subsequent runs: replays from cassette (no real API calls)
    response = call_anthropic_api("Hello")
    assert "Hello" in response
```

API keys are automatically filtered from recordings.

### Command Line Options

```bash
# Run E2E tests (disabled by default)
pytest --e2e

# Run only CrewAI tests
pytest --e2e --framework=crewai

# Run only LangGraph tests
pytest --e2e --framework=langgraph

# Run only Strands tests
pytest --e2e --framework=strands

# VCR recording modes
pytest --vcr-record=once       # Record once, replay thereafter (default)
pytest --vcr-record=none       # Never record, only playback
pytest --vcr-record=new_episodes  # Record new requests, replay existing
pytest --vcr-record=all        # Always record (overwrite cassettes)

# Disable VCR entirely (make real API calls)
pytest --disable-vcr
```

## Available Fixtures

| Fixture | Description |
|---------|-------------|
| `check_api_key` | Skips test if `ANTHROPIC_API_KEY` not set |
| `check_aws_credentials` | Skips test if AWS credentials not configured |
| `simple_agent_config` | Basic agent configuration dict |
| `simple_task_config` | Basic task configuration dict |
| `simple_crew_config` | Single agent/task crew configuration |
| `multi_agent_crew_config` | Multi-agent crew with researcher/writer |
| `crew_with_knowledge` | Crew with knowledge sources |
| `temp_crew_dir` | Temporary `.crewai` directory |

## Model Helpers

```python
from pytest_agentic_crew import (
    DEFAULT_MODEL,           # claude-haiku-4-5-20251001
    DEFAULT_BEDROCK_MODEL,   # anthropic.claude-haiku-4-5-20251001-v1:0
    get_anthropic_model,
    get_bedrock_model,
)

# Get model IDs by short name
model = get_anthropic_model("sonnet-4")  # claude-sonnet-4-20250514
bedrock = get_bedrock_model("opus-4")    # anthropic.claude-opus-4-20250514-v1:0
```

### Available Models

| Short Name | Anthropic API | Bedrock |
|------------|---------------|---------|
| `haiku-4.5` | `claude-haiku-4-5-20251001` | `anthropic.claude-haiku-4-5-20251001-v1:0` |
| `sonnet-4.5` | `claude-sonnet-4-5-20250929` | `anthropic.claude-sonnet-4-5-20250929-v1:0` |
| `sonnet-4` | `claude-sonnet-4-20250514` | `anthropic.claude-sonnet-4-20250514-v1:0` |
| `opus-4` | `claude-opus-4-20250514` | `anthropic.claude-opus-4-20250514-v1:0` |

See [Claude on Amazon Bedrock](https://platform.claude.com/docs/en/build-with-claude/claude-on-amazon-bedrock) for full model documentation.

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `ANTHROPIC_API_KEY` | For direct API | Claude API key |
| `AWS_ACCESS_KEY_ID` | For Bedrock | AWS credentials |
| `AWS_SECRET_ACCESS_KEY` | For Bedrock | AWS credentials |
| `AWS_PROFILE` | For Bedrock | Alternative to access keys |

## License

MIT
