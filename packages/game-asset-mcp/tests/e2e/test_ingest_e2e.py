"""E2E ingest tests against the real ASSETS_ROOT (skipped if not set)."""
from __future__ import annotations

from pathlib import Path

import pytest

from game_asset_mcp.catalog import get_connection, get_stats, list_categories
from game_asset_mcp.ingest import ingest, scan_glbs


@pytest.mark.e2e
class TestRealIngest:
    def test_scan_finds_glbs(self, real_assets_root: Path) -> None:
        """scan_glbs on the real asset root should find at least one GLB."""
        glbs = scan_glbs(real_assets_root)
        assert len(glbs) > 0, "No GLBs found under ASSETS_ROOT"

    def test_ingest_dry_run_reports_assets(
        self, real_assets_root: Path, e2e_db: Path
    ) -> None:
        """Dry-run ingest should count assets without writing to the DB."""
        from game_asset_mcp.catalog import init_db
        init_db(e2e_db)
        result = ingest(root=real_assets_root, db_path=e2e_db, dry_run=True)
        assert result["total_scanned"] > 0
        # No records written
        conn = get_connection(e2e_db)
        stats = get_stats(conn)
        conn.close()
        assert stats["total"] == 0

    def test_ingest_writes_records(
        self, real_assets_root: Path, e2e_db: Path
    ) -> None:
        """Full ingest should populate the DB with asset records."""
        result = ingest(root=real_assets_root, db_path=e2e_db)
        assert result["added"] > 0 or result["updated"] > 0
        conn = get_connection(e2e_db)
        stats = get_stats(conn)
        cats = list_categories(conn)
        conn.close()
        assert stats["total"] > 0
        assert len(cats) > 0

    def test_second_ingest_skips_unchanged(
        self, real_assets_root: Path, e2e_db: Path
    ) -> None:
        """Second ingest with no file changes should skip all previously indexed assets."""
        result = ingest(root=real_assets_root, db_path=e2e_db)
        assert result["skipped"] >= result["total_scanned"] - result["errors"]
