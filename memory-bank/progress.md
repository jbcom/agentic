# Progress Log

## 2026-04-15

- audited the monorepo for production-stability gaps across TypeScript, Python, Rust, docs, CI, and release automation
- corrected a large set of docs contract drift issues around package naming and fake examples
- normalized Python workspace commands onto `uv run python -m pytest` and added missing Nx targets
- expanded CI/release verification so Python, docs, and release jobs use stricter shared checks
- improved `@jbcom/agentic-triage` coverage parsing and GitHub alert/check wrappers
- cleaned and verified `game-asset-mcp` static analysis in the monorepo
- concluded that `game-generator` and `game-asset-mcp` fit better as standalone repositories than as monorepo packages
- created and published standalone public repositories:
  - `https://github.com/jbcom/game-generator`
  - `https://github.com/jbcom/game-asset-mcp`
- cleaned the monorepo to remove the extracted package directories, Rust workspace wiring, and public docs pages
- moved the `game-generator` public docs surface into the standalone repository and trimmed the monorepo docs site back to the retained TypeScript and Python packages
- added ignore rules for local screenshots, Playwright output, and Meshy temp preview directories so the workspace stays cleaner after verification runs
- reorganized the retained monorepo so package directories align with public names, support packages live under `tooling/`, and internal planning/reference material lives under `internal/`
- updated Nx, CI, release, tox, and justfile commands to run Python verification from each package root instead of relying on repo-root imports
- fixed reorg fallout in the vitest workspace link, docs API readme copy, and remaining old package paths in workflow configuration
- updated agentic release-property tests and public docs references to the new `docs/src/content/docs` structure and `packages/agentic` naming
- verified the retained monorepo end to end across TypeScript, Python, docs unit tests, and docs Playwright e2e
- confirmed the docs duplicate-id warning was cache churn; a clean docs build now runs without that warning after clearing stale `.astro` state
- cleared the remaining Biome warning set in `@jbcom/agentic-triage` by refactoring escalation/connectors helpers and tightening tests
- re-ran the full TypeScript workspace gate after the triage cleanup and kept it green
- split the `@jbcom/agentic-triage` AI helpers into narrower `src/ai/` modules and moved MCP client factories into `src/mcp/clients.ts`
- rewired `octokit` and the tool/CLI entrypoints to consume the narrower modules so `tsup` no longer drags the broad MCP/AI helper surface into unrelated bundles
- eliminated the remaining `tsup` unused-import build noise in `@jbcom/agentic-triage`
- re-ran `triage` lint, build, typecheck, tests, and the full TypeScript workspace gate after the internal module split and kept everything green
- corrected stale `@strata/triage` references and "coming soon" stub remarks in `triage` source examples and committed API docs, and added a docs contract test to keep that drift from returning
- confirmed the docs duplicate-id warning was local `.astro` cache state again; a clean docs build runs without the warning
- implemented real Linear label mutation support so `LinearProvider.addLabels()` and `removeLabels()` no longer no-op, and added provider tests for label creation/removal behavior
- re-ran `triage` lint, build, typecheck, and coverage after the Linear provider work and kept the package green
