"""Model configuration helpers for agentic-crew E2E tests.

Provides standardized model identifiers for testing across different providers.

See: https://platform.claude.com/docs/en/build-with-claude/claude-on-amazon-bedrock
"""

from __future__ import annotations

# Direct Anthropic API models
ANTHROPIC_MODELS = {
    "haiku-4.5": "claude-haiku-4-5-20251001",
    "sonnet-4.5": "claude-sonnet-4-5-20250929",
    "sonnet-4": "claude-sonnet-4-20250514",
    "opus-4": "claude-opus-4-20250514",
}

# AWS Bedrock model IDs
BEDROCK_MODELS = {
    "haiku-4.5": "anthropic.claude-haiku-4-5-20251001-v1:0",
    "sonnet-4.5": "anthropic.claude-sonnet-4-5-20250929-v1:0",
    "sonnet-4": "anthropic.claude-sonnet-4-20250514-v1:0",
    "opus-4": "anthropic.claude-opus-4-20250514-v1:0",
}

# Defaults - Haiku 4.5 is fast and cost-effective for testing
DEFAULT_MODEL = ANTHROPIC_MODELS["haiku-4.5"]
DEFAULT_BEDROCK_MODEL = BEDROCK_MODELS["haiku-4.5"]


def get_anthropic_model(name: str = "haiku-4.5") -> str:
    """Get Anthropic API model identifier.

    Args:
        name: Short model name (haiku-4.5, sonnet-4.5, sonnet-4, opus-4)

    Returns:
        Full model identifier for Anthropic API

    Example:
        >>> get_anthropic_model("haiku-4.5")
        'claude-haiku-4-5-20251001'
    """
    return ANTHROPIC_MODELS[name]


def get_bedrock_model(name: str = "haiku-4.5") -> str:
    """Get AWS Bedrock model identifier.

    Args:
        name: Short model name (haiku-4.5, sonnet-4.5, sonnet-4, opus-4)

    Returns:
        Full model identifier for AWS Bedrock

    Example:
        >>> get_bedrock_model("haiku-4.5")
        'anthropic.claude-haiku-4-5-20251001-v1:0'
    """
    return BEDROCK_MODELS[name]
