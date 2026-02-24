"""Tests for framework auto-detection edge cases in the decomposer module."""

from __future__ import annotations

from unittest.mock import patch

import pytest
from agentic_crew.core.decomposer import (
    _framework_cache,
    _get_install_command,
    detect_framework,
    get_available_frameworks,
    is_framework_available,
)


class TestIsFrameworkAvailable:
    """Edge cases for is_framework_available."""

    def setup_method(self):
        """Clear the framework cache before each test."""
        _framework_cache.clear()

    def test_unsupported_framework_returns_false(self) -> None:
        """An unsupported framework name should return False immediately."""
        result = is_framework_available("pytorch")
        assert result is False

    def test_unsupported_framework_is_cached_as_false(self) -> None:
        """Unsupported framework names should still be cached."""
        is_framework_available("unsupported_thing")
        assert "unsupported_thing" in _framework_cache
        assert _framework_cache["unsupported_thing"] is False

    def test_import_failure_returns_false(self) -> None:
        """When importlib.import_module raises ImportError, return False."""
        with patch("agentic_crew.core.decomposer.importlib.import_module", side_effect=ImportError("nope")):
            result = is_framework_available("crewai")
        assert result is False

    def test_successful_import_returns_true_and_caches(self) -> None:
        """A successful import should cache True."""
        with patch("agentic_crew.core.decomposer.importlib.import_module"):
            result = is_framework_available("crewai")
        assert result is True
        assert _framework_cache["crewai"] is True

    def test_cache_hit_does_not_reimport(self) -> None:
        """Once cached, no re-import should occur."""
        _framework_cache["strands"] = True
        with patch("agentic_crew.core.decomposer.importlib.import_module") as mock_import:
            result = is_framework_available("strands")
        assert result is True
        mock_import.assert_not_called()

    def test_empty_string_framework_returns_false(self) -> None:
        """Empty string framework name should return False."""
        result = is_framework_available("")
        assert result is False


class TestDetectFramework:
    """Edge cases for detect_framework."""

    def test_preferred_auto_falls_through_to_detection(self) -> None:
        """preferred='auto' should be treated as no preference."""

        def mock_available(framework):
            return framework == "strands"

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = detect_framework(preferred="auto")

        assert result == "strands"

    def test_preferred_unavailable_falls_back_gracefully(self) -> None:
        """When preferred framework is unavailable, fall back to auto-detect."""

        def mock_available(framework):
            return framework == "crewai"

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = detect_framework(preferred="strands")

        assert result == "crewai"

    def test_preferred_none_auto_detects(self) -> None:
        """preferred=None should auto-detect."""

        def mock_available(framework):
            return framework == "langgraph"

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = detect_framework(preferred=None)

        assert result == "langgraph"

    def test_crewai_has_highest_priority(self) -> None:
        """When all frameworks available, crewai should win."""
        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            return_value=True,
        ):
            result = detect_framework()

        assert result == "crewai"

    def test_error_message_lists_install_options(self) -> None:
        """RuntimeError should include installation instructions."""
        with (
            patch("agentic_crew.core.decomposer.is_framework_available", return_value=False),
            pytest.raises(RuntimeError, match="pip install crewai"),
        ):
            detect_framework()


class TestGetAvailableFrameworks:
    """Edge cases for get_available_frameworks."""

    def test_returns_only_available(self) -> None:
        """Should only include frameworks that are importable."""

        def mock_available(framework):
            return framework == "strands"

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = get_available_frameworks()

        assert result == ["strands"]

    def test_preserves_priority_order(self) -> None:
        """Returned list should preserve priority order."""

        def mock_available(framework):
            return framework in ["strands", "crewai"]

        with patch(
            "agentic_crew.core.decomposer.is_framework_available",
            side_effect=mock_available,
        ):
            result = get_available_frameworks()

        assert result == ["crewai", "strands"]


class TestGetInstallCommand:
    """Tests for _get_install_command."""

    def test_crewai_includes_tools(self) -> None:
        assert "crewai[tools]" in _get_install_command("crewai")

    def test_langgraph_includes_langchain(self) -> None:
        result = _get_install_command("langgraph")
        assert "langgraph" in result
        assert "langchain" in result

    def test_strands_maps_correctly(self) -> None:
        assert "strands-agents" in _get_install_command("strands")

    def test_unknown_framework_returns_itself(self) -> None:
        """Unknown framework name should return as-is."""
        result = _get_install_command("unknown_framework")
        assert result == "unknown_framework"
