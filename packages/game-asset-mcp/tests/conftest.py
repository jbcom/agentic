"""Shared fixtures for game-asset-mcp tests."""
from __future__ import annotations

import struct
import json
from pathlib import Path

import pytest

from game_asset_mcp.catalog import init_db, get_connection, upsert_asset, rebuild_fts
from game_asset_mcp.config import reset_settings


def _make_glb_bytes(gltf_dict: dict | None = None) -> bytes:
    """Build a minimal valid GLB binary from a GLTF dict."""
    if gltf_dict is None:
        gltf_dict = {"asset": {"version": "2.0"}}

    raw_json = json.dumps(gltf_dict).encode("utf-8")
    # GLB JSON chunk must be 4-byte aligned, padded with spaces
    pad = (4 - len(raw_json) % 4) % 4
    raw_json += b" " * pad

    # Header: magic + version + total_length
    json_chunk_len = len(raw_json)
    total_length = 12 + 8 + json_chunk_len  # header + chunk_header + chunk_data
    header = struct.pack("<III", 0x46546C67, 2, total_length)  # 'glTF', v2, length

    # JSON chunk header: length + type (JSON = 0x4E4F534A)
    json_chunk_header = struct.pack("<II", json_chunk_len, 0x4E4F534A)

    return header + json_chunk_header + raw_json


@pytest.fixture
def minimal_glb_bytes() -> bytes:
    """Return a minimal valid GLB binary with no meshes/materials."""
    return _make_glb_bytes({"asset": {"version": "2.0"}})


@pytest.fixture
def rich_glb_bytes() -> bytes:
    """Return a GLB binary with mesh, material, and skin data."""
    gltf = {
        "asset": {"version": "2.0"},
        "meshes": [
            {
                "name": "Cube",
                "primitives": [
                    {
                        "attributes": {"POSITION": 0},
                        "indices": 1,
                    }
                ],
            }
        ],
        "accessors": [
            {"count": 8, "componentType": 5126, "type": "VEC3"},  # POSITION: 8 verts
            {"count": 36, "componentType": 5123, "type": "SCALAR"},  # indices: 36 → 12 tris
        ],
        "materials": [{"name": "Mat"}],
        "images": [{"bufferView": 0}],  # embedded texture
        "skins": [{"name": "Armature", "joints": [0]}],
        "animations": [{"name": "Walk", "channels": [], "samplers": []}],
    }
    return _make_glb_bytes(gltf)


@pytest.fixture
def tmp_db(tmp_path: Path) -> Path:
    """Create and initialise a fresh SQLite catalog DB in a temp dir."""
    db = tmp_path / "test_catalog.db"
    init_db(db)
    return db


@pytest.fixture
def tmp_assets_root(tmp_path: Path) -> Path:
    """Create a minimal taxonomy directory tree with fake GLB files."""
    root = tmp_path / "assets"

    # 3DLowPoly / Characters / Animated / kenney_pack / character.glb
    char_dir = root / "3DLowPoly" / "Characters" / "Animated" / "kenney_pack"
    char_dir.mkdir(parents=True)
    glb_bytes = _make_glb_bytes({"asset": {"version": "2.0"}})
    (char_dir / "character.glb").write_bytes(glb_bytes)

    # 3DPSX / Props / Tools / blender_pack / tool.glb
    psx_dir = root / "3DPSX" / "Props" / "Tools" / "blender_pack"
    psx_dir.mkdir(parents=True)
    (psx_dir / "tool.glb").write_bytes(glb_bytes)

    # 3DLowPoly / _Archive / archived.glb  (should be skipped)
    archive_dir = root / "3DLowPoly" / "_Archive"
    archive_dir.mkdir(parents=True)
    (archive_dir / "archived.glb").write_bytes(glb_bytes)

    return root


@pytest.fixture
def populated_db(tmp_db: Path) -> Path:
    """Populate the DB with two sample asset records for search tests."""
    conn = get_connection(tmp_db)

    _sample_asset = dict(
        path="/fake/3DLowPoly/Characters/Animated/kenney_pack/character.glb",
        name="character",
        style="3DLowPoly",
        category="Characters/Animated/kenney_pack",
        pack="kenney_pack",
        source="Kenney",
        meshes=1,
        vertices=8,
        faces=12,
        materials=1,
        textures=0,
        has_embedded_textures=0,
        has_armature=1,
        animations=2,
        extensions=[],
        file_size_kb=4,
        preview_path=None,
        tags="character animated kenney pack",
    )
    upsert_asset(conn, _sample_asset)

    _sample_asset2 = dict(
        path="/fake/3DPSX/Props/Tools/blender_pack/tool.glb",
        name="tool",
        style="3DPSX",
        category="Props/Tools/blender_pack",
        pack="blender_pack",
        source="Custom",
        meshes=1,
        vertices=24,
        faces=12,
        materials=1,
        textures=1,
        has_embedded_textures=1,
        has_armature=0,
        animations=0,
        extensions=[],
        file_size_kb=8,
        preview_path=None,
        tags="tool psx blender pack",
    )
    upsert_asset(conn, _sample_asset2)

    rebuild_fts(conn)
    conn.commit()
    conn.close()
    return tmp_db


@pytest.fixture(autouse=True)
def reset_settings_cache():
    """Reset the settings LRU cache before and after every test."""
    reset_settings()
    yield
    reset_settings()
