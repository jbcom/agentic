# Python Examples

This directory contains examples of how to use `agentic-crew` programmatically in Python.

## Setup

Ensure you have the package installed in your environment:

```bash
uv sync --extra dev --extra crewai
```

Or if you are developing locally:

```bash
pip install -e packages/agentic-crew
```

## Examples

### 1. Minimal Run (`minimal_run.py`)

Shows how to discover packages, load a crew configuration, and execute it using the best available framework.

```bash
python examples/python/minimal_run.py
```

### 2. Framework Detection

You can check which frameworks are available in your environment:

```python
from agentic_crew import get_available_frameworks, detect_framework

print(f"Available: {get_available_frameworks()}")
print(f"Preferred: {detect_framework()}")
```

## Requirements

Most crews require an LLM provider API key. For the default configurations, you'll need:

- `ANTHROPIC_API_KEY`: For Claude models (preferred)
- `OPENAI_API_KEY`: If using OpenAI models
- `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY`: If using AWS Strands with Bedrock
