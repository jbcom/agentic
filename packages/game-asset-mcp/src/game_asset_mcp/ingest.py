"""
ingest.py — Scan the asset library and populate the SQLite catalog.

Usage:
    python -m game_asset_mcp.ingest [--root /Volumes/home/assets] [--force] [--dry-run]

Does NOT require Blender. Uses pure-Python GLB reader for all mesh stats.
"""
from __future__ import annotations


import os
import re
import time
import json
from pathlib import Path

from pydantic import BaseModel, Field

from .glb_reader import read_glb_stats
from .catalog import DB_PATH, get_connection, init_db, upsert_asset, rebuild_fts

from .config import get_settings

ASSETS_ROOT = get_settings().assets_root


def _taxonomy():
    return get_settings().get_taxonomy()

# Tags to derive from filename words
def derive_tags(name: str, category: str, pack: str) -> str:
    """Build a space-separated tag string for FTS from path components."""
    parts = re.split(r"[_\-\s\.]+", name.lower())
    cat_parts = re.split(r"[/\\]+", category.lower())
    pack_parts = re.split(r"[_\-\s\.]+", pack.lower())
    all_parts = parts + cat_parts + pack_parts
    # Deduplicate while preserving order
    seen = set()
    tags = []
    for p in all_parts:
        if p and p not in seen and len(p) > 1:
            seen.add(p)
            tags.append(p)
    return " ".join(tags)


def detect_source(path_parts: list[str]) -> str:
    path_str = " ".join(p.lower() for p in path_parts)
    for hint, source in _taxonomy().source_hints.items():
        if hint in path_str:
            return source
    return "Unknown"


def detect_style(rel_path: str) -> str:
    for prefix, style in _taxonomy().style_map.items():
        if rel_path.startswith(prefix):
            return style
    return "Unknown"


def detect_category(rel_path: str, style: str) -> str:
    """Extract the category portion (strip style prefix and pack name)."""
    parts = Path(rel_path).parts
    # parts[0] = style (3DLowPoly / 3DPSX)
    # parts[1..n-2] = category path
    # parts[-1] = filename.glb
    if len(parts) >= 3:
        return "/".join(parts[1:-1])
    return "/"


def detect_pack(rel_path: str) -> str:
    """The immediate parent directory of a GLB is the pack."""
    return Path(rel_path).parent.name


def scan_glbs(root: Path) -> list[Path]:
    """Walk the asset root and find all .glb files (skipping _Archive etc.)."""
    result = []
    for dirpath, dirs, files in os.walk(root):
        dirs[:] = sorted([
            d for d in dirs
            if d not in _taxonomy().skip_dirs and not d.startswith("_")
        ])
        for f in files:
            if f.lower().endswith(".glb"):
                result.append(Path(dirpath) / f)
    return result


def ingest(
    root: Path = ASSETS_ROOT,
    db_path: Path = DB_PATH,
    force: bool = False,
    dry_run: bool = False,
    verbose: bool = False,
) -> dict:
    """Scan root for GLBs and populate the catalog DB."""
    init_db(db_path)
    conn = get_connection(db_path)

    # Load all existing records into memory once — avoids N individual DB queries
    known: dict[str, int] = {}
    if not force:
        for row in conn.execute("SELECT path, file_size_kb FROM assets"):
            known[row["path"]] = row["file_size_kb"]

    glbs = scan_glbs(root)
    total = len(glbs)
    added = updated = skipped = errors = 0
    run_start = time.time()

    for glb_path in glbs:
        rel = glb_path.relative_to(root)
        rel_str = str(rel)

        # Fast in-memory skip check — no DB round-trip per file
        if not force:
            current_size_kb = glb_path.stat().st_size // 1024
            if known.get(str(glb_path)) == current_size_kb:
                skipped += 1
                continue

        stats = read_glb_stats(str(glb_path))
        if stats is None:
            if verbose:
                print(f"  SKIP (invalid GLB): {rel_str}")
            errors += 1
            continue

        name = glb_path.stem
        style = detect_style(rel_str)
        category = detect_category(rel_str, style)
        pack = detect_pack(rel_str)
        source = detect_source(list(rel.parts))
        tags = derive_tags(name, category, pack)

        # Check for matching preview PNG
        preview_path = None
        for ext in (".png", ".jpg"):
            candidate = glb_path.with_suffix(ext)
            if candidate.exists():
                preview_path = str(candidate)
                break
        # Also check previews/ sibling directory
        previews_dir = glb_path.parent / "previews"
        candidate = previews_dir / (name + ".png")
        if candidate.exists():
            preview_path = str(candidate)

        asset_record = {
            "path": str(glb_path),
            "name": name,
            "style": style,
            "category": category,
            "pack": pack,
            "source": source,
            "meshes": stats["meshes"],
            "vertices": stats["vertices"],
            "faces": stats["faces"],
            "materials": stats["materials"],
            "textures": stats["textures"],
            "has_embedded_textures": int(stats["has_embedded_textures"]),
            "has_armature": int(stats["has_armature"]),
            "animations": stats["animations"],
            "extensions": stats["extensions"],
            "file_size_kb": stats["file_size_kb"],
            "preview_path": preview_path,
            "tags": tags,
            "ingested_at": time.time(),
        }

        is_update = str(glb_path) in known

        if verbose:
            status = "UPDATE" if is_update else "ADD"
            print(f"  [{status}] {rel_str}: {stats['faces']}f {stats['vertices']}v")

        if not dry_run:
            upsert_asset(conn, asset_record)
            if is_update:
                updated += 1
            else:
                added += 1
        else:
            added += 1  # count as would-be-added in dry run

    # Remove stale entries — files in DB that no longer exist on disk
    found_paths = {str(p) for p in glbs}
    stale = [p for p in known if p not in found_paths]
    removed = 0
    if stale and not dry_run:
        for path in stale:
            conn.execute("DELETE FROM assets WHERE path=?", (path,))
        removed = len(stale)
        if verbose and stale:
            for p in stale:
                print(f"  [REMOVE] {p}")

    if not dry_run:
        rebuild_fts(conn)
        elapsed = time.time() - run_start
        conn.execute(
            "INSERT INTO ingest_log (run_at, total, added, updated, skipped) VALUES (?,?,?,?,?)",
            (run_start, total, added, updated, skipped)
        )
        conn.commit()

    conn.close()
    return {
        "total_scanned": total,
        "added": added,
        "updated": updated,
        "skipped": skipped,
        "removed": removed,
        "errors": errors,
    }


class IngestOptions(BaseModel):
    """Scan the asset library and populate the SQLite catalog."""

    root: Path = Field(
        default_factory=lambda: get_settings().assets_root,
        description="Asset library root directory",
    )
    db: Path = Field(
        default_factory=lambda: get_settings().catalog_db,
        description="SQLite catalog DB path",
    )
    force: bool = Field(False, description="Re-ingest all files (ignore size cache)")
    dry_run: bool = Field(False, description="Scan only — no DB writes")
    verbose: bool = Field(False, description="Print per-file status")


def _run_ingest(opts: IngestOptions) -> int:
    print(f"Ingesting from {opts.root} into {opts.db}")
    if opts.dry_run:
        print("  (dry run — no writes)")

    result = ingest(
        root=opts.root,
        db_path=opts.db,
        force=opts.force,
        dry_run=opts.dry_run,
        verbose=opts.verbose,
    )

    print(
        f"\nDone: {result['added']} added, {result['updated']} updated, "
        f"{result['skipped']} skipped, {result['removed']} removed, {result['errors']} errors"
    )
    print(f"Total GLBs scanned: {result['total_scanned']}")
    return 0


def main() -> None:
    from pydantic_cli import run_and_exit
    run_and_exit(IngestOptions, _run_ingest, description=__doc__ or "game-asset-ingest")


if __name__ == "__main__":
    main()
