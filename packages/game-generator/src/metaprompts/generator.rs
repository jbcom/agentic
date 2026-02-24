use serde::{Deserialize, Serialize};

use crate::wizard::config::ProjectConfig;
use futures::{Stream, StreamExt};

// Import from vintage_ai_client - updated to new API
use vintage_ai_client::{
    AiService, conversation::ConversationContext, game_types::GameConfig, text::TextConfig,
};

/// Progress tracking for game generation
#[derive(Debug, Clone)]
pub struct GenerationProgress {
    pub phase: GenerationPhase,
    pub progress: f32, // 0.0 to 1.0
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationPhase {
    // Core phases
    Initializing,
    Design,
    DesigningCore,
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
    Finalizing,
    Complete,

    // Legacy aliases for compatibility
    GameDesign,
    SpriteGeneration,
    TilesetGeneration,
    GeneratingAssets,
    WritingDialogue,
    ComposingMusic,
}

/// Conversation message for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
}

/// Conversation state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationState {
    pub conversation_id: String,
    pub messages: Vec<ConversationMessage>,
}

impl ConversationState {
    pub fn new(conversation_id: String) -> Self {
        Self {
            conversation_id,
            messages: Vec::new(),
        }
    }
}

pub struct GameGenerator {
    ai_service: AiService,
    project_config: Option<ProjectConfig>,
}

impl GameGenerator {
    pub async fn new() -> anyhow::Result<Self> {
        let ai_service = AiService::from_env()?;

        Ok(Self {
            ai_service,
            project_config: None,
        })
    }

    /// Set the project configuration (the "Bible" from wizard)
    pub fn set_project_config(&mut self, config: ProjectConfig) {
        self.project_config = Some(config);
    }

    /// Start a game design conversation
    pub async fn start_game_design_conversation(
        &self,
        initial_prompt: &str,
    ) -> anyhow::Result<(String, String)> {
        // Build system prompt based on project config
        let system_prompt = self.build_game_design_system_prompt();

        // Create conversation context
        let context = ConversationContext {
            conversation_type: "game_design".to_string(),
            game_concept: None,
            max_context_messages: 20,
            system_prompt: Some(system_prompt),
            generation_phase: None,
            project_config: self
                .project_config
                .as_ref()
                .and_then(|c| serde_json::to_value(c).ok()),
        };

        // Start conversation using ConversationManager
        let conversation_manager = self.ai_service.conversation();
        let conversation_id = conversation_manager
            .start_conversation("Game Design".to_string(), context)
            .await?;

        // Send initial message
        let response = conversation_manager
            .send_message(&conversation_id, initial_prompt.to_string())
            .await?;

        Ok((conversation_id, response))
    }

    /// Continue game design conversation
    pub async fn continue_game_design_conversation(
        &self,
        conversation_id: &str,
        user_input: &str,
    ) -> anyhow::Result<(String, bool, Option<GameConfig>)> {
        // Get AI response using ConversationManager
        let conversation_manager = self.ai_service.conversation();
        let response = conversation_manager
            .send_message(conversation_id, user_input.to_string())
            .await?;

        // Check if the response contains a complete game config
        let (is_complete, game_config) = self.parse_game_config_from_response(&response);

        Ok((response, is_complete, game_config))
    }

    /// Continue game design conversation with streaming
    pub async fn continue_game_design_conversation_stream(
        &self,
        conversation_id: &str,
        user_input: &str,
    ) -> anyhow::Result<impl Stream<Item = anyhow::Result<String>> + use<'_>> {
        let conversation_manager = self.ai_service.conversation();
        let conversation_id = conversation_id.to_string();
        let user_input = user_input.to_string();

        Ok(async_stream::try_stream! {
            let stream = conversation_manager
                .send_message_stream(&conversation_id, user_input)
                .await?;
            futures::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                yield item?;
            }
        })
    }

    /// Generate full game with progress tracking
    pub async fn generate_full_game<F>(
        &self,
        config: &GameConfig,
        progress_callback: F,
    ) -> anyhow::Result<String>
    where
        F: Fn(GenerationProgress) + Send + 'static,
    {
        let text_generator = self.ai_service.text();
        let text_config = TextConfig::for_game_description();

        // Initialize
        progress_callback(GenerationProgress {
            phase: GenerationPhase::Initializing,
            progress: 0.0,
            message: "Starting game generation...".to_string(),
        });

        // Design core
        progress_callback(GenerationProgress {
            phase: GenerationPhase::DesigningCore,
            progress: 0.1,
            message: "Designing core game mechanics...".to_string(),
        });

        let core_prompt = format!(
            "Generate the core game design document for: {}. Include mechanics, story outline, and character descriptions.",
            config.name
        );
        let core_design = text_generator
            .generate(&core_prompt, text_config.clone())
            .await?;

        // Generate assets descriptions
        progress_callback(GenerationProgress {
            phase: GenerationPhase::GeneratingAssets,
            progress: 0.3,
            message: "Generating asset descriptions...".to_string(),
        });

        let assets_prompt = format!(
            "Based on this design: {}\n\nDescribe the visual assets needed: sprites, tilesets, UI elements.",
            core_design.chars().take(1000).collect::<String>()
        );
        let _assets_desc = text_generator
            .generate(&assets_prompt, text_config.clone())
            .await?;

        // Writing dialogue
        progress_callback(GenerationProgress {
            phase: GenerationPhase::WritingDialogue,
            progress: 0.5,
            message: "Writing character dialogue...".to_string(),
        });

        let dialogue_config = TextConfig::for_dialogue();
        let dialogue_prompt = format!(
            "Write sample dialogue for key characters in: {}",
            config.name
        );
        let _dialogue = text_generator
            .generate(&dialogue_prompt, dialogue_config)
            .await?;

        // Composing music descriptions
        progress_callback(GenerationProgress {
            phase: GenerationPhase::ComposingMusic,
            progress: 0.7,
            message: "Describing musical themes...".to_string(),
        });

        let music_prompt = format!(
            "Describe the musical themes and sound design for: {}",
            config.name
        );
        let _music = text_generator.generate(&music_prompt, text_config).await?;

        // Finalize
        progress_callback(GenerationProgress {
            phase: GenerationPhase::Finalizing,
            progress: 0.9,
            message: "Finalizing game package...".to_string(),
        });

        // Complete
        progress_callback(GenerationProgress {
            phase: GenerationPhase::Complete,
            progress: 1.0,
            message: "Game generation complete!".to_string(),
        });

        Ok(core_design)
    }

    /// Load a game template
    pub async fn load_template(&self, name: &str) -> anyhow::Result<GameConfig> {
        let templates_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("rpg-generator")
            .join("templates");

        let path = templates_dir.join(format!("{name}.toml"));
        let content = std::fs::read_to_string(path)?;

        Ok(toml::from_str(&content)?)
    }

    /// Get conversation history for a specific conversation
    pub async fn get_conversation_history(
        &self,
        conversation_id: &str,
    ) -> anyhow::Result<Vec<(String, String)>> {
        let conversation_manager = self.ai_service.conversation();
        let conversation = conversation_manager
            .get_conversation(conversation_id)
            .await?;

        let history: Vec<(String, String)> = conversation
            .messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    vintage_ai_client::conversation::MessageRole::User => "user",
                    vintage_ai_client::conversation::MessageRole::Assistant => "assistant",
                    vintage_ai_client::conversation::MessageRole::System => "system",
                };
                (role.to_string(), m.content.clone())
            })
            .collect();

        Ok(history)
    }

    // Helper methods

    fn build_game_design_system_prompt(&self) -> String {
        let base = "You are an expert vintage game designer specializing in 8-bit and 16-bit era RPGs. \
                    Help design games that capture the charm of classics like Final Fantasy, Dragon Quest, \
                    and Chrono Trigger. Focus on pixel art aesthetics, chiptune music, and engaging gameplay.";

        if let Some(config) = &self.project_config {
            let name = config
                .name
                .as_deref()
                .or(Some(&config.basic_info.name))
                .unwrap_or("Unnamed");
            let description = config
                .description
                .as_deref()
                .or(Some(&config.basic_info.description))
                .unwrap_or("");
            let genre = &config.basic_info.genre;
            let tagline = &config.basic_info.tagline;

            format!(
                "{base}\n\nProject context:\n- Name: {name}\n- Description: {description}\n- Genre: {genre}\n- Tagline: {tagline}"
            )
        } else {
            base.to_string()
        }
    }

    fn parse_game_config_from_response(&self, response: &str) -> (bool, Option<GameConfig>) {
        // Look for JSON game config in the response
        if response.contains("\"title\"") && response.contains("\"genre\"") {
            // Try to extract and parse JSON - use safe string slicing
            if let Some(start) = response.find('{')
                && let Some(end) = response.rfind('}')
            {
                // Use get() for safe UTF-8 string slicing to avoid panics
                if let Some(json_str) = response.get(start..=end)
                    && let Ok(config) = serde_json::from_str::<GameConfig>(json_str)
                {
                    return (true, Some(config));
                }
            }
        }
        (false, None)
    }
}
