"""Connector Builder Crew

This script creates and inits the 'connector_builder' crew, which is
designed to automatically generate HTTP connector code by scraping API
documentation.
"""

from __future__ import annotations

from pathlib import Path

from agentic_crew.utils import load_config
from crewai import Agent, Crew, Task


class ConnectorBuilderCrew:
    """Manages the agents and tasks for the connector builder crew.

        This class loads agent and task configurations from YAML files,
        instantiates the necessary CrewAI components, and provides a method
    to
        execute the crew's workflow.

        Attributes:
            crew: An instance of the CrewAI Crew, configured with agents and
                  tasks for connector building.
    """

    def __init__(self, output_dir: str = "output"):
        """Initializes the ConnectorBuilderCrew.

        Loads agent and task configurations from YAML files, creates Agent
        and Task objects, and assembles them into a Crew.

        Args:
            output_dir: The directory where the generated connector code
                        will be saved.
        """
        config_dir = Path(__file__).parent / "config"
        agent_config = load_config(config_dir / "agents.yaml")
        task_config = load_config(config_dir / "tasks.yaml")

        # Create Agents
        self.doc_scraper = Agent(**agent_config["doc_scraper"])
        self.api_analyzer = Agent(**agent_config["api_analyzer"])
        self.code_generator = Agent(**agent_config["code_generator"])

        # Create Tasks
        self.scrape_docs = Task(**task_config["scrape_docs"])
        self.analyze_api = Task(**task_config["analyze_api"])

        generate_code_config = task_config["generate_code"].copy()
        generate_code_config["description"] = generate_code_config["description"].format(output_dir=output_dir)
        self.generate_code = Task(**generate_code_config)

        self.crew = Crew(
            agents=[self.doc_scraper, self.api_analyzer, self.code_generator],
            tasks=[self.scrape_docs, self.analyze_api, self.generate_code],
            verbose=2,
        )

    def kickoff(self, inputs: dict) -> str:
        """Starts the crew's execution with the given inputs.

        Args:
            inputs: A dictionary containing the necessary inputs for the
                    crew's tasks, such as the URL of the API documentation.

        Returns:
            A string representing the result of the crew's execution.
        """
        return self.crew.kickoff(inputs=inputs)
