"""Tests for single-agent CLI runners."""

from __future__ import annotations

import subprocess
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from agentic_crew.runners.local_cli_runner import LocalCLIConfig, LocalCLIRunner
from agentic_crew.runners.single_agent_runner import SingleAgentRunner


class TestSingleAgentRunner:
    """Test the SingleAgentRunner base class."""

    def test_is_abstract(self):
        """SingleAgentRunner should be abstract."""
        with pytest.raises(TypeError):
            SingleAgentRunner()

    def test_default_is_available(self):
        """Default is_available should return True."""

        class TestRunner(SingleAgentRunner):
            def run(self, task: str, working_dir: str | None = None, **kwargs):
                return "test"

        runner = TestRunner()
        assert runner.is_available() is True

    def test_default_get_required_env_vars(self):
        """Default get_required_env_vars should return empty list."""

        class TestRunner(SingleAgentRunner):
            def run(self, task: str, working_dir: str | None = None, **kwargs):
                return "test"

        runner = TestRunner()
        assert runner.get_required_env_vars() == []


class TestLocalCLIConfig:
    """Test the LocalCLIConfig dataclass."""

    def test_minimal_config(self):
        """Should create config with minimal required fields."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
        )

        assert config.command == "test-tool"
        assert config.task_flag == "--task"
        assert config.auth_env == []
        assert config.timeout == 300

    def test_full_config(self):
        """Should create config with all fields."""
        config = LocalCLIConfig(
            command="aider",
            task_flag="--message",
            subcommand=None,
            auth_env=["OPENAI_API_KEY"],
            auto_approve="--yes-always",
            structured_output="--json",
            model_flag="--model",
            default_model="gpt-4o",
            working_dir_flag="--cwd",
            additional_flags=["--no-git"],
            timeout=600,
            name="Aider",
            description="AI pair programming",
            install_cmd="pipx install aider-chat",
        )

        assert config.command == "aider"
        assert config.auth_env == ["OPENAI_API_KEY"]
        assert config.auto_approve == "--yes-always"
        assert config.timeout == 600


class TestLocalCLIRunner:
    """Test the LocalCLIRunner implementation."""

    @pytest.fixture
    def mock_profiles_file(self, tmp_path: Path):
        """Create a temporary profiles YAML file."""
        profiles_content = """
profiles:
  test-tool:
    name: "Test Tool"
    description: "A test tool"
    command: "test-tool"
    task_flag: "--task"
    auth_env:
      - "TEST_API_KEY"
    auto_approve: "--yes"
    timeout: 120
    install_cmd: "pip install test-tool"

  aider:
    name: "Aider"
    description: "AI pair programming"
    command: "aider"
    task_flag: "--message"
    auth_env:
      - "OPENAI_API_KEY"
    auto_approve: "--yes-always"
    model_flag: "--model"
    additional_flags:
      - "--no-git"
    timeout: 300
    install_cmd: "pipx install aider-chat"
"""
        profiles_file = tmp_path / "local_cli_profiles.yaml"
        profiles_file.write_text(profiles_content)
        return profiles_file

    def test_load_profiles(self, mock_profiles_file: Path):
        """Should load profiles from YAML file."""
        with patch("agentic_crew.runners.local_cli_runner.Path") as mock_path:
            mock_path.return_value.parent = mock_profiles_file.parent
            mock_path.return_value.__truediv__.return_value = mock_profiles_file

            # Clear cache
            LocalCLIRunner._profiles_cache = None

            profiles = LocalCLIRunner._load_profiles()

            assert "test-tool" in profiles
            assert "aider" in profiles
            assert profiles["test-tool"].command == "test-tool"
            assert profiles["aider"].auth_env == ["OPENAI_API_KEY"]

    def test_init_with_profile_name(self, mock_profiles_file: Path):
        """Should initialize with a profile name."""
        with patch("agentic_crew.runners.local_cli_runner.Path") as mock_path:
            mock_path.return_value.parent = mock_profiles_file.parent
            mock_path.return_value.__truediv__.return_value = mock_profiles_file

            LocalCLIRunner._profiles_cache = None

            runner = LocalCLIRunner("aider")

            assert runner.config.command == "aider"
            assert runner.config.task_flag == "--message"

    def test_init_with_unknown_profile(self, mock_profiles_file: Path):
        """Should raise ValueError for unknown profile."""
        with patch("agentic_crew.runners.local_cli_runner.Path") as mock_path:
            mock_path.return_value.parent = mock_profiles_file.parent
            mock_path.return_value.__truediv__.return_value = mock_profiles_file

            LocalCLIRunner._profiles_cache = None

            with pytest.raises(ValueError, match="Unknown profile"):
                LocalCLIRunner("unknown-tool")

    def test_init_with_config_dict(self):
        """Should initialize with a config dict."""
        config_dict = {
            "command": "my-tool",
            "task_flag": "--prompt",
            "auto_approve": "--yes",
        }

        runner = LocalCLIRunner(config_dict)

        assert runner.config.command == "my-tool"
        assert runner.config.task_flag == "--prompt"
        assert runner.config.auto_approve == "--yes"

    def test_init_with_config_object(self):
        """Should initialize with a LocalCLIConfig object."""
        config = LocalCLIConfig(
            command="custom-tool",
            task_flag="--task",
        )

        runner = LocalCLIRunner(config)

        assert runner.config.command == "custom-tool"
        assert runner.config.task_flag == "--task"

    def test_is_available_tool_exists(self):
        """Should return True if tool is in PATH."""
        config = LocalCLIConfig(command="python", task_flag="--task")
        runner = LocalCLIRunner(config)

        assert runner.is_available() is True

    def test_is_available_tool_missing(self):
        """Should return False if tool is not in PATH."""
        config = LocalCLIConfig(command="nonexistent-tool-xyz", task_flag="--task")
        runner = LocalCLIRunner(config)

        assert runner.is_available() is False

    def test_get_required_env_vars(self):
        """Should return required env vars from config."""
        config = LocalCLIConfig(
            command="test",
            task_flag="--task",
            auth_env=["API_KEY", "SECRET"],
        )
        runner = LocalCLIRunner(config)

        assert runner.get_required_env_vars() == ["API_KEY", "SECRET"]

    def test_run_missing_env_vars(self):
        """Should raise RuntimeError if required env vars missing."""
        config = LocalCLIConfig(
            command="test",
            task_flag="--task",
            auth_env=["MISSING_ENV_VAR"],
        )
        runner = LocalCLIRunner(config)

        with pytest.raises(RuntimeError, match="Missing required environment variables"):
            runner.run("test task")

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_success(self, mock_run: MagicMock):
        """Should execute command successfully."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
        )
        runner = LocalCLIRunner(config)

        # Mock successful execution
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="Test output",
            stderr="",
        )

        result = runner.run("Fix the bug")

        assert result == "Test output"
        mock_run.assert_called_once()

        # Check command was built correctly
        call_args = mock_run.call_args
        assert call_args[0][0] == ["test-tool", "--task", "Fix the bug"]

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_with_auto_approve(self, mock_run: MagicMock):
        """Should include auto-approve flag when enabled."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
            auto_approve="--yes",
        )
        runner = LocalCLIRunner(config)

        mock_run.return_value = MagicMock(returncode=0, stdout="Output", stderr="")

        runner.run("Test task", auto_approve=True)

        call_args = mock_run.call_args
        assert "--yes" in call_args[0][0]

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_with_structured_output(self, mock_run: MagicMock):
        """Should include structured output flag when enabled."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
            structured_output="--json",
        )
        runner = LocalCLIRunner(config)

        mock_run.return_value = MagicMock(returncode=0, stdout="Output", stderr="")

        runner.run("Test task", structured_output=True)

        call_args = mock_run.call_args
        assert "--json" in call_args[0][0]

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_with_model(self, mock_run: MagicMock):
        """Should include model flag when model specified."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
            model_flag="--model",
        )
        runner = LocalCLIRunner(config)

        mock_run.return_value = MagicMock(returncode=0, stdout="Output", stderr="")

        runner.run("Test task", model="gpt-4o")

        call_args = mock_run.call_args
        cmd = call_args[0][0]
        assert "--model" in cmd
        assert "gpt-4o" in cmd

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_with_additional_flags(self, mock_run: MagicMock):
        """Should include additional flags from config."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
            additional_flags=["--no-cache", "--verbose"],
        )
        runner = LocalCLIRunner(config)

        mock_run.return_value = MagicMock(returncode=0, stdout="Output", stderr="")

        runner.run("Test task")

        call_args = mock_run.call_args
        cmd = call_args[0][0]
        assert "--no-cache" in cmd
        assert "--verbose" in cmd

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_failure(self, mock_run: MagicMock):
        """Should raise RuntimeError on command failure."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
        )
        runner = LocalCLIRunner(config)

        # Mock failed execution with CalledProcessError (raised when check=True)
        mock_run.side_effect = subprocess.CalledProcessError(
            returncode=1,
            cmd=["test-tool", "--task", "Test task"],
            output="",
            stderr="Error occurred",
        )

        with pytest.raises(RuntimeError, match="Command failed with exit code 1"):
            runner.run("Test task")

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_run_timeout(self, mock_run: MagicMock):
        """Should raise RuntimeError on timeout."""
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="--task",
            auth_env=["TEST_API_KEY"],
            timeout=1,
        )
        runner = LocalCLIRunner(config)

        # Mock timeout
        mock_run.side_effect = subprocess.TimeoutExpired("test-tool", 1)

        with pytest.raises(RuntimeError, match="Command timed out"):
            runner.run("Test task")

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_build_command_positional_task(self, mock_run: MagicMock):
        """Should handle positional task argument (no flag)."""
        # Note: command can contain spaces and will be split with shlex
        # But using subcommand/default_model is more explicit (see next test)
        config = LocalCLIConfig(
            command="test-tool",
            task_flag="",  # Positional
            auth_env=["TEST_API_KEY"],
        )
        runner = LocalCLIRunner(config)

        mock_run.return_value = MagicMock(returncode=0, stdout="Output", stderr="")

        runner.run("Fix the bug")

        call_args = mock_run.call_args
        cmd = call_args[0][0]
        # Should be: ["test-tool", "Fix the bug"]
        assert cmd == ["test-tool", "Fix the bug"]
        assert "--task" not in cmd

    @patch.dict("os.environ", {"TEST_API_KEY": "test-key"})
    @patch("subprocess.run")
    def test_build_command_with_subcommand(self, mock_run: MagicMock):
        """Should include subcommand if specified."""
        config = LocalCLIConfig(
            command="ollama",
            subcommand="run",
            task_flag="",
            auth_env=["TEST_API_KEY"],
            default_model="codellama",
        )
        runner = LocalCLIRunner(config)

        mock_run.return_value = MagicMock(returncode=0, stdout="Output", stderr="")

        runner.run("Test task")

        call_args = mock_run.call_args
        cmd = call_args[0][0]
        assert cmd[0] == "ollama"
        assert cmd[1] == "run"
        assert "codellama" in cmd
