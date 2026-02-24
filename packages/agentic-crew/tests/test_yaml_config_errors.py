"""Tests for YAML configuration parsing error handling."""

from __future__ import annotations

from pathlib import Path

import pytest
import yaml
from agentic_crew.core.discovery import (
    get_crew_config,
    get_framework_from_config_dir,
    load_manifest,
)


class TestLoadManifestErrors:
    """Test error handling in load_manifest."""

    def test_missing_manifest_file_raises(self, tmp_path: Path) -> None:
        """Should raise FileNotFoundError for missing manifest."""
        with pytest.raises(FileNotFoundError):
            load_manifest(tmp_path / "nonexistent")

    def test_empty_manifest_returns_empty_dict(self, tmp_path: Path) -> None:
        """Empty YAML file should return empty dict, not None."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text("")
        result = load_manifest(tmp_path)
        assert result == {}

    def test_manifest_with_only_comments_returns_empty_dict(self, tmp_path: Path) -> None:
        """YAML with only comments should return empty dict."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text("# just a comment\n# another comment\n")
        result = load_manifest(tmp_path)
        assert result == {}

    def test_invalid_yaml_raises_error(self, tmp_path: Path) -> None:
        """Badly formed YAML should raise a yaml error."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(":\n  bad:\n    - ][invalid yaml")
        with pytest.raises(yaml.YAMLError):
            load_manifest(tmp_path)

    def test_manifest_with_no_crews_key(self, tmp_path: Path) -> None:
        """Manifest without 'crews' key should parse without error."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text("name: test_package\ndescription: A test\n")
        result = load_manifest(tmp_path)
        assert result.get("name") == "test_package"
        assert result.get("crews") is None


class TestGetCrewConfigErrors:
    """Test error handling in get_crew_config."""

    def test_crew_not_in_manifest_raises_value_error(self, tmp_path: Path) -> None:
        """Requesting a non-existent crew should raise ValueError."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  existing_crew:\n"
            "    description: An existing crew\n"
            "    agents: crews/existing/agents.yaml\n"
            "    tasks: crews/existing/tasks.yaml\n"
        )
        with pytest.raises(ValueError, match="Crew 'nonexistent' not found"):
            get_crew_config(tmp_path, "nonexistent")

    def test_error_lists_available_crews(self, tmp_path: Path) -> None:
        """ValueError message should list available crew names."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  alpha_crew:\n"
            "    agents: a.yaml\n"
            "    tasks: t.yaml\n"
            "  beta_crew:\n"
            "    agents: a.yaml\n"
            "    tasks: t.yaml\n"
        )
        with pytest.raises(ValueError, match="alpha_crew") as exc_info:
            get_crew_config(tmp_path, "missing")
        assert "beta_crew" in str(exc_info.value)

    def test_missing_agents_yaml_returns_empty_dict(self, tmp_path: Path) -> None:
        """If agents YAML file doesn't exist, agents should be empty dict."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    description: Test\n"
            "    agents: nonexistent_agents.yaml\n"
            "    tasks: nonexistent_tasks.yaml\n"
        )
        config = get_crew_config(tmp_path, "test_crew")
        assert config["agents"] == {}

    def test_missing_tasks_yaml_returns_empty_dict(self, tmp_path: Path) -> None:
        """If tasks YAML file doesn't exist, tasks should be empty dict."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    description: Test\n"
            "    agents: nonexistent_agents.yaml\n"
            "    tasks: nonexistent_tasks.yaml\n"
        )
        config = get_crew_config(tmp_path, "test_crew")
        assert config["tasks"] == {}

    def test_empty_crews_section_raises_for_any_crew(self, tmp_path: Path) -> None:
        """Empty crews section should raise ValueError."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text("crews: {}\n")
        with pytest.raises(ValueError, match="not found"):
            get_crew_config(tmp_path, "anything")

    def test_knowledge_paths_resolves_relative(self, tmp_path: Path) -> None:
        """Knowledge paths should resolve relative to config dir."""
        # Create the knowledge directory
        knowledge_dir = tmp_path / "knowledge"
        knowledge_dir.mkdir()
        (knowledge_dir / "docs.md").write_text("# Docs")

        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    agents: agents.yaml\n"
            "    tasks: tasks.yaml\n"
            "    knowledge:\n"
            "      - knowledge\n"
        )
        config = get_crew_config(tmp_path, "test_crew")
        assert len(config["knowledge_paths"]) == 1
        assert config["knowledge_paths"][0] == knowledge_dir

    def test_nonexistent_knowledge_paths_excluded(self, tmp_path: Path) -> None:
        """Non-existent knowledge paths should be silently excluded."""
        manifest_file = tmp_path / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    agents: agents.yaml\n"
            "    tasks: tasks.yaml\n"
            "    knowledge:\n"
            "      - missing_dir\n"
        )
        config = get_crew_config(tmp_path, "test_crew")
        assert config["knowledge_paths"] == []


class TestGetFrameworkFromConfigDir:
    """Test edge cases for get_framework_from_config_dir."""

    def test_unknown_directory_name_returns_none(self) -> None:
        """Unknown directory name should return None."""
        result = get_framework_from_config_dir(Path("/path/.unknown"))
        assert result is None

    def test_non_hidden_directory_returns_none(self) -> None:
        """Non-hidden directory name should return None."""
        result = get_framework_from_config_dir(Path("/path/crewai"))
        assert result is None

    def test_all_known_dirs_return_expected_framework(self) -> None:
        """All known framework directories should map correctly."""
        expected = {
            ".crew": None,
            ".crewai": "crewai",
            ".langgraph": "langgraph",
            ".strands": "strands",
        }
        for dir_name, framework in expected.items():
            result = get_framework_from_config_dir(Path(f"/any/path/{dir_name}"))
            assert result == framework, f"Expected {framework} for {dir_name}, got {result}"


class TestCrewConfigFrameworkConflict:
    """Test framework conflict warnings in get_crew_config."""

    def test_framework_mismatch_prints_warning(self, tmp_path: Path, capsys) -> None:
        """When manifest preferred_framework differs from directory, warn."""
        crewai_dir = tmp_path / ".crewai"
        crewai_dir.mkdir()

        manifest_file = crewai_dir / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    description: Test\n"
            "    agents: agents.yaml\n"
            "    tasks: tasks.yaml\n"
            "    preferred_framework: strands\n"
        )
        config = get_crew_config(crewai_dir, "test_crew")
        captured = capsys.readouterr()
        assert "Warning" in captured.out
        assert "strands" in captured.out
        assert "crewai" in captured.out
        # required_framework should still be crewai (directory wins)
        assert config["required_framework"] == "crewai"

    def test_matching_preferred_framework_no_warning(self, tmp_path: Path, capsys) -> None:
        """When preferred_framework matches directory, no warning."""
        crewai_dir = tmp_path / ".crewai"
        crewai_dir.mkdir()

        manifest_file = crewai_dir / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    description: Test\n"
            "    agents: agents.yaml\n"
            "    tasks: tasks.yaml\n"
            "    preferred_framework: crewai\n"
        )
        get_crew_config(crewai_dir, "test_crew")
        captured = capsys.readouterr()
        assert "Warning" not in captured.out

    def test_preferred_auto_no_warning(self, tmp_path: Path, capsys) -> None:
        """preferred_framework='auto' should not trigger warning."""
        crewai_dir = tmp_path / ".crewai"
        crewai_dir.mkdir()

        manifest_file = crewai_dir / "manifest.yaml"
        manifest_file.write_text(
            "crews:\n"
            "  test_crew:\n"
            "    description: Test\n"
            "    agents: agents.yaml\n"
            "    tasks: tasks.yaml\n"
            "    preferred_framework: auto\n"
        )
        get_crew_config(crewai_dir, "test_crew")
        captured = capsys.readouterr()
        assert "Warning" not in captured.out
