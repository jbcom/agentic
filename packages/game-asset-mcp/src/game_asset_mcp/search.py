"""
search.py — Hybrid keyword + vector search over the asset catalog.

Keyword search: SQLite FTS5 (fast, exact, no dependencies)
Semantic search: sqlite-vec + sentence-transformers (local, no API)

The two results are merged and ranked: semantic hits boosted by keyword score.
"""
from __future__ import annotations


import os
import json
import sqlite3
from pathlib import Path
from typing import Optional

from .catalog import DB_PATH, get_connection

# Lazy-import heavy deps so startup is fast even if they're not yet installed
_embedder = None
_vec_conn = None


def _get_embedder():
    global _embedder
    if _embedder is None:
        try:
            from sentence_transformers import SentenceTransformer
            _embedder = SentenceTransformer("all-MiniLM-L6-v2")
        except ImportError:
            _embedder = None
    return _embedder


def embed_text(text: str) -> list[float] | None:
    """Embed text using the local sentence-transformers model."""
    embedder = _get_embedder()
    if embedder is None:
        return None
    vec = embedder.encode(text, convert_to_numpy=True)
    return vec.tolist()


def _load_sqlite_vec(conn: sqlite3.Connection) -> bool:
    """Load the sqlite-vec extension into the connection."""
    try:
        import sqlite_vec
        conn.enable_load_extension(True)
        sqlite_vec.load(conn)
        conn.enable_load_extension(False)
        return True
    except Exception:
        return False


def ensure_vec_table(db_path: Path = DB_PATH) -> bool:
    """Create the vec0 virtual table for vector search if it doesn't exist."""
    conn = get_connection(db_path)
    if not _load_sqlite_vec(conn):
        conn.close()
        return False
    try:
        conn.execute("""
            CREATE VIRTUAL TABLE IF NOT EXISTS assets_vec USING vec0(
                asset_id INTEGER PRIMARY KEY,
                embedding FLOAT[384]
            )
        """)
        conn.commit()
        return True
    except Exception:
        return False
    finally:
        conn.close()


def upsert_embedding(conn: sqlite3.Connection, asset_id: int, embedding: list[float]) -> None:
    """Insert or replace an embedding in the vec0 table."""
    import struct
    vec_bytes = struct.pack(f"{len(embedding)}f", *embedding)
    conn.execute(
        "INSERT OR REPLACE INTO assets_vec(asset_id, embedding) VALUES (?, ?)",
        (asset_id, vec_bytes)
    )


def build_asset_text(asset: dict) -> str:
    """Create a searchable text representation of an asset for embedding."""
    parts = [
        asset.get("name", ""),
        asset.get("category", ""),
        asset.get("pack", ""),
        asset.get("source", ""),
        asset.get("style", ""),
        asset.get("tags", ""),
    ]
    return " ".join(p for p in parts if p)


def fts_search(
    conn: sqlite3.Connection,
    query: str,
    style: Optional[str] = None,
    category: Optional[str] = None,
    has_armature: Optional[bool] = None,
    has_textures: Optional[bool] = None,
    limit: int = 20,
) -> list[dict]:
    """FTS5 keyword search."""
    sql = """
        SELECT a.*, fts.rank as fts_rank
        FROM assets_fts fts
        JOIN assets a ON a.id = fts.rowid
        WHERE assets_fts MATCH ?
    """
    params: list = [query]
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
    sql += " ORDER BY rank LIMIT ?"
    params.append(limit)
    try:
        rows = conn.execute(sql, params).fetchall()
        return [dict(r) for r in rows]
    except Exception:
        return []


def vec_search(
    conn: sqlite3.Connection,
    query: str,
    style: Optional[str] = None,
    category: Optional[str] = None,
    limit: int = 20,
) -> list[dict]:
    """Semantic vector search using sqlite-vec."""
    if not _load_sqlite_vec(conn):
        return []

    embedding = embed_text(query)
    if embedding is None:
        return []

    import struct
    vec_bytes = struct.pack(f"{len(embedding)}f", *embedding)

    sql = """
        SELECT a.*, v.distance as vec_distance
        FROM assets_vec v
        JOIN assets a ON a.id = v.asset_id
        WHERE v.embedding MATCH ?
          AND k = ?
    """
    params: list = [vec_bytes, limit * 2]  # fetch extra for post-filtering
    if style:
        sql += " AND a.style = ?"
        params.append(style)
    if category:
        sql += " AND a.category LIKE ?"
        params.append(f"%{category}%")
    sql += " ORDER BY v.distance"

    try:
        rows = conn.execute(sql, params).fetchall()
        return [dict(r) for r in rows[:limit]]
    except Exception:
        return []


def hybrid_search(
    query: str,
    style: Optional[str] = None,
    category: Optional[str] = None,
    has_armature: Optional[bool] = None,
    has_textures: Optional[bool] = None,
    max_results: int = 20,
    db_path: Path = DB_PATH,
) -> list[dict]:
    """
    Hybrid keyword + semantic search.

    Strategy:
    1. FTS5 keyword search (fast, exact)
    2. Vector semantic search (if embedder available)
    3. Merge, deduplicate, rank by combined score
    4. Fall back to FTS5-only if vector search unavailable
    """
    conn = get_connection(db_path)

    kw_results = fts_search(conn, query, style, category, has_armature, has_textures, limit=max_results * 2)
    vec_results = vec_search(conn, query, style, category, limit=max_results * 2)

    conn.close()

    # Merge by path (dedup)
    seen_paths = {}
    for r in kw_results:
        seen_paths[r["path"]] = r
        r["_score"] = 1.0 / (1.0 + abs(r.get("fts_rank", 0)))  # FTS rank is negative

    for r in vec_results:
        if r["path"] not in seen_paths:
            seen_paths[r["path"]] = r
            r["_score"] = 1.0 / (1.0 + r.get("vec_distance", 1.0))
        else:
            # Boost combined hit
            existing = seen_paths[r["path"]]
            vec_score = 1.0 / (1.0 + r.get("vec_distance", 1.0))
            existing["_score"] = existing.get("_score", 0) + vec_score

    results = sorted(seen_paths.values(), key=lambda r: -r.get("_score", 0))
    return results[:max_results]
