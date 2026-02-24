---
title: "game-generator"
description: AI-powered retro RPG generator with multi-provider AI integration, built with Bevy
---

# Vintage Game Generator

> AI-powered retro RPG generator with multi-provider AI integration, built with the Bevy game engine

[![CI](https://github.com/agentic-dev-library/game-generator/workflows/Rust%20CI/badge.svg)](https://github.com/agentic-dev-library/game-generator/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Vintage Game Generator** is part of the [Strata](https://strata.game) gaming platform, bringing AI-powered procedural content generation to retro RPG experiences.

## Features

- 🤖 **Multi-Provider AI**: OpenAI, Anthropic integration with intelligent caching
- 🎮 **Complete Game Generation**: Generates playable Bevy games
- 📝 **TOML Configuration**: Simple dev.toml-based project setup
- 🎨 **Asset Generation**: Sprites, audio, dialogs, maps
- 🧙‍♂️ **Interactive Wizard**: GUI for designing your RPG

## Crate Structure

| Crate | Description | Status |
|-------|-------------|--------|
| `vintage-ai-client` | AI service abstraction with caching | 🚧 Migration |
| `vintage-blending-core` | Game similarity algorithms | 🚧 Migration |
| `vintage-build-tools` | Build-time code generation | 🚧 Migration |
| `vintage-game-generator` | Main application with wizard | 🚧 Migration |

## Quick Start

### Prerequisites

- **Rust 1.75+** (nightly recommended for some features)
- **Cargo** package manager

### Installation

```bash
# Clone the repository
git clone https://github.com/agentic-dev-library/game-generator.git
cd game-generator

# Build all crates
cargo build --all

# Run the generator
cargo run --bin vintage-game-generator
```

### Development

```bash
# Check all crates
cargo check --all

# Run tests
cargo test --all

# Run with release optimizations
cargo run --bin vintage-game-generator --release
```

## Architecture

### vintage-ai-client

AI service abstraction that supports multiple providers with intelligent caching:

```rust
use vintage_ai_client::{AIClient, Provider};

let client = AIClient::new()
    .provider(Provider::Anthropic)
    .model("claude-sonnet-4-20250514")
    .with_cache()
    .build()?;

let response = client.generate("Create a fantasy village description").await?;
```

Features:
- Multi-provider support (OpenAI, Anthropic)
- Response caching for development
- Retry logic with exponential backoff
- Token usage tracking

### vintage-blending-core

Game similarity algorithms for content blending:

```rust
use vintage_blending_core::{GameBlender, SimilarityMetric};

let blender = GameBlender::new();
let games = vec![zelda_features, final_fantasy_features, chrono_trigger_features];
let blend = blender.create_blend(games, SimilarityMetric::Weighted)?;
```

Features:
- Feature vector comparison
- Style blending algorithms
- Weighted similarity metrics

### vintage-build-tools

Build-time code generation for game assets:

```rust
// In build.rs
use vintage_build_tools::AssetCompiler;

fn main() {
    AssetCompiler::new()
        .source_dir("assets/")
        .output_dir("src/generated/")
        .compile_sprites()
        .compile_maps()
        .run();
}
```

Features:
- Sprite atlas generation
- Map data compilation
- Dialog tree processing

### vintage-game-generator

Main application with interactive wizard:

```rust
use vintage_game_generator::{Wizard, GameConfig};

fn main() {
    let config = Wizard::new()
        .with_ai_provider("anthropic")
        .run_interactive()?;
    
    let game = GameGenerator::from_config(config)
        .generate_world()
        .generate_characters()
        .generate_quests()
        .build()?;
    
    game.run();
}
```

Features:
- Interactive GUI wizard
- AI-driven content generation
- Complete game scaffolding
- Bevy integration

## Configuration

Create a `dev.toml` in your project root:

```toml
[project]
name = "My Vintage RPG"
version = "0.1.0"

[ai]
provider = "anthropic"
model = "claude-sonnet-4-20250514"
cache_responses = true

[generation]
world_size = "medium"
character_count = 10
quest_complexity = "medium"

[style]
art_style = "16bit"
music_style = "chiptune"
palette = "nes"
```

## AI-Powered Generation

### World Generation

```rust
let world = generator.generate_world(WorldConfig {
    size: WorldSize::Medium,
    biomes: vec![Biome::Forest, Biome::Mountains, Biome::Desert],
    civilizations: 3,
    magical_level: MagicLevel::High,
})?;
```

### Character Generation

```rust
let characters = generator.generate_characters(CharacterConfig {
    count: 10,
    include_npcs: true,
    party_size: 4,
    personality_depth: PersonalityDepth::Deep,
})?;
```

### Quest Generation

```rust
let quests = generator.generate_quests(QuestConfig {
    main_storyline: true,
    side_quests: 20,
    complexity: QuestComplexity::Medium,
    branching: true,
})?;
```

## Integration with Strata

Vintage Game Generator is part of the [Strata](https://strata.game) gaming platform:

- **Shared AI Infrastructure**: Uses the same AI providers as other Strata tools
- **Asset Pipeline**: Integrates with Strata's asset management system
- **Multiplayer Support**: Generated games can connect to Strata multiplayer services
- **Mod Support**: Games support Strata's modding framework

## Examples

### Minimal Game

```rust
use vintage_game_generator::prelude::*;

fn main() {
    let game = QuickGenerator::new()
        .genre(Genre::Jrpg)
        .theme(Theme::Fantasy)
        .generate_and_run();
}
```

### Custom World

```rust
use vintage_game_generator::prelude::*;

fn main() {
    let config = GameConfig::builder()
        .name("Crystal Quest")
        .world(WorldConfig {
            seed: 42,
            size: WorldSize::Large,
            biomes: vec![Biome::Crystal, Biome::Void, Biome::Ancient],
        })
        .characters(CharacterConfig {
            party_size: 4,
            include_npcs: true,
        })
        .build();
    
    let game = GameGenerator::from_config(config)
        .generate_all()
        .build()?;
    
    game.run();
}
```

## Bevy Integration

The generated games use the Bevy game engine:

```rust
use bevy::prelude::*;
use vintage_game_generator::BevyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VintageGamePlugin::new("my_game"))
        .run();
}
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Related Projects

- **[Strata](https://strata.game)** - AI-powered gaming platform
- **[Bevy](https://bevyengine.org/)** - Game engine used for rendering
- **[@agentic/control](/packages/control/)** - AI agent orchestration
- **[@agentic/crew](/packages/crew/)** - Multi-agent workflows

## Links

- [GitHub Repository](https://github.com/agentic-dev-library/game-generator)
- [Strata Platform](https://strata.game)
