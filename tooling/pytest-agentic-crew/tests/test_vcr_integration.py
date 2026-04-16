"""Tests for VCR integration basics."""

from __future__ import annotations

from typing import Any

from pytest_agentic_crew.vcr import (
    HAS_VCR,
    SENSITIVE_BODY_KEYS,
    SENSITIVE_HEADERS,
    _before_record_request,
    _before_record_response,
    _filter_headers,
)


class TestSensitiveHeaderFiltering:
    """Test that sensitive headers are properly filtered."""

    def test_authorization_header_filtered(self) -> None:
        """Authorization header should be replaced with [FILTERED]."""
        headers = {"authorization": "Bearer sk-secret-key-123", "content-type": "application/json"}
        filtered = _filter_headers(headers)
        assert filtered["authorization"] == "[FILTERED]"
        assert filtered["content-type"] == "application/json"

    def test_api_key_headers_filtered(self) -> None:
        """API key headers should be filtered."""
        headers = {
            "x-api-key": "secret-api-key",
            "anthropic-api-key": "sk-ant-secret",
            "openai-api-key": "sk-openai-secret",
        }
        filtered = _filter_headers(headers)
        for key in headers:
            assert filtered[key] == "[FILTERED]"

    def test_non_sensitive_headers_preserved(self) -> None:
        """Non-sensitive headers should pass through unchanged."""
        headers = {
            "content-type": "application/json",
            "accept": "text/html",
            "user-agent": "test-client/1.0",
        }
        filtered = _filter_headers(headers)
        assert filtered == headers

    def test_case_insensitive_filtering(self) -> None:
        """Header filtering should be case-insensitive."""
        headers = {"Authorization": "Bearer secret"}
        # The current implementation lowercases for comparison
        filtered = _filter_headers(headers)
        # Note: the key case is preserved, but the value is filtered
        # because "Authorization".lower() == "authorization" which is in SENSITIVE_HEADERS
        assert filtered.get("Authorization") == "[FILTERED]"

    def test_empty_headers(self) -> None:
        """Empty headers dict should return empty dict."""
        assert _filter_headers({}) == {}


class TestBeforeRecordCallbacks:
    """Test VCR before-record callbacks."""

    def test_before_record_request_filters_headers(self) -> None:
        """Request callback should filter sensitive headers."""

        class MockRequest:
            headers = {"authorization": "Bearer secret", "content-type": "application/json"}

        request = MockRequest()
        result = _before_record_request(request)
        assert result.headers["authorization"] == "[FILTERED]"
        assert result.headers["content-type"] == "application/json"

    def test_before_record_request_handles_no_headers(self) -> None:
        """Request without headers attribute should pass through."""

        class MockRequest:
            pass

        request = MockRequest()
        result = _before_record_request(request)
        assert result is request

    def test_before_record_response_filters_headers(self) -> None:
        """Response callback should filter sensitive headers."""
        response = {
            "headers": {"x-api-key": "secret", "content-length": "42"},
            "body": "response body",
        }
        result = _before_record_response(response)
        assert result["headers"]["x-api-key"] == "[FILTERED]"
        assert result["headers"]["content-length"] == "42"
        assert result["body"] == "response body"

    def test_before_record_response_no_headers(self) -> None:
        """Response without headers should pass through."""
        response = {"body": "response body"}
        result = _before_record_response(response)
        assert result == {"body": "response body"}


class TestVCRConstants:
    """Test VCR-related constants."""

    def test_sensitive_headers_list(self) -> None:
        """SENSITIVE_HEADERS should include common auth headers."""
        assert "authorization" in SENSITIVE_HEADERS
        assert "x-api-key" in SENSITIVE_HEADERS
        assert "anthropic-api-key" in SENSITIVE_HEADERS

    def test_sensitive_body_keys_list(self) -> None:
        """SENSITIVE_BODY_KEYS should include common API key fields."""
        assert "api_key" in SENSITIVE_BODY_KEYS
        assert "apiKey" in SENSITIVE_BODY_KEYS

    def test_has_vcr_is_boolean(self) -> None:
        """HAS_VCR should be a boolean."""
        assert isinstance(HAS_VCR, bool)


class TestVCRFixtures:
    """Test VCR fixture availability."""

    def test_vcr_config_fixture(self, vcr_config: dict[str, Any]) -> None:
        """vcr_config should return a dict with expected keys."""
        assert isinstance(vcr_config, dict)
        assert "match_on" in vcr_config
        assert "record_mode" in vcr_config
        assert isinstance(vcr_config["match_on"], list)

    def test_vcr_cassette_dir_fixture(self, vcr_cassette_dir: str) -> None:
        """vcr_cassette_dir should return a string path."""
        assert isinstance(vcr_cassette_dir, str)
        assert "cassettes" in vcr_cassette_dir

    def test_vcr_cassette_name_fixture(self, vcr_cassette_name: str) -> None:
        """vcr_cassette_name should return the test name."""
        assert isinstance(vcr_cassette_name, str)
        assert "test_vcr_cassette_name_fixture" in vcr_cassette_name
