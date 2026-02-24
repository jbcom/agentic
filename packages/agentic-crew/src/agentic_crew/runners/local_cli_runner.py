"""Universal local CLI runner for single-agent coding tools.

This runner provides a universal interface for any CLI-based coding agent,
eliminating the need to write custom runners for each tool. Just define the
CLI flags in local_cli_profiles.yaml and it works.

Supported tools (via profiles):
- Aider: AI pair programming
- Claude Code: Anthropic's coding agent
- OpenAI Codex: OpenAI's local agent
- Ollama: Free local LLMs (codellama, deepseek-coder, etc.)
- Custom: Define your own tool's CLI flags

Benefits:
- Zero code to add new agents
- Consistent interface across all tools
- Automatic auth env var handling
- Configurable modes (auto-approve, structured output)
"""

from __future__ import annotations

import os
import shlex
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import yaml

from agentic_crew.runners.single_agent_runner import SingleAgentRunner


@dataclass
class LocalCLIConfig:
    """Configuration for a CLI-based coding agent.

    This defines how to invoke any CLI tool that can execute coding tasks.
    All fields map to the tool's command-line interface.
    """

    # Basic command configuration
    command: str  # Base command (e.g., "aider", "claude", "ollama")
    task_flag: str  # How to pass task (e.g., "--message", "--print", "" for positional)

    # Optional subcommand (e.g., "run" for "ollama run")
    subcommand: str | None = None

    # Authentication
    auth_env: list[str] = field(default_factory=list)  # Required env vars

    # Mode controls
    auto_approve: str | None = None  # Auto-approve flag (e.g., "--yes-always")
    structured_output: str | None = None  # JSON output flag (e.g., "--json")

    # Model configuration
    model_flag: str | None = None  # How to specify model (e.g., "--model")
    default_model: str | None = None  # Default model if not specified

    # Working directory
    working_dir_flag: str | None = None  # Working dir flag (e.g., "--cwd")

    # Additional flags to always include
    additional_flags: list[str] = field(default_factory=list)

    # Execution settings
    timeout: int = 300  # Timeout in seconds

    # Metadata
    name: str = ""
    description: str = ""
    install_cmd: str = ""
    docs_url: str = ""
    notes: str = ""


class LocalCLIRunner(SingleAgentRunner):
    """Universal runner for CLI-based coding agents.

    This runner can invoke any CLI tool that accepts a task/prompt and
    executes it. Configuration is done via profiles (built-in or custom).

    Examples:
        # Use built-in profile
        runner = LocalCLIRunner("aider")
        result = runner.run("Add error handling to auth.py")

        # Use custom config
        config = LocalCLIConfig(
            command="my-tool",
            task_flag="--task",
            auto_approve="--yes",
        )
        runner = LocalCLIRunner(config)
        result = runner.run("Fix the bug")
    """

    # Cache for loaded profiles
    _profiles_cache: dict[str, LocalCLIConfig] | None = None

    def __init__(
        self,
        profile: str | LocalCLIConfig | dict[str, Any],
        model: str | None = None,
    ):
        """Initialize the runner with a profile or custom config.

        Args:
            profile: Profile name (e.g., "aider"), LocalCLIConfig object,
                    or dict of config parameters.
            model: Optional model override (if tool supports model selection).
        """
        if isinstance(profile, str):
            # Load from built-in profiles
            profiles = self._load_profiles()
            if profile not in profiles:
                available = list(profiles.keys())
                raise ValueError(
                    f"Unknown profile '{profile}'. Available: {available}\n"
                    f"Or provide a LocalCLIConfig object for custom tools."
                )
            self.config = profiles[profile]
        elif isinstance(profile, dict):
            # Convert dict to LocalCLIConfig
            self.config = LocalCLIConfig(**profile)
        else:
            # Use provided LocalCLIConfig directly
            self.config = profile

        self.model = model
        self.runner_name = f"local_cli_{self.config.command}"

    @classmethod
    def _load_profiles(cls) -> dict[str, LocalCLIConfig]:
        """Load CLI profiles from local_cli_profiles.yaml.

        Returns:
            Dict mapping profile names to LocalCLIConfig objects.
        """
        if cls._profiles_cache is not None:
            return cls._profiles_cache

        # Find the profiles file
        profiles_file = Path(__file__).parent / "local_cli_profiles.yaml"
        if not profiles_file.exists():
            raise FileNotFoundError(
                f"Profiles file not found: {profiles_file}\nExpected local_cli_profiles.yaml in runners directory."
            )

        # Load and parse YAML
        with open(profiles_file) as f:
            data = yaml.safe_load(f)

        profiles = {}
        for name, config_dict in data.get("profiles", {}).items():
            profiles[name] = LocalCLIConfig(**config_dict)

        cls._profiles_cache = profiles
        return profiles

    @classmethod
    def get_available_profiles(cls) -> list[str]:
        """Get list of available profile names.

        Returns:
            List of profile names that can be used.
        """
        return list(cls._load_profiles().keys())

    def is_available(self) -> bool:
        """Check if the CLI tool is available (installed and accessible).

        Returns:
            True if the command exists in PATH.
        """
        from shutil import which

        # Check if command is in PATH
        command = self.config.command.split()[0]
        return which(command) is not None

    def get_required_env_vars(self) -> list[str]:
        """Get list of required environment variables.

        Returns:
            List of environment variable names from config.auth_env.
        """
        return self.config.auth_env

    def run(
        self,
        task: str,
        working_dir: str | None = None,
        auto_approve: bool = True,
        structured_output: bool = False,
        model: str | None = None,
        **kwargs: Any,
    ) -> str:
        """Execute a task using the configured CLI tool.

        Args:
            task: The task to execute (e.g., "Add error handling").
            working_dir: Optional working directory for execution.
            auto_approve: Whether to auto-approve changes (if supported).
            structured_output: Whether to request JSON output (if supported).
            model: Optional model override.
            **kwargs: Additional tool-specific arguments.

        Returns:
            Tool output as a string.

        Raises:
            RuntimeError: If tool execution fails or required env vars missing.
            subprocess.TimeoutExpired: If execution exceeds timeout.
        """
        # Accept but don't use kwargs (reserved for future extensibility)
        _ = kwargs

        # Check required environment variables
        missing_vars = []
        for var in self.config.auth_env:
            if var not in os.environ:
                missing_vars.append(var)

        if missing_vars:
            raise RuntimeError(
                f"Missing required environment variables for {self.config.command}: "
                f"{', '.join(missing_vars)}\n"
                f"Set these before running: {', '.join(missing_vars)}"
            )

        # Build command
        cmd = self._build_command(
            task=task,
            working_dir=working_dir,
            auto_approve=auto_approve,
            structured_output=structured_output,
            model=model or self.model,
        )

        # Execute
        try:
            result = subprocess.run(
                cmd,
                cwd=working_dir,
                capture_output=True,
                text=True,
                timeout=self.config.timeout,
                env=os.environ.copy(),  # Pass through environment
                check=True,
            )

            return result.stdout

        except subprocess.CalledProcessError as e:
            raise RuntimeError(
                f"Command failed with exit code {e.returncode}\n"
                f"Command: {' '.join(e.cmd)}\n"
                f"stdout: {e.stdout}\n"
                f"stderr: {e.stderr}"
            ) from e
        except subprocess.TimeoutExpired as e:
            raise RuntimeError(f"Command timed out after {self.config.timeout}s\nCommand: {' '.join(cmd)}") from e

    def _build_command(
        self,
        task: str,
        working_dir: str | None,
        auto_approve: bool,
        structured_output: bool,
        model: str | None,
    ) -> list[str]:
        """Build the command array for subprocess.run.

        Args:
            task: The task to execute.
            working_dir: Optional working directory.
            auto_approve: Whether to include auto-approve flag.
            structured_output: Whether to include structured output flag.
            model: Optional model to use.

        Returns:
            Command as list of strings.
        """
        # Start with base command
        cmd = shlex.split(self.config.command)

        # Add subcommand if present (e.g., "ollama run")
        if self.config.subcommand:
            cmd.append(self.config.subcommand)

        # Add model if specified and tool supports it
        # For positional model (like ollama), add before task
        if model:
            if self.config.model_flag:
                cmd.extend([self.config.model_flag, model])
            else:
                # Positional model (e.g., "ollama run <model>")
                cmd.append(model)
        elif self.config.default_model and not self.config.model_flag:
            # Use default model for positional case
            cmd.append(self.config.default_model)

        # Add task
        if self.config.task_flag:
            cmd.extend([self.config.task_flag, task])
        else:
            # Positional argument
            cmd.append(task)

        # Add auto-approve flag if requested and supported
        if auto_approve and self.config.auto_approve:
            cmd.append(self.config.auto_approve)

        # Add structured output flag if requested and supported
        if structured_output and self.config.structured_output:
            cmd.append(self.config.structured_output)

        # Add working directory if supported
        if working_dir and self.config.working_dir_flag:
            cmd.extend([self.config.working_dir_flag, working_dir])

        # Add any additional flags
        cmd.extend(self.config.additional_flags)

        return cmd
