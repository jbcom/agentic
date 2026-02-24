---
title: "game-generator"
description: AI-powered retro RPG generation with Bevy — procedural worlds, characters, quests, and assets
---

# game-generator

<div class="polyglot-bar">
  <span class="lang-badge lang-badge--rs">Rust</span>
</div>

> AI-powered retro RPG generation built on Bevy. Procedural worlds, characters, quests, and assets from a TOML config file.

[![CI](https://github.com/jbcom/agentic/workflows/Rust%20CI/badge.svg)](https://github.com/jbcom/agentic/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Part of the [Strata](https://strata.game) gaming platform.

## What It Does

game-generator takes a TOML configuration and AI provider credentials, then procedurally generates complete retro RPGs:

- **Worlds** — Biomes, civilizations, magic systems
- **Characters** — NPCs with deep personality, party members, dialog trees
- **Quests** — Main storylines and branching side quests
- **Assets** — Sprites, maps, audio (16-bit/chiptune styles)

The output is a playable Bevy game.

## Quick Start

```bash
# Clone the monorepo
git clone https://github.com/jbcom/agentic.git
cd agentic/packages/game-generator

# Build and run
cargo run --bin vintage-game-generator
```

### Minimal Game (3 lines)

```rust
use vintage_game_generator::prelude::*;

fn main() {
    QuickGenerator::new()
        .genre(Genre::Jrpg)
        .theme(Theme::Fantasy)
        .generate_and_run();
}
```

## Configuration

Create `dev.toml` in your project root:

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

## Module Architecture

| Module | Description |
|--------|-------------|
| **vintage-ai-client** | Multi-provider AI abstraction with response caching and retry logic |
| **vintage-blending-core** | Game similarity algorithms — blend styles from classic RPGs |
| **vintage-build-tools** | Build-time code generation for sprites, maps, and dialog trees |
| **vintage-game-generator** | Main application with interactive wizard and Bevy integration |

### AI Client

```rust
use vintage_ai_client::{AIClient, Provider};

let client = AIClient::new()
    .provider(Provider::Anthropic)
    .model("claude-sonnet-4-20250514")
    .with_cache()
    .build()?;

let response = client.generate("Create a fantasy village").await?;
```

### Game Blending

Blend elements from classic games to create unique combinations:

```rust
use vintage_blending_core::{GameBlender, SimilarityMetric};

let blender = GameBlender::new();
let games = vec![zelda_features, final_fantasy_features, chrono_trigger_features];
let blend = blender.create_blend(games, SimilarityMetric::Weighted)?;
```

## Generation API

### Worlds

```rust
let world = generator.generate_world(WorldConfig {
    size: WorldSize::Medium,
    biomes: vec![Biome::Forest, Biome::Mountains, Biome::Desert],
    civilizations: 3,
    magical_level: MagicLevel::High,
})?;
```

### Characters

```rust
let characters = generator.generate_characters(CharacterConfig {
    count: 10,
    include_npcs: true,
    party_size: 4,
    personality_depth: PersonalityDepth::Deep,
})?;
```

### Quests

```rust
let quests = generator.generate_quests(QuestConfig {
    main_storyline: true,
    side_quests: 20,
    complexity: QuestComplexity::Medium,
    branching: true,
})?;
```

## Bevy Integration

Generated games run as Bevy plugins:

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

## Custom World Example

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

## Strata Integration

game-generator powers the content generation for [Strata](https://strata.game):

- **Shared AI infrastructure** with other Strata tools
- **Asset pipeline** integration with [@agentic/meshy](/packages/meshy-content-generator/)
- **Multiplayer support** via Strata's networking layer
- **Mod support** through Strata's modding framework

## Development

```bash
cargo check --all        # Check all modules
cargo test --all         # Run tests
cargo clippy --all       # Lint
cargo run --release      # Optimized build
```

## Related

- **[Strata](https://strata.game)** — AI-powered gaming platform
- **[Bevy](https://bevyengine.org/)** — Game engine
- **[@agentic/meshy](/packages/meshy-content-generator/)** — 3D asset pipelines
- **[@jbcom/agentic](/packages/control/)** — Fleet orchestration

## Links

- [GitHub](https://github.com/jbcom/agentic/tree/main/packages/game-generator)
- [Strata Platform](https://strata.game)
