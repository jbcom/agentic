"""E2E test fixtures — require a real ASSETS_ROOT to be set."""
from __future__ import annotations

import os
from pathlib import Path

import pytest


def pytest_configure(config):
    """Register the e2e marker."""
    config.addinivalue_line(
        "markers",
        "e2e: end-to-end tests that require a real asset library (ASSETS_ROOT must be set)",
    )


@pytest.fixture(scope="session")
def real_assets_root() -> Path:
    """
    Return the real asset library root from ASSETS_ROOT env var.

    Skips the test session if ASSETS_ROOT is not set or doesn't exist.
    """
    root_str = os.environ.get("ASSETS_ROOT") or os.environ.get("GAME_ASSET_ASSETS_ROOT")
    if not root_str:
        pytest.skip("ASSETS_ROOT not set — skipping e2e tests")
    root = Path(root_str)
    if not root.exists():
        pytest.skip(f"ASSETS_ROOT does not exist: {root}")
    return root


@pytest.fixture(scope="session")
def e2e_db(tmp_path_factory: pytest.TempPathFactory) -> Path:
    """Provide a session-scoped temporary DB for e2e ingest tests."""
    db = tmp_path_factory.mktemp("e2e_db") / "e2e_catalog.db"
    return db
