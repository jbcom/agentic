"""
server.py — FastMCP 3.0 stdio MCP server for local 3D game asset libraries.

Install:
  pip install "game-asset-mcp[server]"

Add to Claude Code:
  claude mcp add game-asset-library -- game-asset-mcp

Configure via env vars:
  ASSETS_ROOT=/path/to/assets  CATALOG_DB=~/.local/share/game-asset-mcp/catalog.db
"""
from __future__ import annotations


import os
import shutil
import subprocess
from pathlib import Path
from typing import Optional

from fastmcp import FastMCP

from .catalog import get_connection, get_asset, list_categories, get_stats, DB_PATH
from .search import hybrid_search, fts_search

from .config import get_settings

_settings = get_settings()
ASSETS_ROOT = _settings.assets_root
BLENDER = _settings.blender
PREVIEW_SCRIPT = Path(__file__).parent / "render_preview.py"

try:
    import bpy as _bpy  # noqa: F401
    HAS_BPY = True
except ImportError:
    HAS_BPY = False

HAS_BLENDER = HAS_BPY or BLENDER.exists()

mcp = FastMCP(
    "game-asset-library",
    instructions=(
        "Search, browse, and manage a local 3D game asset library. "
        "Indexes GLB models by style, category, and pack with mesh stats. "
        "Supports PolyHaven CC0 asset search and download."
    ),
)


# ─── Tools ────────────────────────────────────────────────────────────────────


@mcp.tool()
def search_assets(
    query: str,
    style: Optional[str] = None,
    category: Optional[str] = None,
    has_armature: Optional[bool] = None,
    has_textures: Optional[bool] = None,
    max_results: int = 20,
) -> list[dict]:
    """
    Search the 3D asset catalog by keyword or natural language query.

    Args:
        query: Search term — e.g. "tree", "PSX sword", "animated character walking"
        style: Filter by '3DLowPoly' or '3DPSX'
        category: Filter by category path — e.g. 'Characters/Animated', 'Environment/Nature'
        has_armature: True to find rigged/animated models only
        has_textures: True to find models with embedded textures only
        max_results: Max number of results (default 20)

    Returns:
        List of asset dicts with: path, name, style, category, pack, faces, vertices,
        materials, has_armature, has_embedded_textures, file_size_kb, preview_path
    """
    if not query.strip():
        conn = get_connection()
        rows = conn.execute(
            "SELECT * FROM assets ORDER BY ingested_at DESC LIMIT ?", (max_results,)
        ).fetchall()
        conn.close()
        return [_format_asset(dict(r)) for r in rows]

    results = hybrid_search(
        query=query,
        style=style,
        category=category,
        has_armature=has_armature,
        has_textures=has_textures,
        max_results=max_results,
    )
    return [_format_asset(r) for r in results]


@mcp.tool()
def list_categories(style: Optional[str] = None) -> list[dict]:
    """
    List all asset categories with GLB counts.

    Args:
        style: Filter by '3DLowPoly' or '3DPSX' (omit for all)

    Returns:
        List of {style, category, count} dicts sorted by style/category.
    """
    conn = get_connection()
    if style:
        rows = conn.execute(
            "SELECT style, category, COUNT(*) as count FROM assets WHERE style=? "
            "GROUP BY style, category ORDER BY category",
            (style,)
        ).fetchall()
    else:
        rows = conn.execute(
            "SELECT style, category, COUNT(*) as count FROM assets "
            "GROUP BY style, category ORDER BY style, category"
        ).fetchall()
    conn.close()
    return [dict(r) for r in rows]


@mcp.tool()
def get_asset_info(path: str) -> dict:
    """
    Get full metadata for a specific asset by its absolute path.

    Args:
        path: Absolute path to the .glb file

    Returns:
        Full asset record including mesh stats, category, source, preview path.
        Returns {'error': '...'} if not found.
    """
    conn = get_connection()
    asset = get_asset(conn, path)
    conn.close()
    if asset is None:
        # Try live read if not in catalog
        from .glb_reader import read_glb_stats
        if Path(path).exists():
            stats = read_glb_stats(path)
            return {"path": path, "name": Path(path).stem, "note": "not in catalog", **(stats or {})}
        return {"error": f"Asset not found: {path}"}
    return _format_asset(asset)


@mcp.tool()
def copy_asset(src_path: str, dest_dir: str, rename: Optional[str] = None) -> dict:
    """
    Copy a GLB asset to a destination directory (e.g. into your game project).

    Args:
        src_path: Absolute path to the source .glb file
        dest_dir: Destination directory (will be created if needed)
        rename: Optional new filename without extension (default: keep original name)

    Returns:
        {'dest': '/path/to/copied.glb', 'size_kb': 42}
    """
    src = Path(src_path)
    if not src.exists():
        return {"error": f"Source not found: {src_path}"}
    dest = Path(dest_dir)
    dest.mkdir(parents=True, exist_ok=True)
    out_name = (rename or src.stem) + ".glb"
    out_path = dest / out_name
    shutil.copy2(src, out_path)
    return {"dest": str(out_path), "size_kb": out_path.stat().st_size // 1024}


@mcp.tool()
def generate_preview(glb_path: str, output_dir: Optional[str] = None) -> dict:
    """
    Render a 512×512 PNG thumbnail of a GLB model using headless Blender.

    The preview is saved alongside the GLB as <name>.png (or in output_dir).
    Uses a standardized 3/4-view camera with 3-point studio lighting.

    Args:
        glb_path: Absolute path to the .glb file
        output_dir: Optional output directory (default: same dir as GLB)

    Returns:
        {'preview_path': '/path/to/preview.png'} or {'error': '...'}
    """
    src = Path(glb_path)
    if not src.exists():
        return {"error": f"GLB not found: {glb_path}"}

    if not HAS_BLENDER:
        return {"error": "Blender not available. Install bpy (pip install 'game-asset-mcp[blender]') or set BLENDER env var."}

    if not PREVIEW_SCRIPT.exists():
        return {"error": f"Preview script not found: {PREVIEW_SCRIPT}"}

    out_dir = Path(output_dir) if output_dir else src.parent
    out_dir.mkdir(parents=True, exist_ok=True)
    out_png = out_dir / (src.stem + ".png")

    result = subprocess.run(
        [
            str(BLENDER), "--background", "--python", str(PREVIEW_SCRIPT),
            "--", "--input", str(src), "--output", str(out_png),
        ],
        capture_output=True, text=True, timeout=60
    )

    if out_png.exists():
        # Update catalog with preview path
        conn = get_connection()
        conn.execute(
            "UPDATE assets SET preview_path=? WHERE path=?",
            (str(out_png), str(src))
        )
        conn.commit()
        conn.close()
        return {"preview_path": str(out_png)}
    else:
        return {"error": "Blender render failed", "stderr": result.stderr[-500:]}


@mcp.tool()
def get_preview(glb_path: str) -> dict:
    """
    Return the path to an existing PNG preview for a GLB, if available.

    Does NOT generate a new preview — use generate_preview() for that.

    Args:
        glb_path: Absolute path to the .glb file

    Returns:
        {'preview_path': '/path/to/preview.png'} or {'preview_path': None, 'note': '...'}
    """
    src = Path(glb_path)
    # Check common preview locations
    for candidate in [
        src.with_suffix(".png"),
        src.parent / "previews" / (src.stem + ".png"),
    ]:
        if candidate.exists():
            return {"preview_path": str(candidate)}

    # Check catalog
    conn = get_connection()
    row = conn.execute(
        "SELECT preview_path FROM assets WHERE path=?", (str(src),)
    ).fetchone()
    conn.close()
    if row and row["preview_path"] and Path(row["preview_path"]).exists():
        return {"preview_path": row["preview_path"]}

    return {"preview_path": None, "note": "No preview. Run generate_preview() to create one."}


@mcp.tool()
def get_catalog_stats() -> dict:
    """
    Return summary statistics about the asset catalog.

    Returns:
        total GLB count, with_textures, with_armature, with_previews, by_style counts.
    """
    conn = get_connection()
    stats = get_stats(conn)
    by_style = conn.execute(
        "SELECT style, COUNT(*) as count FROM assets GROUP BY style"
    ).fetchall()
    conn.close()
    return {
        **stats,
        "by_style": {r["style"]: r["count"] for r in by_style},
    }


@mcp.tool()
def browse_taxonomy(
    style: Optional[str] = None,
    meso: Optional[str] = None,
    micro: Optional[str] = None,
) -> list[dict]:
    """
    Navigate the asset library's nested category taxonomy at three levels.

    Macro (style):  '3DLowPoly' or '3DPSX'
    Meso (level 1): top category — 'Characters', 'Props', 'Environment', 'Vehicles', etc.
    Micro (level 2): sub-category — 'Weapons', 'Animated', 'Nature', 'Buildings', etc.
    Pack (level 3): individual pack directory name

    Call with no args → all meso categories with counts.
    Pass style        → filter to one library.
    Pass meso         → drill into sub-categories (micro level).
    Pass meso + micro → list individual packs.

    Examples:
        browse_taxonomy()
        browse_taxonomy(style='3DPSX')
        browse_taxonomy(style='3DPSX', meso='Props')
        browse_taxonomy(meso='Props', micro='Weapons')

    Returns:
        List of {style, meso, micro, pack, count, level} dicts.
    """
    conn = get_connection()

    where = ["1=1"]
    params: list = []
    if style:
        where.append("style = ?")
        params.append(style)

    where_sql = " AND ".join(where)

    if meso is None:
        # Meso level: first path segment of category
        rows = conn.execute(f"""
            SELECT style,
                   CASE WHEN instr(category, '/') > 0
                        THEN substr(category, 1, instr(category, '/') - 1)
                        ELSE category END AS meso,
                   COUNT(*) as count
            FROM assets
            WHERE {where_sql}
            GROUP BY style, meso
            ORDER BY style, meso
        """, params).fetchall()
        conn.close()
        return [{"style": r["style"], "meso": r["meso"], "micro": None, "pack": None,
                 "count": r["count"], "level": "meso"} for r in rows]

    # Filter to assets under this meso category
    where.append("(category = ? OR category LIKE ?)")
    params += [meso, meso + "/%"]
    where_sql = " AND ".join(where)

    if micro is None:
        # Micro level: second path segment of category
        rows = conn.execute(f"""
            SELECT style,
                   CASE WHEN instr(category, '/') > 0
                        THEN substr(category, 1, instr(category, '/') - 1)
                        ELSE category END AS meso,
                   CASE WHEN instr(category, '/') > 0
                        THEN (SELECT CASE WHEN instr(rest, '/') > 0
                                          THEN substr(rest, 1, instr(rest, '/') - 1)
                                          ELSE rest END
                              FROM (SELECT substr(category, instr(category, '/') + 1) as rest))
                        ELSE NULL END AS micro,
                   COUNT(*) as count
            FROM assets
            WHERE {where_sql}
            GROUP BY style, meso, micro
            ORDER BY style, micro
        """, params).fetchall()
        conn.close()
        return [{"style": r["style"], "meso": r["meso"], "micro": r["micro"], "pack": None,
                 "count": r["count"], "level": "micro"} for r in rows]

    # Filter to micro sub-category
    where.append("(category = ? OR category LIKE ?)")
    params += [meso + "/" + micro, meso + "/" + micro + "/%"]
    where_sql = " AND ".join(where)

    # Pack level: individual pack directories
    rows = conn.execute(f"""
        SELECT style, pack, COUNT(*) as count
        FROM assets
        WHERE {where_sql}
        GROUP BY style, pack
        ORDER BY style, pack
    """, params).fetchall()
    conn.close()
    return [{"style": r["style"], "meso": meso, "micro": micro, "pack": r["pack"],
             "count": r["count"], "level": "pack"} for r in rows]


@mcp.tool()
def search_polyhaven(
    query: str,
    asset_type: str = "models",
    category: Optional[str] = None,
    limit: int = 20,
) -> list[dict]:
    """
    Search polyhaven.com for free CC0 assets.

    Args:
        query: Keyword to match against asset name/id/tags
        asset_type: 'models', 'hdris', or 'textures'
        category: Optional filter e.g. 'nature', 'furniture', 'architecture'
        limit: Max results (default 20)

    Returns:
        List of {id, name, type, categories, tags, download_count} dicts
    """
    from .polyhaven import search_ph
    return search_ph(query=query, asset_type=asset_type, category=category, limit=limit)


@mcp.tool()
def download_polyhaven_asset(
    asset_id: str,
    asset_type: str = "models",
    resolution: str = "1k",
) -> dict:
    """
    Download a PolyHaven asset and add it to the local library.

    Automatically places the asset in the correct taxonomy directory
    and runs ingest to add it to the catalog.

    Args:
        asset_id: PolyHaven asset ID (from search_polyhaven results)
        asset_type: 'models', 'hdris', or 'textures'
        resolution: '1k', '2k', '4k' (default '1k')

    Returns:
        {dest_dir, files, asset_type, categories, ingested: bool}
    """
    from .polyhaven import download_ph_asset
    from .ingest import ingest
    from pathlib import Path

    result = download_ph_asset(asset_id=asset_id, asset_type=asset_type, resolution=resolution)
    if "error" in result:
        return result

    # Auto-ingest if it's a model (GLB)
    ingested = False
    if asset_type == "models":
        ingest_result = ingest()
        ingested = ingest_result.get("added", 0) > 0

    return {**result, "ingested": ingested}


@mcp.tool()
def run_ingest(force: bool = False) -> dict:
    """
    Re-scan the asset library and update the catalog database.

    Only re-processes files whose size has changed (unless force=True).

    Args:
        force: Re-ingest all files even if unchanged

    Returns:
        {'added': N, 'updated': N, 'skipped': N, 'total_scanned': N}
    """
    from .ingest import ingest
    result = ingest(force=force)
    return result


# ─── Helpers ──────────────────────────────────────────────────────────────────


def _format_asset(asset: dict) -> dict:
    """Return a clean asset dict with only the most useful fields for agents."""
    return {
        "path": asset.get("path"),
        "name": asset.get("name"),
        "style": asset.get("style"),
        "category": asset.get("category"),
        "pack": asset.get("pack"),
        "source": asset.get("source"),
        "faces": asset.get("faces"),
        "vertices": asset.get("vertices"),
        "materials": asset.get("materials"),
        "has_armature": bool(asset.get("has_armature")),
        "has_embedded_textures": bool(asset.get("has_embedded_textures")),
        "animations": asset.get("animations", 0),
        "file_size_kb": asset.get("file_size_kb"),
        "preview_path": asset.get("preview_path"),
    }


def main():
    mcp.run(transport="stdio")


if __name__ == "__main__":
    main()
