# Agentic Dev Library - Polyglot Monorepo
# Usage: just <recipe> or just --list

default:
    @just --list

# ── All Languages ──────────────────────────

# Run all checks across all languages
check: check-ts check-py check-rs

# Run all tests across all languages
test: test-ts test-py test-rs

# Build everything
build: build-ts build-py build-rs build-docs

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
    uvx ruff check packages/agentic-crew/ packages/pytest-agentic-crew/
    uvx ruff format --check packages/agentic-crew/ packages/pytest-agentic-crew/

# Run Python tests
test-py:
    uv run pytest packages/agentic-crew/tests/ packages/pytest-agentic-crew/tests/ -v --tb=short

# Run Python test matrix via tox
test-py-matrix:
    uvx tox

# Build Python packages
build-py:
    uv build --package agentic-crew

# ── Rust (cargo) ────────────────────────────

# Check and lint Rust crates
check-rs:
    cargo clippy --workspace -- -D warnings

# Run Rust tests
test-rs:
    cargo test --workspace

# Build Rust crates
build-rs:
    cargo build --workspace

# Build Rust in release mode
build-rs-release:
    cargo build --workspace --release

# ── Documentation ───────────────────────────

# Build Python API docs via Sphinx, then build Starlight site
build-docs: build-sphinx
    pnpm --filter agentic-docs build

# Build Sphinx markdown output for Python API docs
build-sphinx:
    uv run sphinx-build -b markdown docs/sphinx docs/src/content/docs/api

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
    cargo fetch
