# CHANGELOG

<!-- version list -->

## v1.0.0 (2025-12-25)

### 🎉 Initial Stable Release

Framework-agnostic AI crew orchestration is now production-ready!

### ✨ Features

- **Multi-Framework Support**: Run crews on CrewAI, LangGraph, or Strands
- **Auto-Detection**: Automatically selects best available framework
- **Universal YAML Format**: Define once, run anywhere
- **Single-Agent CLI Runners**: Support for aider, claude-code, ollama, and more
- **Knowledge Base Integration**: Load domain knowledge from markdown files
- **Agent Archetypes**: Reusable agent templates
- **Comprehensive Testing**: 122+ unit tests, E2E test suite

### 💥 Breaking Changes

- **Minimum Python version increased from 3.9 to 3.10**
  - Required for CrewAI 1.5.0+ compatibility
  - Python 3.10, 3.11, 3.12, 3.13, 3.14 now supported

### 📚 Documentation

- Complete API documentation with Sphinx
- Quick start guide and architecture overview
- jbcom dark theme branding applied
- Integration guides for vendor-connectors

### 🔧 Internal

- Full test coverage across all runners
- Ruff linting and formatting
- Type hints with mypy
- Tox for multi-version testing
