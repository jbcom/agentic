"""Tests for the models module."""

from __future__ import annotations

import pytest
from pytest_agentic_crew.models import (
    ANTHROPIC_MODELS,
    BEDROCK_MODELS,
    DEFAULT_BEDROCK_MODEL,
    DEFAULT_MODEL,
    get_anthropic_model,
    get_bedrock_model,
)


class TestAnthropicModels:
    """Test Anthropic model mappings."""

    def test_default_model_is_haiku(self) -> None:
        """Default model should be Haiku 4.5 for cost-effectiveness."""
        assert "haiku" in DEFAULT_MODEL

    def test_get_anthropic_model_default(self) -> None:
        """get_anthropic_model() without args should return default."""
        result = get_anthropic_model()
        assert result == ANTHROPIC_MODELS["haiku-4.5"]

    def test_get_anthropic_model_sonnet(self) -> None:
        """get_anthropic_model('sonnet-4') should return Sonnet 4."""
        result = get_anthropic_model("sonnet-4")
        assert "sonnet-4" in result

    def test_get_anthropic_model_opus(self) -> None:
        """get_anthropic_model('opus-4') should return Opus 4."""
        result = get_anthropic_model("opus-4")
        assert "opus-4" in result

    def test_get_anthropic_model_invalid(self) -> None:
        """Invalid model name should raise KeyError."""
        with pytest.raises(KeyError):
            get_anthropic_model("nonexistent-model")

    def test_all_models_are_strings(self) -> None:
        """All model identifiers should be strings."""
        for name, model_id in ANTHROPIC_MODELS.items():
            assert isinstance(model_id, str), f"Model {name} has non-string id"


class TestBedrockModels:
    """Test AWS Bedrock model mappings."""

    def test_default_bedrock_model_is_haiku(self) -> None:
        """Default Bedrock model should be Haiku 4.5."""
        assert "haiku" in DEFAULT_BEDROCK_MODEL

    def test_get_bedrock_model_default(self) -> None:
        """get_bedrock_model() without args should return default."""
        result = get_bedrock_model()
        assert result == BEDROCK_MODELS["haiku-4.5"]

    def test_bedrock_models_have_anthropic_prefix(self) -> None:
        """All Bedrock models should start with 'anthropic.'."""
        for name, model_id in BEDROCK_MODELS.items():
            assert model_id.startswith("anthropic."), f"Bedrock model {name} missing prefix"

    def test_get_bedrock_model_invalid(self) -> None:
        """Invalid model name should raise KeyError."""
        with pytest.raises(KeyError):
            get_bedrock_model("nonexistent-model")

    def test_model_keys_consistent(self) -> None:
        """ANTHROPIC_MODELS and BEDROCK_MODELS should have the same keys."""
        assert set(ANTHROPIC_MODELS.keys()) == set(BEDROCK_MODELS.keys())
