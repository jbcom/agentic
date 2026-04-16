# Active Context

## Monorepo Reality

`agentic` is now organized around a clearer TypeScript + Python core:

- `@jbcom/agentic` is the most production-ready package
- `@jbcom/agentic-triage` is usable and well-tested, but still has some partial provider surface area
- `agentic-crew` and `pytest-agentic-crew` are in good shape after workspace command cleanup
- docs now build and unit-test cleanly, with contract tests for renamed package surfaces
- package layout now matches the public product names more closely:
  - `packages/agentic`
  - `packages/triage`
  - `packages/providers`
  - `packages/meshy-content-generator`
  - `packages/agentic-crew`
  - `tooling/vitest-agentic`
  - `tooling/pytest-agentic-crew`
  - `internal/` for non-public planning and legacy docs

## Extraction Decision

The Rust `game-generator` package and the Python `game-asset-mcp` package were judged to make more sense as standalone repositories than as first-class monorepo packages.

Standalone repositories were created locally and published publicly:

- `/Users/jbogaty/src/jbcom/game-generator`
- `/Users/jbogaty/src/jbcom/game-asset-mcp`

GitHub remotes:

- `https://github.com/jbcom/game-generator`
- `https://github.com/jbcom/game-asset-mcp`

## Current State Of Those Extracted Repos

### game-generator

- converted from workspace-bound Cargo metadata to a self-contained crate
- added standalone Rust CI
- updated CLI defaults from `vintage_game_generator` to `game_generator`
- rewritten README to describe the package honestly as experimental
- verified with `cargo test`

### game-asset-mcp

- converted from monorepo-root tooling assumptions to self-contained `pyproject.toml` config
- added standalone Python CI
- updated repo metadata and README for standalone use
- verified with `ruff`, `mypy`, and `pytest`

## What Remains In This Monorepo

The extracted package directories have now been removed from the monorepo.

The monorepo is now scoped around:

- `@jbcom/agentic`
- `@jbcom/agentic-triage`
- `agentic-crew`
- `pytest-agentic-crew`
- `@jbcom/agentic-meshy`
- `@jbcom/agentic-providers`
- `@jbcom/vitest-agentic`

Cleanup completed:

- removed root Rust workspace and Rust release/CI assumptions
- removed `game-generator` and `game-asset-mcp` package directories from `packages/`
- rewrote the docs site to describe only the remaining TypeScript and Python product surface
- moved `game-generator` public docs into its standalone repository
- added ignore rules for local docs/playwright/temp artifacts
- aligned Nx, CI, release, tox, justfile, docs tests, and package release checks with the renamed directories and package-local Python environments
- verified the retained monorepo end to end:
  - `pnpm nx run-many -t lint,build,typecheck,test:coverage --projects=tag:lang:ts`
  - `pnpm nx run-many -t lint,typecheck,test --projects=tag:lang:py`
  - `pnpm --dir docs run build`
  - `pnpm --dir docs run test`
  - `pnpm --dir docs run test:e2e --project=chromium`

Remaining work should focus on:

- driving down remaining build-time `tsup` unused-import noise in `@jbcom/agentic-triage`
- continuing production-readiness work in triage and crew
