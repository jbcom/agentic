"""
wizard.py — Interactive setup wizard for game-asset-mcp.

Run: game-asset-init

Asks questions, generates a TOML config, and optionally runs the initial ingest.
"""
from __future__ import annotations

import argparse
import subprocess
import sys
import textwrap
from pathlib import Path


_CONFIG_DIR = Path.home() / ".config" / "game-asset-mcp"
_CONFIG_FILE = _CONFIG_DIR / "config.toml"


def _ask(prompt: str, default: str) -> str:
    try:
        value = input(f"{prompt} [{default}]: ").strip()
        return value or default
    except (EOFError, KeyboardInterrupt):
        print()
        sys.exit(0)


def _ask_bool(prompt: str, default: bool = True) -> bool:
    default_str = "Y/n" if default else "y/N"
    try:
        value = input(f"{prompt} [{default_str}]: ").strip().lower()
        if not value:
            return default
        return value.startswith("y")
    except (EOFError, KeyboardInterrupt):
        print()
        sys.exit(0)


def _detect_styles(root: Path) -> list[tuple[str, str]]:
    """Detect potential style directories in the asset root."""
    if not root.exists():
        return []
    styles = []
    for d in sorted(root.iterdir()):
        if d.is_dir() and not d.name.startswith((".", "_")):
            # Only suggest dirs that likely contain GLBs
            glb_count = sum(1 for _ in d.rglob("*.glb") if True) if d.exists() else 0
            if glb_count > 0:
                styles.append((d.name, d.name))
    return styles[:8]  # cap at 8 suggestions


def run_wizard(non_interactive: bool = False, assets_root: str | None = None) -> None:
    print("\n🎮 game-asset-mcp setup wizard\n" + "=" * 40)

    # Assets root
    default_root = assets_root or str(Path.home() / "assets")
    if non_interactive:
        root_str = default_root
    else:
        root_str = _ask("Path to your 3D asset library", default_root)
    root = Path(root_str).expanduser()

    # Catalog DB
    default_db = str(Path.home() / ".local" / "share" / "game-asset-mcp" / "catalog.db")
    if non_interactive:
        db_str = default_db
    else:
        db_str = _ask("SQLite catalog DB path", default_db)

    # Detect styles
    print(f"\nScanning {root} for style directories...")
    detected = _detect_styles(root)
    style_map: dict[str, str] = {}

    if detected:
        print(f"Found {len(detected)} directories with GLBs:")
        for name, _ in detected:
            print(f"  - {name}")
        if non_interactive or _ask_bool("Use these as taxonomy styles?"):
            style_map = {name: name for name, _ in detected}

    if not style_map:
        # Fallback defaults
        style_map = {"3DLowPoly": "3DLowPoly", "3DPSX": "3DPSX"}

    # Source hints
    print("\nConfiguring asset source detection...")
    default_hints = {
        "kenney": "Kenney",
        "quaternius": "Quaternius",
        "kaykit": "KayKit",
        "kits": "KayKit",
        "custom": "Custom",
        "zappypixel": "Zappypixel",
        "polyhaven": "PolyHaven",
    }

    # Write config
    _CONFIG_DIR.mkdir(parents=True, exist_ok=True)

    style_map_toml = "\n".join(f'"{k}" = "{v}"' for k, v in style_map.items())
    source_hints_toml = "\n".join(f'{k} = "{v}"' for k, v in default_hints.items())

    config_content = textwrap.dedent(f"""\
        # game-asset-mcp configuration
        # Edit to customize your asset library setup.
        # Full docs: https://github.com/jbcom/agentic/tree/main/packages/game-asset-mcp

        [library]
        assets_root = "{root}"
        catalog_db = "{db_str}"

        [taxonomy.style_map]
        # Maps top-level directory prefix → style label
        {style_map_toml}

        [taxonomy.source_hints]
        # Maps lowercase path keywords → source name
        {source_hints_toml}

        [taxonomy.skip_dirs]
        # Directory names to skip during scan
        dirs = ["_Archive", "__pycache__", ".git", "node_modules", "Textures", "textures"]
    """)

    _CONFIG_FILE.write_text(config_content)
    print(f"\n✓ Config written to {_CONFIG_FILE}")
    print(f"  assets_root = {root}")
    print(f"  catalog_db  = {db_str}")
    print(f"  styles      = {list(style_map.keys())}")

    # Offer to run ingest
    if not non_interactive:
        do_ingest = _ask_bool("\nRun initial ingest now? (scans all GLBs — may take a minute)", True)
    else:
        do_ingest = True

    if do_ingest:
        print("\nRunning ingest...")
        try:
            subprocess.run(
                [sys.executable, "-m", "game_asset_mcp.ingest"],
                env={**__import__("os").environ, "ASSETS_ROOT": str(root), "CATALOG_DB": db_str},
                check=True,
            )
        except subprocess.CalledProcessError as exc:
            print(f"\n⚠ Ingest failed (exit {exc.returncode}). Run manually: game-asset-ingest")
    else:
        print("\nSkipped. Run later: game-asset-ingest")

    print("\n✓ Setup complete! Add to Claude Code:")
    print(f'  claude mcp add game-asset-library -e ASSETS_ROOT="{root}" -- game-asset-mcp\n')


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Interactive setup wizard for game-asset-mcp",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=textwrap.dedent("""\
            Examples:
              game-asset-init                          # interactive
              game-asset-init --root /mnt/assets       # pre-fill root, interactive for rest
              game-asset-init --root /mnt/assets --yes # non-interactive, accept defaults
        """),
    )
    parser.add_argument("--root", help="Asset library root directory")
    parser.add_argument(
        "--yes", "-y", action="store_true",
        help="Non-interactive: accept all defaults (good for CI/Docker)",
    )
    args = parser.parse_args()
    run_wizard(non_interactive=args.yes, assets_root=args.root)


if __name__ == "__main__":
    main()
