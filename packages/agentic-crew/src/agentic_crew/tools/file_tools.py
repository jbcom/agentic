"""File manipulation tools for CrewAI agents.

These tools enable agents to read and write code to specific directories
in game package codebases (e.g., packages/example-game).
"""

from __future__ import annotations

import os
from pathlib import Path

from crewai.tools import BaseTool
from pydantic import BaseModel, Field, PrivateAttr

from agentic_crew._targets import construction_root, validate_package_name


def _find_workspace_root() -> Path | None:
    """Search upward for workspace root (contains pyproject.toml with workspace)."""
    current = Path(__file__).resolve().parent
    for parent in [current] + list(current.parents):
        pyproject = parent / "pyproject.toml"
        if pyproject.exists():
            # Check if this is the workspace root (has packages/ directory)
            packages_dir = parent / "packages"
            if packages_dir.exists() and packages_dir.is_dir():
                return parent
    return None


def get_workspace_root(package_name: str | None = None) -> Path:
    """Get the workspace root directory for the target game code package.

    Returns packages/<package_name> as the workspace root, where the game code lives.

    Uses marker file search to find workspace root reliably, regardless of
    where this module is installed or imported from.

    Args:
        package_name: Name of the target package. If not provided,
            uses TARGET_PACKAGE or an identifiable standalone current directory.

    Returns:
        Path to the explicitly selected or standalone project directory.
    """
    # An execution-selected project wins over process environment defaults.
    selected_root = construction_root()
    if package_name is None and selected_root is not None:
        return selected_root
    if package_name is None:
        package_name = os.environ.get("TARGET_PACKAGE")

    if package_name is not None:
        validate_package_name(package_name)

    workspace_root = _find_workspace_root()
    cwd = Path.cwd().resolve()
    if package_name is not None:
        if workspace_root:
            packages_dir = (workspace_root / "packages").resolve()
            target_dir = (packages_dir / package_name).resolve()
            if target_dir.is_dir() and target_dir.is_relative_to(packages_dir):
                return target_dir

        env_root_var = f"{package_name.upper()}_ROOT"
        if env_root_var in os.environ:
            value = os.environ[env_root_var]
            target_dir = Path(value).resolve()
            if value.strip() and target_dir.is_dir():
                return target_dir
            raise ValueError(f"{env_root_var} must name an existing project directory.")
        if cwd.name == package_name and _is_standalone(cwd, workspace_root):
            return cwd
        raise ValueError(f"Package '{package_name}' not found. Set TARGET_PACKAGE and {env_root_var} explicitly.")

    if _is_standalone(cwd, workspace_root):
        return cwd
    raise ValueError("No unambiguous file target. Set TARGET_PACKAGE or pass an explicit package_name.")


def _is_standalone(cwd: Path, workspace_root: Path | None) -> bool:
    """Recognize a standalone cwd without choosing a monorepo package."""
    if workspace_root and cwd.is_relative_to((workspace_root / "packages").resolve()):
        return False
    packages = cwd / "packages"
    if packages.is_dir() and any(path.is_dir() for path in packages.iterdir()):
        return False
    return (cwd / "pyproject.toml").is_file() or any(
        (cwd / directory / "manifest.yaml").is_file() for directory in (".crew", ".crewai", ".langgraph", ".strands")
    )


class _ProjectFileTool(BaseTool):
    """Capture execution binding so a tool keeps its root in worker threads."""

    _bound_root: Path | None = PrivateAttr(default_factory=construction_root)

    def _file_root(self) -> Path:
        return self._bound_root if self._bound_root is not None else get_workspace_root()

    def _file_path(self, relative_path: str) -> Path:
        root = self._file_root().resolve()
        path = (root / relative_path).resolve()
        if not path.is_relative_to(root):
            raise ValueError("Path traversal outside the selected project is not allowed.")
        return path


# Allowed directories for writing (relative to the target package)
ALLOWED_WRITE_DIRS = [
    "src/ecs",  # ECS components, world, data
    "src/ecs/data",  # Species definitions, etc.
    "src/ecs/systems",  # ECS systems
    "src/components",  # React/R3F components
    "src/components/ui",  # UI components
    "src/stores",  # Zustand stores
    "src/systems",  # Non-ECS systems
    "src/utils",  # Utility functions
    "src/types",  # TypeScript types
]

# Allowed file extensions
ALLOWED_EXTENSIONS = {".ts", ".tsx", ".json", ".md"}


class WriteFileInput(BaseModel):
    """Input schema for GameCodeWriterTool."""

    file_path: str = Field(description="Relative path from workspace root (e.g., 'src/ecs/data/NewComponent.ts')")
    content: str = Field(description="The TypeScript/TSX code content to write")


class GameCodeWriterTool(_ProjectFileTool):
    """Tool for writing code files to a game package codebase.

    This tool is restricted to specific directories to ensure agents
    only modify appropriate parts of the codebase.
    """

    name: str = "Write Game Code File"
    description: str = """
    Write a code file to the target game codebase (e.g., packages/<target_package>).

    Discovered crews bind this tool to their selected project at construction.
    Direct use supports TARGET_PACKAGE or an identifiable standalone directory.

    ALLOWED DIRECTORIES:
    - src/ecs - ECS components, world definition
    - src/ecs/data - Species data, biome configs
    - src/ecs/systems - ECS systems
    - src/components - React Three Fiber components
    - src/components/ui - UI components (menus, HUD)
    - src/stores - Zustand state stores
    - src/systems - Non-ECS game systems
    - src/utils - Utility functions
    - src/types - TypeScript type definitions

    ALLOWED EXTENSIONS: .ts, .tsx, .json, .md

    Example:
        file_path: "src/ecs/data/species.ts"
        content: "export const PREDATOR_SPECIES = { ... }"
    """
    args_schema: type[BaseModel] = WriteFileInput

    def _run(self, file_path: str, content: str) -> str:
        """Write the file content to the specified path."""
        try:
            # Validate path
            clean_path = file_path.strip().replace("\\", "/")

            # Check for path traversal
            if ".." in clean_path or clean_path.startswith("/"):
                return f"Error: Invalid path '{clean_path}'. Path traversal not allowed."

            # Check allowed directories
            is_allowed = any(clean_path.startswith(allowed_dir) for allowed_dir in ALLOWED_WRITE_DIRS)
            if not is_allowed:
                return f"Error: Path '{clean_path}' is not in an allowed directory. Allowed: {ALLOWED_WRITE_DIRS}"

            # Check extension
            ext = Path(clean_path).suffix.lower()
            if ext not in ALLOWED_EXTENSIONS:
                return f"Error: Extension '{ext}' not allowed. Allowed: {ALLOWED_EXTENSIONS}"

            # Construct full path
            full_path = self._file_path(clean_path)

            # Create parent directories
            full_path.parent.mkdir(parents=True, exist_ok=True)

            # Write content
            with open(full_path, "w", encoding="utf-8") as f:
                f.write(content)

            return f"Successfully wrote {len(content)} bytes to {clean_path}"

        except PermissionError:
            return f"Error: Permission denied writing to {file_path}"
        except Exception as e:
            return f"Error writing file: {e!s}"


class ReadFileInput(BaseModel):
    """Input schema for GameCodeReaderTool."""

    file_path: str = Field(description="Relative path from workspace root (e.g., 'src/ecs/components.ts')")


class GameCodeReaderTool(_ProjectFileTool):
    """Tool for reading code files from a game package codebase.

    Use this to understand existing patterns before writing new code.
    """

    name: str = "Read Game Code File"
    description: str = """
    Read a code file from the target package's codebase.

    Discovered crews bind this tool to their selected project at construction.
    Direct use supports TARGET_PACKAGE or an identifiable standalone directory.

    Use this tool to:
    - Understand existing patterns
    - See how similar components are structured
    - Check imports and dependencies

    Example:
        file_path: "src/ecs/components.ts"
    """
    args_schema: type[BaseModel] = ReadFileInput

    def _run(self, file_path: str) -> str:
        """Read the file content from the specified path."""
        try:
            clean_path = file_path.strip().replace("\\", "/")

            if ".." in clean_path:
                return f"Error: Path traversal not allowed in '{clean_path}'"

            full_path = self._file_path(clean_path)

            if not full_path.exists():
                return f"Error: File not found: {clean_path}"

            if not full_path.is_file():
                return f"Error: Path is not a file: {clean_path}"

            # Limit file size
            if full_path.stat().st_size > 100_000:  # 100KB limit
                return f"Error: File too large (>{100_000} bytes)"

            with open(full_path, encoding="utf-8") as f:
                content = f.read()

            return content

        except PermissionError:
            return f"Error: Permission denied reading {file_path}"
        except Exception as e:
            return f"Error reading file: {e!s}"


class ListDirInput(BaseModel):
    """Input schema for DirectoryListTool."""

    directory: str = Field(description="Relative directory path from workspace root (e.g., 'src/ecs')")


class DirectoryListTool(_ProjectFileTool):
    """Tool for listing files in a directory.

    Use this to discover existing files and understand project structure.
    """

    name: str = "List Directory Contents"
    description: str = """
    List files and subdirectories in the target package codebase.

    Discovered crews bind this tool to their selected project at construction.
    Direct use supports TARGET_PACKAGE or an identifiable standalone directory.

    Use this to:
    - Discover existing components
    - Understand project structure
    - Find files to read or reference

    Example:
        directory: "src/ecs/data"
    """
    args_schema: type[BaseModel] = ListDirInput

    def _run(self, directory: str) -> str:
        """List directory contents."""
        try:
            clean_path = directory.strip().replace("\\", "/")

            if ".." in clean_path:
                return "Error: Path traversal not allowed"

            full_path = self._file_path(clean_path)

            if not full_path.exists():
                return f"Error: Directory not found: {clean_path}"

            if not full_path.is_dir():
                return f"Error: Path is not a directory: {clean_path}"

            entries = []
            for entry in sorted(full_path.iterdir()):
                if entry.name.startswith("."):
                    continue
                prefix = "📁" if entry.is_dir() else "📄"
                entries.append(f"{prefix} {entry.name}")

            if not entries:
                return f"Directory {clean_path} is empty"

            return f"Contents of {clean_path}:\n" + "\n".join(entries)

        except PermissionError:
            return f"Error: Permission denied accessing {directory}"
        except Exception as e:
            return f"Error listing directory: {e!s}"
