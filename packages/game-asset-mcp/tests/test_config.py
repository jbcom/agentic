"""Basic smoke tests for game-asset-mcp config and catalog."""

import pytest
from pathlib import Path
from unittest.mock import patch

from game_asset_mcp.config import Settings, TaxonomyConfig, reset_settings


def test_settings_defaults():
    s = Settings()
    assert s.assets_root == Path.home() / "assets"
    assert "game-asset-mcp" in str(s.catalog_db)


def test_settings_env_override(monkeypatch):
    monkeypatch.setenv("GAME_ASSET_ASSETS_ROOT", "/tmp/test-assets")
    reset_settings()
    s = Settings()
    assert s.assets_root == Path("/tmp/test-assets")
    reset_settings()


def test_taxonomy_defaults():
    t = TaxonomyConfig()
    assert "3DLowPoly" in t.style_map
    assert "3DPSX" in t.style_map
    assert "kenney" in t.source_hints
    assert "_Archive" in t.skip_dirs


def test_taxonomy_custom():
    t = TaxonomyConfig(
        style_map={"MyStyle": "MyStyle"},
        source_hints={"mycreator": "MyCreator"},
    )
    assert "MyStyle" in t.style_map
    assert "3DLowPoly" not in t.style_map
    assert "mycreator" in t.source_hints


def test_server_imports():
    """Verify server module imports without error."""
    from game_asset_mcp.server import mcp
    assert mcp is not None


def test_ingest_options_defaults():
    from game_asset_mcp.ingest import IngestOptions
    opts = IngestOptions()
    assert opts.force is False
    assert opts.dry_run is False
    assert opts.verbose is False
