# Monorepo Consolidation Design

**Date**: 2026-02-24
**Status**: Approved
**Author**: Claude Opus 4.6 + Jon Bogaty

## Overview

Consolidate the `agentic-dev-library` GitHub organization from multiple repositories into a single Nx-orchestrated polyglot monorepo using pnpm workspaces (TypeScript), uv workspaces (Python), and a cargo workspace (Rust).

## Motivation

The org has a fragmented structure with repos that depend on each other across language boundaries. A monorepo provides:

- Unified CI/CD with Nx-powered caching and affected detection
- Atomic cross-package changes (no more multi-repo PRs)
- Single source of truth for documentation
- Simplified dependency management via workspace references
- Consistent tooling and configuration

## Repositories

| Repo | Language | Lines | Action |
|------|----------|-------|--------|
| agentic-control | TypeScript | ~15.3K | Target monorepo (this repo) |
| triage | TypeScript | ~13.4K | Migrate to `packages/triage/` |
| meshy-content-generator | TypeScript | ~3.3K | Migrate to `packages/meshy-content-generator/` |
| crew | Python | ~8.7K | Migrate to `python/agentic-crew/` and `python/pytest-agentic-crew/` |
| game-generator | Rust | ~23K | Migrate to `crates/` (5 crates) |
| agentic-dev-library.github.io | Astro/MDX | ~15.8K content | Merge into `docs/` |
| asset-pipeline | -- | 0 | Deleted |
| control | TypeScript | ~16.3K | Ignored (frozen duplicate of agentic-control) |
| .github | -- | 1 file | Ignored |

## Directory Layout

```
agentic-control/
├── nx.json                          # Nx orchestrator config
├── package.json                     # Root: pnpm workspace scripts + Nx
├── pnpm-workspace.yaml              # TS packages + docs
├── pyproject.toml                   # uv workspace root
├── Cargo.toml                       # Cargo workspace root
├── tox.ini                          # tox-uv: Python test matrix + docs
├── justfile                         # Human-friendly task aliases
├── biome.json                       # Shared TS lint/format config
├── tsconfig.json                    # Shared TS compiler base
│
├── packages/                        # TypeScript (pnpm workspace)
│   ├── agentic-control/             # @agentic-dev-library/control (existing)
│   ├── triage/                      # @agentic-dev-library/triage (migrated)
│   ├── meshy-content-generator/     # @agentic-dev-library/meshy-content-generator (migrated)
│   ├── providers/                   # @agentic/providers (existing)
│   └── vitest-agentic-control/      # @agentic-dev-library/vitest-control (existing)
│
├── python/                          # Python (uv workspace)
│   ├── agentic-crew/                # agentic-crew (migrated)
│   └── pytest-agentic-crew/         # pytest-agentic-crew (migrated)
│
├── crates/                          # Rust (cargo workspace)
│   ├── vintage-game-generator/      # Main Bevy app (migrated)
│   ├── vintage-ai-client/           # AI service abstraction (migrated)
│   ├── vintage-blending-core/       # Game similarity algorithms (migrated)
│   ├── vintage-build-tools/         # Build-time code gen (migrated)
│   ├── bevy-combat/                 # Combat system (migrated)
│   └── pyo3-bridge/                 # Future: PyO3 bindings for Python
│
├── docs/                            # Documentation (Astro Starlight)
│   ├── package.json                 # agentic-docs (pnpm workspace member)
│   ├── astro.config.mjs
│   ├── src/
│   │   ├── content/docs/            # Starlight MDX pages
│   │   │   ├── getting-started/
│   │   │   ├── packages/            # Per-package docs with tabs
│   │   │   │   ├── control/
│   │   │   │   ├── triage/
│   │   │   │   ├── crew/
│   │   │   │   ├── meshy/
│   │   │   │   └── game-generator/
│   │   │   ├── api/                 # Auto-generated from Sphinx
│   │   │   ├── guides/
│   │   │   ├── integrations/
│   │   │   └── examples/
│   │   ├── styles/custom.css        # Signal Amber theme
│   │   └── assets/
│   └── sphinx/                      # Python API doc source
│       └── conf.py                  # myst-parser + autodoc config
│
├── actions/                         # GitHub composite actions (existing)
├── examples/                        # Cross-language examples (existing)
├── scripts/                         # Build utilities (existing)
└── .github/workflows/               # Unified CI/CD
```

## Workspace Configuration

### pnpm-workspace.yaml

```yaml
packages:
  - 'packages/*'
  - 'docs'
```

### Root pyproject.toml (uv workspace)

```toml
[project]
name = "agentic-monorepo"
version = "0.0.0"
requires-python = ">=3.10"

[tool.uv.workspace]
members = ["python/*"]

[dependency-groups]
dev = ["ruff", "mypy", "pytest", "pytest-cov", "pytest-asyncio", "pytest-timeout", "pytest-mock"]
docs = ["sphinx", "myst-parser", "sphinx-rtd-theme", "sphinx-autodoc-typehints"]
```

### Root Cargo.toml

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
authors = ["Jon Bogaty"]

[workspace.dependencies]
# Inherited from game-generator's existing workspace deps
```

### nx.json

```json
{
  "targetDefaults": {
    "build": { "dependsOn": ["^build"], "cache": true },
    "test": { "dependsOn": ["build"], "cache": true },
    "lint": { "cache": true },
    "typecheck": { "cache": true }
  },
  "namedInputs": {
    "ts-source": ["{projectRoot}/src/**/*", "{projectRoot}/tsconfig.json"],
    "py-source": ["{projectRoot}/src/**/*.py", "{projectRoot}/pyproject.toml"],
    "rs-source": ["{projectRoot}/src/**/*.rs", "{projectRoot}/Cargo.toml"]
  }
}
```

### tox.ini

```ini
[tox]
requires = tox-uv>=1.25 tox-gh>=1.5
env_list = py{310,311,312,313,314}, lint, docs

[testenv]
package = editable
deps = .[dev]
commands = pytest python/ {posargs}

[testenv:lint]
commands = uvx ruff check python/ && uvx ruff format --check python/

[testenv:docs]
deps = .[docs]
commands = sphinx-build -b markdown docs/sphinx docs/src/content/docs/api
```

### justfile

```just
default:
    @just --list

check: check-ts check-py check-rs
test: test-ts test-py test-rs
build: build-ts build-py build-rs build-docs

check-ts:
    pnpm nx run-many -t lint typecheck
test-ts:
    pnpm nx run-many -t test
build-ts:
    pnpm nx run-many -t build

check-py:
    uvx ruff check python/
test-py:
    uv run --directory python pytest
build-py:
    uv build --directory python/agentic-crew

check-rs:
    cargo clippy --workspace
test-rs:
    cargo test --workspace
build-rs:
    cargo build --workspace

build-docs: build-sphinx
    pnpm --filter agentic-docs build
build-sphinx:
    uv run sphinx-build -b markdown docs/sphinx docs/src/content/docs/api

ci: check test build
```

## Documentation Pipeline

```
Python source → Sphinx + myst-parser → sphinx-build -b markdown → docs/src/content/docs/api/
TypeScript source → TypeDoc → .md output ──────────────────────→ docs/src/content/docs/api/
                                                                        │
                                                                        ▼
                                                              Astro Starlight
                                                              (Signal Amber theme)
                                                                        │
                                                                        ▼
                                                              Static site → GitHub Pages
```

### Sphinx Configuration

- `myst_parser` for Markdown source support
- `sphinx.ext.autodoc` + `sphinx_autodoc_typehints` for Python API extraction
- Outputs Markdown (not HTML) via `sphinx-build -b markdown`
- Starlight consumes the Markdown output in `docs/src/content/docs/api/`

### Signal Amber Theme

- Primary Amber: `#F59E0B`
- Background: `#0C0A09` (warm stone)
- Fonts: Instrument Sans (headings), Inter (body), JetBrains Mono (code)

## Cross-Language Dependencies

```
agentic-control (TS)
  ├── depends on: triage (TS)          → workspace:* reference
  ├── depends on: crew (Python)        → subprocess CLI invocation
  └── depends on: providers (TS)       → workspace:* reference

triage (TS)        → standalone leaf
meshy (TS)         → standalone leaf
crew (Python)      → standalone leaf
game-generator     → standalone leaf (future: PyO3 bridge → Python)
```

### Future: PyO3 Bridge

`crates/pyo3-bridge/` will expose Rust functions (vintage_ai_client, vintage_blending_core) as a Python package via PyO3 + maturin. Maturin integrates with uv as a build backend.

## Migration Order

1. Scaffold monorepo infrastructure (nx.json, root pyproject.toml, root Cargo.toml, justfile, tox.ini)
2. Migrate TypeScript packages (triage, meshy-content-generator)
3. Migrate Python packages (crew: agentic-crew + pytest-agentic-crew)
4. Migrate Rust crates (game-generator: 5 crates)
5. Merge docs site (Astro Starlight + Sphinx setup + theme)
6. Wire Nx (project.json per package, task dependencies, verify affected)
7. Update CI (unified workflow using Nx affected)

## Post-Migration Cleanup

| Action | Target |
|--------|--------|
| Archive | triage, crew, game-generator, meshy-content-generator, agentic-dev-library.github.io |
| Ignore | control (frozen duplicate), .github (org profile) |
| Deleted | asset-pipeline (already done) |

Archived repos get a README update pointing to the monorepo.
