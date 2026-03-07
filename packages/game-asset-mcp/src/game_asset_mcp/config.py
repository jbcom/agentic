"""
config.py — Configuration for game-asset-mcp.

Settings are loaded in priority order (lowest → highest):
  1. Built-in defaults
  2. TOML config file (~/.config/game-asset-mcp/config.toml or .game-asset-mcp.toml)
  3. Environment variables (GAME_ASSET_*)
  4. CLI flags (when invoked via pydantic-settings CLI source)

Example TOML config:
    [library]
    assets_root = "/Volumes/home/assets"
    catalog_db = "~/.local/share/game-asset-mcp/catalog.db"

    [taxonomy.style_map]
    "3DLowPoly" = "3DLowPoly"
    "3DPSX" = "3DPSX"

    [taxonomy.source_hints]
    kenney = "Kenney"
    quaternius = "Quaternius"
    kaykit = "KayKit"
"""
from __future__ import annotations

import os
try:
    import tomllib
except ImportError:  # Python 3.10
    import tomli as tomllib  # type: ignore[no-redef]
from functools import lru_cache
from pathlib import Path
from typing import Optional

from pydantic import Field, field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict

_DEFAULT_CONFIG_DIR = Path.home() / ".config" / "game-asset-mcp"
_DEFAULT_CONFIG_FILE = _DEFAULT_CONFIG_DIR / "config.toml"
_DEFAULT_LOCAL_CONFIG = Path(".game-asset-mcp.toml")  # project-local override


class TaxonomyConfig:
    """
    Taxonomy configuration (not a pydantic model — parsed from TOML manually
    so we can support arbitrary style_map and source_hints keys).
    """

    def __init__(
        self,
        style_map: dict[str, str] | None = None,
        source_hints: dict[str, str] | None = None,
        skip_dirs: list[str] | None = None,
    ):
        self.style_map: dict[str, str] = style_map or {
            "3DLowPoly": "3DLowPoly",
            "3DPSX": "3DPSX",
        }
        self.source_hints: dict[str, str] = source_hints or {
            "kenney": "Kenney",
            "quaternius": "Quaternius",
            "kaykit": "KayKit",
            "kits": "KayKit",
            "custom": "Custom",
            "zappypixel": "Zappypixel",
        }
        self.skip_dirs: set[str] = set(skip_dirs or [
            "_Archive", "__pycache__", ".git", "node_modules", "Textures", "textures",
        ])

    @classmethod
    def from_dict(cls, data: dict) -> "TaxonomyConfig":
        return cls(
            style_map=data.get("style_map"),
            source_hints=data.get("source_hints"),
            skip_dirs=data.get("skip_dirs"),
        )


class Settings(BaseSettings):
    """
    Global settings for game-asset-mcp.

    Environment variable prefix: GAME_ASSET_
    Examples:
        GAME_ASSET_ASSETS_ROOT=/mnt/assets
        GAME_ASSET_CATALOG_DB=/tmp/catalog.db
    """

    model_config = SettingsConfigDict(
        env_prefix="GAME_ASSET_",
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )

    assets_root: Path = Field(
        default_factory=lambda: Path(
            os.environ.get("ASSETS_ROOT", str(Path.home() / "assets"))
        ),
        description="Root directory of your 3D asset library",
    )
    catalog_db: Path = Field(
        default_factory=lambda: Path(
            os.environ.get("CATALOG_DB", str(Path.home() / ".local" / "share" / "game-asset-mcp" / "catalog.db"))
        ),
        description="Path to the SQLite catalog database",
    )
    blender: Path = Field(
        default_factory=lambda: Path(os.environ.get("BLENDER", "/opt/homebrew/bin/blender")),
        description="Path to Blender binary (used for generate_preview)",
    )
    config_file: Optional[Path] = Field(
        default=None,
        description="Path to TOML config file (auto-detected if not set)",
    )

    @field_validator("assets_root", "catalog_db", "blender", mode="before")
    @classmethod
    def expand_path(cls, v: object) -> Path:
        if isinstance(v, str):
            return Path(v).expanduser()
        if isinstance(v, Path):
            return v.expanduser()
        return v  # type: ignore[return-value]

    def get_taxonomy(self) -> TaxonomyConfig:
        """Load taxonomy config from TOML file if present, else return defaults."""
        config_path = self._find_config_file()
        if config_path is None:
            return TaxonomyConfig()
        try:
            with open(config_path, "rb") as f:
                data = tomllib.load(f)
            return TaxonomyConfig.from_dict(data.get("taxonomy", {}))
        except Exception:
            return TaxonomyConfig()

    def _find_config_file(self) -> Optional[Path]:
        if self.config_file and self.config_file.exists():
            return self.config_file
        if _DEFAULT_LOCAL_CONFIG.exists():
            return _DEFAULT_LOCAL_CONFIG
        if _DEFAULT_CONFIG_FILE.exists():
            return _DEFAULT_CONFIG_FILE
        return None


@lru_cache(maxsize=1)
def get_settings() -> Settings:
    """Return the singleton Settings instance."""
    return Settings()


def reset_settings() -> None:
    """Clear the cached settings (useful in tests or after config changes)."""
    get_settings.cache_clear()
