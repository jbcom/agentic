//! Type definitions for conversation management

use async_openai::types::chat::Role;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// A single conversation thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub messages: VecDeque<ConversationMessage>,
    pub context: ConversationContext,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub total_tokens: usize,
}

/// Message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub tokens: usize,
}

/// Role in conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl From<MessageRole> for Role {
    fn from(role: MessageRole) -> Self {
        match role {
            MessageRole::System => Role::System,
            MessageRole::User => Role::User,
            MessageRole::Assistant => Role::Assistant,
        }
    }
}

/// Context for the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// Type of conversation (e.g., "game_design", "blend_exploration", "game_generation")
    pub conversation_type: String,
    /// Current game concept being discussed
    pub game_concept: Option<GameConceptContext>,
    /// Maximum messages to keep in context
    pub max_context_messages: usize,
    /// Custom system prompt
    pub system_prompt: Option<String>,
    /// Current generation phase (for game generation conversations)
    pub generation_phase: Option<GenerationPhase>,
    /// Project configuration from wizard
    pub project_config: Option<serde_json::Value>,
}

/// Game generation phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerationPhase {
    Design,
    StyleGuide,
    WorldGeneration,
    AiSystems,
    AssetGeneration,
    CodeGeneration,
    DialogWriting,
    MusicComposition,
    Integration,
    Testing,
    Packaging,
}

/// Generation progress tracking
#[derive(Debug, Clone, Serialize)]
pub struct GenerationProgress {
    pub phase: GenerationPhase,
    pub step: String,
    pub progress: f32,
    pub message: String,
}

/// Game concept context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConceptContext {
    pub title: String,
    pub genre: String,
    pub inspirations: Vec<String>,
    pub current_blend: Option<BlendContext>,
}

/// Blend context for guided mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendContext {
    pub selected_games: Vec<String>,
    pub blend_weights: HashMap<String, f32>,
    pub dominant_attributes: Vec<String>,
}

/// Summary of a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub total_tokens: usize,
}

/// Configuration for message sending
#[derive(Debug, Clone)]
pub struct MessageConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for MessageConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4-turbo".to_string(),
            temperature: 0.8,
            max_tokens: 2000,
        }
    }
}
