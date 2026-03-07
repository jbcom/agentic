"""Mocked API tests for polyhaven.py."""
from __future__ import annotations

from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from game_asset_mcp.polyhaven import (
    download_ph_asset,
    get_taxonomy_path,
    search_ph,
    get_ph_info,
)


# ─── Helpers ──────────────────────────────────────────────────────────────────

def _mock_response(data: dict, status_code: int = 200) -> MagicMock:
    """Build a mock httpx.Response that returns `data` from .json()."""
    resp = MagicMock()
    resp.status_code = status_code
    resp.json.return_value = data
    resp.raise_for_status = MagicMock()
    return resp


FAKE_ASSETS_RESPONSE = {
    "oak_tree": {
        "name": "Oak Tree",
        "type": 0,
        "categories": ["nature"],
        "tags": ["tree", "oak", "plant"],
        "download_count": 5000,
    },
    "pine_tree": {
        "name": "Pine Tree",
        "type": 0,
        "categories": ["nature"],
        "tags": ["tree", "pine", "conifer"],
        "download_count": 3000,
    },
    "wooden_chair": {
        "name": "Wooden Chair",
        "type": 0,
        "categories": ["furniture"],
        "tags": ["chair", "wooden", "seat"],
        "download_count": 1000,
    },
}

FAKE_FILES_RESPONSE = {
    "gltf": {
        "1k": {
            "glb": {
                "url": "https://dl.polyhaven.com/oak_tree_1k.glb",
                "size": 204800,
            }
        }
    }
}

FAKE_INFO_RESPONSE = {
    "name": "Oak Tree",
    "categories": ["nature"],
    "tags": ["tree", "oak"],
}


# ─── Tests ────────────────────────────────────────────────────────────────────


class TestSearchPh:
    def test_returns_matching_assets(self) -> None:
        """search_ph should return assets whose name/id/tags match the query."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("tree", asset_type="models")
        ids = [r["id"] for r in results]
        assert "oak_tree" in ids
        assert "pine_tree" in ids

    def test_filters_by_query_word(self) -> None:
        """search_ph with 'chair' should not return tree assets."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("chair", asset_type="models")
        ids = [r["id"] for r in results]
        assert "wooden_chair" in ids
        assert "oak_tree" not in ids

    def test_sorted_by_download_count(self) -> None:
        """Results should be sorted by download_count descending."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("tree", asset_type="models")
        counts = [r["download_count"] for r in results]
        assert counts == sorted(counts, reverse=True)

    def test_limit_respected(self) -> None:
        """limit parameter should cap results."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("tree", asset_type="models", limit=1)
        assert len(results) <= 1

    def test_no_match_returns_empty(self) -> None:
        """Query that matches nothing should return an empty list."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("zzznomatch", asset_type="models")
        assert results == []

    def test_result_dict_has_required_keys(self) -> None:
        """Each result dict must have id, name, type, categories, tags, download_count."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("tree", asset_type="models")
        for r in results:
            for key in ("id", "name", "type", "categories", "tags", "download_count"):
                assert key in r, f"Missing key '{key}' in result: {r}"

    def test_multi_word_query(self) -> None:
        """Multi-word query should require all words to match."""
        with patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_ASSETS_RESPONSE):
            results = search_ph("oak tree", asset_type="models")
        ids = [r["id"] for r in results]
        assert "oak_tree" in ids
        assert "pine_tree" not in ids


class TestGetTaxonomyPath:
    def test_hdri_type_maps_to_2d_photorealistic(self, tmp_path: Path, monkeypatch) -> None:
        """HDRI assets should be placed under 2DPhotorealistic/HDRIs/polyhaven."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        path = get_taxonomy_path("sky_01", "hdris", [])
        assert "2DPhotorealistic" in path.parts
        assert "HDRIs" in path.parts

    def test_texture_type_maps_to_textures(self, tmp_path: Path, monkeypatch) -> None:
        """Texture assets should be placed under 2DPhotorealistic/Textures/polyhaven."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        path = get_taxonomy_path("wood_01", "textures", [])
        assert "2DPhotorealistic" in path.parts
        assert "Textures" in path.parts

    def test_nature_model_maps_correctly(self, tmp_path: Path, monkeypatch) -> None:
        """Models with 'nature' category should go to 3DLowPoly/Environment/Nature/polyhaven."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        path = get_taxonomy_path("oak_tree", "models", ["nature"])
        assert "Environment" in path.parts
        assert "Nature" in path.parts

    def test_furniture_model_maps_correctly(self, tmp_path: Path, monkeypatch) -> None:
        """Models with 'furniture' category should go to 3DLowPoly/Props/Furniture/polyhaven."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        path = get_taxonomy_path("chair_01", "models", ["furniture"])
        assert "Furniture" in path.parts

    def test_unknown_category_uses_default(self, tmp_path: Path, monkeypatch) -> None:
        """Models with unrecognised category should use the default path."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        path = get_taxonomy_path("thing_01", "models", ["unknown_category"])
        assert "Misc" in path.parts

    def test_asset_id_is_leaf(self, tmp_path: Path, monkeypatch) -> None:
        """The asset_id should be the leaf directory of the returned path."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        path = get_taxonomy_path("oak_tree", "models", ["nature"])
        assert path.name == "oak_tree"


class TestDownloadPhAsset:
    def test_model_download_success(self, tmp_path: Path, monkeypatch) -> None:
        """Successful model download should return dest_dir, files, and categories."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)

        with (
            patch("game_asset_mcp.polyhaven.get_ph_info", return_value=FAKE_INFO_RESPONSE),
            patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_FILES_RESPONSE),
            patch("game_asset_mcp.polyhaven._download_bytes", return_value=b"fake-glb-data"),
        ):
            result = download_ph_asset("oak_tree", asset_type="models", resolution="1k")

        assert "error" not in result
        assert "dest_dir" in result
        assert "files" in result
        assert len(result["files"]) == 1
        assert result["files"][0].endswith(".glb")

    def test_model_download_writes_file(self, tmp_path: Path, monkeypatch) -> None:
        """Downloaded GLB should actually exist on disk."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)

        with (
            patch("game_asset_mcp.polyhaven.get_ph_info", return_value=FAKE_INFO_RESPONSE),
            patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_FILES_RESPONSE),
            patch("game_asset_mcp.polyhaven._download_bytes", return_value=b"fake-glb-data"),
        ):
            result = download_ph_asset("oak_tree", asset_type="models", resolution="1k")

        assert Path(result["files"][0]).exists()

    def test_info_fetch_failure_returns_error(self, tmp_path: Path, monkeypatch) -> None:
        """HTTP error fetching asset info should return an error dict."""
        import httpx

        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)

        with patch("game_asset_mcp.polyhaven.get_ph_info", side_effect=httpx.HTTPError("not found")):
            result = download_ph_asset("bad_id", asset_type="models")

        assert "error" in result

    def test_no_gltf_section_returns_error(self, tmp_path: Path, monkeypatch) -> None:
        """Files response with no gltf section should return an error dict."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)

        with (
            patch("game_asset_mcp.polyhaven.get_ph_info", return_value=FAKE_INFO_RESPONSE),
            patch("game_asset_mcp.polyhaven.ph_get", return_value={}),
        ):
            result = download_ph_asset("oak_tree", asset_type="models")

        assert "error" in result

    def test_unknown_asset_type_returns_error(self, tmp_path: Path, monkeypatch) -> None:
        """Unsupported asset_type should return an error dict."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)

        with (
            patch("game_asset_mcp.polyhaven.get_ph_info", return_value=FAKE_INFO_RESPONSE),
            patch("game_asset_mcp.polyhaven.ph_get", return_value=FAKE_FILES_RESPONSE),
        ):
            result = download_ph_asset("oak_tree", asset_type="unknown_type")

        assert "error" in result
        assert "unknown_type" in result["error"]

    def test_hdri_download(self, tmp_path: Path, monkeypatch) -> None:
        """HDRI download should save an .hdr file."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        fake_hdri_info = {"name": "Sky", "categories": []}
        fake_hdri_files = {
            "hdri": {
                "1k": {
                    "hdr": {"url": "https://example.com/sky_1k.hdr", "size": 1024}
                }
            }
        }

        with (
            patch("game_asset_mcp.polyhaven.get_ph_info", return_value=fake_hdri_info),
            patch("game_asset_mcp.polyhaven.ph_get", return_value=fake_hdri_files),
            patch("game_asset_mcp.polyhaven._download_bytes", return_value=b"hdr-data"),
        ):
            result = download_ph_asset("sky_01", asset_type="hdris", resolution="1k")

        assert "error" not in result
        assert any(f.endswith(".hdr") for f in result["files"])

    def test_texture_download(self, tmp_path: Path, monkeypatch) -> None:
        """Texture download should save multiple map files."""
        monkeypatch.setattr("game_asset_mcp.polyhaven.ASSETS_ROOT", tmp_path)
        fake_tex_info = {"name": "Brick Wall", "categories": []}
        fake_tex_files = {
            "1k": {
                "diffuse": {"url": "https://example.com/brick_diff_1k.png"},
                "rough": {"url": "https://example.com/brick_rough_1k.png"},
            }
        }

        with (
            patch("game_asset_mcp.polyhaven.get_ph_info", return_value=fake_tex_info),
            patch("game_asset_mcp.polyhaven.ph_get", return_value=fake_tex_files),
            patch("game_asset_mcp.polyhaven._download_bytes", return_value=b"img-data"),
        ):
            result = download_ph_asset("brick_wall", asset_type="textures", resolution="1k")

        assert "error" not in result
        assert len(result["files"]) == 2
