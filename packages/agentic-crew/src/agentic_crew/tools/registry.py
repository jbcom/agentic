"""Tool resolution helpers for configured crew tools.

This module turns YAML-declared tool names into instantiated tool objects
without forcing every optional dependency to import at module load time.
"""

from __future__ import annotations

import importlib
import logging
from collections.abc import Callable
from typing import Any

logger = logging.getLogger(__name__)

ToolFactory = Callable[[], Any]


def _build_factory(module_name: str, attr_name: str) -> ToolFactory:
    """Create a lazy factory for a tool class or callable."""

    def factory() -> Any:
        module = importlib.import_module(module_name)
        tool = getattr(module, attr_name)
        return tool() if callable(tool) else tool

    return factory


_TOOL_FACTORIES: dict[str, ToolFactory] = {
    "GameCodeWriterTool": _build_factory("agentic_crew.tools.file_tools", "GameCodeWriterTool"),
    "GameCodeReaderTool": _build_factory("agentic_crew.tools.file_tools", "GameCodeReaderTool"),
    "DirectoryListTool": _build_factory("agentic_crew.tools.file_tools", "DirectoryListTool"),
    "ScrapeWebsiteTool": _build_factory("crewai_tools", "ScrapeWebsiteTool"),
    "CrawlWebsiteTool": _build_factory("agentic_crew.tools.scraping_tools", "CrawlWebsiteTool"),
}

_TOOL_ALIASES = {
    "FileWriteTool": "GameCodeWriterTool",
    "WriteFileTool": "GameCodeWriterTool",
    "FileReadTool": "GameCodeReaderTool",
    "ReadFileTool": "GameCodeReaderTool",
    "ListDirectoryTool": "DirectoryListTool",
    "ListFilesTool": "DirectoryListTool",
    "mcp://filesystem/write_file": "GameCodeWriterTool",
    "mcp://filesystem/read_file": "GameCodeReaderTool",
    "mcp://filesystem/list_directory": "DirectoryListTool",
}


def _canonical_tool_name(tool_name: str) -> str:
    """Normalize aliases to a canonical tool identifier."""
    return _TOOL_ALIASES.get(tool_name, tool_name)


def resolve_tool(tool_name: str) -> Any | None:
    """Resolve a configured tool name to an instantiated tool object.

    Supports:
    - built-in aliases like ``FileWriteTool``
    - selected filesystem MCP URIs
    - fully-qualified ``module:attribute`` references
    - fully-qualified ``package.module.ClassName`` references
    """

    canonical_name = _canonical_tool_name(tool_name)

    if canonical_name in _TOOL_FACTORIES:
        return _TOOL_FACTORIES[canonical_name]()

    if canonical_name.startswith("mcp://"):
        return None

    try:
        if ":" in canonical_name:
            module_name, attr_name = canonical_name.split(":", 1)
            return _build_factory(module_name, attr_name)()

        if "." in canonical_name:
            module_name, _, attr_name = canonical_name.rpartition(".")
            if module_name and attr_name:
                return _build_factory(module_name, attr_name)()
    except (ImportError, AttributeError) as exc:
        logger.warning("Failed to resolve tool '%s': %s", tool_name, exc)
        return None

    return None


def resolve_tools(tool_names: list[str]) -> list[Any]:
    """Resolve configured tool names to instantiated tool objects.

    Unknown or unavailable tools are skipped with a warning so that crews can
    continue to run with the capabilities available in the current environment.
    """

    resolved: list[Any] = []
    seen: set[str] = set()

    for tool_name in tool_names:
        canonical_name = _canonical_tool_name(tool_name)
        if canonical_name in seen:
            continue

        tool = resolve_tool(tool_name)
        if tool is None:
            logger.warning("Skipping unresolved tool '%s'", tool_name)
            continue

        resolved.append(tool)
        seen.add(canonical_name)

    return resolved
