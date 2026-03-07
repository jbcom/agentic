"""GLB binary parsing tests using synthetic GLB bytes."""
from __future__ import annotations

import struct
import json
from pathlib import Path

import pytest

from game_asset_mcp.glb_reader import read_glb_stats, is_valid_glb


def _write_glb(path: Path, gltf_dict: dict) -> Path:
    """Write a minimal GLB file to disk and return the path."""
    raw_json = json.dumps(gltf_dict).encode("utf-8")
    # Pad to 4-byte alignment
    pad = (4 - len(raw_json) % 4) % 4
    raw_json += b" " * pad

    json_chunk_len = len(raw_json)
    total_length = 12 + 8 + json_chunk_len
    header = struct.pack("<III", 0x46546C67, 2, total_length)
    json_chunk_header = struct.pack("<II", json_chunk_len, 0x4E4F534A)

    path.write_bytes(header + json_chunk_header + raw_json)
    return path


class TestReadGlbStats:
    def test_minimal_glb_returns_dict(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """read_glb_stats on a minimal valid GLB should return a stats dict."""
        glb_file = tmp_path / "minimal.glb"
        glb_file.write_bytes(minimal_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert isinstance(result, dict)

    def test_minimal_glb_has_zero_meshes(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """Minimal GLB with no meshes should report meshes=0."""
        glb_file = tmp_path / "minimal.glb"
        glb_file.write_bytes(minimal_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["meshes"] == 0
        assert result["vertices"] == 0
        assert result["faces"] == 0

    def test_rich_glb_mesh_stats(self, tmp_path: Path, rich_glb_bytes: bytes) -> None:
        """GLB with one mesh (8 verts, 36 indices) should report correct stats."""
        glb_file = tmp_path / "rich.glb"
        glb_file.write_bytes(rich_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["meshes"] == 1
        assert result["vertices"] == 8
        assert result["faces"] == 12  # 36 indices / 3

    def test_rich_glb_has_armature(self, tmp_path: Path, rich_glb_bytes: bytes) -> None:
        """GLB with skins should report has_armature=True."""
        glb_file = tmp_path / "rich.glb"
        glb_file.write_bytes(rich_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["has_armature"] is True

    def test_rich_glb_has_embedded_textures(self, tmp_path: Path, rich_glb_bytes: bytes) -> None:
        """GLB with images having bufferView should report has_embedded_textures=True."""
        glb_file = tmp_path / "rich.glb"
        glb_file.write_bytes(rich_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["has_embedded_textures"] is True

    def test_rich_glb_animations_count(self, tmp_path: Path, rich_glb_bytes: bytes) -> None:
        """GLB with one animation block should report animations=1."""
        glb_file = tmp_path / "rich.glb"
        glb_file.write_bytes(rich_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["animations"] == 1

    def test_materials_count(self, tmp_path: Path) -> None:
        """GLB with 2 materials should report materials=2."""
        gltf = {
            "asset": {"version": "2.0"},
            "materials": [{"name": "A"}, {"name": "B"}],
        }
        glb_file = _write_glb(tmp_path / "two_mats.glb", gltf)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["materials"] == 2

    def test_file_size_kb_is_int(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """file_size_kb should be a non-negative integer."""
        glb_file = tmp_path / "minimal.glb"
        glb_file.write_bytes(minimal_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert isinstance(result["file_size_kb"], int)
        assert result["file_size_kb"] >= 0

    def test_extensions_are_listed(self, tmp_path: Path) -> None:
        """extensionsUsed in GLTF JSON should appear in the result."""
        gltf = {
            "asset": {"version": "2.0"},
            "extensionsUsed": ["KHR_draco_mesh_compression"],
        }
        glb_file = _write_glb(tmp_path / "ext.glb", gltf)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert "KHR_draco_mesh_compression" in result["extensions"]

    def test_invalid_magic_returns_none(self, tmp_path: Path) -> None:
        """File with wrong magic bytes should return None."""
        bad = tmp_path / "bad.glb"
        bad.write_bytes(b"\x00\x00\x00\x00" * 10)
        result = read_glb_stats(str(bad))
        assert result is None

    def test_truncated_file_returns_none(self, tmp_path: Path) -> None:
        """File that is too short to be a valid GLB should return None."""
        short = tmp_path / "short.glb"
        short.write_bytes(b"glTF\x02\x00")  # only 6 bytes
        result = read_glb_stats(str(short))
        assert result is None

    def test_nonexistent_file_returns_none(self, tmp_path: Path) -> None:
        """Non-existent file path should return None."""
        result = read_glb_stats(str(tmp_path / "no_such_file.glb"))
        assert result is None

    def test_no_embedded_textures_when_no_buffer_view(self, tmp_path: Path) -> None:
        """Image with URI (no bufferView) should not set has_embedded_textures."""
        gltf = {
            "asset": {"version": "2.0"},
            "images": [{"uri": "texture.png"}],
        }
        glb_file = _write_glb(tmp_path / "ext_tex.glb", gltf)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        assert result["has_embedded_textures"] is False

    def test_result_has_all_expected_keys(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """Result dict should contain all documented keys."""
        glb_file = tmp_path / "minimal.glb"
        glb_file.write_bytes(minimal_glb_bytes)
        result = read_glb_stats(str(glb_file))
        assert result is not None
        for key in (
            "meshes", "primitives", "vertices", "faces", "materials",
            "textures", "has_embedded_textures", "has_armature", "animations",
            "extensions", "file_size_kb",
        ):
            assert key in result, f"Missing key: {key}"


class TestIsValidGlb:
    def test_valid_glb_returns_true(self, tmp_path: Path, minimal_glb_bytes: bytes) -> None:
        """is_valid_glb should return True for a file starting with glTF magic."""
        glb_file = tmp_path / "valid.glb"
        glb_file.write_bytes(minimal_glb_bytes)
        assert is_valid_glb(str(glb_file)) is True

    def test_invalid_file_returns_false(self, tmp_path: Path) -> None:
        """is_valid_glb should return False for a non-GLB file."""
        bad = tmp_path / "bad.bin"
        bad.write_bytes(b"\xFF\xFE\xFD\xFC" + b"\x00" * 8)
        assert is_valid_glb(str(bad)) is False

    def test_nonexistent_returns_false(self, tmp_path: Path) -> None:
        """is_valid_glb should return False for a missing file."""
        assert is_valid_glb(str(tmp_path / "ghost.glb")) is False
