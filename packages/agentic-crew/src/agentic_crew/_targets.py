"""Private, execution-local target binding for built-in file tools."""

from __future__ import annotations

import re
from collections.abc import Iterator
from contextlib import contextmanager
from contextvars import ContextVar
from pathlib import Path

_construction_root: ContextVar[Path | None] = ContextVar("crew_file_root", default=None)


def validate_package_name(name: str) -> str:
    """Accept a package identifier, never a path or an empty selection."""
    if not re.fullmatch(r"[A-Za-z0-9_][A-Za-z0-9_.-]*", name) or ".." in name:
        raise ValueError("Invalid package name. Select a package name from 'agentic-crew list'.")
    return name


def construction_root() -> Path | None:
    """Snapshot the selected root when a tool instance is constructed."""
    return _construction_root.get()


@contextmanager
def bind_file_root(root: Path) -> Iterator[None]:
    """Bind a discovered project for construction without changing process env."""
    root = root.resolve()
    if not root.is_dir():
        raise ValueError(f"Selected project directory does not exist: {root}")
    token = _construction_root.set(root)
    try:
        yield
    finally:
        _construction_root.reset(token)
