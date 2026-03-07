"""Wizard smoke tests — non-interactive mode only."""
from __future__ import annotations

from pathlib import Path
from unittest.mock import patch

import pytest

from game_asset_mcp.wizard import _detect_styles, run_wizard


class TestDetectStyles:
    def test_empty_root_returns_empty(self, tmp_path: Path) -> None:
        """_detect_styles on an empty directory should return []."""
        result = _detect_styles(tmp_path)
        assert result == []

    def test_nonexistent_root_returns_empty(self, tmp_path: Path) -> None:
        """_detect_styles on a non-existent directory should return []."""
        result = _detect_styles(tmp_path / "no_such_dir")
        assert result == []

    def test_detects_dir_with_glbs(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """_detect_styles should return dirs that contain at least one GLB."""
        style_dir = tmp_path / "3DLowPoly"
        style_dir.mkdir()
        (style_dir / "hero.glb").write_bytes(minimal_glb_bytes)
        result = _detect_styles(tmp_path)
        names = [name for name, _ in result]
        assert "3DLowPoly" in names

    def test_skips_dirs_without_glbs(self, tmp_path: Path) -> None:
        """_detect_styles should skip directories with no GLBs."""
        (tmp_path / "EmptyDir").mkdir()
        result = _detect_styles(tmp_path)
        names = [name for name, _ in result]
        assert "EmptyDir" not in names

    def test_skips_hidden_dirs(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """_detect_styles should skip directories starting with '.' or '_'."""
        hidden = tmp_path / ".hidden"
        hidden.mkdir()
        (hidden / "asset.glb").write_bytes(minimal_glb_bytes)
        private = tmp_path / "_Private"
        private.mkdir()
        (private / "asset.glb").write_bytes(minimal_glb_bytes)
        result = _detect_styles(tmp_path)
        names = [name for name, _ in result]
        assert ".hidden" not in names
        assert "_Private" not in names

    def test_caps_at_eight_results(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """_detect_styles should return at most 8 directories."""
        for i in range(12):
            d = tmp_path / f"Style{i}"
            d.mkdir()
            (d / "asset.glb").write_bytes(minimal_glb_bytes)
        result = _detect_styles(tmp_path)
        assert len(result) <= 8


class TestRunWizardNonInteractive:
    def test_writes_config_file(self, tmp_path: Path) -> None:
        """run_wizard(non_interactive=True) should write a config.toml."""
        config_dir = tmp_path / "config" / "game-asset-mcp"
        config_file = config_dir / "config.toml"

        with (
            patch("game_asset_mcp.wizard._CONFIG_DIR", config_dir),
            patch("game_asset_mcp.wizard._CONFIG_FILE", config_file),
            patch("game_asset_mcp.wizard.subprocess.run"),  # suppress ingest subprocess
        ):
            run_wizard(non_interactive=True, assets_root=str(tmp_path))

        assert config_file.exists()

    def test_config_contains_assets_root(self, tmp_path: Path) -> None:
        """Written config.toml should reference the provided assets_root path."""
        config_dir = tmp_path / "config" / "game-asset-mcp"
        config_file = config_dir / "config.toml"

        with (
            patch("game_asset_mcp.wizard._CONFIG_DIR", config_dir),
            patch("game_asset_mcp.wizard._CONFIG_FILE", config_file),
            patch("game_asset_mcp.wizard.subprocess.run"),
        ):
            run_wizard(non_interactive=True, assets_root=str(tmp_path))

        content = config_file.read_text()
        assert str(tmp_path) in content

    def test_config_contains_style_map_section(self, tmp_path: Path) -> None:
        """Written config.toml should include the [taxonomy.style_map] section."""
        config_dir = tmp_path / "config" / "game-asset-mcp"
        config_file = config_dir / "config.toml"

        with (
            patch("game_asset_mcp.wizard._CONFIG_DIR", config_dir),
            patch("game_asset_mcp.wizard._CONFIG_FILE", config_file),
            patch("game_asset_mcp.wizard.subprocess.run"),
        ):
            run_wizard(non_interactive=True, assets_root=str(tmp_path))

        content = config_file.read_text()
        assert "[taxonomy.style_map]" in content

    def test_uses_detected_styles_in_config(
        self, tmp_path: Path, minimal_glb_bytes: bytes
    ) -> None:
        """When GLB dirs are found, config should include them as styles."""
        style_dir = tmp_path / "MyStyle"
        style_dir.mkdir()
        (style_dir / "asset.glb").write_bytes(minimal_glb_bytes)

        config_dir = tmp_path / "config" / "game-asset-mcp"
        config_file = config_dir / "config.toml"

        with (
            patch("game_asset_mcp.wizard._CONFIG_DIR", config_dir),
            patch("game_asset_mcp.wizard._CONFIG_FILE", config_file),
            patch("game_asset_mcp.wizard.subprocess.run"),
        ):
            run_wizard(non_interactive=True, assets_root=str(tmp_path))

        content = config_file.read_text()
        assert "MyStyle" in content

    def test_falls_back_to_default_styles(self, tmp_path: Path) -> None:
        """If no GLBs found, config should use default 3DLowPoly/3DPSX styles."""
        config_dir = tmp_path / "config" / "game-asset-mcp"
        config_file = config_dir / "config.toml"
        empty_root = tmp_path / "empty"
        empty_root.mkdir()

        with (
            patch("game_asset_mcp.wizard._CONFIG_DIR", config_dir),
            patch("game_asset_mcp.wizard._CONFIG_FILE", config_file),
            patch("game_asset_mcp.wizard.subprocess.run"),
        ):
            run_wizard(non_interactive=True, assets_root=str(empty_root))

        content = config_file.read_text()
        assert "3DLowPoly" in content

    def test_ingest_subprocess_called_in_non_interactive(self, tmp_path: Path) -> None:
        """run_wizard(non_interactive=True) should invoke the ingest subprocess."""
        config_dir = tmp_path / "config" / "game-asset-mcp"
        config_file = config_dir / "config.toml"

        with (
            patch("game_asset_mcp.wizard._CONFIG_DIR", config_dir),
            patch("game_asset_mcp.wizard._CONFIG_FILE", config_file),
            patch("game_asset_mcp.wizard.subprocess.run") as mock_run,
        ):
            run_wizard(non_interactive=True, assets_root=str(tmp_path))

        mock_run.assert_called_once()
