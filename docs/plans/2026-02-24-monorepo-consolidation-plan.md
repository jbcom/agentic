# Monorepo Consolidation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Consolidate the agentic-dev-library org into a single Nx-orchestrated polyglot monorepo.

**Architecture:** Nx sits on top of three native workspace systems (pnpm for TypeScript, uv for Python, cargo for Rust). Each language owns a distinct directory subtree. A Justfile provides human-friendly aliases. Sphinx generates Markdown consumed by Astro Starlight for unified documentation.

**Tech Stack:** Nx, pnpm 9, uv, cargo, Astro Starlight, Sphinx + myst-parser, tox-uv, Just, Biome, Vitest, pytest, clippy

**Design doc:** `docs/plans/2026-02-24-monorepo-consolidation-design.md`

**Source repos (already cloned at `~/src/agentic-dev-library/`):**
- `triage/` — TypeScript triage library
- `meshy-content-generator/` — TypeScript Meshy pipeline tool
- `crew/` — Python crew orchestration (uv workspace with 2 packages)
- `game-generator/` — Rust Bevy game generator (cargo workspace with 5 crates)
- `agentic-dev-library.github.io/` — Astro Starlight docs site

---

## Phase 1: Scaffold Monorepo Infrastructure

### Task 1.1: Install Nx and configure root

**Files:**
- Modify: `package.json`
- Create: `nx.json`

**Step 1: Install Nx as a dev dependency**

Run: `pnpm add -Dw nx @nx/js`
Expected: Packages added to root `package.json` devDependencies

**Step 2: Create `nx.json`**

Create `nx.json` with this content:

```json
{
  "$schema": "https://raw.githubusercontent.com/nrwl/nx/master/packages/nx/schemas/nx-schema.json",
  "namedInputs": {
    "default": ["{projectRoot}/**/*", "!{projectRoot}/**/*.md"],
    "ts-source": ["{projectRoot}/src/**/*", "{projectRoot}/tsconfig.json", "{projectRoot}/package.json"],
    "py-source": ["{projectRoot}/src/**/*.py", "{projectRoot}/pyproject.toml"],
    "rs-source": ["{projectRoot}/src/**/*.rs", "{projectRoot}/Cargo.toml"]
  },
  "targetDefaults": {
    "build": {
      "dependsOn": ["^build"],
      "inputs": ["default"],
      "cache": true
    },
    "test": {
      "dependsOn": ["build"],
      "inputs": ["default"],
      "cache": true
    },
    "lint": {
      "inputs": ["default"],
      "cache": true
    },
    "typecheck": {
      "inputs": ["ts-source"],
      "cache": true
    }
  },
  "defaultBase": "main"
}
```

**Step 3: Add Nx scripts to root `package.json`**

Replace the `scripts` section in root `package.json`:

```json
{
  "scripts": {
    "build": "nx run-many -t build",
    "dev": "nx run-many -t dev --parallel",
    "test": "nx run-many -t test",
    "test:coverage": "nx run-many -t test:coverage",
    "lint": "nx run-many -t lint",
    "typecheck": "nx run-many -t typecheck",
    "check": "biome check .",
    "check:fix": "biome check --write .",
    "format": "biome format --write .",
    "format:check": "biome format .",
    "docs": "nx run agentic-docs:build",
    "clean": "pnpm -r exec rm -rf dist && rm -rf node_modules .nx",
    "affected:test": "nx affected -t test",
    "affected:build": "nx affected -t build",
    "affected:lint": "nx affected -t lint"
  }
}
```

**Step 4: Verify Nx detects existing packages**

Run: `pnpm nx show projects`
Expected: Lists `agentic-control`, `providers`, `vitest-agentic-control`

**Step 5: Commit**

```bash
git add nx.json package.json pnpm-lock.yaml
git commit -m "feat(monorepo): add Nx orchestrator with task caching and affected detection"
```

---

### Task 1.2: Update pnpm-workspace.yaml for docs

**Files:**
- Modify: `pnpm-workspace.yaml`

**Step 1: Add docs to workspace**

```yaml
packages:
  - 'packages/*'
  - 'docs'
```

Remove the `onlyBuiltDependencies` section (will be handled per-package).

**Step 2: Commit**

```bash
git add pnpm-workspace.yaml
git commit -m "feat(monorepo): expand pnpm workspace to include docs"
```

---

### Task 1.3: Create root pyproject.toml for uv workspace

**Files:**
- Create: `pyproject.toml`

**Step 1: Create root pyproject.toml**

```toml
[project]
name = "agentic-monorepo"
version = "0.0.0"
description = "Agentic Dev Library - polyglot monorepo"
requires-python = ">=3.10"
readme = "README.md"
license = "MIT"
authors = [{ name = "Jon Bogaty" }]

[tool.uv.workspace]
members = ["python/*"]

[dependency-groups]
dev = [
    "ruff>=0.11",
    "mypy>=1.15",
    "pytest>=8.3",
    "pytest-cov>=6.1",
    "pytest-asyncio>=0.25",
    "pytest-timeout>=2.3",
    "pytest-mock>=3.14",
]
docs = [
    "sphinx>=8.2",
    "myst-parser>=4.0",
    "sphinx-rtd-theme>=3.0",
    "sphinx-autodoc-typehints>=2.5",
    "sphinx-markdown-builder>=0.6",
]

[tool.ruff]
target-version = "py310"
line-length = 120

[tool.ruff.lint]
select = ["E", "F", "I", "UP", "B", "C4", "SIM"]

[tool.pytest.ini_options]
testpaths = ["python"]
markers = [
    "e2e: end-to-end tests (may require external services)",
    "slow: slow-running tests",
]

[tool.mypy]
python_version = "3.10"
warn_return_any = true
warn_unused_configs = true

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

**Step 2: Verify uv recognizes the workspace**

Run: `uv sync --dry-run`
Expected: Should complete without errors (no members yet)

**Step 3: Commit**

```bash
git add pyproject.toml
git commit -m "feat(monorepo): add uv workspace root for Python packages"
```

---

### Task 1.4: Create root Cargo.toml for cargo workspace

**Files:**
- Create: `Cargo.toml`

**Step 1: Create root Cargo.toml**

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
repository = "https://github.com/agentic-dev-library/agentic-control"

[workspace.dependencies]
# Game engine
bevy = "0.16.1"
bevy_egui = "0.35"
bevy-inspector-egui = "0.32"
egui_dock = "0.16"
catppuccin-egui = "5.7"

# AI
async-openai = "0.32"
tiktoken-rs = "0.7"

# Async
tokio = { version = "1.48", features = ["full"] }
futures = "0.3"
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.9"
ron = "0.9"
bincode = "1.3"

# Templating
minijinja = { version = "2.14", features = ["loader"] }

# Caching
sled = "0.34"

# Graph algorithms
petgraph = "0.7"

# HTTP
reqwest = { version = "0.12", features = ["json", "stream"] }
reqwest-middleware = "0.4"

# Utilities
anyhow = "1.0"
thiserror = "2.0"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { version = "1.0", features = ["v4"] }
dirs = "6.0"
clap = { version = "4.5", features = ["derive"] }
dotenv = "0.15"
base64 = "0.22"
rayon = "1.10"
regex = "1.11"
image = "0.25"
notify = { version = "8.0", features = ["serde"] }
syn = { version = "2.0", features = ["full"] }
fs_extra = "1.3"
zstd = "0.13"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

**Step 2: Verify cargo recognizes the workspace**

Run: `cargo metadata --no-deps 2>&1 | head -5`
Expected: Should complete (no members yet, but workspace is valid)

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "feat(monorepo): add cargo workspace root for Rust crates"
```

---

### Task 1.5: Create tox.ini

**Files:**
- Create: `tox.ini`

**Step 1: Create tox.ini**

```ini
[tox]
requires =
    tox-uv>=1.25
    tox-gh>=1.5
env_list =
    py{310,311,312,313,314}
    lint
    docs

[gh]
python =
    3.10 = py310
    3.11 = py311
    3.12 = py312
    3.13 = py313
    3.14 = py314

[testenv]
runner = uv-venv-lock-runner
package = editable
commands =
    pytest python/ {posargs:-v --tb=short}

[testenv:lint]
skip_install = true
commands =
    uvx ruff check python/
    uvx ruff format --check python/

[testenv:docs]
extras = docs
commands =
    sphinx-build -b markdown docs/sphinx docs/src/content/docs/api
```

**Step 2: Commit**

```bash
git add tox.ini
git commit -m "feat(monorepo): add tox.ini with uv runner for Python test matrix"
```

---

### Task 1.6: Create justfile

**Files:**
- Create: `justfile`

**Step 1: Create justfile**

```just
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
    pnpm nx run-many -t lint typecheck

# Run TypeScript tests
test-ts:
    pnpm nx run-many -t test

# Build TypeScript packages
build-ts:
    pnpm nx run-many -t build

# Run only affected TypeScript targets
affected target="test":
    pnpm nx affected -t {{target}}

# ── Python (uv + tox) ──────────────────────

# Lint Python packages
check-py:
    uvx ruff check python/
    uvx ruff format --check python/

# Run Python tests
test-py:
    uv run pytest python/ -v --tb=short

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

# ── CI ──────────────────────────────────────

# Full CI pipeline (all languages)
ci: check test build

# Install all dependencies
setup:
    pnpm install
    uv sync
    cargo fetch
    just --version > /dev/null || echo "Install just: cargo install just"
```

**Step 2: Verify justfile parses**

Run: `just --list`
Expected: Lists all recipes

**Step 3: Commit**

```bash
git add justfile
git commit -m "feat(monorepo): add justfile for cross-language task orchestration"
```

---

### Task 1.7: Update .gitignore for monorepo

**Files:**
- Modify: `.gitignore`

**Step 1: Add monorepo-specific ignores**

Append to `.gitignore`:

```
# Nx
.nx/

# uv
uv.lock

# Tox
.tox/

# Sphinx output
docs/src/content/docs/api/_generated/

# Astro
docs/.astro/
docs/dist/

# Just
.justfile.env
```

**Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: update gitignore for Nx, uv, tox, and Astro"
```

---

## Phase 2: Migrate TypeScript Packages

### Task 2.1: Migrate triage into packages/triage

**Files:**
- Create: `packages/triage/` (entire directory tree from source repo)
- Modify: `packages/triage/package.json` (update for workspace)
- Modify: `packages/triage/tsconfig.json` (extend root)

**Step 1: Copy triage source (excluding git, node_modules, dist, .github)**

Run:
```bash
rsync -av --exclude='.git' --exclude='node_modules' --exclude='dist' \
  --exclude='.github' --exclude='pnpm-lock.yaml' --exclude='.cursor' \
  --exclude='memory-bank' --exclude='.releaserc.json' \
  --exclude='.pre-commit-config.yaml' --exclude='commitlint.config.js' \
  --exclude='CONTRIBUTING.md' --exclude='SECURITY.md' --exclude='LICENSE' \
  --exclude='FINAL-STATUS-COMPLETE.md' --exclude='FINAL-SUMMARY.md' \
  --exclude='COMPLETE-RELEASE-SUMMARY.md' --exclude='.final-status-report.md' \
  --exclude='CODEOWNERS' --exclude='.github' \
  ~/src/agentic-dev-library/triage/ packages/triage/
```

**Step 2: Update triage's package.json for workspace**

Key changes:
- Remove `packageManager` field (root manages this)
- Update `repository.directory` to `packages/triage`
- Remove standalone lint-staged/husky config (root handles this)
- Update devDependencies — remove biome, vitest, typescript (hoisted to root)

**Step 3: Update triage's tsconfig.json to extend root**

```json
{
  "extends": "../../tsconfig.json",
  "compilerOptions": {
    "outDir": "./dist",
    "rootDir": "./src"
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist", "tests"]
}
```

**Step 4: Verify pnpm detects the new package**

Run: `pnpm install`
Expected: Installs triage dependencies, resolves workspace

**Step 5: Verify triage builds**

Run: `pnpm --filter @agentic-dev-library/triage build`
Expected: Successful build

**Step 6: Verify triage tests pass**

Run: `pnpm --filter @agentic-dev-library/triage test`
Expected: Tests pass

**Step 7: Update agentic-control dependency on triage to workspace reference**

In `packages/agentic-control/package.json`, change:
```json
"@agentic-dev-library/triage": "^1.1.0"
```
to:
```json
"@agentic-dev-library/triage": "workspace:*"
```

**Step 8: Verify agentic-control still builds with workspace triage**

Run: `pnpm install && pnpm --filter @agentic-dev-library/control build`
Expected: Successful build

**Step 9: Commit**

```bash
git add packages/triage/ packages/agentic-control/package.json pnpm-lock.yaml
git commit -m "feat(monorepo): migrate triage into packages/triage with workspace reference"
```

---

### Task 2.2: Migrate meshy-content-generator into packages/meshy-content-generator

**Files:**
- Create: `packages/meshy-content-generator/` (entire directory tree from source repo)
- Modify: `packages/meshy-content-generator/package.json`
- Modify: `packages/meshy-content-generator/tsconfig.json`

**Step 1: Copy meshy source (excluding git, node_modules, dist, .github, docs site)**

Run:
```bash
rsync -av --exclude='.git' --exclude='node_modules' --exclude='dist' \
  --exclude='.github' --exclude='pnpm-lock.yaml' --exclude='.cursor' \
  --exclude='memory-bank' --exclude='.releaserc.json' \
  --exclude='.pre-commit-config.yaml' --exclude='commitlint.config.js' \
  --exclude='CONTRIBUTING.md' --exclude='SECURITY.md' --exclude='LICENSE' \
  --exclude='CODEOWNERS' --exclude='src/content' --exclude='astro.config.mjs' \
  --exclude='release-please-config.json' --exclude='.release-please-manifest.json' \
  ~/src/agentic-dev-library/meshy-content-generator/ packages/meshy-content-generator/
```

Note: Exclude `src/content` and `astro.config.mjs` — docs will be in the unified docs site.

**Step 2: Update meshy package.json for workspace**

Key changes:
- Remove `packageManager` field
- Remove astro/starlight dev dependencies (docs handled centrally)
- Remove `docs:*` scripts
- Update `repository.directory` to `packages/meshy-content-generator`

**Step 3: Update meshy tsconfig.json to extend root**

```json
{
  "extends": "../../tsconfig.json",
  "compilerOptions": {
    "outDir": "dist",
    "rootDir": "src",
    "module": "ES2022",
    "moduleResolution": "Bundler",
    "lib": ["ES2022"],
    "types": ["node"]
  },
  "include": ["src"]
}
```

**Step 4: Verify build and tests**

Run:
```bash
pnpm install
pnpm --filter @agentic-dev-library/meshy-content-generator build
pnpm --filter @agentic-dev-library/meshy-content-generator test
```
Expected: Build succeeds, tests pass

**Step 5: Commit**

```bash
git add packages/meshy-content-generator/ pnpm-lock.yaml
git commit -m "feat(monorepo): migrate meshy-content-generator into packages/"
```

---

## Phase 3: Migrate Python Packages

### Task 3.1: Migrate agentic-crew into python/agentic-crew

**Files:**
- Create: `python/agentic-crew/` (from crew repo's `packages/agentic-crew/`)

**Step 1: Copy agentic-crew package source**

Run:
```bash
rsync -av --exclude='__pycache__' --exclude='.pytest_cache' \
  --exclude='*.egg-info' --exclude='.ruff_cache' --exclude='.mypy_cache' \
  ~/src/agentic-dev-library/crew/packages/agentic-crew/ python/agentic-crew/
```

**Step 2: Update python/agentic-crew/pyproject.toml**

Key changes:
- Ensure `[tool.uv.sources]` references workspace peers if needed
- Keep all existing dependencies, extras, entry-points
- Update repository URL to point to monorepo

**Step 3: Verify uv recognizes the package**

Run: `uv sync`
Expected: Resolves workspace with agentic-crew

**Step 4: Verify tests pass**

Run: `uv run pytest python/agentic-crew/tests/ -v --tb=short`
Expected: Tests pass (some may skip without framework extras)

**Step 5: Commit**

```bash
git add python/agentic-crew/
git commit -m "feat(monorepo): migrate agentic-crew into python/agentic-crew"
```

---

### Task 3.2: Migrate pytest-agentic-crew into python/pytest-agentic-crew

**Files:**
- Create: `python/pytest-agentic-crew/` (from crew repo's `packages/pytest-agentic-crew/`)

**Step 1: Copy pytest-agentic-crew package source**

Run:
```bash
rsync -av --exclude='__pycache__' --exclude='.pytest_cache' \
  --exclude='*.egg-info' --exclude='.ruff_cache' --exclude='.mypy_cache' \
  ~/src/agentic-dev-library/crew/packages/pytest-agentic-crew/ python/pytest-agentic-crew/
```

**Step 2: Update pyproject.toml to reference workspace sibling**

If `pytest-agentic-crew` depends on `agentic-crew`, ensure it uses a workspace source:

```toml
[tool.uv.sources]
agentic-crew = { workspace = true }
```

**Step 3: Verify uv sync resolves both packages**

Run: `uv sync`
Expected: Both Python packages resolve

**Step 4: Commit**

```bash
git add python/pytest-agentic-crew/
git commit -m "feat(monorepo): migrate pytest-agentic-crew into python/"
```

---

## Phase 4: Migrate Rust Crates

### Task 4.1: Copy all 5 Rust crates into crates/

**Files:**
- Create: `crates/vintage-game-generator/`
- Create: `crates/vintage-ai-client/`
- Create: `crates/vintage-blending-core/`
- Create: `crates/vintage-build-tools/`
- Create: `crates/bevy-combat/`

**Step 1: Copy each crate (source only, no target/)**

Run:
```bash
for crate in vintage_game_generator vintage_ai_client vintage_blending_core vintage_build_tools bevy-combat; do
  rsync -av --exclude='target' \
    ~/src/agentic-dev-library/game-generator/$crate/ crates/$crate/
done
```

**Step 2: Copy shared assets from game-generator root**

Run:
```bash
# Copy prompt templates and metaprompts if they exist at crate level
# These are already inside vintage_game_generator/ so rsync handles them
```

**Step 3: Update each crate's Cargo.toml for new workspace paths**

In each crate that references workspace dependencies, the path references need updating.

For `crates/vintage_build_tools/Cargo.toml`, change:
```toml
vintage_blending_core = { path = "../vintage_blending_core" }
vintage_ai_client = { path = "../vintage_ai_client" }
```
(These relative paths are still correct since all crates are siblings under `crates/`)

For `crates/vintage_game_generator/Cargo.toml`, same — sibling paths remain valid.

For workspace-inherited deps, change from local `workspace = true` to the root workspace. Each crate's `Cargo.toml` should use `<dep>.workspace = true` which resolves from the root `Cargo.toml` `[workspace.dependencies]`.

**Step 4: Verify cargo workspace resolves**

Run: `cargo check --workspace`
Expected: All 5 crates compile

**Step 5: Verify tests pass**

Run: `cargo test --workspace`
Expected: Tests pass (0 tests expected based on assessment)

**Step 6: Commit**

```bash
git add crates/ Cargo.lock
git commit -m "feat(monorepo): migrate game-generator Rust crates into crates/"
```

---

## Phase 5: Merge Documentation Site

### Task 5.1: Copy Astro Starlight site into docs/

**Files:**
- Create: `docs/` (from agentic-dev-library.github.io)

**Step 1: Copy docs site source (excluding git, node_modules, .github)**

Run:
```bash
rsync -av --exclude='.git' --exclude='node_modules' --exclude='dist' \
  --exclude='.github' --exclude='.cursor' --exclude='memory-bank' \
  --exclude='package-lock.json' --exclude='pnpm-lock.yaml' \
  --exclude='commitlint.config.js' --exclude='CODEOWNERS' \
  --exclude='docs/' \
  ~/src/agentic-dev-library/agentic-dev-library.github.io/ docs/
```

**Step 2: Update docs/package.json for workspace**

```json
{
  "name": "agentic-docs",
  "description": "Agentic documentation site",
  "type": "module",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "dev": "astro dev",
    "build": "astro build",
    "preview": "astro preview"
  },
  "dependencies": {
    "@astrojs/starlight": "^0.37.1",
    "astro": "^5.6.1",
    "sharp": "^0.34.2"
  }
}
```

**Step 3: Verify pnpm detects docs workspace member**

Run: `pnpm install`
Expected: Installs docs dependencies

**Step 4: Commit**

```bash
git add docs/
git commit -m "feat(monorepo): merge docs site into docs/ as pnpm workspace member"
```

---

### Task 5.2: Create jbcom dark theme CSS

**Files:**
- Create: `docs/src/styles/custom.css`

**Step 1: Create the custom CSS theme**

```css
/* jbcom Dark Theme for Astro Starlight */

@import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap');

:root {
  --sl-color-accent-low: #083344;
  --sl-color-accent: #06b6d4;
  --sl-color-accent-high: #67e8f9;
  --sl-color-white: #e2e8f0;
  --sl-color-gray-1: #cbd5e1;
  --sl-color-gray-2: #94a3b8;
  --sl-color-gray-3: #64748b;
  --sl-color-gray-4: #334155;
  --sl-color-gray-5: #1e293b;
  --sl-color-gray-6: #0f172a;
  --sl-color-black: #0a0f1a;

  --sl-font: 'Inter', sans-serif;
  --sl-font-mono: 'JetBrains Mono', monospace;

  --sl-color-bg-nav: var(--sl-color-black);
  --sl-color-bg-sidebar: var(--sl-color-gray-6);
}

h1, h2, h3, h4, h5, h6 {
  font-family: 'Space Grotesk', sans-serif;
}

.sl-markdown-content code:not(pre code) {
  background: var(--sl-color-gray-5);
  border: 1px solid var(--sl-color-gray-4);
  border-radius: 4px;
  padding: 0.15em 0.35em;
}

/* Accent links */
.sl-markdown-content a {
  color: var(--sl-color-accent);
}

.sl-markdown-content a:hover {
  color: var(--sl-color-accent-high);
}
```

**Step 2: Verify the astro.config.mjs references this CSS**

The existing `astro.config.mjs` already has `customCss: ['./src/styles/custom.css']` — this file was missing before; now it exists.

**Step 3: Commit**

```bash
git add docs/src/styles/custom.css
git commit -m "feat(docs): add jbcom dark theme CSS for Starlight"
```

---

### Task 5.3: Set up Sphinx with myst-parser

**Files:**
- Create: `docs/sphinx/conf.py`
- Create: `docs/sphinx/index.md`

**Step 1: Create Sphinx configuration**

Create `docs/sphinx/conf.py`:

```python
"""Sphinx configuration for Agentic Python API documentation."""

project = "Agentic"
author = "Jon Bogaty"
copyright = "2026, Jon Bogaty"

extensions = [
    "myst_parser",
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.intersphinx",
    "sphinx_autodoc_typehints",
]

# MyST parser settings
myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "fieldlist",
    "tasklist",
]

# Autodoc settings
autodoc_member_order = "bysource"
autodoc_typehints = "description"
always_document_param_types = True

# Intersphinx mapping
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "pydantic": ("https://docs.pydantic.dev/latest/", None),
}

# Markdown builder output
# Used with: sphinx-build -b markdown docs/sphinx docs/src/content/docs/api
```

Create `docs/sphinx/index.md`:

```markdown
# Agentic Python API Reference

```{toctree}
:maxdepth: 2

agentic-crew/index
pytest-agentic-crew/index
```
```

**Step 2: Verify Sphinx builds**

Run: `uv run sphinx-build -b markdown docs/sphinx docs/src/content/docs/api`
Expected: Generates markdown files in docs/src/content/docs/api/

**Step 3: Commit**

```bash
git add docs/sphinx/
git commit -m "feat(docs): add Sphinx conf with myst-parser for Python API docs"
```

---

### Task 5.4: Update Starlight sidebar for monorepo packages

**Files:**
- Modify: `docs/astro.config.mjs`

**Step 1: Update astro.config.mjs sidebar with package tabs**

Update the sidebar configuration to reflect the monorepo structure, with each package as its own collapsible section under "Core Packages", and add a new "API Reference" autogenerated section that consumes Sphinx output.

**Step 2: Verify docs site builds**

Run: `pnpm --filter agentic-docs build`
Expected: Astro builds successfully

**Step 3: Commit**

```bash
git add docs/astro.config.mjs
git commit -m "feat(docs): update Starlight sidebar for monorepo package structure"
```

---

## Phase 6: Wire Nx Project Configuration

### Task 6.1: Add project.json for each TypeScript package

**Files:**
- Create: `packages/triage/project.json`
- Create: `packages/meshy-content-generator/project.json`

**Step 1: Create project.json for triage**

```json
{
  "name": "@agentic-dev-library/triage",
  "projectType": "library",
  "sourceRoot": "packages/triage/src",
  "targets": {}
}
```

Note: Nx auto-infers targets from `package.json` scripts for pnpm workspace members. The `project.json` just provides metadata. If scripts are defined in package.json, Nx picks them up.

**Step 2: Create project.json for meshy-content-generator**

```json
{
  "name": "@agentic-dev-library/meshy-content-generator",
  "projectType": "library",
  "sourceRoot": "packages/meshy-content-generator/src",
  "targets": {}
}
```

**Step 3: Verify Nx sees all projects**

Run: `pnpm nx show projects`
Expected: Lists all 5+ TS packages including triage and meshy

**Step 4: Verify Nx can run tasks across all TS packages**

Run: `pnpm nx run-many -t build --verbose`
Expected: Builds all TS packages in dependency order

**Step 5: Commit**

```bash
git add packages/triage/project.json packages/meshy-content-generator/project.json
git commit -m "feat(monorepo): add Nx project.json for migrated TypeScript packages"
```

---

### Task 6.2: Add Nx targets for Python and Rust

**Files:**
- Create: `python/agentic-crew/project.json`
- Create: `python/pytest-agentic-crew/project.json`
- Create: `crates/project.json`

**Step 1: Create project.json for agentic-crew (Python)**

```json
{
  "name": "agentic-crew",
  "projectType": "library",
  "sourceRoot": "python/agentic-crew/src",
  "targets": {
    "test": {
      "command": "uv run pytest python/agentic-crew/tests/ -v --tb=short",
      "inputs": ["py-source"],
      "cache": true
    },
    "lint": {
      "command": "uvx ruff check python/agentic-crew/",
      "inputs": ["py-source"],
      "cache": true
    },
    "build": {
      "command": "uv build --package agentic-crew",
      "inputs": ["py-source"],
      "cache": true
    }
  }
}
```

**Step 2: Create project.json for pytest-agentic-crew (Python)**

```json
{
  "name": "pytest-agentic-crew",
  "projectType": "library",
  "sourceRoot": "python/pytest-agentic-crew/src",
  "targets": {
    "test": {
      "command": "uv run pytest python/pytest-agentic-crew/tests/ -v --tb=short",
      "inputs": ["py-source"],
      "cache": true
    },
    "lint": {
      "command": "uvx ruff check python/pytest-agentic-crew/",
      "inputs": ["py-source"],
      "cache": true
    }
  }
}
```

**Step 3: Create project.json for Rust workspace**

```json
{
  "name": "rust-crates",
  "projectType": "library",
  "root": "crates",
  "targets": {
    "check": {
      "command": "cargo clippy --workspace -- -D warnings",
      "inputs": ["rs-source"],
      "cache": true
    },
    "test": {
      "command": "cargo test --workspace",
      "inputs": ["rs-source"],
      "cache": true
    },
    "build": {
      "command": "cargo build --workspace",
      "inputs": ["rs-source"],
      "cache": true
    }
  }
}
```

**Step 4: Verify Nx sees all projects**

Run: `pnpm nx show projects`
Expected: Lists TS packages, Python packages, and rust-crates

**Step 5: Verify cross-language task execution**

Run: `pnpm nx run-many -t test --verbose`
Expected: Runs TS tests via vitest, Python tests via pytest, Rust tests via cargo

**Step 6: Commit**

```bash
git add python/agentic-crew/project.json python/pytest-agentic-crew/project.json crates/project.json
git commit -m "feat(monorepo): add Nx project.json for Python and Rust targets"
```

---

### Task 6.3: Verify Nx affected detection

**Step 1: Test affected detection on a TS change**

Run:
```bash
touch packages/triage/src/index.ts
pnpm nx affected -t test --dry-run
```
Expected: Shows triage (and agentic-control which depends on it) would be tested

**Step 2: Test affected detection on a Python change**

Run:
```bash
touch python/agentic-crew/src/agentic_crew/__init__.py
pnpm nx affected -t test --dry-run
```
Expected: Shows agentic-crew would be tested

**Step 3: Test affected detection on a Rust change**

Run:
```bash
touch crates/vintage_ai_client/src/lib.rs
pnpm nx affected -t test --dry-run
```
Expected: Shows rust-crates would be tested

**Step 4: Commit (no files changed, just verification)**

No commit needed — this is a verification step.

---

## Phase 7: Update CI/CD

### Task 7.1: Create unified CI workflow

**Files:**
- Modify: `.github/workflows/ci.yml`

**Step 1: Replace ci.yml with unified multi-language CI**

```yaml
name: CI

on:
  pull_request:
  push:
    branches: [main]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  detect-changes:
    runs-on: ubuntu-latest
    outputs:
      ts: ${{ steps.filter.outputs.ts }}
      py: ${{ steps.filter.outputs.py }}
      rs: ${{ steps.filter.outputs.rs }}
      docs: ${{ steps.filter.outputs.docs }}
    steps:
      - uses: actions/checkout@v4
      - uses: dorny/paths-filter@v3
        id: filter
        with:
          filters: |
            ts:
              - 'packages/**'
              - 'package.json'
              - 'tsconfig.json'
            py:
              - 'python/**'
              - 'pyproject.toml'
            rs:
              - 'crates/**'
              - 'Cargo.toml'
              - 'Cargo.lock'
            docs:
              - 'docs/**'

  typescript:
    needs: detect-changes
    if: needs.detect-changes.outputs.ts == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm
      - run: pnpm install --frozen-lockfile
      - run: pnpm nx affected -t lint typecheck test build --base=origin/main

  python:
    needs: detect-changes
    if: needs.detect-changes.outputs.py == 'true'
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ['3.10', '3.11', '3.12', '3.13']
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v5
        with:
          enable-cache: true
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}
      - run: uv sync --all-extras
      - run: uvx ruff check python/
      - run: uv run pytest python/ -v --tb=short

  rust:
    needs: detect-changes
    if: needs.detect-changes.outputs.rs == 'true'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: sudo apt-get update && sudo apt-get install -y libasound2-dev libudev-dev
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
      - run: cargo fmt --check

  docs:
    needs: [detect-changes, typescript, python]
    if: |
      always() &&
      needs.detect-changes.outputs.docs == 'true' &&
      (needs.typescript.result == 'success' || needs.typescript.result == 'skipped') &&
      (needs.python.result == 'success' || needs.python.result == 'skipped')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm
      - uses: astral-sh/setup-uv@v5
      - run: pnpm install --frozen-lockfile
      - run: uv sync --group docs
      - run: uv run sphinx-build -b markdown docs/sphinx docs/src/content/docs/api
      - run: pnpm --filter agentic-docs build
```

**Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "feat(ci): unified multi-language CI with path-based change detection"
```

---

### Task 7.2: Final integration verification

**Step 1: Run the full Justfile CI recipe**

Run: `just ci`
Expected: All checks, tests, and builds pass across all three languages

**Step 2: Verify Nx graph**

Run: `pnpm nx graph`
Expected: Opens browser with visualization of all projects and their dependencies

**Step 3: Final commit with any fixups**

```bash
git add -A
git commit -m "feat(monorepo): complete polyglot monorepo consolidation

Migrated repos:
- triage (TypeScript) → packages/triage/
- meshy-content-generator (TypeScript) → packages/meshy-content-generator/
- agentic-crew (Python) → python/agentic-crew/
- pytest-agentic-crew (Python) → python/pytest-agentic-crew/
- game-generator (Rust, 5 crates) → crates/
- docs site (Astro Starlight) → docs/

Orchestration: Nx + pnpm workspaces + uv workspaces + cargo workspace
Task runner: Justfile + tox-uv for Python matrix"
```

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | 1.1–1.7 | Scaffold infrastructure (Nx, pyproject.toml, Cargo.toml, tox.ini, justfile, .gitignore) |
| 2 | 2.1–2.2 | Migrate TypeScript packages (triage, meshy) |
| 3 | 3.1–3.2 | Migrate Python packages (agentic-crew, pytest-agentic-crew) |
| 4 | 4.1 | Migrate Rust crates (5 crates) |
| 5 | 5.1–5.4 | Merge docs site (Astro + Sphinx + theme) |
| 6 | 6.1–6.3 | Wire Nx (project.json, cross-language targets, affected verification) |
| 7 | 7.1–7.2 | Update CI/CD (unified workflow, final verification) |

**Total: 18 tasks across 7 phases**
