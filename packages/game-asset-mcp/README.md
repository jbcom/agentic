# game-asset-mcp

**MCP server for local 3D game asset libraries** — search, browse, catalog, and download GLB/GLTF assets from your file system and PolyHaven.

Built with [FastMCP](https://github.com/jlowin/fastmcp) · Works with Claude Code, Cursor, Windsurf, and any MCP-compatible client · CC0 PolyHaven integration included

---

## Quick Start

### Install

```bash
pip install "game-asset-mcp[server,polyhaven]"
```

### Configure

Set your asset library root:

```bash
export ASSETS_ROOT=/path/to/your/3d-assets
export CATALOG_DB=~/.local/share/game-asset-mcp/catalog.db  # optional, this is the default
```

### Ingest your library

```bash
game-asset-ingest
```

This scans all `.glb` files in `ASSETS_ROOT`, extracts mesh stats (vertices, faces, materials, animations), and builds a searchable SQLite catalog. **Idempotent** — only re-processes files that have changed size.

---

## Add to Claude Code

```bash
claude mcp add game-asset-library \
  -e ASSETS_ROOT=/path/to/your/3d-assets \
  -- game-asset-mcp
```

Or add to `~/.claude.json` manually:

```json
{
  "mcpServers": {
    "game-asset-library": {
      "type": "stdio",
      "command": "game-asset-mcp",
      "env": {
        "ASSETS_ROOT": "/path/to/your/3d-assets"
      }
    }
  }
}
```

## Add to Cursor

Add to `.cursor/mcp.json` in your project (or `~/.cursor/mcp.json` globally):

```json
{
  "mcpServers": {
    "game-asset-library": {
      "command": "game-asset-mcp",
      "env": {
        "ASSETS_ROOT": "/path/to/your/3d-assets"
      }
    }
  }
}
```

## Add to Windsurf

Add to `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "game-asset-library": {
      "command": "game-asset-mcp",
      "env": {
        "ASSETS_ROOT": "/path/to/your/3d-assets"
      }
    }
  }
}
```

---

## Available Tools

| Tool | Description |
|------|-------------|
| `search_assets` | Hybrid keyword + FTS search across all GLBs |
| `browse_taxonomy` | Navigate your directory taxonomy (macro → meso → micro → pack) |
| `list_categories` | List all categories with GLB counts |
| `get_asset_info` | Full metadata for one asset (mesh stats, preview path, etc.) |
| `copy_asset` | Copy a GLB into your game project directory |
| `get_preview` | Return path to an existing PNG thumbnail |
| `generate_preview` | Render a 512×512 thumbnail via headless Blender |
| `run_ingest` | Re-scan library and update catalog (idempotent) |
| `get_catalog_stats` | Summary statistics (total assets, with textures, with armatures) |
| `search_polyhaven` | Search [PolyHaven](https://polyhaven.com) for free CC0 models/HDRIs/textures |
| `download_polyhaven_asset` | Download a PolyHaven asset and auto-add to your catalog |

---

## Taxonomy Convention

`game-asset-mcp` works best with assets organized as:

```
ASSETS_ROOT/
├── 3DLowPoly/
│   ├── Characters/
│   │   ├── Animated/
│   │   │   └── <pack-name>/  ← GLBs here
│   │   └── Animals/
│   ├── Props/
│   │   └── Weapons/
│   └── Environment/
│       └── Nature/
├── 3DPSX/
│   └── ...
└── 2DPhotorealistic/
    ├── HDRIs/
    └── Textures/
```

The `browse_taxonomy` tool navigates this as **style → category → sub-category → pack**. Flat libraries work too — `search_assets` does full-text search on filenames and directory names regardless of structure.

---

## PolyHaven Integration

Search and download free CC0 assets from [polyhaven.com](https://polyhaven.com):

```
search_polyhaven("oak tree", asset_type="models")
download_polyhaven_asset("oak_tree", asset_type="models", resolution="1k")
```

Downloads are automatically placed in the correct taxonomy directory and added to the catalog.

| PolyHaven Type | Local Path |
|----------------|------------|
| `models` | `ASSETS_ROOT/3DLowPoly/<category>/polyhaven/<id>/` |
| `hdris` | `ASSETS_ROOT/2DPhotorealistic/HDRIs/polyhaven/<id>/` |
| `textures` | `ASSETS_ROOT/2DPhotorealistic/Textures/polyhaven/<id>/` |

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ASSETS_ROOT` | `~/assets` | Root directory of your 3D asset library |
| `CATALOG_DB` | `~/.local/share/game-asset-mcp/catalog.db` | SQLite catalog path |
| `BLENDER` | `/opt/homebrew/bin/blender` | Blender binary (only needed for `generate_preview`) |

---

## Development

```bash
# From the monorepo root
uv sync
uv run --package game-asset-mcp game-asset-mcp
```

Or directly:

```bash
cd packages/game-asset-mcp
uv run game-asset-mcp
```

---

## License

MIT © Jon Bogaty — PolyHaven assets are [CC0](https://polyhaven.com/license)
