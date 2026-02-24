"""Tests for the scraping tools."""

from __future__ import annotations

from unittest.mock import MagicMock, patch

import pytest

pytest.importorskip("crewai", reason="crewai not installed")

from agentic_crew.tools.scraping_tools import CrawlWebsiteTool


@patch("agentic_crew.tools.scraping_tools.requests.get")
def test_crawl_website_tool(mock_get: MagicMock):
    """Tests that the CrawlWebsiteTool scrapes content and discovers links."""
    mock_response_page1 = MagicMock()
    mock_response_page1.content = b'<html><body><a href="/page2">Page 2</a><p>Content 1</p></body></html>'
    mock_response_page1.raise_for_status.return_value = None

    mock_response_page2 = MagicMock()
    mock_response_page2.content = b"<html><body><p>Content 2</p></body></html>"
    mock_response_page2.raise_for_status.return_value = None

    mock_get.side_effect = [mock_response_page1, mock_response_page2]

    tool = CrawlWebsiteTool()
    result = tool._run("http://example.com")

    assert "Content 1" in result
    assert "Page 2" in result
    assert "Content 2" in result
    assert mock_get.call_count == 2
