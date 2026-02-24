//! Unified AI client - the public API for all AI services
//!
//! This is the main entry point for any part of the application that needs AI functionality.
//! It manages all AI service instances and provides a clean, consistent interface.

use anyhow::Result;
use futures::Stream;
use futures::StreamExt;
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{
    AiConfig, AiGenerator, AiService,
    audio::{AudioConfig, AudioGenerator},
    conversation::{ConversationContext, ConversationManager},
    image::{ImageConfig, ImageGenerator},
    text::{TextConfig, TextGenerator},
};

/// The unified AI client - your one-stop shop for all AI services
#[derive(Clone)]
pub struct AiClient {
    /// Core AI service manager
    service: Arc<AiService>,
    /// Configuration
    config: Arc<RwLock<AiConfig>>,
    /// Request history for debugging/monitoring
    history: Arc<RwLock<Vec<AiRequest>>>,
    /// Template environment for prompts
    templates: Arc<Environment<'static>>,
}

/// Record of an AI request for monitoring/debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_type: AiRequestType,
    pub tokens_used: usize,
    pub cost_estimate: f64,
    pub cache_hit: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiRequestType {
    Text { purpose: String },
    Image { purpose: String },
    Audio { purpose: String },
    Conversation { context: String },
}

/// High-level request types that automatically route to appropriate services
#[derive(Debug, Clone)]
pub enum AiTask {
    /// Generate a game description from a blend result
    GenerateGameDescription {
        blend_name: String,
        genres: Vec<String>,
        mechanics: Vec<String>,
        themes: Vec<String>,
    },
    /// Generate concept art for a game
    GenerateConceptArt {
        game_name: String,
        art_style: String,
        subjects: Vec<String>,
    },
    /// Generate a theme song or sound effect
    GenerateAudio {
        game_name: String,
        audio_type: AudioType,
        mood: String,
    },
    /// Have a conversation about game design
    DiscussGameDesign { context: String, question: String },
    /// Generate code for a game component
    GenerateCode {
        language: String,
        component_type: String,
        specifications: String,
    },
    /// Custom text generation
    CustomText {
        prompt: String,
        config: Option<TextConfig>,
    },
    /// Custom image generation
    CustomImage {
        prompt: String,
        config: Option<ImageConfig>,
    },
}

#[derive(Debug, Clone)]
pub enum AudioType {
    ThemeSong,
    BattleMusic,
    VictoryFanfare,
    SoundEffect(String),
}

/// Result from an AI task
#[derive(Debug, Clone)]
pub enum AiResult {
    Text(String),
    Image(Vec<u8>),
    Audio(Vec<u8>),
    Conversation {
        response: String,
        context_updated: bool,
    },
}

impl AiClient {
    /// Create a new AI client with default configuration
    pub fn new() -> Result<Self> {
        let service = Arc::new(AiService::from_env()?);
        let config = Arc::new(RwLock::new(AiConfig::default()));
        let history = Arc::new(RwLock::new(Vec::new()));
        let templates = Arc::new(Self::create_template_env()?);

        Ok(Self {
            service,
            config,
            history,
            templates,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: AiConfig) -> Result<Self> {
        let service = Arc::new(AiService::from_env()?);
        let config = Arc::new(RwLock::new(config));
        let history = Arc::new(RwLock::new(Vec::new()));
        let templates = Arc::new(Self::create_template_env()?);

        Ok(Self {
            service,
            config,
            history,
            templates,
        })
    }

    /// Create the template environment
    fn create_template_env() -> Result<Environment<'static>> {
        let mut env = Environment::new();

        // Get the crate root directory
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let prompts_dir = Path::new(&manifest_dir).join("prompts");

        // Load all templates
        env.set_loader(minijinja::path_loader(prompts_dir));

        Ok(env)
    }

    /// Execute a high-level AI task
    pub async fn execute(&self, task: AiTask) -> Result<AiResult> {
        let start = std::time::Instant::now();

        let (result, request_type, tokens, cost, cache_hit) = match task {
            AiTask::GenerateGameDescription {
                blend_name,
                genres,
                mechanics,
                themes,
            } => {
                let prompt =
                    self.build_game_description_prompt(&blend_name, &genres, &mechanics, &themes);
                let config = TextConfig::for_game_description();
                let text_gen = self.service.text();

                let cache_key = format!("game_desc_{blend_name}");
                let cache_hit = text_gen.is_cached(&cache_key).await;

                let result = text_gen.generate(&prompt, config).await?;
                let tokens = text_gen.estimate_tokens(&prompt).await?;
                let cost = text_gen.estimate_cost(&prompt).await?;

                (
                    AiResult::Text(result),
                    AiRequestType::Text {
                        purpose: "game_description".to_string(),
                    },
                    tokens,
                    cost,
                    cache_hit,
                )
            }

            AiTask::GenerateConceptArt {
                game_name,
                art_style,
                subjects,
            } => {
                let prompt = self.build_concept_art_prompt(&game_name, &art_style, &subjects);
                let config = ImageConfig::for_sprites(); // Use sprites config for concept art
                let image_gen = self.service.image();

                let cache_key = format!("concept_art_{}_{}", game_name, subjects.join("_"));
                let cache_hit = image_gen.is_cached(&cache_key).await;

                // Generate image using ImageGenerator's specific method with config
                let result = image_gen.generate_single(&prompt, config).await?;
                let tokens = image_gen.estimate_tokens(&prompt).await?;
                let cost = image_gen.estimate_cost(&prompt).await?;

                (
                    AiResult::Image(result),
                    AiRequestType::Image {
                        purpose: "concept_art".to_string(),
                    },
                    tokens,
                    cost,
                    cache_hit,
                )
            }

            AiTask::GenerateAudio {
                game_name,
                audio_type,
                mood,
            } => {
                let prompt = self.build_audio_prompt(&game_name, &audio_type, &mood);
                let config = match audio_type {
                    AudioType::ThemeSong => AudioConfig::for_exploration(), // Use exploration for theme
                    AudioType::BattleMusic => AudioConfig::for_battle(),
                    AudioType::VictoryFanfare => AudioConfig::for_ui(), // Use UI config for fanfare
                    AudioType::SoundEffect(_) => AudioConfig::for_ui(), // Use UI config for sound effects
                };
                let audio_gen = self.service.audio();

                let cache_key = format!("audio_{}_{}_{:?}_{}", game_name, "", audio_type, mood);
                let cache_hit = audio_gen.is_cached(&cache_key).await;

                // Generate audio description based on type
                let result = match &audio_type {
                    AudioType::ThemeSong | AudioType::BattleMusic | AudioType::VictoryFanfare => {
                        let music_desc = audio_gen
                            .generate_music_description(&prompt, config)
                            .await?;
                        // Convert to bytes (placeholder - would need actual audio generation)
                        serde_json::to_vec(&music_desc)?
                    }
                    AudioType::SoundEffect(effect) => {
                        let sound_desc = audio_gen.generate_sound_effect(effect, 0.5).await?;
                        // Convert to bytes (placeholder - would need actual audio generation)
                        serde_json::to_vec(&sound_desc)?
                    }
                };
                let tokens = audio_gen.estimate_tokens(&prompt).await?;
                let cost = audio_gen.estimate_cost(&prompt).await?;

                (
                    AiResult::Audio(result),
                    AiRequestType::Audio {
                        purpose: format!("{audio_type:?}"),
                    },
                    tokens,
                    cost,
                    cache_hit,
                )
            }

            AiTask::DiscussGameDesign { context, question } => {
                let conv_manager = self.service.conversation();
                let conv_context = ConversationContext {
                    conversation_type: "game_design".to_string(),
                    game_concept: None,
                    max_context_messages: 20,
                    system_prompt: Some(context.clone()),
                    generation_phase: None,
                    project_config: None,
                };

                // Start a conversation and send the message
                let conv_id = conv_manager
                    .start_conversation("Game Design Discussion".to_string(), conv_context)
                    .await?;
                let response = conv_manager
                    .send_message(&conv_id, question.clone())
                    .await?;

                let tokens = conv_manager.estimate_tokens(&question).await?;
                let cost = conv_manager.estimate_cost(&question).await?;

                (
                    AiResult::Conversation {
                        response,
                        context_updated: true,
                    },
                    AiRequestType::Conversation {
                        context: "game_design".to_string(),
                    },
                    tokens,
                    cost,
                    false, // Conversations typically aren't cached
                )
            }

            AiTask::GenerateCode {
                language,
                component_type,
                specifications,
            } => {
                let prompt = self.build_code_prompt(&language, &component_type, &specifications);
                let config = TextConfig::for_code_generation();
                let text_gen = self.service.text();

                let cache_key = format!("code_{language}_{component_type}");
                let cache_hit = text_gen.is_cached(&cache_key).await;

                let result = text_gen.generate(&prompt, config).await?;
                let tokens = text_gen.estimate_tokens(&prompt).await?;
                let cost = text_gen.estimate_cost(&prompt).await?;

                (
                    AiResult::Text(result),
                    AiRequestType::Text {
                        purpose: "code_generation".to_string(),
                    },
                    tokens,
                    cost,
                    cache_hit,
                )
            }

            AiTask::CustomText { prompt, config } => {
                let config = config.unwrap_or_default();
                let text_gen = self.service.text();

                let cache_key = format!("custom_text_{}", &prompt[..prompt.len().min(50)]);
                let cache_hit = text_gen.is_cached(&cache_key).await;

                let result = text_gen.generate(&prompt, config).await?;
                let tokens = text_gen.estimate_tokens(&prompt).await?;
                let cost = text_gen.estimate_cost(&prompt).await?;

                (
                    AiResult::Text(result),
                    AiRequestType::Text {
                        purpose: "custom".to_string(),
                    },
                    tokens,
                    cost,
                    cache_hit,
                )
            }

            AiTask::CustomImage { prompt, config } => {
                let config = config.unwrap_or_default();
                let image_gen = self.service.image();

                let cache_key = format!("custom_image_{}", &prompt[..prompt.len().min(50)]);
                let cache_hit = image_gen.is_cached(&cache_key).await;

                // Generate image using the provided config
                let result = image_gen.generate_single(&prompt, config).await?;
                let tokens = image_gen.estimate_tokens(&prompt).await?;
                let cost = image_gen.estimate_cost(&prompt).await?;

                (
                    AiResult::Image(result),
                    AiRequestType::Image {
                        purpose: "custom".to_string(),
                    },
                    tokens,
                    cost,
                    cache_hit,
                )
            }
        };

        // Record the request
        let duration_ms = start.elapsed().as_millis() as u64;
        let request_record = AiRequest {
            timestamp: chrono::Utc::now(),
            request_type,
            tokens_used: tokens,
            cost_estimate: cost,
            cache_hit,
            duration_ms,
        };

        self.history.write().await.push(request_record);

        Ok(result)
    }

    /// Get direct access to text generator for advanced use
    pub fn text(&self) -> TextGenerator {
        self.service.text()
    }

    /// Get direct access to image generator for advanced use
    pub fn image(&self) -> ImageGenerator {
        self.service.image()
    }

    /// Get direct access to audio generator for advanced use
    pub fn audio(&self) -> AudioGenerator {
        self.service.audio()
    }

    /// Get direct access to conversation manager for advanced use
    pub fn conversation(&self) -> ConversationManager {
        self.service.conversation()
    }

    /// Start a streaming chat response
    pub async fn chat_stream(
        &self,
        conversation_id: &str,
        message: String,
    ) -> Result<impl Stream<Item = Result<String>> + use<'_>> {
        let manager = self.service.conversation();
        let conversation_id = conversation_id.to_string();
        Ok(async_stream::try_stream! {
            let stream = manager.send_message_stream(&conversation_id, message).await?;
            futures::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                yield item?;
            }
        })
    }

    /// Update configuration
    pub async fn update_config(&self, config: AiConfig) {
        *self.config.write().await = config;
    }

    /// Get current configuration
    pub async fn get_config(&self) -> AiConfig {
        self.config.read().await.clone()
    }

    /// Get request history
    pub async fn get_history(&self) -> Vec<AiRequest> {
        self.history.read().await.clone()
    }

    /// Clear request history
    pub async fn clear_history(&self) {
        self.history.write().await.clear();
    }

    /// Get usage statistics
    pub async fn get_usage_stats(&self) -> UsageStats {
        let history = self.history.read().await;

        let total_requests = history.len();
        let total_tokens: usize = history.iter().map(|r| r.tokens_used).sum();
        let total_cost: f64 = history.iter().map(|r| r.cost_estimate).sum();
        let cache_hits = history.iter().filter(|r| r.cache_hit).count();
        let avg_duration_ms = if total_requests > 0 {
            history.iter().map(|r| r.duration_ms).sum::<u64>() / total_requests as u64
        } else {
            0
        };

        UsageStats {
            total_requests,
            total_tokens,
            total_cost,
            cache_hit_rate: if total_requests > 0 {
                cache_hits as f64 / total_requests as f64
            } else {
                0.0
            },
            avg_duration_ms,
        }
    }

    // Helper methods for building prompts

    fn build_game_description_prompt(
        &self,
        blend_name: &str,
        genres: &[String],
        mechanics: &[String],
        themes: &[String],
    ) -> String {
        let tmpl = self
            .templates
            .get_template("text/game_description.jinja")
            .expect("game_description.jinja template not found");

        tmpl.render(context! {
            blend_name => blend_name,
            genres_list => genres.join(", "),
            mechanics_list => mechanics.join(", "),
            themes_list => themes.join(", "),
        })
        .expect("Failed to render game_description template")
    }

    fn build_concept_art_prompt(
        &self,
        game_name: &str,
        art_style: &str,
        subjects: &[String],
    ) -> String {
        let tmpl = self
            .templates
            .get_template("text/concept_art.jinja")
            .expect("concept_art.jinja template not found");

        tmpl.render(context! {
            game_name => game_name,
            art_style => art_style,
            subjects => subjects.join(", "),
        })
        .expect("Failed to render concept_art template")
    }

    fn build_audio_prompt(&self, game_name: &str, audio_type: &AudioType, mood: &str) -> String {
        let template_name = match audio_type {
            AudioType::ThemeSong => "audio/theme_song.jinja",
            AudioType::BattleMusic => "audio/battle_music.jinja",
            AudioType::VictoryFanfare => "audio/victory_fanfare.jinja",
            AudioType::SoundEffect(_) => "audio/sound_effect.jinja",
        };

        let tmpl = self
            .templates
            .get_template(template_name)
            .unwrap_or_else(|_| panic!("{template_name} template not found"));

        let ctx = if let AudioType::SoundEffect(effect) = audio_type {
            context! {
                game_name => game_name,
                mood => mood,
                effect => effect,
            }
        } else {
            context! {
                game_name => game_name,
                mood => mood,
            }
        };

        tmpl.render(ctx)
            .unwrap_or_else(|_| panic!("Failed to render {template_name} template"))
    }

    fn build_code_prompt(
        &self,
        language: &str,
        component_type: &str,
        specifications: &str,
    ) -> String {
        let tmpl = self
            .templates
            .get_template("text/code_generation.jinja")
            .expect("code_generation.jinja template not found");

        tmpl.render(context! {
            language => language,
            component_type => component_type,
            specifications => specifications,
        })
        .expect("Failed to render code_generation template")
    }
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_requests: usize,
    pub total_tokens: usize,
    pub total_cost: f64,
    pub cache_hit_rate: f64,
    pub avg_duration_ms: u64,
}

/// Data structure to represent a blend result for AI integration
#[derive(Debug, Clone)]
pub struct BlendData {
    pub name: String,
    pub genres: Vec<String>,
    pub mechanics: Vec<String>,
    pub themes: Vec<String>,
    pub recommended_features: Vec<String>,
}

/// Extension trait for blend integration
#[allow(async_fn_in_trait)]
pub trait BlendAiIntegration {
    /// Generate AI content from a blend result
    async fn generate_from_blend(&self, blend: &BlendData) -> Result<GeneratedContent>;
}

/// Generated content from a blend
#[derive(Debug, Clone)]
pub struct GeneratedContent {
    pub description: String,
    pub suggested_features: Vec<String>,
    pub code_snippets: HashMap<String, String>,
    pub marketing_tagline: String,
}

impl BlendAiIntegration for AiClient {
    async fn generate_from_blend(&self, blend: &BlendData) -> Result<GeneratedContent> {
        // Generate game description
        let description = match self
            .execute(AiTask::GenerateGameDescription {
                blend_name: blend.name.clone(),
                genres: blend.genres.clone(),
                mechanics: blend.mechanics.clone(),
                themes: blend.themes.clone(),
            })
            .await?
        {
            AiResult::Text(text) => text,
            _ => return Err(anyhow::anyhow!("Expected text result")),
        };

        // Generate marketing tagline
        let tagline_prompt = format!(
            "Create a short, punchy marketing tagline for '{}'. \
            Maximum 10 words. Capture the essence of: {}",
            blend.name,
            blend.genres.join(" meets ")
        );

        let marketing_tagline = match self
            .execute(AiTask::CustomText {
                prompt: tagline_prompt,
                config: Some(TextConfig {
                    max_tokens: 50,
                    temperature: 0.9,
                    ..Default::default()
                }),
            })
            .await?
        {
            AiResult::Text(text) => text.trim().to_string(),
            _ => return Err(anyhow::anyhow!("Expected text result")),
        };

        Ok(GeneratedContent {
            description,
            suggested_features: blend.recommended_features.clone(),
            code_snippets: HashMap::new(), // Could generate actual code snippets
            marketing_tagline,
        })
    }
}
