# Agentic Dev Library - Polyglot Monorepo
# Usage: just <recipe> or just --list

default:
    @just --list

# ── All Languages ──────────────────────────

# Run all checks across all languages
check: check-ts check-py

# Run all tests across all languages
test: test-ts test-py

# Build everything
build: build-ts build-py build-docs

# ── TypeScript (pnpm + Nx) ─────────────────

# Lint and typecheck TypeScript packages
check-ts:
    pnpm nx run-many -t lint typecheck --projects=tag:lang:ts

# Run TypeScript tests
test-ts:
    pnpm nx run-many -t test --projects=tag:lang:ts

# Build TypeScript packages
build-ts:
    pnpm nx run-many -t build --projects=tag:lang:ts

# ── Python (uv + tox) ──────────────────────

# Lint Python packages
check-py:
    uv run --project packages/agentic-crew --extra dev python -m ruff check .
    uv run --project packages/agentic-crew --extra dev python -m ruff format --check .
    uv run --project tooling/pytest-agentic-crew --with ruff python -m ruff check .
    uv run --project tooling/pytest-agentic-crew --with ruff python -m ruff format --check .

# Run Python tests
test-py:
    uv run --project packages/agentic-crew --extra tests python -m pytest tests -v --tb=short
    uv run --project tooling/pytest-agentic-crew python -m pytest tests -v --tb=short

# Run Python test matrix via tox
test-py-matrix:
    uvx tox

# Build Python packages
build-py:
    uv build --package agentic-crew
    uv build --package pytest-agentic-crew

# ── Documentation ───────────────────────────

# Build Sphinx Python API reference, then build the Starlight site
build-docs: build-sphinx
    pnpm --filter agentic-docs build

# Build Sphinx markdown output for the Python API docs (agentic-crew)
build-sphinx:
    uv run sphinx-build -b markdown docs/sphinx docs/src/content/docs/api/_generated

# Start docs dev server
docs-dev:
    pnpm --filter agentic-docs dev

# ── Nx Utilities ────────────────────────────

# Run affected targets only (for CI)
affected target="test":
    pnpm nx affected -t {{target}}

# Show project graph in browser
graph:
    pnpm nx graph

# ── CI ──────────────────────────────────────

# Full CI pipeline (all languages)
ci: check test build

# Install all dependencies
setup:
    pnpm install
    uv sync
