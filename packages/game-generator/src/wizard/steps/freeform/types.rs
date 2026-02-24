//! Types and data structures for freeform mode

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The current step in the freeform wizard process
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FreeformStep {
    #[default]
    Introduction, // Welcome and explain the process
    BasicInfo,         // Game name, genre, tagline
    GameplayDesign,    // Core mechanics, progression
    VisualStyle,       // Art style, references
    Features,          // Combat, inventory, dialogue
    TechnicalSettings, // Performance, platforms
    Review,            // Review before conversation
    Conversation,      // AI conversation phase
}

/// Main state for freeform mode
#[derive(Resource, Default)]
pub struct FreeformModeState {
    pub current_step: FreeformStep,
    pub game_config: FreeformGameConfig,
    pub conversation: ConversationState,
    pub export: Option<FreeformExport>,
}

/// Game configuration being built through the wizard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreeformGameConfig {
    // Basic Information
    pub game_name: String,
    pub tagline: String,
    pub genre: GameGenre,
    pub target_audience: TargetAudience,
    pub description: String,
    pub inspiration_notes: String,

    // Gameplay Design
    pub core_mechanics: Vec<CoreMechanic>,
    pub progression_type: ProgressionType,
    pub gameplay_loop: String,
    pub player_motivation: String,
    pub difficulty_settings: DifficultySettings,

    // Visual Style
    pub reference_games: Vec<String>,
    pub color_mood: ColorMood,
    pub sprite_size: u32,
    pub outline_enabled: bool,
    pub outline_style: OutlineStyle,
    pub shading_technique: ShadingTechnique,
    pub animation_complexity: AnimationComplexity,
    pub ui_theme: String,

    // Features
    pub combat_system: Option<CombatSettings>,
    pub inventory_system: Option<InventorySettings>,
    pub dialogue_system: Option<DialogueSettings>,
    pub additional_features: AdditionalFeatures,

    // Technical Settings
    pub world_size: WorldSize,
    pub performance_target: PerformanceTarget,
    pub target_platforms: Vec<Platform>,
    pub multiplayer_settings: Option<MultiplayerSettings>,
}

/// Conversation state for AI interaction
#[derive(Default)]
pub struct ConversationState {
    pub conversation_id: Option<String>,
    pub history: Vec<ConversationEntry>,
    pub current_input: String,
    pub is_processing: bool,
    pub is_streaming: bool,
    pub error_message: Option<String>,
    pub context_summary: String,
}

#[derive(Clone)]
pub struct ConversationEntry {
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: std::time::SystemTime,
    pub metadata: Option<ConversationMetadata>,
}

#[derive(Clone, PartialEq)]
pub enum ConversationRole {
    User,
    Assistant,
    System,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConversationMetadata {
    pub topic: String,
    pub decisions_made: Vec<String>,
    pub alternatives_considered: Vec<String>,
}

/// Events for streaming conversation
pub enum ConversationStreamEvent {
    Token(String),
    Finished,
    Error(String),
}

/// Resource to hold the streaming channel
#[derive(Resource, Default)]
pub struct ConversationStream {
    pub receiver: Option<tokio::sync::mpsc::UnboundedReceiver<ConversationStreamEvent>>,
}

/// Export configuration for generation
#[derive(Clone, Serialize, Deserialize)]
pub struct FreeformExport {
    pub config_path: String,
    pub conversation_path: String,
    pub timestamp: std::time::SystemTime,
}

// Enums for configuration options

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum GameGenre {
    #[default]
    ActionRPG,
    TurnBasedRPG,
    PuzzleRPG,
    PlatformRPG,
    RoguelikeRPG,
    Custom(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum TargetAudience {
    Casual,
    #[default]
    Core,
    Hardcore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreMechanic {
    Combat,
    Exploration,
    PuzzleSolving,
    Platforming,
    ResourceManagement,
    Crafting,
    Stealth,
    DialogChoices,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ProgressionType {
    #[default]
    Linear,
    OpenWorld,
    Metroidvania,
    HubBased,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DifficultySettings {
    pub starting_difficulty: f32, // 0.0 - 1.0
    pub ramp_speed: f32,
    pub max_difficulty: f32,
    pub adaptive_difficulty: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ColorMood {
    #[default]
    Vibrant,
    Pastel,
    Dark,
    Earthy,
    Neon,
    Monochrome,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum OutlineStyle {
    #[default]
    Black,
    Colored,
    None,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ShadingTechnique {
    #[default]
    Simple,
    Dithered,
    Gradient,
    CelShaded,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AnimationComplexity {
    Minimal,
    #[default]
    Standard,
    Detailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSettings {
    pub combat_type: CombatType,
    pub damage_numbers: bool,
    pub combo_system: bool,
    pub special_abilities_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatType {
    RealTime,
    TurnBased,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySettings {
    pub slot_count: u32,
    pub stack_size: u32,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSettings {
    pub dialogue_type: DialogueType,
    pub portrait_style: PortraitStyle,
    pub text_speed: TextSpeed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueType {
    Linear,
    Branching,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortraitStyle {
    None,
    PixelArt,
    Illustrated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextSpeed {
    Instant,
    Fast,
    Normal,
    Slow,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdditionalFeatures {
    pub save_system: bool,
    pub day_night_cycle: bool,
    pub weather_effects: bool,
    pub minimap: bool,
    pub achievements: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum WorldSize {
    Small,
    #[default]
    Medium,
    Large,
    Massive,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum PerformanceTarget {
    LowEnd,
    #[default]
    Standard,
    HighEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    Windows,
    Mac,
    Linux,
    Web,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerSettings {
    pub max_players: u32,
    pub multiplayer_type: MultiplayerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MultiplayerType {
    LocalCoop,
    OnlineCoop,
    Competitive,
}
