# Installation

agentic-control is available as both a TypeScript package (npm) and a Python companion package (PyPI), plus Docker images for containerized deployments.

## TypeScript Package (npm)

### Prerequisites

- Node.js >= 20.0.0
- npm, yarn, or pnpm

### Install via npm

```bash
npm install -g agentic-control
```

### Install via pnpm (recommended)

```bash
pnpm add -g agentic-control
```

### Install via yarn

```bash
yarn global add agentic-control
```

### Verify Installation

```bash
agentic --version
agentic --help
```

## Python Companion Package (PyPI)

The Python package provides CrewAI-based autonomous development crews that work alongside the TypeScript core.

### Prerequisites

- Python >= 3.11
- pip or uv (recommended)

### Install via uv (recommended)

```bash
# Install uv if you haven't already
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install agentic-crew
uv tool install agentic-crew
```

### Install via pip

```bash
pip install agentic-crew
```

### Verify Installation

```bash
agentic-crew --version
crew-agents --help
```

## Docker Installation

For containerized deployments, use the official Docker images that include both TypeScript and Python packages.

### Pull the Image

```bash
# Latest version
docker pull jbcom/agentic-control:latest

# Specific version
docker pull jbcom/agentic-control:1.1.0

# Specific runtime versions
docker pull jbcom/agentic-control:python3.13-node22
```

### Run with Docker

```bash
# Basic usage
docker run --rm jbcom/agentic-control:latest --help

# With environment variables
docker run --rm \
  -e GITHUB_TOKEN=your_token \
  -e ANTHROPIC_API_KEY=your_key \
  jbcom/agentic-control:latest fleet list

# With volume mounts for workspace
docker run --rm \
  -v $(pwd):/workspace \
  -v $(pwd)/output:/output \
  -e GITHUB_TOKEN=your_token \
  jbcom/agentic-control:latest sandbox run "Analyze this codebase"
```

## Development Installation

For development or contributing to the project:

### TypeScript Development

```bash
# Clone the repository
git clone https://github.com/jbcom/agentic-control.git
cd agentic-control

# Install dependencies
pnpm install

# Build the project
pnpm run build

# Run tests
pnpm run test

# Run CLI from source
pnpm run agentic
```

### Python Development

```bash
# Navigate to Python directory
cd python

# Install with development dependencies
uv sync --extra tests

# Run tests
uv run pytest tests/ -v

# Run CLI from source
uv run crew-agents --help
```

## Environment Setup

agentic-control requires several environment variables for full functionality:

### Required Tokens

```bash
# GitHub access (required)
export GITHUB_TOKEN=ghp_your_github_token

# AI providers (at least one required)
export ANTHROPIC_API_KEY=sk-ant-your_anthropic_key
export OPENAI_API_KEY=sk-your_openai_key
export GOOGLE_API_KEY=your_google_key
export MISTRAL_API_KEY=your_mistral_key

# Cursor integration (optional)
export CURSOR_API_KEY=your_cursor_key
```

### Multi-Organization Setup

For managing multiple GitHub organizations:

```bash
# Organization-specific tokens
export GITHUB_ORG1_TOKEN=ghp_token_for_org1
export GITHUB_ORG2_TOKEN=ghp_token_for_org2
export GITHUB_COMPANY_TOKEN=ghp_token_for_company

# PR review token (optional, defaults to GITHUB_TOKEN)
export PR_REVIEW_TOKEN=ghp_dedicated_review_token
```

## Configuration

Initialize your configuration:

```bash
agentic init
```

This creates an `agentic.config.json` file with your preferences:

```json
{
  "tokens": {
    "organizations": {
      "my-org": {
        "name": "my-org",
        "tokenEnvVar": "GITHUB_MY_ORG_TOKEN"
      }
    },
    "defaultTokenEnvVar": "GITHUB_TOKEN",
    "prReviewTokenEnvVar": "GITHUB_TOKEN"
  },
  "triage": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514"
  },
  "fleet": {
    "autoCreatePr": false
  },
  "logLevel": "info"
}
```

## Troubleshooting

### Common Issues

**Command not found after installation:**
- Ensure your PATH includes the global package installation directory
- For npm: `npm config get prefix` and add `<prefix>/bin` to PATH
- For pnpm: Add `~/.local/share/pnpm` to PATH

**Permission errors:**
- Use `sudo` for global npm installs, or configure npm to use a different directory
- Consider using a Node version manager like nvm

**Docker permission errors:**
- Ensure Docker daemon is running
- Add your user to the docker group: `sudo usermod -aG docker $USER`

**API key errors:**
- Verify environment variables are set: `echo $GITHUB_TOKEN`
- Check token permissions and expiration
- Use `agentic tokens status` to verify token availability

### Getting Help

- Check the [documentation](https://jbcom.github.io/agentic-control/)
- View command help: `agentic <command> --help`
- Report issues: [GitHub Issues](https://github.com/jbcom/agentic-control/issues)
- Join discussions: [GitHub Discussions](https://github.com/jbcom/agentic-control/discussions)