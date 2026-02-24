# game-generator

[![CI](https://github.com/jbcom/agentic/actions/workflows/ci.yml/badge.svg)](https://github.com/jbcom/agentic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)

Visual-first vintage game generator with AI assistance. A Bevy-powered desktop application that guides you through creating retro-inspired games using AI-driven content generation, game mechanic blending, and metaprompt-based code scaffolding. Combines a guided wizard with freeform conversation modes for game design.

[Full Documentation](https://agentic.coach) | [Package Docs](https://agentic.coach/packages/game-generator/)

## Installation

### From source

```bash
git clone https://github.com/jbcom/agentic.git
cd agentic
cargo build --release -p game-generator
```

### Run directly

```bash
cargo run -p game-generator
```

## Quick Start

```bash
# Launch the wizard UI
game-generator

# Launch in list mode (browse vintage games)
game-generator --mode list

# Launch in generate mode (skip to generation)
game-generator --mode generate
```

## Key Features

- **Guided wizard** -- step-by-step game creation with timeline browsing, game card selection, and mechanic blending
- **Freeform conversation** -- chat with AI to iteratively design your game concept
- **Vintage game database** -- browse games across computing eras with platform and genre metadata
- **Mechanic blending** -- graph-based system for combining mechanics from different games
- **AI content generation** -- text, image, audio, and code generation via OpenAI-compatible APIs
- **Metaprompt engine** -- template-driven code scaffolding with validation
- **Combat system** -- configurable damage, effects, and progression systems
- **Build tools** -- AI-assisted analysis and code generation pipeline

## Architecture

| Module | Description |
|--------|-------------|
| `wizard` | Bevy + egui desktop UI with guided and freeform modes |
| `vintage_games` | Era/platform/genre database with game metadata |
| `blending` | Graph-based mechanic blending with similarity scoring |
| `ai_client` | OpenAI-compatible client for text, image, audio, and embeddings |
| `metaprompts` | Template engine for generating game code from specs |
| `build_tools` | AI analysis, code generation, and project scaffolding |
| `combat` | Damage, effects, and progression system generators |

## Environment

```bash
OPENAI_API_KEY=your_api_key   # Required for AI generation features
```

## Development

```bash
cargo check -p game-generator
cargo test -p game-generator
cargo clippy -p game-generator
cargo fmt
```

## Requirements

- Rust >= 1.85
- A GPU-capable system (for Bevy rendering)
- OpenAI-compatible API key (for AI features)

## Documentation

Visit [agentic.coach](https://agentic.coach) for full documentation including the [Game Generator guide](https://agentic.coach/packages/game-generator/).

## License

MIT
