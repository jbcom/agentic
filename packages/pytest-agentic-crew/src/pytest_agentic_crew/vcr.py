"""VCR.py integration for recording/replaying LLM API calls.

This module provides pytest fixtures for recording HTTP interactions with
LLM APIs (Anthropic, OpenAI, Bedrock) for deterministic test replay.

Usage:
    @pytest.mark.vcr
    def test_llm_call(vcr_cassette):
        # First run: records to cassettes/test_llm_call.yaml
        # Subsequent runs: replays from cassette
        result = call_llm("Hello")
        assert "Hello" in result

Command line options:
    --vcr-record=MODE: Set recording mode (once, new_episodes, none, all)
    --disable-vcr: Disable VCR, make real API calls
"""

from __future__ import annotations

import os
from pathlib import Path
from typing import Any

import pytest

try:
    from vcr import VCR

    HAS_VCR = True
except ImportError:
    HAS_VCR = False
    VCR = None  # type: ignore[misc, assignment]


# Headers containing sensitive data to filter
SENSITIVE_HEADERS = [
    "authorization",
    "x-api-key",
    "anthropic-api-key",
    "openai-api-key",
    "api-key",
]

# Request body keys containing sensitive data
SENSITIVE_BODY_KEYS = [
    "api_key",
    "apiKey",
]


def _filter_headers(headers: dict[str, Any]) -> dict[str, Any]:
    """Filter sensitive headers from recorded requests/responses."""
    filtered = {}
    for key, value in headers.items():
        if key.lower() in SENSITIVE_HEADERS:
            filtered[key] = "[FILTERED]"
        else:
            filtered[key] = value
    return filtered


def _before_record_request(request: Any) -> Any:
    """Filter sensitive data from request before recording."""
    # Filter headers
    if hasattr(request, "headers"):
        request.headers = _filter_headers(dict(request.headers))
    return request


def _before_record_response(response: dict[str, Any]) -> dict[str, Any]:
    """Filter sensitive data from response before recording."""
    # Filter response headers
    if "headers" in response:
        response["headers"] = _filter_headers(response["headers"])
    return response


def pytest_addoption(parser: Any) -> None:
    """Add VCR-related command line options."""
    group = parser.getgroup("vcr")
    group.addoption(
        "--vcr-record",
        action="store",
        dest="vcr_record",
        default=None,
        choices=["once", "new_episodes", "none", "all"],
        help="Set the recording mode for VCR.py cassettes",
    )
    group.addoption(
        "--disable-vcr",
        action="store_true",
        dest="disable_vcr",
        help="Disable VCR, make real API calls (same as --vcr-record=new_episodes with no storage)",
    )


@pytest.fixture(autouse=True)
def _vcr_marker(request: pytest.FixtureRequest) -> None:
    """Auto-trigger vcr_cassette fixture when @pytest.mark.vcr is used."""
    if not HAS_VCR:
        return
    marker = request.node.get_closest_marker("vcr")
    if marker:
        request.getfixturevalue("vcr_cassette")


@pytest.fixture(scope="module")
def vcr_config() -> dict[str, Any]:
    """VCR configuration for LLM API recording.

    Override in conftest.py to customize:

        @pytest.fixture(scope='module')
        def vcr_config():
            return {
                'record_mode': 'none',
                'match_on': ['method', 'uri'],
            }
    """
    return {
        # Match on method, scheme, host, port, path, query, and body
        "match_on": ["method", "scheme", "host", "port", "path", "query", "body"],
        # Filter sensitive headers
        "before_record_request": _before_record_request,
        "before_record_response": _before_record_response,
        # Decode compressed responses for readability
        "decode_compressed_response": True,
        # Record once by default (first run records, subsequent runs replay)
        "record_mode": "once",
    }


@pytest.fixture(scope="module")
def vcr_cassette_dir(request: pytest.FixtureRequest) -> str:
    """Directory for VCR cassette files.

    Default: tests/cassettes/<module_name>/
    """
    test_file = Path(request.node.fspath)  # type: ignore[arg-type]
    return str(test_file.parent / "cassettes" / test_file.stem)


@pytest.fixture
def vcr_cassette_name(request: pytest.FixtureRequest) -> str:
    """Name of the VCR cassette file (without extension)."""
    test_class = request.cls
    if test_class:
        return f"{test_class.__name__}.{request.node.name}"
    return request.node.name


@pytest.fixture(scope="module")
def vcr(request: pytest.FixtureRequest, vcr_config: dict[str, Any], vcr_cassette_dir: str) -> Any:
    """The VCR instance configured for LLM API recording."""
    if not HAS_VCR:
        pytest.skip("vcrpy not installed")

    kwargs: dict[str, Any] = {
        "cassette_library_dir": vcr_cassette_dir,
        "path_transformer": VCR.ensure_suffix(".yaml"),
    }
    kwargs.update(vcr_config)

    # Apply CLI options
    record_mode = request.config.getoption("--vcr-record")
    if record_mode:
        kwargs["record_mode"] = record_mode

    if request.config.getoption("--disable-vcr"):
        # Disable recording and playback
        kwargs["record_mode"] = "new_episodes"
        kwargs["before_record_response"] = lambda *args, **kwargs: None

    # Create cassette directory if needed
    os.makedirs(vcr_cassette_dir, exist_ok=True)

    return VCR(**kwargs)


@pytest.fixture
def vcr_cassette(
    request: pytest.FixtureRequest,
    vcr: Any,
    vcr_cassette_name: str,
) -> Any:
    """Context manager for VCR cassette recording/playback.

    Usage:
        def test_api_call(vcr_cassette):
            # Requests are recorded/replayed
            response = requests.get("https://api.example.com")
    """
    kwargs: dict[str, Any] = {}

    # Allow marker to override settings
    marker = request.node.get_closest_marker("vcr")
    if marker:
        kwargs.update(marker.kwargs)

    # CLI overrides marker
    record_mode = request.config.getoption("--vcr-record")
    if record_mode:
        kwargs["record_mode"] = record_mode

    if request.config.getoption("--disable-vcr"):
        kwargs["record_mode"] = "new_episodes"
        kwargs["before_record_response"] = lambda *args, **kwargs: None

    with vcr.use_cassette(vcr_cassette_name, **kwargs) as cassette:
        yield cassette
