"""Discovery module - finds packages with crew configuration directories.

Supports framework-specific configuration directories:
- .crewai/   - CrewAI-specific configurations (default)
- .langgraph/ - LangGraph-specific configurations
- .strands/  - Strands-specific configurations

The discovery order matches framework priority for auto-detection.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml

# Framework directory names in priority order
# .crew is framework-agnostic (can run on any available framework)
# Framework-specific dirs enforce that framework
FRAMEWORK_DIRS = [".crew", ".crewai", ".langgraph", ".strands"]

# Mapping from directory name to framework name
# .crew maps to None (framework-agnostic, auto-detect at runtime)
DIR_TO_FRAMEWORK: dict[str, str | None] = {
    ".crew": None,  # Framework-agnostic
    ".crewai": "crewai",
    ".langgraph": "langgraph",
    ".strands": "strands",
}

# Mapping from framework name to directory name
# None (agnostic) maps to .crew
FRAMEWORK_TO_DIR: dict[str | None, str] = {
    None: ".crew",
    "crewai": ".crewai",
    "langgraph": ".langgraph",
    "strands": ".strands",
}


def get_workspace_root() -> Path:
    """Get the workspace root directory.

    Walks up from the current file to find the root (where pyproject.toml is).
    """
    current = Path(__file__).resolve()
    for parent in current.parents:
        if (parent / "pyproject.toml").exists() and (parent / "packages").exists():
            return parent
    # Fallback to current working directory
    return Path.cwd()


def discover_packages(
    workspace_root: Path | None = None,
    framework: str | None = None,
) -> dict[str, Path]:
    """Discover all packages with crew configuration directories.

    Args:
        workspace_root: Root of the workspace. If None, auto-detected.
        framework: Optional framework to filter by (crewai, langgraph, strands).
                   If None, returns first found directory per package.

    Returns:
        Dict mapping package name to its config directory path.
    """
    if workspace_root is None:
        workspace_root = get_workspace_root()

    packages = {}

    # Determine which directories to look for
    if framework:
        dir_name = FRAMEWORK_TO_DIR.get(framework)
        dirs_to_check = [dir_name] if dir_name else FRAMEWORK_DIRS
    else:
        dirs_to_check = FRAMEWORK_DIRS

    # Check packages/ directory
    packages_dir = workspace_root / "packages"
    if packages_dir.exists():
        for pkg_dir in packages_dir.iterdir():
            if not pkg_dir.is_dir():
                continue

            # Try each framework directory in priority order
            for dir_name in dirs_to_check:
                config_dir = pkg_dir / dir_name
                if config_dir.exists() and (config_dir / "manifest.yaml").exists():
                    packages[pkg_dir.name] = config_dir
                    break  # Use first found (highest priority)

    # Also check workspace root for standalone projects
    for dir_name in dirs_to_check:
        config_dir = workspace_root / dir_name
        if config_dir.exists() and (config_dir / "manifest.yaml").exists():
            # Use the workspace name or a default
            pkg_name = workspace_root.name or "default"
            if pkg_name not in packages:
                packages[pkg_name] = config_dir
            break

    return packages


def discover_all_framework_configs(
    workspace_root: Path | None = None,
) -> dict[str, dict[str | None, Path]]:
    """Discover all framework-specific config directories for all packages.

    This finds ALL framework directories, not just the first one per package.

    Args:
        workspace_root: Root of the workspace. If None, auto-detected.

    Returns:
        Dict mapping package name to dict of framework -> config_dir.
        Example: {"otterfall": {"crewai": Path(...), "strands": Path(...)}}
        Framework can be None for framework-agnostic .crew/ directories.
    """
    if workspace_root is None:
        workspace_root = get_workspace_root()

    packages: dict[str, dict[str | None, Path]] = {}

    # Check packages/ directory
    packages_dir = workspace_root / "packages"
    if packages_dir.exists():
        for pkg_dir in packages_dir.iterdir():
            if not pkg_dir.is_dir():
                continue

            pkg_configs: dict[str | None, Path] = {}
            for dir_name in FRAMEWORK_DIRS:
                config_dir = pkg_dir / dir_name
                if config_dir.exists() and (config_dir / "manifest.yaml").exists():
                    framework = DIR_TO_FRAMEWORK[dir_name]
                    pkg_configs[framework] = config_dir

            if pkg_configs:
                packages[pkg_dir.name] = pkg_configs

    # Also check workspace root
    root_configs: dict[str | None, Path] = {}
    for dir_name in FRAMEWORK_DIRS:
        config_dir = workspace_root / dir_name
        if config_dir.exists() and (config_dir / "manifest.yaml").exists():
            framework = DIR_TO_FRAMEWORK[dir_name]
            root_configs[framework] = config_dir

    if root_configs:
        pkg_name = workspace_root.name or "default"
        if pkg_name not in packages:
            packages[pkg_name] = root_configs

    return packages


def load_manifest(crewai_dir: Path) -> dict[str, Any]:
    """Load a package's CrewAI manifest.

    Args:
        crewai_dir: Path to the .crewai/ directory.

    Returns:
        Parsed manifest as a dictionary.
    """
    manifest_path = crewai_dir / "manifest.yaml"
    with open(manifest_path) as f:
        result = yaml.safe_load(f)
        return result if result else {}


def get_framework_from_config_dir(config_dir: Path) -> str | None:
    """Determine the required framework from a config directory path.

    Args:
        config_dir: Path to a framework config directory (.crewai, .strands, etc.)

    Returns:
        Framework name if directory indicates a specific framework, None otherwise.
    """
    dir_name = config_dir.name
    return DIR_TO_FRAMEWORK.get(dir_name)


def get_crew_config(config_dir: Path, crew_name: str) -> dict:
    """Load a specific crew's configuration.

    Args:
        config_dir: Path to the config directory (.crewai/, .strands/, .langgraph/).
        crew_name: Name of the crew to load.

    Returns:
        Dict with agents, tasks, knowledge_paths, and required_framework.
        The required_framework field indicates which framework MUST be used
        if the config is in a framework-specific directory.

    Raises:
        ValueError: If crew not found in manifest.
    """
    manifest = load_manifest(config_dir)
    crews = manifest.get("crews", {})
    crew_config = crews.get(crew_name)

    if not crew_config:
        available = list(crews.keys())
        raise ValueError(f"Crew '{crew_name}' not found. Available: {available}")

    # Load agents and tasks YAML
    agents_path = config_dir / crew_config["agents"]
    tasks_path = config_dir / crew_config["tasks"]

    agents = yaml.safe_load(agents_path.read_text()) if agents_path.exists() else {}
    tasks = yaml.safe_load(tasks_path.read_text()) if tasks_path.exists() else {}

    # Resolve knowledge paths
    knowledge_paths = []
    for kp in crew_config.get("knowledge", []):
        full_path = config_dir / kp
        if full_path.exists():
            knowledge_paths.append(full_path)

    # Determine required framework from directory name
    # If config is in .crewai/, it MUST run on CrewAI, etc.
    required_framework = get_framework_from_config_dir(config_dir)

    # Also check manifest-level preferred_framework (can be overridden by dir)
    manifest_framework = crew_config.get("preferred_framework")
    if (
        manifest_framework
        and manifest_framework != "auto"
        and required_framework
        and manifest_framework != required_framework
    ):
        # Manifest can specify preference, but directory takes precedence
        print(
            f"Warning: Crew '{crew_name}' specifies preferred_framework={manifest_framework} "
            f"but is in {config_dir.name}/ directory which requires {required_framework}"
        )

    # Get LLM config from manifest
    llm_config = manifest.get("llm", crew_config.get("llm", {}))

    return {
        "name": crew_name,
        "description": crew_config.get("description", ""),
        "agents": agents,
        "tasks": tasks,
        "knowledge_paths": knowledge_paths,
        "manifest": manifest,
        "config_dir": config_dir,
        # Framework enforcement
        "required_framework": required_framework,
        "preferred_framework": manifest_framework,
        # LLM configuration
        "llm": llm_config,
    }


def list_crews(
    package_name: str | None = None,
    framework: str | None = None,
) -> dict[str, list[dict]]:
    """List all available crews, optionally filtered by package or framework.

    Args:
        package_name: If provided, only list crews for this package.
        framework: If provided, only list crews that can run on this framework.

    Returns:
        Dict mapping package name to list of crew info dicts.
        Each crew dict includes:
        - name: Crew name
        - description: Crew description
        - required_framework: Framework required (if in framework-specific dir)
    """
    packages = discover_packages(framework=framework)
    result = {}

    for pkg_name, config_dir in packages.items():
        if package_name and pkg_name != package_name:
            continue

        manifest = load_manifest(config_dir)
        required_framework = get_framework_from_config_dir(config_dir)

        crews = []
        for crew_name, crew_config in manifest.get("crews", {}).items():
            crews.append(
                {
                    "name": crew_name,
                    "description": crew_config.get("description", ""),
                    "required_framework": required_framework,
                    "preferred_framework": crew_config.get("preferred_framework"),
                }
            )
        result[pkg_name] = crews

    return result
