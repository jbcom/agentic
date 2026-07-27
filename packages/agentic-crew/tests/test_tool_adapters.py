"""Tests for framework-specific configured tool adapters."""

from __future__ import annotations

import sys
from types import ModuleType, SimpleNamespace
from unittest.mock import MagicMock, patch

from agentic_crew.tools.adapters import resolve_langgraph_tools, resolve_strands_tools
from pydantic import BaseModel


class WriteFileArgs(BaseModel):
    """Schema used to verify adapter schema forwarding."""

    file_path: str


class DummyTool:
    """Simple tool object with a CrewAI-style `_run` entrypoint."""

    name = "Write Game Code File"
    description = "Write code into the workspace."
    args_schema = WriteFileArgs

    def __init__(self) -> None:
        self.calls: list[dict[str, str]] = []

    def _run(self, **kwargs: str) -> str:
        self.calls.append(kwargs)
        return f"wrote:{kwargs['file_path']}"


class FakeStructuredTool:
    """Minimal LangChain-style tool wrapper used for adapter tests."""

    def __init__(
        self,
        *,
        func,
        name: str,
        description: str,
        args_schema: type[BaseModel] | None,
        infer_schema: bool,
    ) -> None:
        self.func = func
        self.name = name
        self.description = description
        self.args_schema = args_schema
        self.infer_schema = infer_schema

    @classmethod
    def from_function(
        cls,
        *,
        func,
        name: str,
        description: str,
        args_schema: type[BaseModel] | None,
        infer_schema: bool,
    ) -> FakeStructuredTool:
        return cls(
            func=func,
            name=name,
            description=description,
            args_schema=args_schema,
            infer_schema=infer_schema,
        )

    def invoke(self, kwargs: dict[str, str]) -> str:
        return self.func(**kwargs)


def fake_strands_tool(*, name: str, description: str, inputSchema: dict | None):
    """Minimal Strands decorator used for adapter tests."""

    def decorator(func):
        def wrapped(**kwargs: str) -> str:
            return func(**kwargs)

        wrapped.__name__ = func.__name__
        wrapped.__doc__ = func.__doc__
        wrapped.TOOL_SPEC = {
            "name": name,
            "description": description,
            "inputSchema": inputSchema,
        }
        return wrapped

    return decorator


class TestLangGraphToolAdapters:
    """Tests for LangGraph configured-tool wrapping."""

    @patch("agentic_crew.tools.adapters.resolve_tools")
    def test_wraps_resolved_tools_with_structured_tool(self, mock_resolve_tools: MagicMock) -> None:
        """Resolved tools should be adapted into LangChain-compatible wrappers."""
        tool = DummyTool()
        mock_resolve_tools.return_value = [tool]

        fake_tools_module = ModuleType("langchain_core.tools")
        fake_tools_module.StructuredTool = FakeStructuredTool

        with patch.dict(sys.modules, {"langchain_core.tools": fake_tools_module}):
            adapted = resolve_langgraph_tools(["FileWriteTool"])

        assert len(adapted) == 1
        wrapped = adapted[0]
        assert wrapped.name == "Write_Game_Code_File"
        assert wrapped.description == "Write code into the workspace."
        assert wrapped.args_schema is WriteFileArgs
        assert wrapped.infer_schema is False
        assert wrapped.invoke({"file_path": "src/game.ts"}) == "wrote:src/game.ts"
        assert tool.calls == [{"file_path": "src/game.ts"}]

    @patch("agentic_crew.tools.adapters.resolve_tools")
    def test_preserves_existing_langgraph_tool_objects(self, mock_resolve_tools: MagicMock) -> None:
        """Existing invoke-capable tools should pass through unchanged."""
        existing_tool = SimpleNamespace(name="get_magic_number", invoke=MagicMock(return_value=42))
        mock_resolve_tools.return_value = [existing_tool]

        fake_tools_module = ModuleType("langchain_core.tools")
        fake_tools_module.StructuredTool = FakeStructuredTool

        with patch.dict(sys.modules, {"langchain_core.tools": fake_tools_module}):
            adapted = resolve_langgraph_tools(["get_magic_number"])

        assert adapted == [existing_tool]


class TestStrandsToolAdapters:
    """Tests for Strands configured-tool wrapping."""

    @patch("agentic_crew.tools.adapters.resolve_tools")
    def test_wraps_resolved_tools_with_strands_decorator(self, mock_resolve_tools: MagicMock) -> None:
        """Resolved tools should be adapted into callable Strands tool wrappers."""
        tool = DummyTool()
        mock_resolve_tools.return_value = [tool]

        fake_strands_module = ModuleType("strands")
        fake_strands_module.tool = fake_strands_tool

        with patch.dict(sys.modules, {"strands": fake_strands_module}):
            adapted = resolve_strands_tools(["FileWriteTool"])

        assert len(adapted) == 1
        wrapped = adapted[0]
        assert wrapped(file_path="src/game.ts") == "wrote:src/game.ts"
        assert tool.calls == [{"file_path": "src/game.ts"}]
        assert wrapped.TOOL_SPEC["name"] == "Write_Game_Code_File"
        assert wrapped.TOOL_SPEC["description"] == "Write code into the workspace."
        assert wrapped.TOOL_SPEC["inputSchema"] == WriteFileArgs.model_json_schema()

    @patch("agentic_crew.tools.adapters.resolve_tools")
    def test_preserves_existing_strands_tool_objects(self, mock_resolve_tools: MagicMock) -> None:
        """Existing Strands tool objects should pass through unchanged."""
        existing_tool = SimpleNamespace(TOOL_SPEC={"name": "get_secret_number"})
        mock_resolve_tools.return_value = [existing_tool]

        fake_strands_module = ModuleType("strands")
        fake_strands_module.tool = fake_strands_tool

        with patch.dict(sys.modules, {"strands": fake_strands_module}):
            adapted = resolve_strands_tools(["get_secret_number"])

        assert adapted == [existing_tool]
