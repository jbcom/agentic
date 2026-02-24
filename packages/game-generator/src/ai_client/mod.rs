//! AI service modules for game generation
//!
//! This module provides a unified interface for all AI-powered features including:
//! - Text generation (game descriptions, narratives, code)
//! - Image generation (sprites, tilesets, UI elements)
//! - Audio generation (music, sound effects)
//! - Real-time conversation and blend calculations
//! - Token counting and cost optimization
//! - Intelligent caching to reduce API calls

pub mod audio;
pub mod cache;
pub mod client;
pub mod consistency;
pub mod conversation;
pub mod embeddings;
pub mod game_types;
pub mod image;
pub mod text;
pub mod tokens;

use anyhow::Result;
use async_openai::{Client, config::OpenAIConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Global AI service manager
#[derive(Clone)]
pub struct AiService {
    /// OpenAI client for API calls
    pub client: Arc<Client<OpenAIConfig>>,
    /// Cache manager for all AI operations
    pub cache: Arc<Mutex<cache::AiCache>>,
    /// Token counter for cost tracking
    pub token_counter: Arc<Mutex<tokens::TokenCounter>>,
    /// Style consistency manager for visual coherence
    pub style_manager: Arc<Mutex<consistency::StyleManager>>,
}

impl AiService {
    /// Create a new AI service instance
    pub fn new() -> Result<Self> {
        let config = OpenAIConfig::new();
        let client = Arc::new(Client::with_config(config));

        Ok(Self {
            client: client.clone(),
            cache: Arc::new(Mutex::new(cache::AiCache::new()?)),
            token_counter: Arc::new(Mutex::new(tokens::TokenCounter::new())),
            style_manager: Arc::new(Mutex::new(consistency::StyleManager::new())),
        })
    }

    /// Initialize from environment variables
    pub fn from_env() -> Result<Self> {
        // This will use OPENAI_API_KEY from environment
        Self::new()
    }

    /// Get a reference to the text generation service
    pub fn text(&self) -> text::TextGenerator {
        text::TextGenerator::new(
            self.client.clone(),
            self.cache.clone(),
            self.token_counter.clone(),
        )
    }

    /// Get a reference to the image generation service
    pub fn image(&self) -> image::ImageGenerator {
        image::ImageGenerator::new(
            self.client.clone(),
            self.cache.clone(),
            self.token_counter.clone(),
            self.style_manager.clone(),
        )
    }

    /// Get a reference to the audio generation service
    pub fn audio(&self) -> audio::AudioGenerator {
        audio::AudioGenerator::new(
            self.client.clone(),
            self.cache.clone(),
            self.token_counter.clone(),
        )
    }

    /// Get a reference to the conversation service
    pub fn conversation(&self) -> conversation::ConversationManager {
        conversation::ConversationManager::new(self.client.clone(), self.token_counter.clone())
    }

    /// Get a reference to the embeddings service
    pub fn embeddings(&self) -> embeddings::EmbeddingsGenerator {
        embeddings::EmbeddingsGenerator::new(
            self.client.clone(),
            self.cache.clone(),
            self.token_counter.clone(),
        )
    }
}

/// Common trait for all AI generators
#[async_trait::async_trait]
pub trait AiGenerator: Send + Sync {
    /// Get estimated tokens for a request
    async fn estimate_tokens(&self, request: &str) -> Result<usize>;

    /// Get estimated cost for a request
    async fn estimate_cost(&self, request: &str) -> Result<f64>;

    /// Check if result is cached
    async fn is_cached(&self, key: &str) -> bool;

    /// Clear cache for specific key
    async fn clear_cache(&self, key: &str) -> Result<()>;
}

/// Configuration for AI services
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct AiConfig {
    // Model Selection
    /// Text generation model (e.g., gpt-4, gpt-3.5-turbo, claude-3-opus)
    pub text_model: String,
    /// Image generation model (e.g., dall-e-3, dall-e-2)
    pub image_model: String,
    /// Audio generation model (e.g., tts-1, tts-1-hd)
    pub audio_model: String,
    /// Embedding model (e.g., text-embedding-3-small, text-embedding-3-large)
    pub embedding_model: String,

    // Generation Parameters
    /// Temperature for generation (0.0 - 2.0)
    pub temperature: f32,
    /// Top-p for text generation (0.0-1.0)
    pub top_p: f32,
    /// Maximum tokens per request
    pub max_tokens: u32,
    /// Frequency penalty for text generation (-2.0 to 2.0)
    pub frequency_penalty: f32,
    /// Presence penalty for text generation (-2.0 to 2.0)
    pub presence_penalty: f32,

    // Image Parameters
    /// Image quality (standard or hd for DALL-E 3)
    pub image_quality: String,
    /// Image size (1024x1024, 1792x1024, 1024x1792 for DALL-E 3)
    pub image_size: String,

    // Provider Settings
    /// AI provider (openai, anthropic)
    pub ai_provider: String,

    // Cache and Performance
    /// Enable AI response caching
    pub cache_enabled: bool,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// AI request timeout in seconds
    pub timeout_secs: u64,
    /// Enable cost optimization features
    pub optimize_costs: bool,
    /// Maximum concurrent requests
    pub max_concurrent: usize,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            // Model defaults
            text_model: "gpt-4".to_string(),
            image_model: "dall-e-3".to_string(),
            audio_model: "tts-1".to_string(),
            embedding_model: "text-embedding-3-small".to_string(),

            // Generation defaults
            temperature: 0.8,
            top_p: 0.95,
            max_tokens: 2000,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,

            // Image defaults
            image_quality: "standard".to_string(),
            image_size: "1024x1024".to_string(),

            // Provider defaults
            ai_provider: "openai".to_string(),

            // Cache and performance defaults
            cache_enabled: true,
            cache_ttl: 3600 * 24 * 7, // 1 week
            timeout_secs: 120,
            optimize_costs: true,
            max_concurrent: 5,
        }
    }
}

impl AiConfig {
    /// Create a new config with custom values
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder pattern for model configuration
    pub fn with_text_model(mut self, model: impl Into<String>) -> Self {
        self.text_model = model.into();
        self
    }

    pub fn with_image_model(mut self, model: impl Into<String>) -> Self {
        self.image_model = model.into();
        self
    }

    pub fn with_audio_model(mut self, model: impl Into<String>) -> Self {
        self.audio_model = model.into();
        self
    }

    /// Builder pattern for generation parameters
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp.clamp(0.0, 2.0);
        self
    }

    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = tokens.min(128000); // Max for GPT-4
        self
    }

    /// Validate and clamp configuration values
    pub fn validate(mut self) -> Self {
        self.temperature = self.temperature.clamp(0.0, 2.0);
        self.top_p = self.top_p.clamp(0.0, 1.0);
        self.frequency_penalty = self.frequency_penalty.clamp(-2.0, 2.0);
        self.presence_penalty = self.presence_penalty.clamp(-2.0, 2.0);
        self.max_tokens = self.max_tokens.min(128000);
        self
    }
}
