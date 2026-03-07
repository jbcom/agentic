"""Smoke tests for each MCP tool function in server.py.

These tests call the tool functions directly (without the MCP runtime).
The catalog DB is routed to a tmp_path DB via monkeypatching.
"""
from __future__ import annotations

import os
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

import game_asset_mcp.server as server_module
from game_asset_mcp.catalog import (
    get_connection,
    init_db,
    rebuild_fts,
    upsert_asset,
)
from game_asset_mcp.server import (
    browse_taxonomy,
    copy_asset,
    download_polyhaven_asset,
    generate_preview,
    get_asset_info,
    get_catalog_stats,
    get_preview,
    list_categories,
    run_ingest,
    search_assets,
    search_polyhaven,
    _format_asset,
)


# ─── Helpers ──────────────────────────────────────────────────────────────────


def _patch_db(monkeypatch, db_path: Path) -> None:
    """Redirect all server catalog calls to use db_path instead of the default."""
    from game_asset_mcp import catalog as cat_module

    def _get_conn(dp=None):
        return get_connection(db_path)

    monkeypatch.setattr(server_module, "get_connection", _get_conn)
    # Also patch catalog-level calls that accept no arguments from server.py
    monkeypatch.setattr(cat_module, "DB_PATH", db_path)


def _insert_sample(db_path: Path, **overrides) -> None:
    """Insert a single test asset record into db_path."""
    base = dict(
        path="/fake/3DLowPoly/Characters/hero.glb",
        name="hero",
        style="3DLowPoly",
        category="Characters/Animated",
        pack="kenney_pack",
        source="Kenney",
        meshes=1, vertices=8, faces=4,
        materials=1, textures=0,
        has_embedded_textures=0, has_armature=1, animations=1,
        extensions=[], file_size_kb=4, preview_path=None,
        tags="hero character kenney",
    )
    base.update(overrides)
    conn = get_connection(db_path)
    upsert_asset(conn, base)
    rebuild_fts(conn)
    conn.commit()
    conn.close()


# ─── Tests ────────────────────────────────────────────────────────────────────


class TestFormatAsset:
    def test_contains_expected_keys(self) -> None:
        """_format_asset should include path, name, style, faces, etc."""
        asset = dict(
            path="/a/b.glb", name="b", style="3DLowPoly", category="X",
            pack="p", source="S", faces=4, vertices=8, materials=1,
            has_armature=1, has_embedded_textures=0, animations=0,
            file_size_kb=2, preview_path=None,
        )
        result = _format_asset(asset)
        for key in ("path", "name", "style", "faces", "vertices", "has_armature",
                    "has_embedded_textures", "preview_path", "file_size_kb"):
            assert key in result

    def test_has_armature_is_bool(self) -> None:
        """_format_asset should convert has_armature to a Python bool."""
        asset = dict(
            path="/a/b.glb", name="b", style="3DLowPoly", category="X",
            pack="p", source="S", faces=4, vertices=8, materials=1,
            has_armature=1, has_embedded_textures=0, animations=0,
            file_size_kb=2, preview_path=None,
        )
        result = _format_asset(asset)
        assert isinstance(result["has_armature"], bool)
        assert result["has_armature"] is True


class TestSearchAssets:
    def test_empty_query_returns_list(self, populated_db: Path, monkeypatch) -> None:
        """search_assets with empty query should return a list."""
        _patch_db(monkeypatch, populated_db)
        results = search_assets(query="")
        assert isinstance(results, list)

    def test_fts_query_returns_list(self, populated_db: Path, monkeypatch) -> None:
        """search_assets with FTS query should return a list.

        hybrid_search has DB_PATH as a default arg (captured at import time),
        so we patch the name in server.py's namespace directly.
        """
        fake_results = [
            {
                "path": "/fake/char.glb", "name": "character", "style": "3DLowPoly",
                "category": "Characters", "pack": "k", "source": "Kenney",
                "faces": 4, "vertices": 8, "materials": 1, "has_armature": 0,
                "has_embedded_textures": 0, "animations": 0, "file_size_kb": 2,
                "preview_path": None,
            }
        ]
        monkeypatch.setattr(server_module, "hybrid_search", lambda **kw: fake_results)
        results = search_assets(query="character")
        assert isinstance(results, list)
        assert len(results) == 1

    def test_each_result_is_formatted(self, populated_db: Path, monkeypatch) -> None:
        """Each result should have 'name' and 'path' from _format_asset."""
        _patch_db(monkeypatch, populated_db)
        results = search_assets(query="")
        for r in results:
            assert "name" in r
            assert "path" in r


class TestListCategories:
    def test_returns_list(self, populated_db: Path, monkeypatch) -> None:
        """list_categories should return a list of category dicts."""
        _patch_db(monkeypatch, populated_db)
        result = list_categories()
        assert isinstance(result, list)

    def test_style_filter(self, populated_db: Path, monkeypatch) -> None:
        """list_categories(style='3DLowPoly') should only return 3DLowPoly entries."""
        _patch_db(monkeypatch, populated_db)
        result = list_categories(style="3DLowPoly")
        assert all(r["style"] == "3DLowPoly" for r in result)


class TestGetAssetInfo:
    def test_existing_asset(self, populated_db: Path, monkeypatch) -> None:
        """get_asset_info should return asset dict for a known path."""
        _patch_db(monkeypatch, populated_db)
        info = get_asset_info("/fake/3DLowPoly/Characters/Animated/kenney_pack/character.glb")
        assert "name" in info
        assert info["name"] == "character"

    def test_unknown_path_not_on_disk(self, populated_db: Path, monkeypatch) -> None:
        """get_asset_info should return error dict for missing path."""
        _patch_db(monkeypatch, populated_db)
        result = get_asset_info("/totally/nonexistent/path.glb")
        assert "error" in result

    def test_live_read_for_uncatalogued_glb(
        self, tmp_path: Path, tmp_db: Path, minimal_glb_bytes: bytes, monkeypatch
    ) -> None:
        """get_asset_info on a real GLB not in catalog should return live stats."""
        glb_file = tmp_path / "uncatalogued.glb"
        glb_file.write_bytes(minimal_glb_bytes)
        _patch_db(monkeypatch, tmp_db)
        result = get_asset_info(str(glb_file))
        assert "error" not in result
        assert result.get("name") == "uncatalogued"


class TestCopyAsset:
    def test_copies_file(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """copy_asset should copy the file to dest_dir."""
        src = tmp_path / "src" / "model.glb"
        src.parent.mkdir()
        src.write_bytes(minimal_glb_bytes)
        dest_dir = tmp_path / "dest"

        result = copy_asset(str(src), str(dest_dir))
        assert "error" not in result
        assert Path(result["dest"]).exists()

    def test_rename_option(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """copy_asset with rename should use the new filename."""
        src = tmp_path / "src" / "model.glb"
        src.parent.mkdir()
        src.write_bytes(minimal_glb_bytes)
        dest_dir = tmp_path / "dest"

        result = copy_asset(str(src), str(dest_dir), rename="renamed_model")
        assert Path(result["dest"]).name == "renamed_model.glb"

    def test_missing_source_returns_error(self, tmp_path: Path) -> None:
        """copy_asset with non-existent source should return error dict."""
        result = copy_asset("/no/such/file.glb", str(tmp_path))
        assert "error" in result

    def test_creates_dest_dir(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """copy_asset should create the destination directory if it doesn't exist."""
        src = tmp_path / "model.glb"
        src.write_bytes(minimal_glb_bytes)
        deep_dest = tmp_path / "a" / "b" / "c"

        result = copy_asset(str(src), str(deep_dest))
        assert "error" not in result
        assert deep_dest.exists()


class TestGetPreview:
    def test_no_preview_returns_none(self, tmp_path: Path, tmp_db: Path, monkeypatch) -> None:
        """get_preview for a GLB with no preview should return preview_path=None."""
        glb = tmp_path / "model.glb"
        glb.write_bytes(b"fake")
        _patch_db(monkeypatch, tmp_db)
        result = get_preview(str(glb))
        assert result["preview_path"] is None

    def test_existing_preview_sibling(self, tmp_path: Path) -> None:
        """get_preview should find a .png next to the GLB."""
        glb = tmp_path / "model.glb"
        glb.write_bytes(b"fake")
        png = tmp_path / "model.png"
        png.write_bytes(b"PNG")
        result = get_preview(str(glb))
        assert result["preview_path"] == str(png)


class TestGeneratePreview:
    def test_missing_glb_returns_error(self, tmp_path: Path) -> None:
        """generate_preview on a non-existent GLB should return error dict."""
        result = generate_preview("/no/such/file.glb")
        assert "error" in result

    def test_no_blender_returns_error(
        self, tmp_path: Path, minimal_glb_bytes: bytes, monkeypatch
    ) -> None:
        """generate_preview without Blender available should return error dict."""
        glb = tmp_path / "model.glb"
        glb.write_bytes(minimal_glb_bytes)
        monkeypatch.setattr(server_module, "HAS_BLENDER", False)
        result = generate_preview(str(glb))
        assert "error" in result


class TestGetCatalogStats:
    def test_returns_dict(self, populated_db: Path, monkeypatch) -> None:
        """get_catalog_stats should return a dict with total key."""
        _patch_db(monkeypatch, populated_db)
        stats = get_catalog_stats()
        assert isinstance(stats, dict)
        assert "total" in stats

    def test_by_style_present(self, populated_db: Path, monkeypatch) -> None:
        """get_catalog_stats should include by_style breakdown."""
        _patch_db(monkeypatch, populated_db)
        stats = get_catalog_stats()
        assert "by_style" in stats
        assert isinstance(stats["by_style"], dict)

    def test_total_is_two(self, populated_db: Path, monkeypatch) -> None:
        """populated_db has 2 assets, so total should be 2."""
        _patch_db(monkeypatch, populated_db)
        stats = get_catalog_stats()
        assert stats["total"] == 2


class TestBrowseTaxonomy:
    def test_no_args_returns_meso_level(self, populated_db: Path, monkeypatch) -> None:
        """browse_taxonomy() with no args should return meso-level entries."""
        _patch_db(monkeypatch, populated_db)
        result = browse_taxonomy()
        assert isinstance(result, list)
        assert all(r["level"] == "meso" for r in result)

    def test_style_filter(self, populated_db: Path, monkeypatch) -> None:
        """browse_taxonomy(style='3DLowPoly') should only return 3DLowPoly rows."""
        _patch_db(monkeypatch, populated_db)
        result = browse_taxonomy(style="3DLowPoly")
        assert all(r["style"] == "3DLowPoly" for r in result)

    def test_meso_filter_returns_micro_level(self, populated_db: Path, monkeypatch) -> None:
        """browse_taxonomy(meso='Characters') should return micro-level entries."""
        _patch_db(monkeypatch, populated_db)
        result = browse_taxonomy(meso="Characters")
        assert isinstance(result, list)
        if result:
            assert result[0]["level"] == "micro"


class TestSearchPolyhaven:
    def test_calls_search_ph(self) -> None:
        """search_polyhaven should return what search_ph returns.

        server.py does `from .polyhaven import search_ph` inside the function body,
        so we patch at the source module (game_asset_mcp.polyhaven.search_ph).
        """
        fake_results = [{"id": "oak_tree", "name": "Oak Tree"}]
        with patch("game_asset_mcp.polyhaven.search_ph", return_value=fake_results):
            result = search_polyhaven("tree")
        assert result == fake_results


class TestDownloadPolyhavenAsset:
    def test_delegates_to_download_ph_asset(self) -> None:
        """download_polyhaven_asset should call download_ph_asset and return result.

        server.py imports download_ph_asset and ingest inside the function body,
        so we patch at the source modules.
        """
        fake_dl_result = {
            "dest_dir": "/tmp/fake",
            "files": ["/tmp/fake/x.glb"],
            "asset_type": "models",
            "categories": [],
        }
        with (
            patch("game_asset_mcp.polyhaven.download_ph_asset", return_value=fake_dl_result),
            patch("game_asset_mcp.ingest.ingest", return_value={"added": 1}),
        ):
            result = download_polyhaven_asset("oak_tree", asset_type="models")
        assert "error" not in result
        assert result["ingested"] is True

    def test_error_propagates(self) -> None:
        """If download_ph_asset returns error, it should propagate unchanged."""
        with patch("game_asset_mcp.polyhaven.download_ph_asset", return_value={"error": "not found"}):
            result = download_polyhaven_asset("bad_id")
        assert "error" in result


class TestRunIngest:
    def test_returns_summary_dict(self) -> None:
        """run_ingest should return a dict with added/updated/skipped keys.

        server.py imports ingest inside the function body, so patch at the
        source module (game_asset_mcp.ingest.ingest).
        """
        fake_result = {
            "total_scanned": 2, "added": 2, "updated": 0,
            "skipped": 0, "removed": 0, "errors": 0,
        }
        with patch("game_asset_mcp.ingest.ingest", return_value=fake_result) as mock:
            result = run_ingest(force=False)
        assert "added" in result
        assert result["added"] == 2
        mock.assert_called_once_with(force=False)
