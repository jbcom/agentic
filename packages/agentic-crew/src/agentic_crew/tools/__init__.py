"""Custom tools for CrewAI game development crews.

This module provides file manipulation and web scraping tools.

For Meshy 3D asset generation tools, use mesh-toolkit directly:
    from mesh_toolkit.agent_tools.crewai import get_tools as get_meshy_tools

Usage:
    from agentic_crew.tools import (
        GameCodeReaderTool,
        GameCodeWriterTool,
        DirectoryListTool,
        CrawlWebsiteTool,
        ScrapeWebsiteTool,
        get_all_tools,
    )
"""

from __future__ import annotations

from agentic_crew.tools.file_tools import (
    DirectoryListTool,
    GameCodeReaderTool,
    GameCodeWriterTool,
)
from agentic_crew.tools.scraping_tools import CrawlWebsiteTool, ScrapeWebsiteTool


def get_file_tools():
    """Get the standard file manipulation tools.

    Returns:
        List of file tool instances
    """
    return [
        GameCodeReaderTool(),
        GameCodeWriterTool(),
        DirectoryListTool(),
    ]


def get_scraping_tools():
    """Get the web scraping tools.

    Returns:
        List of scraping tool instances
    """
    return [
        ScrapeWebsiteTool(),
        CrawlWebsiteTool(),
    ]


def get_all_tools():
    """Get all available tools.

    Returns file and scraping tools. For Meshy tools, use mesh_toolkit.agent_tools.crewai.

    Returns:
        List of all tool instances
    """
    tools = get_file_tools()
    tools.extend(get_scraping_tools())

    # Try to load mesh-toolkit tools if available
    try:
        from mesh_toolkit.agent_tools.crewai import get_tools as get_meshy_tools

        tools.extend(get_meshy_tools())
    except ImportError:
        pass  # mesh-toolkit not installed with crewai extra

    return tools


__all__ = [
    "DirectoryListTool",
    "GameCodeReaderTool",
    "GameCodeWriterTool",
    "CrawlWebsiteTool",
    "ScrapeWebsiteTool",
    "get_file_tools",
    "get_scraping_tools",
    "get_all_tools",
]
