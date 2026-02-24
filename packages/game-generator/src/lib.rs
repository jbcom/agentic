// lib.rs
pub mod ai_client;
pub mod blending;
pub mod build_tools;
pub mod combat;
pub mod metaprompts;
pub mod vintage_games;
pub mod wizard;

pub use metaprompts::{GameConfig, GameGenerator, GenerationPhase, GenerationProgress};

pub use wizard::{AppDirectories, AppMode, AppState, WizardPlugin};

// Re-export common types for Tauri frontend
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub genre: String,
    pub setting: String,
    pub art_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    pub include_voice_acting: bool,
    pub include_orchestral_music: bool,
    pub target_playtime_hours: u32,
    pub difficulty_options: Vec<String>,
    pub accessibility_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub phase: String,
    pub step: String,
    pub progress: f32,
    pub message: String,
    pub artifact: Option<GeneratedArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedArtifact {
    pub artifact_type: String,
    pub name: String,
    pub path: Option<String>,
    pub preview: Option<String>, // Base64 encoded preview for images
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Generation failed: {0}")]
    GenerationFailed(String),
}

pub type Result<T> = std::result::Result<T, GeneratorError>;
