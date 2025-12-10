# Project Structure

## Root Layout

```
agentic-control/
├── src/                    # TypeScript source
├── python/                 # Python CrewAI package
├── dist/                   # TypeScript build output
├── tests/                  # TypeScript tests
├── docs/                   # Sphinx documentation
├── memory-bank/            # Active context for agents
└── .kiro/                  # Kiro steering rules
```

## TypeScript Source (`src/`)

```
src/
├── core/                   # Shared types, config, tokens
│   ├── types.ts           # Core type definitions
│   ├── tokens.ts          # Multi-org token switching logic
│   ├── config.ts          # Cosmiconfig loader
│   └── providers.ts       # AI provider factory
├── fleet/                  # Cursor agent fleet management
│   ├── fleet.ts           # High-level Fleet API
│   └── cursor-api.ts      # Direct Cursor API client
├── triage/                 # AI-powered analysis
│   ├── analyzer.ts        # Multi-provider AI analysis
│   ├── agent.ts           # Agent conversation analysis
│   ├── pr-triage-agent.ts # PR review agent
│   ├── resolver.ts        # Issue resolution
│   ├── security.ts        # Security scanning
│   └── types.ts           # Triage types
├── github/                 # Token-aware GitHub operations
│   └── client.ts          # Multi-org GitHub client
├── handoff/                # Agent continuity protocol
│   └── manager.ts         # Handoff state management
├── cli.ts                  # Commander CLI entry point
└── index.ts                # Main package exports
```

### Module Exports

The package provides subpath exports:
- `agentic-control` → Main exports (Fleet, AIAnalyzer, etc.)
- `agentic-control/fleet` → Fleet management only
- `agentic-control/triage` → AI triage only
- `agentic-control/github` → GitHub client only
- `agentic-control/handoff` → Handoff protocol only
- `agentic-control/core` → Core types and config

## Python Source (`python/src/crew_agents/`)

```
python/src/crew_agents/
├── core/                   # Generic CrewAI engine
│   ├── discovery.py       # Discovers .crewai/ directories
│   ├── loader.py          # Loads crews from YAML
│   └── runner.py          # Executes crews
├── base/                   # Reusable templates
│   └── archetypes.yaml    # Agent archetypes
├── config/                 # Configuration
│   ├── agents.yaml        # Agent definitions
│   ├── tasks.yaml         # Task definitions
│   └── llm.py             # LLM configuration
├── crews/                  # Specific crew implementations
│   ├── asset_pipeline/
│   ├── creature_design/
│   ├── ecs_implementation/
│   ├── game_builder/
│   ├── gameplay_design/
│   ├── qa_validation/
│   ├── rendering/
│   └── world_design/
├── flows/                  # Multi-crew workflows
│   ├── asset_generation_flow.py
│   ├── game_design_flow.py
│   ├── implementation_flow.py
│   └── tdd_prototype_flow.py
├── tools/                  # Shared tools
│   └── file_tools.py      # File I/O tools
├── crew.py                 # Main crew orchestration
├── main.py                 # CLI entry point
└── run_flow.py             # Flow runner
```

## Configuration Files

### TypeScript
- `package.json` → npm package config, scripts, dependencies
- `tsconfig.json` → TypeScript compiler options
- `typedoc.json` → API documentation generation
- `.prettierrc` → Code formatting rules
- `.prettierignore` → Files to skip formatting

### Python
- `python/pyproject.toml` → Python package config, dependencies, tools
- `python/uv.lock` → Locked dependency versions
- `python/crewbase.yaml` → CrewAI base configuration

### Project
- `agentic.config.json` → User configuration (tokens, orgs, AI providers)
- `.gitignore` → Git exclusions
- `LICENSE` → MIT license

## Documentation (`docs/`)

Sphinx-based documentation with:
- `docs/conf.py` → Sphinx configuration
- `docs/index.rst` → Documentation home
- `docs/api/` → API reference (TypeScript + Python)
- `docs/getting-started/` → Installation and quickstart
- `docs/development/` → Architecture and contributing

## Tests

### TypeScript (`tests/`)
- Vitest test files: `*.test.ts`
- Example: `tests/tokens.test.ts`

### Python (`python/tests/`)
- Pytest test files: `test_*.py`
- `conftest.py` → Shared fixtures
- Examples: `test_discovery.py`, `test_loader.py`, `test_flows.py`

## Key Patterns

### TypeScript
- **Barrel exports**: Each module has `index.ts` re-exporting public API
- **Type-first**: Types defined in `types.ts`, implementation separate
- **Factory pattern**: Providers created via factory functions
- **Token resolution**: Automatic org-based token selection

### Python
- **Discovery pattern**: Dynamically finds crew definitions
- **YAML-driven**: Agents and tasks defined in YAML, loaded at runtime
- **Archetype system**: Base agent templates extended per crew
- **Flow orchestration**: Multi-crew workflows with state management

## Memory Bank (`memory-bank/`)

- `activeContext.md` → Current project context for AI agents
- Updated manually to track project state
- Read by agents before starting work
