"""Tests for the Connector Builder Crew."""

from __future__ import annotations

from unittest.mock import MagicMock, patch

from agentic_crew.crews.connector_builder.connector_builder_crew import ConnectorBuilderCrew


@patch("agentic_crew.crews.connector_builder.connector_builder_crew.Crew")
@patch("agentic_crew.crews.connector_builder.connector_builder_crew.Agent")
@patch("agentic_crew.crews.connector_builder.connector_builder_crew.Task")
def test_connector_builder_crew(
    mock_task: MagicMock,
    mock_agent: MagicMock,
    mock_crew: MagicMock,
):
    """Tests that the ConnectorBuilderCrew initializes correctly and can be kicked off."""
    # Test initialization
    crew_instance = ConnectorBuilderCrew(output_dir="test_output")

    assert mock_agent.call_count == 3
    assert mock_task.call_count == 3
    assert crew_instance.crew is not None
    mock_crew.assert_called_once()

    # Test kickoff
    mock_crew_instance = mock_crew.return_value
    mock_crew_instance.kickoff.return_value = "Success"

    result = crew_instance.kickoff(inputs={"url": "http://example.com"})

    mock_crew_instance.kickoff.assert_called_once_with(inputs={"url": "http://example.com"})
    assert result == "Success"
