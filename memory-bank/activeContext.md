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

Additional cleanup completed in `@jbcom/agentic-triage`:

- split the old monolithic `src/ai.ts` helper into narrower internal modules under `src/ai/`
- split MCP client factories into `src/mcp/clients.ts` so `octokit` no longer pulls the broader MCP/AI helper surface into every build artifact
- rewired `cli`, `mcp`, `octokit`, `sage`, and `visual` to depend on the narrower modules
- eliminated the remaining build-time `tsup` unused-import noise while keeping lint, typecheck, tests, and the full TypeScript workspace gate green
- corrected stale `triage` contract text in source examples and committed API docs so project/review APIs are no longer described as "coming soon" stubs
- added docs contract coverage for stale `@strata/triage` branding and old stub remarks
- implemented real Linear label mutation support in `LinearProvider` for create/add/remove label flows and covered it with provider tests

Remaining work should now focus on:

- deeper production-readiness work in triage provider/runtime behavior rather than packaging or build-noise cleanup
- cross-framework tool wiring for `agentic-crew` runners beyond CrewAI, especially LangGraph and Strands
- closing the highest-value remaining implementation gaps in `agentic-crew` after the CrewAI execution path

Additional runtime work completed in `agentic-crew`:

- added a lazy tool registry that resolves configured tool names, aliases, and selected filesystem MCP-style identifiers to concrete tool instances
- wired the default `CrewAIRunner` to actually attach configured tools instead of dropping them on the floor
- wired the legacy loader path and `ConnectorBuilderCrew` to use the same resolver
- made `agentic_crew.tools` lazy so the core package can import without immediately requiring CrewAI-only dependencies
- added framework-specific tool adapters for LangGraph and Strands so config-declared tools now flow through all three runner families instead of only CrewAI
- made those adapters fail soft when framework-specific wrapping helpers are unavailable or no tools are declared, keeping runner construction stable in partial environments and test doubles
- added regression coverage for LangGraph and Strands crew builds that resolve configured tools from agent config
- added direct adapter execution coverage so LangGraph and Strands wrappers are now tested for real invocation, schema forwarding, metadata naming, and passthrough behavior for already-native tool objects
- verified `agentic-crew` and `pytest-agentic-crew` end to end with lint, typecheck, tests, and the repo-level Python workspace gate

Additional runtime work completed in `@jbcom/agentic-triage`:

- switched GitHub review comment and unresolved-feedback reads in `TriageConnectors` away from the gh-cli approximation and onto the existing `octokit` review helpers
- unresolved PR feedback now uses actual unresolved, non-outdated review threads instead of treating every inline comment as unresolved forever
- general GitHub review summaries are still surfaced, but unresolved thread feedback now uses the latest comment from each still-open thread
- added connector tests that stub GitHub review comments, reviews, and review threads to verify mapping and unresolved filtering behavior
- verified `triage` with lint, build, typecheck, coverage tests, and the full TypeScript workspace gate

Additional runtime work completed in `@jbcom/agentic` triage orchestration:

- fixed `Triage.plan()` so CI-only remediation plans no longer create a self-referential "Request re-review" dependency
- made plan generation status-aware for merged and closed PRs, returning no-op plans instead of suggesting follow-up work on already-finished pull requests
- tightened "wait for CI" and merge-step dependency wiring so the returned sequence reflects the actual preceding remediation steps
- added direct orchestration coverage for `plan()`, `resolve()`, `runUntilReady()`, `requestReviews()`, and report formatting via mocked `Analyzer`, `Resolver`, and `GitHubClient` collaborators
- re-ran `agentic` lint, build, typecheck, coverage tests, and the full TypeScript workspace gate after the orchestration changes and kept everything green

Remaining work should now focus on:

- deeper `@jbcom/agentic` triage runtime behavior, especially `resolver.ts`, `analyzer.ts`, and the MCP-backed triage agent layer
- turning the current comment-only or placeholder resolution paths into more trustworthy runtime behavior before calling the `agentic` triage surface production-stable

Additional runtime work completed in `@jbcom/agentic` triage resolution and analysis:

- hardened `resolver.ts` so review suggestions now apply line-scoped local edits instead of overwriting entire files with the suggestion body
- added fenced-code extraction for AI-generated fixes, allowing safe local edits when the model returns an actual replacement snippet and falling back to PR comments when the response is not safe to apply
- added direct resolver coverage for safe suggestion application, unsafe-suggestion refusal, local fenced-fix application, comment fallback, and inline-comment justification replies
- fixed `analyzer.ts` review blocker classification so mixed critical/high feedback is only considered auto-resolvable when every blocking item can be automated
- added direct analyzer coverage for blocked vs. needs-work review feedback mixes and clean ready-to-merge PR classification
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the resolver and analyzer changes and kept everything green

Additional runtime work completed in the MCP-backed triage agent layer:

- fixed a real lifecycle bug in both `agent.ts` and `pr-triage-agent.ts` where `initialize()` could reuse a stale resolved initialization promise after `close()`, preventing proper reinitialization
- added direct `Agent` lifecycle coverage for post-close reinitialization and concurrent initialization lock behavior
- added direct `PRTriageAgent` coverage for MCP-backed analysis, ready-to-merge workflow execution, blocked workflow exit, post-close reinitialization, concurrent initialization locking, and `triagePR()` cleanup on failure
- verified that `PRTriageAgent` now formats and returns stable reports while exercising the real workflow methods instead of only lower-level helpers
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the agent-layer changes and kept everything green

Additional runtime work completed in `mcp-clients.ts`:

- fixed a bug where per-call overrides like `{ github: { enabled: false } }` did not actually disable a default MCP server because the enabled check ran before overrides were merged
- fixed a bug where custom tokenless stdio MCP servers were silently skipped unless they were on a hardcoded optional-server list
- added direct MCP client coverage for override-based disabling, custom tokenless server initialization, override token injection, tool namespacing/error isolation, and close-time error isolation
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the MCP client changes and kept everything green

Additional runtime work completed in `packages/agentic/src/triage/agent.ts` tool execution:

- fixed an approval-policy gap so dangerous bash commands now always consult `onApprovalRequest` even when `bash` is not explicitly listed in `requireApproval`
- fixed delete-file approval handling so `delete_file` always requests approval before removing a file, matching the documented safety expectation
- replaced shell-string git invocations in `git_status` and `git_diff` with `execFileSync('git', [...])`, removing argument interpolation risk and making validated filenames with spaces or shell metacharacters safe to pass through
- added direct tool-layer coverage for dangerous bash rejection, mandatory delete approval, safe `git diff` argument handling, and editor path-traversal security failures
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the tool-security changes, keeping the coverage runs sequential so generated V8 artifacts do not collide

Additional runtime work completed in `packages/agentic/src/triage/agent.ts` convenience helpers:

- hardened `fixFile()` so it now validates target paths before execution instead of accepting traversal attempts into the convenience API
- normalized `fixFile()` diff collection onto `git diff -- <path>` so filenames beginning with `-` are treated as pathspecs rather than git options
- added direct coverage for traversal rejection in `fixFile()` and for option-like filenames such as `--stat.ts`
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the `fixFile()` hardening, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/handoff/manager.ts`:

- fixed takeover branch handling so it now resolves the repository's actual default branch before any merge side effects instead of assuming `main`
- made takeover fail early when default-branch resolution fails, preventing the predecessor PR from being merged before the local branch-sync step is known to be safe
- changed the local sync step to `git pull --ff-only origin <defaultBranch>` after checking out the resolved default branch, making the update path explicit and non-merging
- added direct handoff coverage for non-`main` default branches and for failure-before-merge when branch resolution cannot succeed
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the handoff changes, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/github/client.ts`:

- fixed `getCIStatus()` so it now merges classic combined commit-status contexts with check runs instead of treating repos without GitHub Checks as implicitly green
- classic commit statuses in `pending`, `error`, and `failure` states now flow into the returned `checks`, `failures`, `anyPending`, and `allPassing` fields correctly
- added direct runtime coverage for legacy commit-status-only repos, including pending and failing status contexts with no check runs present
- re-ran `agentic` lint, build, typecheck, tests, coverage, and the full TypeScript workspace gate after the GitHub client changes, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/github/client.ts` review feedback mapping:

- fixed `collectFeedback()` so review-summary feedback now derives its status from the underlying GitHub review state instead of hardcoding every summary item as already addressed
- `CHANGES_REQUESTED`, `COMMENTED`, and other non-terminal review states now remain `unaddressed`, while `APPROVED` maps to `addressed` and `DISMISSED` maps to `dismissed`
- added direct runtime coverage for changes-requested review summaries and for approved versus dismissed review-summary handling in `packages/agentic/tests/github-client-runtime.test.ts`
- re-ran `agentic` tests, `agentic` coverage, and the full TypeScript workspace gate after the review-feedback fix, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/github/client.ts` top-level PR feedback collection:

- fixed `collectFeedback()` so it now includes top-level PR conversation comments from `issues.listComments(...)` instead of silently ignoring non-inline review feedback
- analyzer-driven triage can now see blocking maintainer comments left on the PR conversation timeline rather than only review summaries and inline review comments
- added direct runtime coverage for top-level PR comment ingestion in `packages/agentic/tests/github-client-runtime.test.ts`
- re-ran `agentic` lint, typecheck, tests, coverage, and the full TypeScript workspace gate after the PR conversation feedback fix, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/github/client.ts` clone-path handling:

- fixed `cloneRepo()` so authenticated GitHub SSH clone targets like `git@github.com:owner/repo.git` and `ssh://git@github.com/owner/repo.git` are rewritten to tokenized HTTPS URLs instead of being passed through unchanged
- kept bare `owner/repo` and `https://github.com/...` clone targets on the same tokenized path and preserved stderr redaction for embedded tokens on clone failures
- added direct clone helper coverage in `packages/agentic/tests/github-client-clone.test.ts` for bare repo names, HTTPS URLs, SSH URLs, token-redacted failures, and no-token early exits
- re-ran `agentic` lint, typecheck, tests, coverage, and the full TypeScript workspace gate after the clone-path fix, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/github/client.ts` static PR comment pagination:

- fixed `listPRComments()` so it now paginates through all PR issue-comment pages instead of returning only the first 100 comments
- this closes a fleet coordination failure mode where new `@cursor` instructions on long-lived coordination PRs could disappear once the PR exceeded 100 comments
- added direct static-client coverage in `packages/agentic/tests/github-client-static.test.ts` for multi-page PR comment retrieval across page 1 and page 2
- re-ran `agentic` lint, typecheck, tests, coverage, and the full TypeScript workspace gate after the pagination fix, again keeping coverage runs sequential to avoid V8 artifact collisions

Additional runtime work completed in `packages/agentic/src/fleet/fleet.ts` coordination comment handling:

- fixed inbound coordination polling so agent status comments like `✅ DONE:` and `⚠️ BLOCKED:` are processed even when they do not include an `@cursor` mention
- extracted the single-pass inbound poll into `pollCoordinationComments()` and added explicit coordination-signal detection so the loop can distinguish actionable DONE/BLOCKED comments from ordinary chatter
- added direct fleet coordination coverage in `packages/agentic/tests/fleet-management.test.ts` for DONE comments, BLOCKED comments, and unrelated comments
- re-ran `agentic` lint, typecheck, tests, coverage, and the full TypeScript workspace gate after the fleet coordination fix, again keeping coverage runs sequential to avoid V8 artifact collisions
