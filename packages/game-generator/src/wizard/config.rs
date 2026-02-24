// app/config.rs - TOML-based game configuration that bridges wizard and AI conversation

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Project configuration built through wizard and enriched by AI conversation
/// This represents the user's preferences and constraints, not the full game specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    // Top-level fields for easy access
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    #[serde(default = "default_version")]
    pub version: String,

    #[serde(default)]
    pub metadata: ProjectMetadata,

    #[serde(default)]
    pub basic_info: BasicInfo,

    #[serde(default)]
    pub gameplay: GameplayDesign,

    #[serde(default)]
    pub visual_style: VisualStyle,

    #[serde(default)]
    pub features: Features,

    #[serde(default)]
    pub technical: TechnicalSettings,

    #[serde(default)]
    pub ai_context: AiContext,

    #[serde(default)]
    pub wizard_state: WizardState,

    // Simplified view for list mode
    pub game_specification: Option<GameSpecification>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

/// Simplified game specification for list mode display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSpecification {
    pub title: String,
    pub genre: String,
    pub theme: String,
    pub art_style: String,
    pub key_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
            version: "0.1.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BasicInfo {
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub genre: String,
    pub target_audience: String,
    pub inspiration_notes: String, // Added for AI context
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameplayDesign {
    pub core_mechanics: Vec<String>,
    pub gameplay_loop: String,
    pub progression_type: String,
    pub victory_conditions: Vec<String>,
    pub difficulty_curve: DifficultyCurve,
    pub unique_mechanics: Vec<String>, // AI can expand on these
    pub player_motivation: String,     // AI can help define
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyCurve {
    pub starting_difficulty: f32,
    pub ramp_speed: f32,
    pub max_difficulty: f32,
    pub adaptive: bool,
}

impl Default for DifficultyCurve {
    fn default() -> Self {
        Self {
            starting_difficulty: 0.3,
            ramp_speed: 0.5,
            max_difficulty: 0.85,
            adaptive: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VisualStyle {
    pub reference_games: Vec<String>,
    pub color_mood: String,
    pub sprite_size: u32,
    pub use_outline: bool,
    pub outline_style: String,
    pub shading_technique: String,
    pub animation_complexity: String,
    pub ui_theme: String,
    pub art_direction_notes: String,  // AI can elaborate
    pub special_effects: Vec<String>, // AI can suggest
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Features {
    pub combat_system: Option<CombatConfig>,
    pub inventory_system: Option<InventoryConfig>,
    pub dialogue_system: Option<DialogueConfig>,
    pub crafting_system: Option<CraftingConfig>,
    pub save_system: bool,
    pub day_night_cycle: bool,
    pub weather_effects: bool,
    pub minimap: bool,
    pub achievements: bool,
    pub custom_features: Vec<CustomFeature>, // AI can add unique features
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFeature {
    pub name: String,
    pub description: String,
    pub complexity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    pub combat_type: String,
    pub damage_numbers: bool,
    pub combos: bool,
    pub special_abilities: Vec<String>, // AI can design these
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryConfig {
    pub slot_count: u32,
    pub stack_size: u32,
    pub categories: Vec<String>,
    pub special_items: Vec<String>, // AI can create these
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueConfig {
    pub dialogue_type: String,
    pub portrait_style: String,
    pub text_speed: String,
    pub branching_depth: u32,
    pub personality_system: bool, // AI can design this
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingConfig {
    pub recipe_discovery: String,
    pub crafting_time: bool,
    pub failure_chance: bool,
    pub ingredient_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TechnicalSettings {
    pub world_size: String,
    pub performance_target: String,
    pub target_platforms: Vec<String>,
    pub multiplayer: Option<MultiplayerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerConfig {
    pub max_players: u32,
    pub network_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiContext {
    pub conversation_history: Vec<ConversationEntry>,
    pub design_decisions: Vec<DesignDecision>,
    pub style_guide: Option<String>,
    pub world_lore: Option<String>,
    pub character_concepts: Vec<CharacterConcept>,
    pub level_themes: Vec<LevelTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub role: String,
    pub content: String,
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDecision {
    pub category: String,
    pub decision: String,
    pub rationale: String,
    pub alternatives_considered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConcept {
    pub name: String,
    pub role: String,
    pub description: String,
    pub abilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelTheme {
    pub name: String,
    pub atmosphere: String,
    pub key_mechanics: Vec<String>,
    pub visual_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WizardState {
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub is_complete: bool,
    pub conversation_started: bool,
}

impl ProjectConfig {
    /// Create a summary for AI context
    pub fn to_ai_summary(&self) -> String {
        format!(
            r#"# Game Configuration Summary

## Basic Information
- **Name**: {}
- **Genre**: {}
- **Tagline**: {}
- **Target Audience**: {}

## Gameplay Design
- **Core Mechanics**: {}
- **Progression Type**: {}
- **Unique Elements**: {}

## Visual Style
- **Mood**: {}
- **References**: {}
- **Sprite Size**: {}px

## Key Features
{}

## Technical Constraints
- **World Size**: {}
- **Performance Target**: {}
- **Platforms**: {}

## Player Experience Goals
{}
"#,
            self.basic_info.name,
            self.basic_info.genre,
            self.basic_info.tagline,
            self.basic_info.target_audience,
            self.gameplay.core_mechanics.join(", "),
            self.gameplay.progression_type,
            self.gameplay.unique_mechanics.join(", "),
            self.visual_style.color_mood,
            self.visual_style.reference_games.join(", "),
            self.visual_style.sprite_size,
            self.list_features(),
            self.technical.world_size,
            self.technical.performance_target,
            self.technical.target_platforms.join(", "),
            self.gameplay.player_motivation
        )
    }

    fn list_features(&self) -> String {
        let mut features = Vec::new();

        if self.features.combat_system.is_some() {
            features.push("- ⚔️ Combat System".to_string());
        }
        if self.features.inventory_system.is_some() {
            features.push("- 🎒 Inventory System".to_string());
        }
        if self.features.dialogue_system.is_some() {
            features.push("- 💬 Dialogue System".to_string());
        }
        if self.features.save_system {
            features.push("- 💾 Save System".to_string());
        }
        if self.features.day_night_cycle {
            features.push("- 🌅 Day/Night Cycle".to_string());
        }

        for custom in &self.features.custom_features {
            features.push(format!("- 🎯 {}", custom.name));
        }

        features.join("\n")
    }

    /// Add a conversation entry
    pub fn add_conversation(&mut self, role: &str, content: &str, phase: &str) {
        self.ai_context
            .conversation_history
            .push(ConversationEntry {
                timestamp: chrono::Utc::now(),
                role: role.to_string(),
                content: content.to_string(),
                phase: phase.to_string(),
            });
        self.metadata.last_modified = chrono::Utc::now();
    }

    /// Add a design decision made during conversation
    pub fn add_design_decision(&mut self, category: &str, decision: &str, rationale: &str) {
        self.ai_context.design_decisions.push(DesignDecision {
            category: category.to_string(),
            decision: decision.to_string(),
            rationale: rationale.to_string(),
            alternatives_considered: Vec::new(),
        });
        self.metadata.last_modified = chrono::Utc::now();
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            author: None,
            version: default_version(),
            metadata: ProjectMetadata::default(),
            basic_info: BasicInfo::default(),
            gameplay: GameplayDesign::default(),
            visual_style: VisualStyle::default(),
            features: Features::default(),
            technical: TechnicalSettings::default(),
            ai_context: AiContext::default(),
            wizard_state: WizardState::default(),
            game_specification: None,
        }
    }
}

/// Config manager for loading/saving TOML files
pub struct ConfigManager {
    config_path: PathBuf,
    pub config: ProjectConfig,
}

impl ConfigManager {
    pub fn new(config_dir: &Path, project_name: Option<&str>) -> Result<Self> {
        let config_path = if let Some(name) = project_name {
            config_dir.join(format!("{name}.toml"))
        } else {
            config_dir.join("current.toml")
        };

        let config = if config_path.exists() {
            let content =
                std::fs::read_to_string(&config_path).context("Failed to read config file")?;
            toml::from_str(&content).context("Failed to parse config file")?
        } else {
            ProjectConfig::default()
        };

        Ok(Self {
            config_path,
            config,
        })
    }

    /// Set the target programming language
    pub fn set_language(&mut self, language: &str) -> Result<()> {
        // Store language in technical settings
        self.config.technical.target_platforms = vec![language.to_string()];
        self.config
            .wizard_state
            .completed_steps
            .push("language".to_string());
        self.config.wizard_state.current_step = "language".to_string();
        self.save()
    }

    /// Set the wizard mode (guided or freeform)
    pub fn set_wizard_mode(&mut self, mode: &str) -> Result<()> {
        // Store mode in wizard state
        self.config.wizard_state.current_step = mode.to_string();
        self.config
            .wizard_state
            .completed_steps
            .push("mode_selection".to_string());
        self.save()
    }

    pub fn save(&mut self) -> Result<()> {
        // Sync top-level fields with nested data
        self.config.name = Some(self.config.basic_info.name.clone());
        self.config.description = Some(self.config.basic_info.description.clone());
        self.config.version = self.config.metadata.version.clone();

        // Create game specification summary
        self.config.game_specification = Some(GameSpecification {
            title: self.config.basic_info.name.clone(),
            genre: self.config.basic_info.genre.clone(),
            theme: self.config.basic_info.tagline.clone(),
            art_style: self.config.visual_style.color_mood.clone(),
            key_features: self.collect_key_features(),
        });

        self.config.metadata.last_modified = chrono::Utc::now();

        let content = toml::to_string_pretty(&self.config).context("Failed to serialize config")?;

        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        std::fs::write(&self.config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    pub fn export_for_generation(&self) -> String {
        toml::to_string_pretty(&self.config).unwrap_or_default()
    }

    pub fn config_loaded(&self) -> bool {
        self.config_path.exists()
    }

    fn collect_key_features(&self) -> Vec<String> {
        let mut features = Vec::new();

        if self.config.features.combat_system.is_some() {
            features.push("Combat System".to_string());
        }
        if self.config.features.inventory_system.is_some() {
            features.push("Inventory System".to_string());
        }
        if self.config.features.dialogue_system.is_some() {
            features.push("Dialogue System".to_string());
        }
        if self.config.features.save_system {
            features.push("Save System".to_string());
        }
        if self.config.features.day_night_cycle {
            features.push("Day/Night Cycle".to_string());
        }

        for custom in &self.config.features.custom_features {
            features.push(custom.name.clone());
        }

        features
    }
}

impl ProjectConfig {
    /// Load from a TOML file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).context("Failed to read project config")?;
        let mut config: Self =
            toml::from_str(&content).context("Failed to parse project config")?;

        // Ensure top-level fields are synced
        if config.name.is_none() && !config.basic_info.name.is_empty() {
            config.name = Some(config.basic_info.name.clone());
        }
        if config.description.is_none() && !config.basic_info.description.is_empty() {
            config.description = Some(config.basic_info.description.clone());
        }

        Ok(config)
    }
}
