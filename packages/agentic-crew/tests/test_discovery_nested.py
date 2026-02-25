"""Tests for crew discovery in nested directories and edge cases."""

from __future__ import annotations

from pathlib import Path

from agentic_crew.core.discovery import (
    DIR_TO_FRAMEWORK,
    FRAMEWORK_DIRS,
    FRAMEWORK_TO_DIR,
    discover_all_framework_configs,
    discover_packages,
)


class TestDiscoverPackagesNested:
    """Test discovery across nested directory structures."""

    def test_skips_non_directory_entries(self, tmp_path: Path) -> None:
        """Files in the packages/ directory should be ignored."""
        packages_dir = tmp_path / "packages"
        packages_dir.mkdir()

        # Create a file (not a directory) in packages/
        (packages_dir / "README.md").write_text("# Packages")

        # Create a real package too
        pkg_dir = packages_dir / "real_pkg"
        crew_dir = pkg_dir / ".crew"
        crew_dir.mkdir(parents=True)
        (crew_dir / "manifest.yaml").write_text("name: real\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path)
        assert "real_pkg" in packages
        assert "README.md" not in packages

    def test_standalone_project_at_root(self, tmp_path: Path) -> None:
        """A .crew directory at workspace root should be discovered."""
        crew_dir = tmp_path / ".crew"
        crew_dir.mkdir()
        (crew_dir / "manifest.yaml").write_text("name: root_project\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path)
        assert tmp_path.name in packages

    def test_standalone_and_packages_coexist(self, tmp_path: Path) -> None:
        """Root .crew and packages/.crew should both be discovered."""
        # Root level .crew
        root_crew = tmp_path / ".crew"
        root_crew.mkdir()
        (root_crew / "manifest.yaml").write_text("name: root\ncrews: {}")

        # Package level .crew
        pkg_dir = tmp_path / "packages" / "sub_pkg"
        pkg_crew = pkg_dir / ".crew"
        pkg_crew.mkdir(parents=True)
        (pkg_crew / "manifest.yaml").write_text("name: sub\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path)
        assert "sub_pkg" in packages
        assert tmp_path.name in packages

    def test_packages_dir_not_present(self, tmp_path: Path) -> None:
        """When there is no packages/ directory, only root is checked."""
        packages = discover_packages(workspace_root=tmp_path)
        assert packages == {}

    def test_framework_filter_crewai(self, tmp_path: Path) -> None:
        """Filtering by framework='crewai' should only find .crewai dirs."""
        pkg_dir = tmp_path / "packages" / "test_pkg"

        # Create .crew (agnostic) and .crewai (specific)
        crew_dir = pkg_dir / ".crew"
        crew_dir.mkdir(parents=True)
        (crew_dir / "manifest.yaml").write_text("name: test\ncrews: {}")

        crewai_dir = pkg_dir / ".crewai"
        crewai_dir.mkdir()
        (crewai_dir / "manifest.yaml").write_text("name: test\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path, framework="crewai")
        assert "test_pkg" in packages
        assert packages["test_pkg"].name == ".crewai"

    def test_framework_filter_no_match(self, tmp_path: Path) -> None:
        """Filtering by framework with no matching dirs returns empty."""
        pkg_dir = tmp_path / "packages" / "test_pkg"
        crew_dir = pkg_dir / ".crewai"
        crew_dir.mkdir(parents=True)
        (crew_dir / "manifest.yaml").write_text("name: test\ncrews: {}")

        packages = discover_packages(workspace_root=tmp_path, framework="strands")
        assert packages == {}

    def test_missing_manifest_skips_directory(self, tmp_path: Path) -> None:
        """A .crewai directory without manifest.yaml should be skipped."""
        pkg_dir = tmp_path / "packages" / "test_pkg"
        crewai_dir = pkg_dir / ".crewai"
        crewai_dir.mkdir(parents=True)
        # No manifest.yaml created

        packages = discover_packages(workspace_root=tmp_path)
        assert packages == {}

    def test_multiple_packages_discovered(self, tmp_path: Path) -> None:
        """Multiple packages should all be discovered."""
        packages_dir = tmp_path / "packages"
        for name in ["alpha", "beta", "gamma"]:
            crew_dir = packages_dir / name / ".crew"
            crew_dir.mkdir(parents=True)
            (crew_dir / "manifest.yaml").write_text(f"name: {name}\ncrews: {{}}")

        packages = discover_packages(workspace_root=tmp_path)
        assert set(packages.keys()) == {"alpha", "beta", "gamma"}


class TestDiscoverAllFrameworkConfigs:
    """Test discover_all_framework_configs."""

    def test_multiple_frameworks_per_package(self, tmp_path: Path) -> None:
        """A package with .crew, .crewai, and .strands should return all."""
        pkg_dir = tmp_path / "packages" / "multi"

        for dir_name in [".crew", ".crewai", ".strands"]:
            fw_dir = pkg_dir / dir_name
            fw_dir.mkdir(parents=True)
            (fw_dir / "manifest.yaml").write_text(f"name: multi\nframework_dir: {dir_name}\ncrews: {{}}")

        configs = discover_all_framework_configs(workspace_root=tmp_path)

        assert "multi" in configs
        assert None in configs["multi"]  # .crew -> agnostic
        assert "crewai" in configs["multi"]
        assert "strands" in configs["multi"]

    def test_root_configs_discovered(self, tmp_path: Path) -> None:
        """Root-level framework configs should be discovered."""
        crew_dir = tmp_path / ".langgraph"
        crew_dir.mkdir()
        (crew_dir / "manifest.yaml").write_text("name: root\ncrews: {}")

        configs = discover_all_framework_configs(workspace_root=tmp_path)
        assert tmp_path.name in configs
        assert "langgraph" in configs[tmp_path.name]

    def test_empty_workspace(self, tmp_path: Path) -> None:
        """An empty workspace should return empty dict."""
        configs = discover_all_framework_configs(workspace_root=tmp_path)
        assert configs == {}


class TestFrameworkConstants:
    """Test the framework constant mappings."""

    def test_dir_to_framework_completeness(self) -> None:
        """Every entry in FRAMEWORK_DIRS should have a DIR_TO_FRAMEWORK entry."""
        for dir_name in FRAMEWORK_DIRS:
            assert dir_name in DIR_TO_FRAMEWORK

    def test_framework_to_dir_completeness(self) -> None:
        """Every framework in DIR_TO_FRAMEWORK should have a FRAMEWORK_TO_DIR entry."""
        for framework in DIR_TO_FRAMEWORK.values():
            assert framework in FRAMEWORK_TO_DIR

    def test_round_trip_mapping(self) -> None:
        """DIR_TO_FRAMEWORK -> FRAMEWORK_TO_DIR should be a round trip."""
        for dir_name, framework in DIR_TO_FRAMEWORK.items():
            assert FRAMEWORK_TO_DIR[framework] == dir_name
