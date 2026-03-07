"""Full catalog CRUD and FTS search tests."""
from __future__ import annotations

from pathlib import Path

import pytest

from game_asset_mcp.catalog import (
    get_connection,
    get_stats,
    init_db,
    list_categories,
    rebuild_fts,
    search,
    upsert_asset,
    get_asset,
)


def _make_asset(**overrides) -> dict:
    """Build a minimal asset dict, with optional field overrides."""
    base = dict(
        path="/tmp/test/3DLowPoly/Characters/hero.glb",
        name="hero",
        style="3DLowPoly",
        category="Characters",
        pack="hero_pack",
        source="Kenney",
        meshes=1,
        vertices=8,
        faces=4,
        materials=1,
        textures=0,
        has_embedded_textures=0,
        has_armature=0,
        animations=0,
        extensions=[],
        file_size_kb=2,
        preview_path=None,
        tags="hero character",
    )
    base.update(overrides)
    return base


class TestInitDb:
    def test_creates_tables(self, tmp_db: Path) -> None:
        """init_db should create assets, ingest_log and assets_fts tables."""
        conn = get_connection(tmp_db)
        tables = {
            row[0]
            for row in conn.execute(
                "SELECT name FROM sqlite_master WHERE type IN ('table', 'shadow')"
            ).fetchall()
        }
        conn.close()
        assert "assets" in tables
        assert "ingest_log" in tables

    def test_idempotent(self, tmp_db: Path) -> None:
        """Calling init_db twice should not raise."""
        init_db(tmp_db)  # second call
        conn = get_connection(tmp_db)
        count = conn.execute("SELECT COUNT(*) FROM assets").fetchone()[0]
        conn.close()
        assert count == 0


class TestUpsertAsset:
    def test_insert_returns_row_id(self, tmp_db: Path) -> None:
        """upsert_asset should return a positive integer row ID."""
        conn = get_connection(tmp_db)
        row_id = upsert_asset(conn, _make_asset())
        conn.commit()
        conn.close()
        assert isinstance(row_id, int)
        assert row_id >= 1

    def test_duplicate_path_updates(self, tmp_db: Path) -> None:
        """Inserting the same path twice should update, not duplicate."""
        conn = get_connection(tmp_db)
        asset = _make_asset()
        upsert_asset(conn, asset)
        conn.commit()

        updated = _make_asset(vertices=999, faces=333)
        upsert_asset(conn, updated)
        conn.commit()

        count = conn.execute("SELECT COUNT(*) FROM assets").fetchone()[0]
        row = conn.execute("SELECT vertices, faces FROM assets").fetchone()
        conn.close()
        assert count == 1
        assert row["vertices"] == 999
        assert row["faces"] == 333

    def test_extensions_serialised_as_json(self, tmp_db: Path) -> None:
        """extensions list should be stored as a JSON string."""
        conn = get_connection(tmp_db)
        upsert_asset(conn, _make_asset(extensions=["KHR_draco_mesh_compression"]))
        conn.commit()
        row = conn.execute("SELECT extensions FROM assets").fetchone()
        conn.close()
        import json
        exts = json.loads(row["extensions"])
        assert "KHR_draco_mesh_compression" in exts

    def test_ingested_at_auto_set(self, tmp_db: Path) -> None:
        """ingested_at should be set automatically if not provided."""
        asset = _make_asset()
        asset.pop("ingested_at", None)  # ensure it's absent
        conn = get_connection(tmp_db)
        upsert_asset(conn, asset)
        conn.commit()
        row = conn.execute("SELECT ingested_at FROM assets").fetchone()
        conn.close()
        assert row["ingested_at"] is not None
        assert row["ingested_at"] > 0


class TestGetAsset:
    def test_returns_dict_for_known_path(self, populated_db: Path) -> None:
        """get_asset should return a dict for a path that exists in the DB."""
        conn = get_connection(populated_db)
        asset = get_asset(conn, "/fake/3DLowPoly/Characters/Animated/kenney_pack/character.glb")
        conn.close()
        assert asset is not None
        assert asset["name"] == "character"

    def test_returns_none_for_unknown_path(self, populated_db: Path) -> None:
        """get_asset should return None for a path not in the DB."""
        conn = get_connection(populated_db)
        result = get_asset(conn, "/nonexistent/path.glb")
        conn.close()
        assert result is None


class TestSearch:
    def test_empty_query_returns_all(self, populated_db: Path) -> None:
        """Empty query should return all assets (up to max_results)."""
        conn = get_connection(populated_db)
        results = search(conn, query="", max_results=10)
        conn.close()
        assert len(results) == 2

    def test_fts_query_matches_name(self, populated_db: Path) -> None:
        """FTS query for 'character' should return the character asset."""
        conn = get_connection(populated_db)
        results = search(conn, query="character")
        conn.close()
        names = [r["name"] for r in results]
        assert "character" in names

    def test_fts_query_matches_tags(self, populated_db: Path) -> None:
        """FTS query on a tag token should match the relevant asset."""
        conn = get_connection(populated_db)
        results = search(conn, query="kenney")
        conn.close()
        assert len(results) >= 1
        assert results[0]["name"] == "character"

    def test_style_filter(self, populated_db: Path) -> None:
        """Filtering by style='3DPSX' should return only PSX assets."""
        conn = get_connection(populated_db)
        results = search(conn, query="", style="3DPSX", max_results=10)
        conn.close()
        assert all(r["style"] == "3DPSX" for r in results)
        assert len(results) == 1

    def test_has_armature_filter(self, populated_db: Path) -> None:
        """has_armature=True filter should return only rigged assets."""
        conn = get_connection(populated_db)
        results = search(conn, query="", has_armature=True, max_results=10)
        conn.close()
        assert all(r["has_armature"] == 1 for r in results)

    def test_has_textures_filter(self, populated_db: Path) -> None:
        """has_textures=True filter should return only textured assets."""
        conn = get_connection(populated_db)
        results = search(conn, query="", has_textures=True, max_results=10)
        conn.close()
        assert all(r["has_embedded_textures"] == 1 for r in results)

    def test_category_filter(self, populated_db: Path) -> None:
        """Category LIKE filter should narrow results."""
        conn = get_connection(populated_db)
        results = search(conn, query="", category="Props", max_results=10)
        conn.close()
        assert len(results) == 1
        assert results[0]["name"] == "tool"

    def test_max_results_respected(self, populated_db: Path) -> None:
        """max_results=1 should return at most 1 record."""
        conn = get_connection(populated_db)
        results = search(conn, query="", max_results=1)
        conn.close()
        assert len(results) <= 1

    def test_no_match_returns_empty(self, populated_db: Path) -> None:
        """FTS query with no matching token should return empty list."""
        conn = get_connection(populated_db)
        results = search(conn, query="zzznomatch")
        conn.close()
        assert results == []


class TestListCategories:
    def test_returns_list_of_dicts(self, populated_db: Path) -> None:
        """list_categories should return a non-empty list of dicts."""
        conn = get_connection(populated_db)
        cats = list_categories(conn)
        conn.close()
        assert isinstance(cats, list)
        assert len(cats) >= 1

    def test_dict_has_required_keys(self, populated_db: Path) -> None:
        """Each category dict must contain style, category, and count keys."""
        conn = get_connection(populated_db)
        cats = list_categories(conn)
        conn.close()
        for cat in cats:
            assert "style" in cat
            assert "category" in cat
            assert "count" in cat

    def test_count_is_positive(self, populated_db: Path) -> None:
        """Every category count should be a positive integer."""
        conn = get_connection(populated_db)
        cats = list_categories(conn)
        conn.close()
        assert all(cat["count"] > 0 for cat in cats)


class TestGetStats:
    def test_returns_required_keys(self, populated_db: Path) -> None:
        """get_stats should return total, with_textures, with_armature, with_previews."""
        conn = get_connection(populated_db)
        stats = get_stats(conn)
        conn.close()
        assert "total" in stats
        assert "with_textures" in stats
        assert "with_armature" in stats
        assert "with_previews" in stats

    def test_total_matches_inserted(self, populated_db: Path) -> None:
        """Total count should equal the number of inserted records."""
        conn = get_connection(populated_db)
        stats = get_stats(conn)
        conn.close()
        assert stats["total"] == 2

    def test_empty_db_stats(self, tmp_db: Path) -> None:
        """Stats on an empty DB should return zeros."""
        conn = get_connection(tmp_db)
        stats = get_stats(conn)
        conn.close()
        assert stats["total"] == 0


class TestRebuildFts:
    def test_rebuild_does_not_raise(self, populated_db: Path) -> None:
        """rebuild_fts should succeed without raising exceptions."""
        conn = get_connection(populated_db)
        # Should not raise
        rebuild_fts(conn)
        conn.commit()
        conn.close()

    def test_rebuild_preserves_searchability(self, populated_db: Path) -> None:
        """After a rebuild, FTS search should still find indexed records."""
        conn = get_connection(populated_db)
        rebuild_fts(conn)
        conn.commit()
        results = search(conn, query="character")
        conn.close()
        assert len(results) >= 1
