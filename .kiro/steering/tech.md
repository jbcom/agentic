# Tech Stack

## TypeScript Package

### Runtime & Build
- **Node.js**: >=20.0.0
- **TypeScript**: 5.7.0
- **Module System**: ES Modules (NodeNext)
- **Build**: `tsc` (TypeScript compiler)
- **Package Manager**: pnpm

### Core Dependencies
- **AI SDK**: Vercel AI SDK with pluggable providers (@ai-sdk/*)
- **MCP**: @modelcontextprotocol/sdk, @ai-sdk/mcp
- **GitHub**: @octokit/rest
- **CLI**: commander, @inquirer/prompts
- **Config**: cosmiconfig, env-var
- **Validation**: zod
- **Git**: simple-git

### Testing & Quality
- **Test Runner**: vitest
- **Linter**: eslint
- **Formatter**: prettier
- **Type Checking**: TypeScript strict mode

### Common Commands
```bash
# Install dependencies
pnpm install

# Development (watch mode)
pnpm run dev

# Build
pnpm run build

# Test
pnpm run test
pnpm run test:coverage

# Format
pnpm run format
pnpm run format:check

# Lint
pnpm run lint

# Type check
pnpm run typecheck

# Run CLI from source
pnpm run agentic
```

## Python Package

### Runtime & Build
- **Python**: >=3.11
- **Package Manager**: uv (Astral's fast Python package manager)
- **Build System**: hatchling

### Core Dependencies
- **CrewAI**: crewai[tools,anthropic] >=1.5.0
- **Config**: pyyaml, pydantic >=2.12.4
- **Integrations**: vendor-connectors[meshy]
- **MCP**: mcp >=1.0.0 (optional)

### Testing & Quality
- **Test Runner**: pytest
- **Linter/Formatter**: ruff
- **Coverage**: pytest-cov

### Common Commands
```bash
cd python

# Install dependencies
uv sync --extra tests

# Run tests
uv run pytest tests/ -v
uv run pytest tests/ --cov

# Lint and format
uvx ruff check --fix src/ tests/
uvx ruff format src/ tests/

# Run CLI
uv run crew-agents run <crew-name>

# Start MCP server
uv run crew-mcp
```

## Configuration

### TypeScript Config (tsconfig.json)
- Target: ES2022
- Module: NodeNext
- Strict mode enabled
- Declaration maps for debugging
- No unused locals/parameters

### Python Config (pyproject.toml)
- Line length: 100
- Target: Python 3.11+
- Ruff linting: E, W, F, I, UP, B, C4
- Ignores: D (docstrings), T201 (print statements)

## Code Style

### TypeScript
- ES Modules with .js extensions in imports
- Strict TypeScript with no implicit any
- Prettier for formatting (no config overrides)
- Functional patterns preferred

### Python
- Type hints required (Pydantic models)
- Ruff formatting (Black-compatible)
- 100 character line limit
- Modern Python features (3.11+)

## Commit Conventions

Use conventional commits with scope:
- `feat(node): description` or `feat(fleet): description` → minor bump
- `feat(python): description` or `feat(crew): description` → minor bump
- `fix(scope): description` → patch bump
- `docs(scope): description` → no bump
