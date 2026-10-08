"""Explicit target selection and construction scope tests without LLM calls."""

from __future__ import annotations

import os
import socket
from concurrent.futures import ThreadPoolExecutor
from types import SimpleNamespace
from unittest.mock import patch

import pytest
from agentic_crew._targets import bind_file_root, construction_root, validate_package_name
from agentic_crew.main import main


@pytest.fixture(autouse=True)
def reject_network(monkeypatch):
    """These filesystem contracts must never call a model or other service."""

    def deny(*_args, **_kwargs):
        raise AssertionError("Network access is forbidden in target tests")

    monkeypatch.setattr(socket.socket, "connect", deny)


@pytest.mark.parametrize("name", ["", "../demo", "/demo", "a/b", "a\\b", ".", "..", "demo..name"])
def test_invalid_names(name):
    with pytest.raises(ValueError, match="Invalid package"):
        validate_package_name(name)


def test_nested_scope_restores_on_failure(tmp_path):
    other = tmp_path / "other"
    other.mkdir()
    with bind_file_root(tmp_path):
        with pytest.raises(RuntimeError), bind_file_root(other):
            assert construction_root() == other
            raise RuntimeError("construction failed")
        assert construction_root() == tmp_path
    assert construction_root() is None


@pytest.mark.parametrize(
    "argv",
    [
        ["build", "spec"],
        ["build", "spec", "--package", "missing"],
        ["build", "spec", "--package", "../demo"],
        ["list-knowledge"],
        ["list-knowledge", "--package", "missing"],
    ],
)
def test_legacy_commands_fail_before_execution(argv, capsys):
    with (
        patch("sys.argv", ["agentic-crew", *argv]),
        patch("agentic_crew.main.discover_packages", return_value={}),
        patch("agentic_crew.main.run_crew") as run,
        pytest.raises(SystemExit) as error,
    ):
        main()
    assert error.value.code == 2
    run.assert_not_called()
    assert "agentic-crew list" in capsys.readouterr().err


def test_build_preserves_input_and_explicit_target(tmp_path):
    with (
        patch("sys.argv", ["agentic-crew", "build", "spec", "--package", "legacy-game"]),
        patch("agentic_crew.main.discover_packages", return_value={"legacy-game": tmp_path / ".crew"}),
        patch("agentic_crew.main.run_crew", return_value="result") as run,
    ):
        main()
    run.assert_called_once_with("legacy-game", "game_builder", {"spec": "spec", "component_spec": "spec"})


@pytest.mark.parametrize("target", ["missing", "../demo"])
def test_single_runner_rejects_unknown_explicit_target_before_execution(target):
    with (
        patch("sys.argv", ["agentic-crew", "run", target, "--runner", "demo", "--input", "spec"]),
        patch("agentic_crew.main.discover_packages", return_value={}),
        patch("agentic_crew.core.decomposer.get_cli_runner") as runner,
        pytest.raises(SystemExit) as error,
    ):
        main()
    assert error.value.code == 2
    runner.assert_not_called()


def test_knowledge_uses_selected_crew(tmp_path, capsys):
    config_dir = tmp_path / ".crew"
    with (
        patch("sys.argv", ["agentic-crew", "list-knowledge", "--package", "demo", "--crew", "reader"]),
        patch("agentic_crew.main.discover_packages", return_value={"demo": config_dir}),
        patch("agentic_crew.main.get_crew_config", return_value={"knowledge_paths": ["knowledge/demo.md"]}) as config,
    ):
        main()
    config.assert_called_once_with(config_dir, "reader")
    assert "knowledge/demo.md" in capsys.readouterr().out


def test_runner_binds_construction_and_resets_on_failure(tmp_path):
    from agentic_crew.core.runner import run_crew, run_crew_from_path

    def fail(_config):
        assert construction_root() == tmp_path
        raise RuntimeError("build failed")

    with (
        patch("agentic_crew.core.runner.discover_packages", return_value={"demo": tmp_path / ".crew"}),
        patch("agentic_crew.core.runner.get_crew_config", return_value={}),
        patch("agentic_crew.core.runner.load_crew_from_config", side_effect=fail),
    ):
        with pytest.raises(RuntimeError, match="build failed"):
            run_crew("demo", "builder")
        assert construction_root() is None
        with pytest.raises(RuntimeError, match="build failed"):
            run_crew_from_path(tmp_path / ".crew", "builder")
        assert construction_root() is None


def test_auto_cli_binds_selected_root(tmp_path):
    def run(_config, **_kwargs):
        assert construction_root() == tmp_path
        return "result"

    with (
        patch("sys.argv", ["agentic-crew", "run", "demo", "builder", "--input", "spec"]),
        patch("agentic_crew.main.discover_packages", return_value={"demo": tmp_path / ".crew"}),
        patch("agentic_crew.main.get_crew_config", return_value={}),
        patch("agentic_crew.core.decomposer.detect_framework", return_value="crewai"),
        patch("agentic_crew.core.decomposer.run_crew_auto", side_effect=run),
    ):
        main()
    assert construction_root() is None


class TestFileTargetSafety:
    @pytest.fixture(autouse=True)
    def tools(self):
        pytest.importorskip("crewai")

    def test_pydantic_tool_copy_retains_private_binding(self, tmp_path):
        from agentic_crew.tools.file_tools import GameCodeWriterTool

        with bind_file_root(tmp_path):
            tool = GameCodeWriterTool()
        copied = tool.model_copy()
        assert "_bound_root" not in copied.model_dump()
        assert "Successfully wrote" in copied._run("src/ecs/demo.ts", "selected")
        assert (tmp_path / "src/ecs/demo.ts").read_text() == "selected"

    def test_deep_copy_preserves_upstream_contract(self, tmp_path):
        from agentic_crew.tools.file_tools import GameCodeWriterTool
        from crewai.tools import BaseTool

        class PlainTool(BaseTool):
            name: str = "Plain Tool"
            description: str = "Upstream copy contract baseline"

            def _run(self) -> str:
                return "plain"

        with bind_file_root(tmp_path):
            selected = GameCodeWriterTool()
        try:
            PlainTool().model_copy(deep=True)
        except TypeError as baseline_error:
            # CrewAI 1.15.25 carries a non-pickleable _usage_lock.
            # Preserve that upstream behavior; do not override framework locks.
            assert "lock" in str(baseline_error)
            with pytest.raises(TypeError) as actual_error:
                selected.model_copy(deep=True)
            assert str(actual_error.value) == str(baseline_error)
            assert not (tmp_path / "src").exists()
            assert selected._file_root() == tmp_path
        else:
            # If upstream adds deep-copy support, require the same root safety.
            copied = selected.model_copy(deep=True)
            assert "_bound_root" not in copied.model_dump()
            assert "Successfully wrote" in copied._run("src/ecs/demo.ts", "selected")
            assert (tmp_path / "src/ecs/demo.ts").read_text() == "selected"
        assert construction_root() is None

    def test_discovered_auto_runner_binds_registry_tools(self, tmp_path, monkeypatch):
        from agentic_crew.core.decomposer import run_crew_auto
        from agentic_crew.tools.registry import resolve_tool

        monkeypatch.setenv("TARGET_PACKAGE", "other")
        config = {"config_dir": tmp_path / ".crew"}
        tools = []

        def build(actual_config):
            assert actual_config is config
            tools.append(resolve_tool("FileWriteTool"))
            return "crew"

        runner = SimpleNamespace(build_crew=build, run=lambda _crew, _inputs: "output")
        with patch("agentic_crew.core.decomposer.get_runner", return_value=runner):
            assert run_crew_auto(config) == "output"
        assert construction_root() is None
        assert "Successfully wrote" in tools[0]._run("src/ecs/demo.ts", "selected")
        assert (tmp_path / "src/ecs/demo.ts").read_text() == "selected"

    @pytest.mark.parametrize("from_path", [False, True])
    def test_programmatic_runner_writes_selected_project(self, tmp_path, monkeypatch, from_path):
        from agentic_crew.core.runner import run_crew, run_crew_from_path
        from agentic_crew.tools.registry import resolve_tool

        selected, other = tmp_path / "selected", tmp_path / "other"
        selected.mkdir()
        other.mkdir()
        monkeypatch.setenv("TARGET_PACKAGE", "other")
        monkeypatch.setenv("OTHER_ROOT", str(other))

        def build(_config):
            tool = resolve_tool("GameCodeWriterTool")

            def kickoff(**_kwargs):
                with ThreadPoolExecutor(max_workers=1) as executor:
                    return executor.submit(tool._run, "src/ecs/demo.ts", "selected").result()

            return SimpleNamespace(kickoff=kickoff)

        with (
            patch("agentic_crew.core.runner.discover_packages", return_value={"demo": selected / ".crew"}),
            patch("agentic_crew.core.runner.get_crew_config", return_value={}),
            patch("agentic_crew.core.runner.load_crew_from_config", side_effect=build),
        ):
            result = run_crew_from_path(selected / ".crew", "builder") if from_path else run_crew("demo", "builder")
        assert "Successfully wrote" in result
        assert construction_root() is None
        assert (selected / "src/ecs/demo.ts").read_text() == "selected"
        assert not (other / "src").exists()

    @pytest.mark.parametrize("target", ["missing", "", "../demo"])
    def test_invalid_explicit_target_never_falls_back(self, tmp_path, monkeypatch, target):
        from agentic_crew.tools.file_tools import get_workspace_root

        monkeypatch.chdir(tmp_path)
        monkeypatch.delenv("TARGET_PACKAGE", raising=False)
        (tmp_path / "pyproject.toml").touch()
        with (
            patch("agentic_crew.tools.file_tools._find_workspace_root", return_value=None),
            pytest.raises(ValueError),
        ):
            get_workspace_root(target)
        assert not (tmp_path / "src").exists()

    def test_explicit_argument_overrides_environment(self, tmp_path, monkeypatch):
        from agentic_crew.tools.file_tools import get_workspace_root

        selected = tmp_path / "packages" / "demo"
        selected.mkdir(parents=True)
        monkeypatch.setenv("TARGET_PACKAGE", "other")
        with patch("agentic_crew.tools.file_tools._find_workspace_root", return_value=tmp_path):
            assert get_workspace_root("demo") == selected

    def test_environment_root_must_exist(self, tmp_path, monkeypatch):
        from agentic_crew.tools.file_tools import get_workspace_root

        monkeypatch.setenv("TARGET_PACKAGE", "demo")
        monkeypatch.setenv("DEMO_ROOT", str(tmp_path))
        with patch("agentic_crew.tools.file_tools._find_workspace_root", return_value=None):
            assert get_workspace_root() == tmp_path
            monkeypatch.setenv("DEMO_ROOT", str(tmp_path / "missing"))
            with pytest.raises(ValueError, match="existing project"):
                get_workspace_root()

    def test_standalone_and_workspace_ambiguity(self, tmp_path, monkeypatch):
        from agentic_crew.tools.file_tools import get_workspace_root

        monkeypatch.chdir(tmp_path)
        monkeypatch.delenv("TARGET_PACKAGE", raising=False)
        (tmp_path / ".crew").mkdir()
        (tmp_path / ".crew" / "manifest.yaml").touch()
        with patch("agentic_crew.tools.file_tools._find_workspace_root", return_value=None):
            assert get_workspace_root() == tmp_path
            (tmp_path / "packages" / "demo").mkdir(parents=True)
            with pytest.raises(ValueError, match="unambiguous"):
                get_workspace_root()

    def test_frozen_tools_survive_thread_and_conflicting_environment(self, tmp_path, monkeypatch):
        from agentic_crew.tools.file_tools import DirectoryListTool, GameCodeReaderTool, GameCodeWriterTool

        selected = tmp_path / "selected"
        other = tmp_path / "other"
        selected.mkdir()
        other.mkdir()
        monkeypatch.setenv("TARGET_PACKAGE", "other")
        monkeypatch.setenv("OTHER_ROOT", str(other))
        with bind_file_root(selected):
            writer, reader, listing = GameCodeWriterTool(), GameCodeReaderTool(), DirectoryListTool()
        assert construction_root() is None
        with ThreadPoolExecutor(max_workers=1) as executor:
            result = executor.submit(writer._run, "src/ecs/demo.ts", "selected").result()
        assert "Successfully wrote" in result
        assert (selected / "src/ecs/demo.ts").read_text() == "selected"
        assert reader._run("src/ecs/demo.ts") == "selected"
        assert "demo.ts" in listing._run("src/ecs")
        assert not (other / "src").exists()
        assert {"TARGET_PACKAGE": "other", "OTHER_ROOT": str(other)} == {
            key: os.environ[key] for key in ("TARGET_PACKAGE", "OTHER_ROOT")
        }

    def test_concurrent_construction_has_no_cross_target_leak(self, tmp_path):
        from threading import Barrier

        from agentic_crew.tools.file_tools import GameCodeWriterTool

        roots = [tmp_path / name for name in ("a", "b")]
        for root in roots:
            root.mkdir()
        barrier = Barrier(2)

        def construct(root):
            with bind_file_root(root):
                barrier.wait(timeout=5)
                tool = GameCodeWriterTool()
            assert construction_root() is None
            return tool

        with ThreadPoolExecutor(max_workers=2) as executor:
            tools = list(executor.map(construct, roots))
            results = list(
                executor.map(lambda pair: pair[1]._run("src/ecs/demo.ts", pair[0].name), zip(roots, tools, strict=True))
            )
        assert all("Successfully wrote" in result for result in results)
        assert [root.joinpath("src/ecs/demo.ts").read_text() for root in roots] == ["a", "b"]
        assert construction_root() is None

    def test_symlink_cannot_redirect_writes_to_sibling(self, tmp_path):
        from agentic_crew.tools.file_tools import GameCodeWriterTool

        selected, sibling = tmp_path / "selected", tmp_path / "sibling"
        (selected / "src").mkdir(parents=True)
        sibling.mkdir()
        (selected / "src/ecs").symlink_to(sibling, target_is_directory=True)
        with bind_file_root(selected):
            writer = GameCodeWriterTool()
        assert "Error" in writer._run("src/ecs/demo.ts", "unsafe")
        assert not (sibling / "demo.ts").exists()

    @pytest.mark.parametrize("swapped", ["parent", "file"])
    def test_symlink_swap_after_resolution_cannot_redirect_write(self, tmp_path, monkeypatch, swapped):
        from agentic_crew.tools.file_tools import GameCodeWriterTool

        selected, sibling = tmp_path / "selected", tmp_path / "sibling"
        (selected / "src/ecs").mkdir(parents=True)
        sibling.mkdir()
        outside = sibling / "demo.ts"
        outside.write_text("untouched")
        with bind_file_root(selected):
            writer = GameCodeWriterTool()
        original = GameCodeWriterTool._file_path

        def swap_after_resolution(tool, relative):
            path = original(tool, relative)
            if swapped == "parent":
                path.parent.symlink_to(sibling, target_is_directory=True)
            else:
                path.symlink_to(outside)
            return path

        monkeypatch.setattr(GameCodeWriterTool, "_file_path", swap_after_resolution)
        relative = "src/ecs/data/demo.ts" if swapped == "parent" else "src/ecs/demo.ts"
        assert "Error" in writer._run(relative, "unsafe")
        assert outside.read_text() == "untouched"

    def test_unsupported_safe_write_platform_fails_without_io(self, tmp_path, monkeypatch):
        from agentic_crew.tools.file_tools import GameCodeWriterTool

        with bind_file_root(tmp_path):
            writer = GameCodeWriterTool()
        monkeypatch.setattr(os, "supports_dir_fd", set())
        assert "no-follow support" in writer._run("src/ecs/demo.ts", "unsafe")
        assert not (tmp_path / "src").exists()

    def test_failed_selection_creates_no_files(self, tmp_path, monkeypatch):
        from agentic_crew.tools.file_tools import GameCodeWriterTool

        monkeypatch.chdir(tmp_path)
        monkeypatch.setenv("TARGET_PACKAGE", "missing")
        monkeypatch.delenv("MISSING_ROOT", raising=False)
        with patch("agentic_crew.tools.file_tools._find_workspace_root", return_value=tmp_path):
            writer = GameCodeWriterTool()
            assert "Error" in writer._run("src/ecs/demo.ts", "unsafe")
        assert not (tmp_path / "src").exists()
