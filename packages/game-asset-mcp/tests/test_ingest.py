"""Taxonomy detection and ingest() integration tests."""
from __future__ import annotations

from pathlib import Path

import pytest

from game_asset_mcp.catalog import get_connection, get_stats
from game_asset_mcp.ingest import (
    IngestOptions,
    derive_tags,
    detect_category,
    detect_pack,
    detect_source,
    detect_style,
    ingest,
    scan_glbs,
)


class TestDeriveTags:
    def test_basic_split(self) -> None:
        """Name tokens should appear in tags."""
        tags = derive_tags("hero_warrior", "Characters/Animated", "kenney_pack")
        assert "hero" in tags
        assert "warrior" in tags

    def test_category_parts_included(self) -> None:
        """Category path parts should be included in tags."""
        tags = derive_tags("sword", "Props/Weapons", "custom_pack")
        assert "props" in tags
        assert "weapons" in tags

    def test_pack_parts_included(self) -> None:
        """Pack name parts should be included in tags."""
        tags = derive_tags("tree", "Environment/Nature", "quaternius_nature_pack")
        assert "quaternius" in tags
        assert "nature" in tags

    def test_deduplication(self) -> None:
        """Repeated tokens should appear only once."""
        tags = derive_tags("nature", "Environment/Nature", "nature_kit")
        parts = tags.split()
        assert parts.count("nature") == 1

    def test_short_tokens_excluded(self) -> None:
        """Single-character tokens should be excluded from tags."""
        tags = derive_tags("a_b_c", "X/Y", "z_q")
        parts = tags.split()
        for p in parts:
            assert len(p) > 1

    def test_returns_string(self) -> None:
        """derive_tags should return a str."""
        result = derive_tags("something", "A/B", "packname")
        assert isinstance(result, str)


class TestDetectStyle:
    def test_3dlowpoly_prefix(self) -> None:
        """Path starting with '3DLowPoly' should return '3DLowPoly'."""
        assert detect_style("3DLowPoly/Characters/hero.glb") == "3DLowPoly"

    def test_3dpsx_prefix(self) -> None:
        """Path starting with '3DPSX' should return '3DPSX'."""
        assert detect_style("3DPSX/Props/Tools/tool.glb") == "3DPSX"

    def test_unknown_prefix(self) -> None:
        """Path not matching any known style should return 'Unknown'."""
        assert detect_style("Audio/music.mp3") == "Unknown"


class TestDetectCategory:
    def test_extracts_middle_parts(self) -> None:
        """Category should be everything between style dir and filename."""
        cat = detect_category("3DLowPoly/Characters/Animated/kenney_pack/hero.glb", "3DLowPoly")
        assert cat == "Characters/Animated/kenney_pack"

    def test_shallow_path_returns_root(self) -> None:
        """Very shallow paths should fall back to '/'."""
        cat = detect_category("3DLowPoly/hero.glb", "3DLowPoly")
        assert cat == "/"

    def test_psx_path(self) -> None:
        """PSX-style paths should also extract correctly."""
        cat = detect_category("3DPSX/Props/Tools/blender_pack/tool.glb", "3DPSX")
        assert cat == "Props/Tools/blender_pack"


class TestDetectPack:
    def test_returns_parent_dir(self) -> None:
        """Pack should be the immediate parent directory of the GLB."""
        assert detect_pack("3DLowPoly/Characters/kenney_pack/hero.glb") == "kenney_pack"

    def test_psx_pack(self) -> None:
        """PSX paths should also extract the parent dir correctly."""
        assert detect_pack("3DPSX/Props/blender_pack/tool.glb") == "blender_pack"


class TestDetectSource:
    def test_kenney_hint(self) -> None:
        """Path parts containing 'kenney' should return 'Kenney'."""
        assert detect_source(["3DLowPoly", "Characters", "kenney_pack"]) == "Kenney"

    def test_quaternius_hint(self) -> None:
        """Path parts containing 'quaternius' should return 'Quaternius'."""
        assert detect_source(["3DLowPoly", "Props", "Quaternius Nature Pack"]) == "Quaternius"

    def test_kaykit_hint(self) -> None:
        """Path parts containing 'kaykit' should return 'KayKit'."""
        assert detect_source(["3DLowPoly", "Vehicles", "kaykit_pack"]) == "KayKit"

    def test_unknown_source(self) -> None:
        """Path parts with no matching hint should return 'Unknown'."""
        assert detect_source(["3DLowPoly", "Misc", "random_pack"]) == "Unknown"


class TestScanGlbs:
    def test_finds_glb_files(self, tmp_assets_root: Path) -> None:
        """scan_glbs should find .glb files in the asset tree."""
        glbs = scan_glbs(tmp_assets_root)
        names = [g.name for g in glbs]
        assert "character.glb" in names
        assert "tool.glb" in names

    def test_skips_archive_dirs(self, tmp_assets_root: Path) -> None:
        """scan_glbs should skip _Archive and other skip_dirs."""
        glbs = scan_glbs(tmp_assets_root)
        for glb in glbs:
            assert "_Archive" not in glb.parts

    def test_returns_path_objects(self, tmp_assets_root: Path) -> None:
        """scan_glbs should return a list of Path objects."""
        glbs = scan_glbs(tmp_assets_root)
        assert all(isinstance(g, Path) for g in glbs)

    def test_empty_root_returns_empty(self, tmp_path: Path) -> None:
        """scan_glbs on an empty directory should return an empty list."""
        empty = tmp_path / "empty"
        empty.mkdir()
        assert scan_glbs(empty) == []


class TestIngest:
    def test_basic_ingest_adds_assets(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """ingest() should add GLB files to the catalog."""
        result = ingest(root=tmp_assets_root, db_path=tmp_db)
        assert result["added"] >= 2
        assert result["errors"] == 0

    def test_ingest_result_keys(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """ingest() should return all expected summary keys."""
        result = ingest(root=tmp_assets_root, db_path=tmp_db)
        for key in ("total_scanned", "added", "updated", "skipped", "removed", "errors"):
            assert key in result

    def test_ingest_skips_unchanged_on_second_run(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """Second ingest with unchanged files should skip all previously added."""
        ingest(root=tmp_assets_root, db_path=tmp_db)
        result2 = ingest(root=tmp_assets_root, db_path=tmp_db)
        # On second run with force=False, all files are skipped
        assert result2["skipped"] >= 2
        assert result2["added"] == 0

    def test_force_flag_reingest_all(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """force=True should re-ingest all files even if unchanged."""
        ingest(root=tmp_assets_root, db_path=tmp_db)
        result2 = ingest(root=tmp_assets_root, db_path=tmp_db, force=True)
        # With force=True, no files are skipped
        assert result2["skipped"] == 0
        assert result2["added"] + result2["updated"] >= 2

    def test_dry_run_does_not_write(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """dry_run=True should not persist any records to the DB."""
        result = ingest(root=tmp_assets_root, db_path=tmp_db, dry_run=True)
        assert result["added"] >= 2  # counted as would-be-added
        conn = get_connection(tmp_db)
        stats = get_stats(conn)
        conn.close()
        assert stats["total"] == 0  # nothing actually written

    def test_ingest_detects_styles(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """Ingested assets should have correct styles detected."""
        ingest(root=tmp_assets_root, db_path=tmp_db)
        conn = get_connection(tmp_db)
        rows = conn.execute("SELECT DISTINCT style FROM assets ORDER BY style").fetchall()
        styles = {r["style"] for r in rows}
        conn.close()
        assert "3DLowPoly" in styles
        assert "3DPSX" in styles

    def test_ingest_does_not_count_archive(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """Files under _Archive should never appear in the catalog."""
        ingest(root=tmp_assets_root, db_path=tmp_db)
        conn = get_connection(tmp_db)
        rows = conn.execute("SELECT path FROM assets").fetchall()
        conn.close()
        for row in rows:
            assert "_Archive" not in row["path"]

    def test_stale_records_removed(self, tmp_assets_root: Path, tmp_db: Path) -> None:
        """Records whose files no longer exist should be pruned on re-ingest.

        Stale detection compares the `known` dict (populated from the DB when
        force=False) against the set of GLBs found on disk.  So the second
        ingest must use force=False to populate `known`.
        """
        ingest(root=tmp_assets_root, db_path=tmp_db)
        # Delete one GLB from disk
        char = tmp_assets_root / "3DLowPoly" / "Characters" / "Animated" / "kenney_pack" / "character.glb"
        char.unlink()
        # force=False so `known` is populated and stale entries are detected
        result2 = ingest(root=tmp_assets_root, db_path=tmp_db, force=False)
        assert result2["removed"] >= 1


class TestIngestOptions:
    def test_defaults(self) -> None:
        """IngestOptions should have sensible defaults."""
        opts = IngestOptions()
        assert opts.force is False
        assert opts.dry_run is False
        assert opts.verbose is False

    def test_custom_values(self, tmp_path: Path, tmp_db: Path) -> None:
        """IngestOptions should accept custom root and db paths."""
        opts = IngestOptions(root=tmp_path, db=tmp_db, force=True, dry_run=True)
        assert opts.root == tmp_path
        assert opts.db == tmp_db
        assert opts.force is True
        assert opts.dry_run is True
