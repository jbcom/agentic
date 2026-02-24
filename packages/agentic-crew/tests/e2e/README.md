# End-to-End Tests

This directory contains end-to-end tests for framework runners (CrewAI, LangGraph, Strands).

## Overview

E2E tests verify that each framework runner works correctly with:
- Real LLM API calls
- Complete crew execution flows
- Multi-agent collaboration
- Knowledge source integration
- Tool usage

## Running E2E Tests

E2E tests are **disabled by default** to avoid:
- Unexpected API costs
- Network dependency in CI/CD
- Slow test execution

### Run all E2E tests

```bash
# Requires ANTHROPIC_API_KEY environment variable
export ANTHROPIC_API_KEY="your-api-key"
uv run pytest tests/e2e/ --e2e -v
```

### Run framework-specific tests

```bash
# Only CrewAI tests
uv run pytest tests/e2e/ --e2e --framework=crewai -v

# Only LangGraph tests (requires langgraph installed)
uv run pytest tests/e2e/ --e2e --framework=langgraph -v

# Only Strands tests (requires strands-agents installed)
uv run pytest tests/e2e/ --e2e --framework=strands -v
```

### Run specific test

```bash
uv run pytest tests/e2e/test_crewai_e2e.py::TestCrewAISimpleExecution::test_simple_crew_execution --e2e -v
```

## Environment Variables

### Required

- `ANTHROPIC_API_KEY` - For Claude LLM (CrewAI, LangGraph)

### Optional (for Strands with AWS Bedrock)

- `AWS_ACCESS_KEY_ID` - AWS credentials
- `AWS_SECRET_ACCESS_KEY` - AWS credentials
- `AWS_REGION` - AWS region (e.g., `us-west-2`)

#### Bedrock Model IDs

When configuring Strands tests with Bedrock, use the official model IDs from [Claude on Amazon Bedrock](https://platform.claude.com/docs/en/build-with-claude/claude-on-amazon-bedrock):

| Model | Bedrock Model ID |
|-------|------------------|
| Claude Haiku 4.5 ⭐ | `anthropic.claude-haiku-4-5-20251001-v1:0` |
| Claude Sonnet 4.5 | `anthropic.claude-sonnet-4-5-20250929-v1:0` |
| Claude Sonnet 4 | `anthropic.claude-sonnet-4-20250514-v1:0` |

**Recommended**: Use Claude Haiku 4.5 for E2E tests - it's fast, cost-effective, and capable.

Example configuration:
```python
crew_config = {
    "llm": {
        "provider": "bedrock",
        "model": "anthropic.claude-haiku-4-5-20251001-v1:0",
    },
    # ... rest of config
}
```

## Test Structure

```
tests/e2e/
├── conftest.py              # E2E fixtures and pytest configuration
├── test_crewai_e2e.py       # CrewAI runner E2E tests
├── test_langgraph_e2e.py    # LangGraph runner E2E tests
└── test_strands_e2e.py      # Strands runner E2E tests
```

## Test Coverage

### CrewAI (`test_crewai_e2e.py`)

- ✅ Simple single-agent crew execution
- ✅ Build and run convenience method
- ✅ Multi-agent crew collaboration
- ✅ Knowledge source integration
- ✅ Tool usage with agents

### LangGraph (`test_langgraph_e2e.py`)

- ✅ ReAct agent execution
- ✅ Build and run method
- ✅ Multi-step graph execution
- ✅ State management between nodes
- ✅ Tool usage with agents

### Strands (`test_strands_e2e.py`)

- ✅ Simple agent execution
- ✅ Build and run method
- ✅ Agent building from configuration
- ✅ Task building
- ✅ System prompt generation
- ✅ AWS Bedrock integration (if credentials available)
- ✅ Tool usage (basic)

## CI/CD Integration

E2E tests are **excluded from regular CI runs** in `.github/workflows/ci.yml`:

```yaml
- name: Run tests
  run: uv run pytest tests -v --ignore=tests/e2e
```

To run E2E tests in CI (e.g., on a schedule):

```yaml
# .github/workflows/e2e.yml
name: E2E Tests
on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday
  workflow_dispatch:

jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: hynek/setup-cached-uv@v2
      - run: uv sync --extra tests --extra crewai --extra langgraph --extra strands
      - run: uv run pytest tests/e2e/ --e2e -v
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
```

## Writing New E2E Tests

1. Mark tests with `@pytest.mark.e2e` and framework marker
2. Use `check_api_key` fixture to verify required credentials
3. Keep tests focused and minimal to reduce API costs
4. Use fixtures from `conftest.py` for common configurations

Note: Tests marked with `@pytest.mark.e2e` are automatically skipped unless the `--e2e` flag is provided.

Example:

```python
@pytest.mark.e2e
@pytest.mark.crewai
def test_my_feature(
    check_api_key: None,
    simple_crew_config: dict[str, Any],
) -> None:
    """Test my new feature."""
    from agentic_crew.runners.crewai_runner import CrewAIRunner
    
    runner = CrewAIRunner()
    result = runner.build_and_run(simple_crew_config, {"input": "test"})
    assert result is not None
```

## Debugging

If tests fail:

1. Check API key is set: `echo $ANTHROPIC_API_KEY`
2. Verify framework is installed: `pip list | grep crewai`
3. Run with verbose output: `-vv`
4. Run single test to isolate issue
5. Check network connectivity to LLM APIs

## Cost Considerations

E2E tests make real API calls which incur costs:

- **CrewAI tests**: ~5-10 tests × ~$0.01-0.05 per test = ~$0.05-0.50
- **LangGraph tests**: ~5 tests × ~$0.01-0.05 per test = ~$0.05-0.25
- **Strands tests**: ~6-7 tests × $0.01-0.05 per test = ~$0.06-0.35

**Total estimated cost per full E2E run**: ~$0.16-1.10

To minimize costs:
- Run E2E tests only when needed
- Use `--framework` filter for specific tests
- Run on schedule instead of per-PR
- Use test timeouts to prevent runaway costs
