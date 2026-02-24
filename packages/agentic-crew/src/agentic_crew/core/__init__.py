"""Core CrewAI engine - discovery, loading, and running of package-defined crews."""

from __future__ import annotations

from agentic_crew.core.discovery import discover_packages, get_crew_config, load_manifest
from agentic_crew.core.loader import load_crew_from_config
from agentic_crew.core.manager import ManagerAgent
from agentic_crew.core.runner import run_crew

__all__ = [
    "discover_packages",
    "load_manifest",
    "get_crew_config",
    "load_crew_from_config",
    "run_crew",
    "ManagerAgent",
]
