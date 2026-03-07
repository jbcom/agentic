"""
catalog.py — SQLite catalog database for the asset library.

Database location: ~/.local/share/assets-mcp/catalog.db  (local, fast)
Overridable via CATALOG_DB env var.
"""
from __future__ import annotations


import sqlite3
import os
from pathlib import Path
from typing import Optional

_DEFAULT_DB = Path.home() / ".local" / "share" / "game-asset-mcp" / "catalog.db"
DB_PATH = Path(os.environ.get("CATALOG_DB", str(_DEFAULT_DB)))


def get_connection(db_path: Path = DB_PATH) -> sqlite3.Connection:
    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA journal_mode=WAL")
    return conn


def init_db(db_path: Path = DB_PATH) -> None:
    """Create tables if they don't exist."""
    conn = get_connection(db_path)
    conn.executescript("""
        CREATE TABLE IF NOT EXISTS assets (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            path            TEXT UNIQUE NOT NULL,
            name            TEXT NOT NULL,
            style           TEXT,          -- '3DLowPoly' | '3DPSX'
            category        TEXT,          -- e.g. 'Characters/Animated'
            pack            TEXT,          -- pack directory name
            source          TEXT,          -- 'Kenney' | 'Quaternius' | 'KayKit' | 'Custom'
            meshes          INTEGER,
            vertices        INTEGER,
            faces           INTEGER,
            materials       INTEGER,
            textures        INTEGER,
            has_embedded_textures INTEGER DEFAULT 0,
            has_armature    INTEGER DEFAULT 0,
            animations      INTEGER DEFAULT 0,
            extensions      TEXT,          -- JSON array
            file_size_kb    INTEGER,
            preview_path    TEXT,          -- absolute path to PNG thumbnail
            tags            TEXT,          -- space-separated searchable tags
            ingested_at     REAL           -- unix timestamp
        );

        CREATE TABLE IF NOT EXISTS ingest_log (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            run_at      REAL,
            total       INTEGER,
            added       INTEGER,
            updated     INTEGER,
            skipped     INTEGER
        );
    """)

    # FTS5 virtual table for full-text search
    conn.execute("""
        CREATE VIRTUAL TABLE IF NOT EXISTS assets_fts USING fts5(
            name,
            category,
            pack,
            source,
            tags,
            style,
            content=assets,
            content_rowid=id
        )
    """)
    conn.commit()
    conn.close()


def upsert_asset(conn: sqlite3.Connection, asset: dict) -> int:
    """Insert or replace an asset record. Returns the row id."""
    import json, time

    conn.execute("""
        INSERT INTO assets (
            path, name, style, category, pack, source,
            meshes, vertices, faces, materials, textures,
            has_embedded_textures, has_armature, animations, extensions,
            file_size_kb, preview_path, tags, ingested_at
        ) VALUES (
            :path, :name, :style, :category, :pack, :source,
            :meshes, :vertices, :faces, :materials, :textures,
            :has_embedded_textures, :has_armature, :animations, :extensions,
            :file_size_kb, :preview_path, :tags, :ingested_at
        ) ON CONFLICT(path) DO UPDATE SET
            meshes=excluded.meshes,
            vertices=excluded.vertices,
            faces=excluded.faces,
            materials=excluded.materials,
            textures=excluded.textures,
            has_embedded_textures=excluded.has_embedded_textures,
            has_armature=excluded.has_armature,
            animations=excluded.animations,
            extensions=excluded.extensions,
            file_size_kb=excluded.file_size_kb,
            preview_path=excluded.preview_path,
            tags=excluded.tags,
            ingested_at=excluded.ingested_at
    """, {
        **asset,
        "extensions": json.dumps(asset.get("extensions", [])),
        "ingested_at": asset.get("ingested_at", time.time()),
    })
    row = conn.execute("SELECT id FROM assets WHERE path=?", (asset["path"],)).fetchone()
    return row["id"]


def rebuild_fts(conn: sqlite3.Connection) -> None:
    """Rebuild the FTS5 index from the assets table."""
    conn.execute("INSERT INTO assets_fts(assets_fts) VALUES('rebuild')")


def search(
    conn: sqlite3.Connection,
    query: str,
    style: Optional[str] = None,
    category: Optional[str] = None,
    has_armature: Optional[bool] = None,
    has_textures: Optional[bool] = None,
    max_results: int = 20,
) -> list[dict]:
    """Full-text search over the asset catalog."""
    # Build FTS query
    fts_query = query.strip()
    if not fts_query:
        # No text query — fall back to plain SELECT with filters
        sql = "SELECT a.* FROM assets a WHERE 1=1"
        params: list = []
        if style:
            sql += " AND a.style = ?"
            params.append(style)
        if category:
            sql += " AND a.category LIKE ?"
            params.append(f"%{category}%")
        if has_armature is not None:
            sql += " AND a.has_armature = ?"
            params.append(1 if has_armature else 0)
        if has_textures is not None:
            sql += " AND a.has_embedded_textures = ?"
            params.append(1 if has_textures else 0)
        sql += f" LIMIT {max_results}"
        rows = conn.execute(sql, params).fetchall()
    else:
        sql = """
            SELECT a.*
            FROM assets_fts fts
            JOIN assets a ON a.id = fts.rowid
            WHERE assets_fts MATCH ?
        """
        params = [fts_query]
        if style:
            sql += " AND a.style = ?"
            params.append(style)
        if category:
            sql += " AND a.category LIKE ?"
            params.append(f"%{category}%")
        if has_armature is not None:
            sql += " AND a.has_armature = ?"
            params.append(1 if has_armature else 0)
        if has_textures is not None:
            sql += " AND a.has_embedded_textures = ?"
            params.append(1 if has_textures else 0)
        sql += " ORDER BY rank"
        sql += f" LIMIT {max_results}"
        rows = conn.execute(sql, params).fetchall()

    return [dict(r) for r in rows]


def get_asset(conn: sqlite3.Connection, path: str) -> Optional[dict]:
    row = conn.execute("SELECT * FROM assets WHERE path = ?", (path,)).fetchone()
    return dict(row) if row else None


def list_categories(conn: sqlite3.Connection) -> list[dict]:
    rows = conn.execute("""
        SELECT style, category, COUNT(*) as count
        FROM assets
        GROUP BY style, category
        ORDER BY style, category
    """).fetchall()
    return [dict(r) for r in rows]


def get_stats(conn: sqlite3.Connection) -> dict:
    row = conn.execute("""
        SELECT
            COUNT(*) as total,
            SUM(CASE WHEN has_embedded_textures=1 THEN 1 ELSE 0 END) as with_textures,
            SUM(CASE WHEN has_armature=1 THEN 1 ELSE 0 END) as with_armature,
            SUM(CASE WHEN preview_path IS NOT NULL THEN 1 ELSE 0 END) as with_previews
        FROM assets
    """).fetchone()
    return dict(row)
