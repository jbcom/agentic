"""
polyhaven.py — PolyHaven API integration.
Free 3D assets, HDRIs, and textures from polyhaven.com (CC0).
"""
from __future__ import annotations

import os
import httpx
from pathlib import Path
from typing import Optional

ASSETS_ROOT = Path(os.environ.get("ASSETS_ROOT", str(Path.home() / "assets")))
PH_API = "https://api.polyhaven.com"

MODEL_CATEGORY_MAP = {
    "nature": "3DLowPoly/Environment/Nature/polyhaven",
    "architecture": "3DLowPoly/Environment/City/polyhaven",
    "furniture": "3DLowPoly/Props/Furniture/polyhaven",
    "food": "3DLowPoly/Props/Food/polyhaven",
    "vehicles": "3DLowPoly/Vehicles/polyhaven",
    "electronics": "3DLowPoly/Props/Electronics/polyhaven",
    "animals": "3DLowPoly/Characters/Animals/polyhaven",
}
DEFAULT_MODEL_PATH = "3DLowPoly/Props/Misc/polyhaven"

# Maps used by texture download: keys are PolyHaven map keys → local filename suffixes
TEXTURE_MAP_KEYS = ["diffuse", "rough", "metal", "nor_gl", "ao", "disp", "arm"]


def ph_get(endpoint: str, params: dict = None) -> dict:
    """GET from PolyHaven API with 30s timeout."""
    url = f"{PH_API}/{endpoint.lstrip('/')}"
    response = httpx.get(url, params=params, timeout=30.0)
    response.raise_for_status()
    return response.json()


def search_ph(
    query: str,
    asset_type: str,
    category: Optional[str] = None,
    limit: int = 20,
) -> list[dict]:
    """
    Search PolyHaven. query matches id/name/tags.
    Returns list sorted by download_count descending.
    """
    params: dict = {"type": asset_type}
    if category:
        params["categories"] = category

    assets = ph_get("assets", params=params)

    query_lower = query.lower()
    query_words = query_lower.split()

    results = []
    for asset_id, info in assets.items():
        name = info.get("name", "")
        tags = info.get("tags", [])
        categories = info.get("categories", [])

        # Build searchable text from id, name, tags, categories
        searchable = " ".join(
            [asset_id, name.lower()] + [t.lower() for t in tags] + [c.lower() for c in categories]
        )

        if all(word in searchable for word in query_words):
            results.append({
                "id": asset_id,
                "name": name,
                "type": info.get("type", asset_type),
                "categories": categories,
                "tags": tags,
                "download_count": info.get("download_count", 0),
            })

    results.sort(key=lambda x: x["download_count"], reverse=True)
    return results[:limit]


def get_ph_info(asset_id: str) -> dict:
    """Get full info for one asset."""
    return ph_get(f"info/{asset_id}")


def get_taxonomy_path(asset_id: str, asset_type: str, categories: list[str]) -> Path:
    """Map asset to local filesystem path."""
    if asset_type == "hdris":
        return ASSETS_ROOT / "2DPhotorealistic" / "HDRIs" / "polyhaven" / asset_id
    if asset_type == "textures":
        return ASSETS_ROOT / "2DPhotorealistic" / "Textures" / "polyhaven" / asset_id

    # Models: match first category against the map
    for cat in categories:
        cat_lower = cat.lower()
        if cat_lower in MODEL_CATEGORY_MAP:
            return ASSETS_ROOT / MODEL_CATEGORY_MAP[cat_lower] / asset_id

    return ASSETS_ROOT / DEFAULT_MODEL_PATH / asset_id


def _download_bytes(url: str) -> bytes:
    """Download raw bytes from a URL with 120s timeout."""
    response = httpx.get(url, timeout=120.0, follow_redirects=True)
    response.raise_for_status()
    return response.content


def download_ph_asset(
    asset_id: str,
    asset_type: str,
    resolution: str = "1k",
) -> dict:
    """
    Download asset from PolyHaven and save to taxonomy path.

    For models: downloads the GLB at the requested resolution.
    For HDRIs: downloads the .hdr file at the requested resolution.
    For textures: downloads all available map files (diffuse, rough, metal,
                  normal, ao, disp, arm) at the requested resolution.

    Returns:
        {dest_dir: str, files: [str], asset_type: str, categories: list}
        or {error: str} on failure.
    """
    # Fetch asset info to get categories
    try:
        info = get_ph_info(asset_id)
    except httpx.HTTPError as exc:
        return {"error": f"Failed to fetch asset info: {exc}"}

    categories = info.get("categories", [])

    # Fetch file manifest
    try:
        files_data = ph_get(f"files/{asset_id}")
    except httpx.HTTPError as exc:
        return {"error": f"Failed to fetch file manifest: {exc}"}

    dest_dir = get_taxonomy_path(asset_id, asset_type, categories)
    dest_dir.mkdir(parents=True, exist_ok=True)

    downloaded_files: list[str] = []

    if asset_type == "models":
        # Structure: {"gltf": {"1k": {"glb": {"url": "...", "size": N}, ...}, ...}, ...}
        gltf_section = files_data.get("gltf", {})
        res_section = gltf_section.get(resolution) or next(iter(gltf_section.values()), None)
        if not res_section:
            return {"error": f"No gltf files found for {asset_id}"}

        glb_info = res_section.get("glb")
        if not glb_info:
            return {"error": f"No GLB variant found for {asset_id} at resolution {resolution}"}

        url = glb_info["url"]
        dest_file = dest_dir / f"{asset_id}.glb"
        try:
            data = _download_bytes(url)
        except httpx.HTTPError as exc:
            return {"error": f"Download failed: {exc}"}
        dest_file.write_bytes(data)
        downloaded_files.append(str(dest_file))

    elif asset_type == "hdris":
        # Structure: {"hdri": {"1k": {"hdr": {"url": "...", "size": N}, "exr": {...}}, ...}}
        hdri_section = files_data.get("hdri", {})
        res_section = hdri_section.get(resolution) or next(iter(hdri_section.values()), None)
        if not res_section:
            return {"error": f"No HDRI files found for {asset_id}"}

        # Prefer .hdr; fall back to .exr
        for fmt in ("hdr", "exr"):
            fmt_info = res_section.get(fmt)
            if fmt_info:
                url = fmt_info["url"]
                dest_file = dest_dir / f"{asset_id}_{resolution}.{fmt}"
                try:
                    data = _download_bytes(url)
                except httpx.HTTPError as exc:
                    return {"error": f"Download failed: {exc}"}
                dest_file.write_bytes(data)
                downloaded_files.append(str(dest_file))
                break

        if not downloaded_files:
            return {"error": f"No hdr/exr variant found for {asset_id} at resolution {resolution}"}

    elif asset_type == "textures":
        # Structure: {"1k": {"diffuse": {"url": "...", ...}, "rough": {...}, ...}, ...}
        res_section = files_data.get(resolution) or next(iter(files_data.values()), None)
        if not res_section:
            return {"error": f"No texture files found for {asset_id}"}

        for map_key, map_info in res_section.items():
            if not isinstance(map_info, dict) or "url" not in map_info:
                continue
            url = map_info["url"]
            # Derive extension from URL
            url_path = url.split("?")[0]
            ext = url_path.rsplit(".", 1)[-1] if "." in url_path else "png"
            dest_file = dest_dir / f"{asset_id}_{map_key}.{ext}"
            try:
                data = _download_bytes(url)
            except httpx.HTTPError as exc:
                # Don't abort entire texture set if one map fails
                continue
            dest_file.write_bytes(data)
            downloaded_files.append(str(dest_file))

        if not downloaded_files:
            return {"error": f"No texture maps downloaded for {asset_id} at resolution {resolution}"}

    else:
        return {"error": f"Unknown asset_type '{asset_type}'. Use 'models', 'hdris', or 'textures'."}

    return {
        "dest_dir": str(dest_dir),
        "files": downloaded_files,
        "asset_type": asset_type,
        "categories": categories,
    }
