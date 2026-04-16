"""Tests for configured tool resolution."""

from __future__ import annotations

from unittest.mock import MagicMock, patch

from agentic_crew.tools.registry import resolve_tool, resolve_tools


class TestResolveTool:
    """Tests for resolving configured tool names."""

    @patch("agentic_crew.tools.registry.importlib.import_module")
    def test_resolves_builtin_alias(self, mock_import_module: MagicMock) -> None:
        """FileWriteTool should map to the game code writer implementation."""
        mock_tool_class = MagicMock()
        mock_tool_instance = MagicMock()
        mock_tool_class.return_value = mock_tool_instance

        mock_module = MagicMock()
        mock_module.GameCodeWriterTool = mock_tool_class
        mock_import_module.return_value = mock_module

        result = resolve_tool("FileWriteTool")

        mock_import_module.assert_called_once_with("agentic_crew.tools.file_tools")
        mock_tool_class.assert_called_once_with()
        assert result is mock_tool_instance

    @patch("agentic_crew.tools.registry.importlib.import_module")
    def test_resolves_fully_qualified_reference(self, mock_import_module: MagicMock) -> None:
        """module:attribute references should be resolved dynamically."""
        mock_tool_class = MagicMock()
        mock_tool_instance = MagicMock()
        mock_tool_class.return_value = mock_tool_instance

        mock_module = MagicMock()
        mock_module.CustomTool = mock_tool_class
        mock_import_module.return_value = mock_module

        result = resolve_tool("custom.module:CustomTool")

        mock_import_module.assert_called_once_with("custom.module")
        mock_tool_class.assert_called_once_with()
        assert result is mock_tool_instance

    def test_returns_none_for_unresolved_mcp_tool(self) -> None:
        """Unsupported MCP tool references should be skipped cleanly."""
        assert resolve_tool("mcp://git/execute_command") is None

    @patch("agentic_crew.tools.registry.importlib.import_module")
    def test_resolve_tools_deduplicates_aliases(self, mock_import_module: MagicMock) -> None:
        """Aliases pointing to the same tool should not instantiate duplicates."""
        mock_tool_class = MagicMock()
        mock_tool_class.return_value = MagicMock()

        mock_module = MagicMock()
        mock_module.GameCodeReaderTool = mock_tool_class
        mock_import_module.return_value = mock_module

        tools = resolve_tools(["FileReadTool", "mcp://filesystem/read_file", "FileReadTool"])

        assert len(tools) == 1
        mock_tool_class.assert_called_once_with()
