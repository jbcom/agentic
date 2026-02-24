---
title: "Game Generator API Reference"
description: "Complete API reference for the game-generator Rust crate, covering project configuration, AI-powered generation, vintage game systems, and the Bevy wizard plugin."
---

# Game Generator API Reference

The `game-generator` crate is an AI-powered vintage game generator built with Rust and Bevy. It provides a visual wizard interface for configuring game projects, AI conversation-driven design, procedural content generation, and a curated vintage game timeline (1980--1995) for creative inspiration.

## Crate Structure

```
game_generator
  ai_client        -- AI provider clients (OpenAI, Anthropic), caching, embeddings
  blending         -- Game concept blending and similarity analysis
  build_tools      -- Code generation templates and build graph
  combat           -- Turn-based combat system (damage, effects, progression)
  metaprompts      -- AI conversation-driven game design pipeline
  vintage_games    -- Curated timeline of exemplar games (1980-1995)
  wizard           -- Bevy/egui visual wizard plugin
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
game-generator = { path = "../packages/game-generator" }
```

Or use the binary directly:

```bash
cd packages/game-generator
cargo run -- --help
```

---

## CLI Usage

```bash
# Launch in generate mode (default) -- creates a new project
game-generator

# Launch in list mode -- browse existing projects
game-generator --list

# Specify base directory
game-generator --base-dir ~/.config/vintage_games

# Open an existing project
game-generator --project-dir ./my-game

# AI configuration
game-generator \
  --ai-provider openai \
  --text-model gpt-4 \
  --image-model dall-e-3 \
  --temperature 0.8 \
  --max-tokens 2000 \
  --cache true
```

**CLI Flags:**

| Flag | Default | Description |
|------|---------|-------------|
| `--base-dir`, `-b` | XDG config dir | Base directory for all projects |
| `--project-dir`, `-p` | New UUID in base dir | Specific project directory |
| `--config-file`, `-c` | `project.toml` in project dir | Config file path |
| `--list`, `-l` | `false` | Browse existing projects |
| `--generate`, `-g` | `true` | Run in generate mode |
| `--ai-provider` | `openai` | AI provider (`openai`, `anthropic`) |
| `--text-model` | `gpt-4` | Text generation model |
| `--image-model` | `dall-e-3` | Image generation model |
| `--audio-model` | `tts-1` | Audio generation model |
| `--temperature` | `0.8` | Sampling temperature (0.0--2.0) |
| `--top-p` | `0.95` | Nucleus sampling (0.0--1.0) |
| `--max-tokens` | `2000` | Maximum token output |
| `--frequency-penalty` | `0.0` | Frequency penalty (-2.0 to 2.0) |
| `--presence-penalty` | `0.0` | Presence penalty (-2.0 to 2.0) |
| `--image-quality` | `standard` | Image quality (`standard`, `hd`) |
| `--image-size` | `1024x1024` | Image size |
| `--cache` | `true` | Enable AI response caching |
| `--ai-timeout` | `120` | Request timeout in seconds |

---

## TOML Configuration

Each project is stored as a TOML file containing the complete game specification. The wizard builds this incrementally through user interaction and AI conversation.

### ProjectConfig

The top-level configuration structure:

```toml
name = "Chrono Legends"
description = "A retro-inspired JRPG with time travel mechanics"
author = "Game Dev Studio"
version = "0.1.0"

[metadata]
id = "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
created_at = "2025-01-15T10:30:00Z"
last_modified = "2025-01-15T14:22:00Z"
version = "0.1.0"

[basic_info]
name = "Chrono Legends"
tagline = "Time bends to the will of heroes"
description = "A retro JRPG where players manipulate time to solve puzzles and defeat enemies"
genre = "JRPG"
target_audience = "Retro game enthusiasts, JRPG fans"
inspiration_notes = "Chrono Trigger meets Final Fantasy VI"

[gameplay]
core_mechanics = ["turn-based combat", "time manipulation", "party management"]
gameplay_loop = "Explore -> Battle -> Story -> Time Shift -> New Era"
progression_type = "experience-based with skill trees"
victory_conditions = ["Defeat the Temporal Rift Boss", "Restore the timeline"]
unique_mechanics = ["time rewind in combat", "era-specific abilities"]
player_motivation = "Uncover the mystery across multiple time periods"

[gameplay.difficulty_curve]
starting_difficulty = 0.3
ramp_speed = 0.5
max_difficulty = 0.85
adaptive = true

[visual_style]
reference_games = ["Chrono Trigger", "Final Fantasy VI", "Secret of Mana"]
color_mood = "warm-nostalgic"
sprite_size = 32
use_outline = true
outline_style = "pixel-perfect"
shading_technique = "dithered"
animation_complexity = "medium"
ui_theme = "parchment-scroll"
art_direction_notes = "16-bit SNES aesthetic with modern color depth"
special_effects = ["screen flash on critical", "parallax backgrounds"]

[features]
save_system = true
day_night_cycle = true
weather_effects = false
minimap = true
achievements = true

[features.combat_system]
combat_type = "turn-based"
damage_numbers = true
combos = true
special_abilities = ["Time Stop", "Chrono Slash", "Temporal Heal"]

[features.inventory_system]
slot_count = 40
stack_size = 99
categories = ["Weapons", "Armor", "Items", "Key Items"]
special_items = ["Chrono Crystal", "Era Key"]

[features.dialogue_system]
dialogue_type = "branching"
portrait_style = "detailed"
text_speed = "variable"
branching_depth = 3
personality_system = true

[technical]
world_size = "large"
performance_target = "60fps"
target_platforms = ["PC", "Steam Deck"]
```

---

## Core Types

### GameConfig

The complete game configuration used by the AI generation pipeline:

```rust
use game_generator::ai_client::game_types::GameConfig;

pub struct GameConfig {
    // Core Identity
    pub name: String,
    pub tagline: String,
    pub genre: String,       // "JRPG", "Roguelike", "Action RPG", etc.
    pub setting: String,     // "Fantasy", "Sci-Fi", "Post-Apocalyptic"
    pub era: String,         // Target vintage era for art style

    // Visual Style
    pub art_style: ArtStyle,
    pub color_palette: ColorPalette,
    pub reference_games: Vec<String>,

    // World Design
    pub world: WorldConfig,
    pub towns: Vec<TownConfig>,
    pub dungeons: Vec<DungeonConfig>,

    // RPG Systems
    pub party_system: PartySystem,
    pub combat_system: CombatSystem,
    pub dialog_system: DialogSystem,
    pub inventory_system: InventorySystem,
    pub shop_system: ShopSystem,
    pub quest_system: QuestSystem,

    // Story
    pub main_quest: QuestLine,
    pub side_quests: Vec<QuestLine>,
    pub characters: Vec<Character>,

    // Audio
    pub music_style: String,
    pub sound_effects_style: String,
}
```

### ArtStyle

```rust
pub struct ArtStyle {
    pub sprite_size: u32,              // e.g., 16, 32, 64
    pub tile_size: u32,
    pub animation_frames: HashMap<String, u32>,
    pub perspective: String,           // "3/4 top-down", "side-scroll"
    pub shading: String,               // "flat", "dithered", "gradient"
    pub outline: OutlineStyle,
}

pub struct OutlineStyle {
    pub enabled: bool,
    pub color: String,                 // Hex color
    pub thickness: u32,
}
```

---

## World Generation

### WorldConfig

```rust
pub struct WorldConfig {
    pub name: String,
    pub size: String,                  // "small", "medium", "large"
    pub regions: Vec<Region>,
    pub connections: Vec<Connection>,
}

pub struct Region {
    pub name: String,
    pub biome: String,                 // "forest", "desert", "tundra", etc.
    pub description: String,
    pub key_locations: Vec<String>,
}

pub struct Connection {
    pub from: String,
    pub to: String,
    pub connection_type: String,       // "road", "bridge", "cave", "portal"
}
```

### Town and Dungeon Configuration

```rust
pub struct TownConfig {
    pub name: String,
    pub size: String,                  // "hamlet", "village", "city"
    pub description: String,
    pub shops: Vec<String>,
    pub key_npcs: Vec<String>,
    pub inn: bool,
    pub save_point: bool,
}

pub struct DungeonConfig {
    pub name: String,
    pub theme: String,                 // "fire temple", "ice cavern"
    pub floors: u32,
    pub boss: String,
    pub treasures: Vec<String>,
    pub gimmick: String,               // Unique dungeon mechanic
}
```

---

## Character Generation

### Character

```rust
pub struct Character {
    pub name: String,
    pub role: String,                  // "protagonist", "companion", "villain"
    pub personality: String,
    pub backstory: String,
    pub portrait_description: String,  // For AI image generation
}

pub struct CharacterClass {
    pub name: String,
    pub description: String,
    pub stat_growth: HashMap<String, String>,  // e.g., {"HP": "high", "MP": "low"}
    pub abilities: Vec<String>,
}

pub struct PartySystem {
    pub max_party_size: u32,
    pub switchable: bool,
    pub formation_system: bool,
    pub character_classes: Vec<CharacterClass>,
}
```

---

## Quest Generation

### QuestSystem and QuestLine

```rust
pub struct QuestSystem {
    pub journal: bool,
    pub markers: bool,
    pub reward_types: Vec<String>,     // "experience", "gold", "items", "abilities"
}

pub struct QuestLine {
    pub name: String,
    pub description: String,
    pub steps: Vec<QuestStep>,
    pub rewards: Vec<String>,
}

pub struct QuestStep {
    pub description: String,
    pub objective_type: String,        // "defeat", "collect", "escort", "explore"
    pub location: String,
}
```

---

## Combat System

The combat module provides a data-driven turn-based combat system:

```rust
use game_generator::combat::{
    damage,
    effects,
    progression,
    state,
};
```

### CombatSystem

```rust
pub struct CombatSystem {
    pub style: String,                 // "turn-based", "atb", "tactical"
    pub features: Vec<String>,         // "elemental weakness", "combo system"
}
```

---

## Vintage Games Timeline

The `vintage_games` module provides a curated timeline of exemplar games from 1980--1995 for creative inspiration:

```rust
use game_generator::vintage_games::{
    TIMELINE_GAMES,       // All timeline entries
    TIMELINE_START,       // 1980
    TIMELINE_END,         // 1995
    TimelineGame,
    Era,
    era_for_year,
    games_by_year,
    games_by_genre,
    games_by_era,
    all_genres,
    search_games,
    build_game_graph,
    PlatformInfo,
    get_platform_info,
};
```

### Eras

| Era | Years | Description |
|-----|-------|-------------|
| Golden Age | 1980--1983 | Arcade origins, early home consoles |
| 8-bit Renaissance | 1984--1987 | NES/Master System era, genre foundations |
| 16-bit Golden Age | 1988--1991 | SNES/Genesis, genre maturation |
| Peak Innovation | 1992--1995 | 3D beginnings, genre refinement |

```rust
use game_generator::vintage_games::{era_for_year, Era};

let era = era_for_year(1991);
// -> Some(Era::SixteenBitGoldenAge)
```

### Game Search

```rust
use game_generator::vintage_games::search_games;

let results = search_games("zelda");
for game in results {
    println!("{} ({}) - {}", game.name, game.year, game.genre);
}
```

### Game Influence Graph

Build a graph of game influences for AI context:

```rust
use game_generator::vintage_games::build_game_graph;

let graph = build_game_graph();
// GameNode { game, influences: Vec<GameNode>, influenced_by: Vec<GameNode> }
```

---

## Bevy Integration

### WizardPlugin

The `WizardPlugin` is a Bevy plugin that provides the visual wizard interface using `bevy_egui`:

```rust
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use game_generator::{WizardPlugin, AppMode, AppDirectories};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Vintage Game Generator".to_string(),
            resolution: (1280.0, 800.0).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EguiPlugin::default())
    .insert_resource(AppMode::Generate)  // or AppMode::List
    .insert_resource(AppDirectories {
        base_dir: dirs::config_dir()
            .unwrap()
            .join("vintage_game_generator"),
        project_dir: PathBuf::from("./my-project"),
        config_file: Some(PathBuf::from("./my-project/project.toml")),
        prompts_dir: PathBuf::from("./my-project/prompts"),
        assets_dir: PathBuf::from("./my-project/assets"),
        mode: AppMode::Generate,
    })
    .add_plugins(WizardPlugin)
    .run();
}
```

### AppMode

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum AppMode {
    /// Create or edit a game project
    Generate,
    /// Browse existing projects
    List,
}
```

### AppDirectories

```rust
pub struct AppDirectories {
    pub base_dir: PathBuf,           // Root directory for all projects
    pub project_dir: PathBuf,        // Current project directory
    pub config_file: Option<PathBuf>, // Path to project.toml
    pub prompts_dir: PathBuf,        // AI prompts directory
    pub assets_dir: PathBuf,         // Generated assets directory
    pub mode: AppMode,
}
```

The plugin uses the Catppuccin Mocha theme for a modern, dark interface via `catppuccin_egui`.

---

## ConfigManager

The `ConfigManager` handles loading and saving TOML project files:

```rust
use game_generator::wizard::config::{ConfigManager, ProjectConfig};
use std::path::Path;

// Create a new project config
let mut manager = ConfigManager::new(
    Path::new("./projects"),
    Some("chrono-legends"),
)?;

// Modify configuration
manager.config.basic_info.name = "Chrono Legends".to_string();
manager.config.basic_info.genre = "JRPG".to_string();

// Save to disk
manager.save()?;

// Export for AI generation context
let toml_string = manager.export_for_generation();
```

### Loading Existing Projects

```rust
use game_generator::wizard::config::ProjectConfig;
use std::path::Path;

let config = ProjectConfig::load(Path::new("./projects/chrono-legends.toml"))?;

// Generate AI context summary
let summary = config.to_ai_summary();
println!("{}", summary);
```

---

## AI Context

The `AiContext` struct stores conversation history, design decisions, and generated content concepts:

```rust
pub struct AiContext {
    pub conversation_history: Vec<ConversationEntry>,
    pub design_decisions: Vec<DesignDecision>,
    pub style_guide: Option<String>,
    pub world_lore: Option<String>,
    pub character_concepts: Vec<CharacterConcept>,
    pub level_themes: Vec<LevelTheme>,
}

pub struct DesignDecision {
    pub category: String,              // "combat", "story", "art"
    pub decision: String,
    pub rationale: String,
    pub alternatives_considered: Vec<String>,
}

pub struct CharacterConcept {
    pub name: String,
    pub role: String,
    pub description: String,
    pub abilities: Vec<String>,
}

pub struct LevelTheme {
    pub name: String,
    pub atmosphere: String,
    pub key_mechanics: Vec<String>,
    pub visual_elements: Vec<String>,
}
```

```rust
// Add a design decision during AI conversation
config.add_design_decision(
    "combat",
    "Use ATB (Active Time Battle) system",
    "Better pacing for modern players while retaining retro feel",
);

// Track conversation
config.add_conversation(
    "assistant",
    "I recommend an ATB system for the combat...",
    "gameplay_design",
);
```

---

## Error Handling

```rust
use game_generator::{GeneratorError, Result};

pub enum GeneratorError {
    ApiError(String),
    TemplateError(String),
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    GenerationFailed(String),
}

// All public functions return Result<T>
pub type Result<T> = std::result::Result<T, GeneratorError>;
```

---

## Exported Types

Top-level re-exports from `lib.rs`:

```rust
pub use metaprompts::{GameConfig, GameGenerator, GenerationPhase, GenerationProgress};
pub use wizard::{AppDirectories, AppMode, AppState, WizardPlugin};

// Additional public types
pub struct GameInfo {
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub genre: String,
    pub setting: String,
    pub art_style: String,
}

pub struct GenerationOptions {
    pub include_voice_acting: bool,
    pub include_orchestral_music: bool,
    pub target_playtime_hours: u32,
    pub difficulty_options: Vec<String>,
    pub accessibility_features: Vec<String>,
}

pub struct ProgressUpdate {
    pub phase: String,
    pub step: String,
    pub progress: f32,
    pub message: String,
    pub artifact: Option<GeneratedArtifact>,
}

pub struct GeneratedArtifact {
    pub artifact_type: String,
    pub name: String,
    pub path: Option<String>,
    pub preview: Option<String>,  // Base64 encoded image preview
}
```

---

## Related Pages

- [Crew API Reference](/api/crew/) -- Python crew orchestration
- [Fleet API Reference](/api/fleet-management/) -- Agent fleet management
- [Configuration API](/api/configuration/) -- Config schema reference
