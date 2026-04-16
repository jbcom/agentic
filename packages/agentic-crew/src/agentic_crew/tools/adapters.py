"""Framework-specific adapters for configured tools."""

from __future__ import annotations

import inspect
import logging
import re
from typing import Any

from agentic_crew.tools.registry import resolve_tools

logger = logging.getLogger(__name__)


def _tool_name(tool_obj: Any) -> str:
    """Build a framework-safe tool name."""
    raw_name = getattr(tool_obj, "name", None) or getattr(tool_obj, "__name__", None) or tool_obj.__class__.__name__
    cleaned = re.sub(r"[^0-9A-Za-z_]+", "_", str(raw_name)).strip("_") or "tool"
    if cleaned[0].isdigit():
        cleaned = f"tool_{cleaned}"
    return cleaned


def _tool_description(tool_obj: Any) -> str:
    """Get the best available description for a tool object."""
    return str(
        getattr(tool_obj, "description", None)
        or inspect.getdoc(tool_obj)
        or f"Execute the {_tool_name(tool_obj)} tool."
    )


def _invoke_tool(tool_obj: Any, kwargs: dict[str, Any]) -> Any:
    """Execute a configured tool object."""
    if hasattr(tool_obj, "_run"):
        return tool_obj._run(**kwargs)
    if callable(tool_obj):
        return tool_obj(**kwargs)
    raise TypeError(f"Tool {tool_obj!r} is not callable")


def _build_runner(tool_obj: Any, name: str, description: str) -> Any:
    """Create a plain callable wrapper for a configured tool."""

    def runner(**kwargs: Any) -> Any:
        return _invoke_tool(tool_obj, kwargs)

    runner.__name__ = name
    runner.__doc__ = description
    return runner


def resolve_langgraph_tools(tool_names: list[str]) -> list[Any]:
    """Resolve configured tool names to LangGraph-compatible tools."""
    if not tool_names:
        return []

    resolved_tools = resolve_tools(tool_names)
    if not resolved_tools:
        return []

    try:
        from langchain_core.tools import StructuredTool
    except ImportError as exc:
        logger.warning("LangGraph tool adaptation unavailable; skipping configured tools: %s", exc)
        return []

    adapted_tools: list[Any] = []
    for tool_obj in resolved_tools:
        if hasattr(tool_obj, "invoke") and hasattr(tool_obj, "name"):
            adapted_tools.append(tool_obj)
            continue

        name = _tool_name(tool_obj)
        description = _tool_description(tool_obj)
        args_schema = getattr(tool_obj, "args_schema", None)
        runner = _build_runner(tool_obj, name, description)

        adapted_tools.append(
            StructuredTool.from_function(
                func=runner,
                name=name,
                description=description,
                args_schema=args_schema,
                infer_schema=args_schema is None,
            )
        )

    return adapted_tools


def resolve_strands_tools(tool_names: list[str]) -> list[Any]:
    """Resolve configured tool names to Strands-compatible tools."""
    if not tool_names:
        return []

    resolved_tools = resolve_tools(tool_names)
    if not resolved_tools:
        return []

    try:
        from strands import tool as strands_tool
    except ImportError as exc:
        logger.warning("Strands tool adaptation unavailable; skipping configured tools: %s", exc)
        return []

    adapted_tools: list[Any] = []
    for tool_obj in resolved_tools:
        if hasattr(tool_obj, "TOOL_SPEC"):
            adapted_tools.append(tool_obj)
            continue

        name = _tool_name(tool_obj)
        description = _tool_description(tool_obj)
        args_schema = getattr(tool_obj, "args_schema", None)
        input_schema = args_schema.model_json_schema() if args_schema is not None else None
        runner = _build_runner(tool_obj, name, description)

        adapted_tools.append(strands_tool(name=name, description=description, inputSchema=input_schema)(runner))

    return adapted_tools
