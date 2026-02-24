"""LangGraph runner implementation.

LangGraph excels at:
- Complex conditional flows
- State management
- Cycles and loops
- Integration with LangChain ecosystem
"""

from __future__ import annotations

from typing import Any

from agentic_crew.runners.base import BaseRunner


class LangGraphRunner(BaseRunner):
    """Runner that uses LangGraph for crew execution."""

    framework_name = "langgraph"

    def __init__(self):
        """Initialize LangGraph runner."""
        try:
            import langgraph  # noqa: F401
        except ImportError as e:
            raise RuntimeError("LangGraph not installed. Install with: pip install langgraph") from e

    def build_crew(self, crew_config: dict[str, Any]) -> Any:
        """Build a LangGraph workflow from configuration.

        Converts crew format to a LangGraph StateGraph with agents as nodes
        and tasks as edges.

        Args:
            crew_config: Universal crew configuration.

        Returns:
            Compiled LangGraph StateGraph.
        """
        from langgraph.prebuilt import create_react_agent

        # For simple crews, create a ReAct agent
        # More complex crews could be converted to full StateGraphs

        # Get LLM from config (respects the framework's configuration)
        llm_config = crew_config.get("llm", {})
        model = llm_config.get("model") if isinstance(llm_config, dict) else llm_config
        llm = self.get_llm(model)

        # Build tools from task descriptions
        tools = self._build_tools_from_tasks(crew_config)

        # Create agent
        agent = create_react_agent(llm, tools)

        return agent

    def run(self, crew: Any, inputs: dict[str, Any]) -> str:
        """Execute the LangGraph workflow.

        Args:
            crew: Compiled LangGraph.
            inputs: Inputs for the workflow.

        Returns:
            Workflow output as string.
        """
        # Convert inputs to messages format
        user_message = inputs.get("input", inputs.get("task", str(inputs)))

        result = crew.invoke({"messages": [("user", user_message)]})

        # Extract final message
        messages = result.get("messages", [])
        if messages:
            final = messages[-1]
            return final.content if hasattr(final, "content") else str(final)

        return str(result)

    def get_llm(self, model: str | None = None) -> Any:
        """Get LangChain-compatible LLM.

        Args:
            model: Optional model name override.

        Returns:
            LangChain ChatAnthropic LLM.
        """
        from langchain_anthropic import ChatAnthropic

        # Default to Claude 3.5 Sonnet if no model specified
        default_model = "claude-haiku-4-5-20251001"
        return ChatAnthropic(model=model or default_model)

    def build_agent(self, agent_config: dict[str, Any], tools: list | None = None) -> Any:
        """Build a LangGraph-compatible agent.

        Args:
            agent_config: Agent configuration.
            tools: Optional tools.

        Returns:
            LangGraph agent.
        """
        from langgraph.prebuilt import create_react_agent

        # Get LLM from agent config if specified
        llm = self.get_llm(agent_config.get("llm"))
        return create_react_agent(llm, tools or [])

    def build_task(self, task_config: dict[str, Any], agent: Any) -> Any:
        """Build a task representation for LangGraph.

        In LangGraph, tasks are typically represented as graph nodes or
        prompts to agents. Returns a dict for now.

        Args:
            task_config: Task configuration.
            agent: Agent for the task.

        Returns:
            Task configuration dict with agent reference.
        """
        return {
            "description": task_config.get("description", ""),
            "expected_output": task_config.get("expected_output", ""),
            "agent": agent,
        }

    def _build_tools_from_tasks(self, crew_config: dict[str, Any]) -> list:
        """Convert crew tasks to LangChain tools.

        For simple crews, we create tools that represent each task's capability.

        Args:
            crew_config: Crew configuration.

        Returns:
            List of LangChain tools.
        """
        # For now, return empty - tools should be provided separately
        # A more sophisticated implementation would create tools from task definitions
        return []
