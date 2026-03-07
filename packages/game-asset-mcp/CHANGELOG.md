# Changelog

## [0.1.0] - 2026-03-07

### Added
- Initial release
- SQLite catalog with FTS5 full-text search
- Idempotent ingest (O(1) skip check, stale removal)
- `browse_taxonomy` tool for macro/meso/micro/pack hierarchy navigation
- `search_assets` hybrid keyword + FTS search
- `copy_asset`, `get_preview`, `generate_preview` tools
- PolyHaven CC0 integration (`search_polyhaven`, `download_polyhaven_asset`)
- Pydantic-settings configuration with TOML file support
- Interactive setup wizard (`game-asset-init`)
- Optional `bpy` extra for in-process Blender rendering
- Pure-Python GLB stats reader (no Blender required for ingest)
