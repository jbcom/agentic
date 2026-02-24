//! Text generation module for game content
//!
//! Handles generation of:
//! - Game descriptions and narratives
//! - Character dialogues and backstories
//! - Quest text and world lore
//! - Code generation for game mechanics
//! - Documentation and tutorials

use anyhow::{Context, Result};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    AiGenerator,
    cache::{AiCache, CachedData},
    tokens::TokenCounter,
};

/// Text generator for all text-based content
#[derive(Clone)]
pub struct TextGenerator {
    client: Arc<Client<OpenAIConfig>>,
    cache: Arc<Mutex<AiCache>>,
    token_counter: Arc<Mutex<TokenCounter>>,
}

/// Configuration for text generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextConfig {
    /// Model to use (e.g., "gpt-4", "gpt-3.5-turbo")
    pub model: String,
    /// Temperature for creativity (0.0-2.0)
    pub temperature: f32,
    /// Maximum tokens to generate
    pub max_tokens: u16,
    /// Top-p nucleus sampling
    pub top_p: f32,
    /// Frequency penalty
    pub frequency_penalty: f32,
    /// Presence penalty
    pub presence_penalty: f32,
    /// System prompt for context
    pub system_prompt: Option<String>,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            temperature: 0.7,
            max_tokens: 2000,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            system_prompt: None,
        }
    }
}

/// Specialized configurations for different text types
impl TextConfig {
    /// Configuration for game descriptions
    pub fn for_game_description() -> Self {
        Self {
            model: "gpt-4-turbo".to_string(),
            temperature: 0.8,
            max_tokens: 1500,
            system_prompt: Some(
                "You are a creative game designer specializing in nostalgic 16-bit era games. \
                Create engaging, concise game descriptions that capture the essence of classic \
                gaming while feeling fresh and exciting."
                    .to_string(),
            ),
            ..Default::default()
        }
    }

    /// Configuration for character dialogue
    pub fn for_dialogue() -> Self {
        Self {
            temperature: 0.9,
            max_tokens: 500,
            frequency_penalty: 0.5,
            presence_penalty: 0.5,
            system_prompt: Some(
                "You are a dialogue writer for classic RPGs. Create memorable, \
                personality-driven dialogue that fits within the constraints of \
                16-bit era text boxes. Keep responses concise but impactful."
                    .to_string(),
            ),
            ..Default::default()
        }
    }

    /// Configuration for code generation
    pub fn for_code_generation() -> Self {
        Self {
            model: "gpt-4-turbo".to_string(),
            temperature: 0.3,
            max_tokens: 3000,
            system_prompt: Some(
                "You are an expert game programmer specializing in retro game mechanics. \
                Generate clean, efficient code that captures the feel of classic games \
                while using modern best practices. Focus on clarity and performance."
                    .to_string(),
            ),
            ..Default::default()
        }
    }

    /// Configuration for world lore
    pub fn for_world_building() -> Self {
        Self {
            model: "gpt-4-turbo".to_string(),
            temperature: 0.85,
            max_tokens: 2000,
            system_prompt: Some(
                "You are a world-building expert for nostalgic games. Create rich, \
                interconnected lore that feels both familiar and surprising. Draw \
                inspiration from classic game worlds while adding unique twists."
                    .to_string(),
            ),
            ..Default::default()
        }
    }
}

impl TextGenerator {
    /// Create a new text generator
    pub fn new(
        client: Arc<Client<OpenAIConfig>>,
        cache: Arc<Mutex<AiCache>>,
        token_counter: Arc<Mutex<TokenCounter>>,
    ) -> Self {
        Self {
            client,
            cache,
            token_counter,
        }
    }

    /// Generate text with caching and token tracking
    pub async fn generate(&self, prompt: &str, config: TextConfig) -> Result<String> {
        // Generate cache key
        let mut params = HashMap::new();
        params.insert("model".to_string(), config.model.clone());
        params.insert("temperature".to_string(), config.temperature.to_string());
        params.insert("max_tokens".to_string(), config.max_tokens.to_string());

        let cache_key = self
            .cache
            .lock()
            .await
            .generate_key("text", prompt, &params);

        // Check cache first
        if let Some(cached) = self.cache.lock().await.get(&cache_key).await
            && let CachedData::Text(text) = cached.data
        {
            return Ok(text);
        }

        // Build messages
        let mut messages = Vec::new();

        if let Some(system) = &config.system_prompt {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system.as_str())
                    .build()?
                    .into(),
            );
        }

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );

        // Create request
        let request = CreateChatCompletionRequestArgs::default()
            .model(&config.model)
            .messages(messages)
            .temperature(config.temperature)
            .max_tokens(config.max_tokens)
            .top_p(config.top_p)
            .frequency_penalty(config.frequency_penalty)
            .presence_penalty(config.presence_penalty)
            .build()?;

        // Make API call
        let response = self
            .client
            .chat()
            .create(request)
            .await
            .context("Failed to generate text")?;

        // Extract text
        let text = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .unwrap_or_default();

        // Track tokens
        if let Some(usage) = response.usage {
            self.token_counter
                .lock()
                .await
                .record_usage(
                    &config.model,
                    usage.prompt_tokens as usize,
                    usage.completion_tokens as usize,
                )
                .await?;
        }

        // Cache result
        let cache_params: HashMap<String, serde_json::Value> = params
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::String(v)))
            .collect();

        self.cache
            .lock()
            .await
            .put(cache_key, CachedData::Text(text.clone()), cache_params)
            .await?;

        Ok(text)
    }

    /// Generate multiple related texts (e.g., character descriptions)
    pub async fn generate_batch(
        &self,
        prompts: Vec<String>,
        config: TextConfig,
    ) -> Result<Vec<String>> {
        let mut results = Vec::new();

        // Process in parallel with rate limiting
        let semaphore = Arc::new(tokio::sync::Semaphore::new(3));
        let mut tasks = Vec::new();

        for prompt in prompts {
            let generator = self.clone();
            let config = config.clone();
            let permit = semaphore.clone().acquire_owned().await?;

            let task = tokio::spawn(async move {
                let _permit = permit;
                generator.generate(&prompt, config).await
            });

            tasks.push(task);
        }

        // Collect results
        for task in tasks {
            results.push(task.await??);
        }

        Ok(results)
    }

    /// Generate structured content (e.g., JSON game data)
    pub async fn generate_structured<T: for<'de> Deserialize<'de>>(
        &self,
        prompt: &str,
        config: TextConfig,
    ) -> Result<T> {
        let structured_prompt = format!(
            "{prompt}\n\nIMPORTANT: Respond ONLY with valid JSON, no additional text or formatting."
        );

        let mut config = config;
        config.temperature = config.temperature.min(0.5); // Lower temperature for structured output

        let json_text = self.generate(&structured_prompt, config).await?;

        // Try to parse JSON, cleaning if necessary
        let cleaned = json_text
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        serde_json::from_str(cleaned).context("Failed to parse structured output")
    }

    /// Generate with specific style consistency
    pub async fn generate_with_style(
        &self,
        prompt: &str,
        style_examples: Vec<String>,
        config: TextConfig,
    ) -> Result<String> {
        let style_prompt = if !style_examples.is_empty() {
            format!(
                "Match the style and tone of these examples:\n\n{}\n\nNow generate:\n{}",
                style_examples.join("\n\n"),
                prompt
            )
        } else {
            prompt.to_string()
        };

        self.generate(&style_prompt, config).await
    }
}

#[async_trait::async_trait]
impl AiGenerator for TextGenerator {
    async fn estimate_tokens(&self, request: &str) -> Result<usize> {
        let counter = self.token_counter.lock().await;
        counter.count_tokens(request, "gpt-3.5-turbo")
    }

    async fn estimate_cost(&self, request: &str) -> Result<f64> {
        let counter = self.token_counter.lock().await;
        counter.estimate_cost(request, "gpt-3.5-turbo", 1000)
    }

    async fn is_cached(&self, key: &str) -> bool {
        self.cache.lock().await.get(key).await.is_some()
    }

    async fn clear_cache(&self, key: &str) -> Result<()> {
        self.cache.lock().await.clear(key).await
    }
}

/// Specialized generators for game content
pub mod game_content {
    use super::*;

    /// Generate a complete game concept
    pub async fn generate_game_concept(
        generator: &TextGenerator,
        theme: &str,
        inspirations: Vec<String>,
    ) -> Result<GameConcept> {
        let prompt = format!(
            "Create a unique game concept with theme '{}' inspired by: {}. \
            Include title, genre, core mechanics, visual style, and target audience.",
            theme,
            inspirations.join(", ")
        );

        generator
            .generate_structured(&prompt, TextConfig::for_game_description())
            .await
    }

    /// Generate character backstory
    pub async fn generate_character_backstory(
        generator: &TextGenerator,
        character_name: &str,
        character_class: &str,
        world_context: &str,
    ) -> Result<String> {
        let prompt = format!(
            "Create a compelling backstory for {character_name}, a {character_class} in a world where {world_context}. \
            Keep it concise but memorable, suitable for a 16-bit RPG."
        );

        generator
            .generate(&prompt, TextConfig::for_world_building())
            .await
    }

    /// Generate quest text
    pub async fn generate_quest(
        generator: &TextGenerator,
        quest_type: &str,
        difficulty: &str,
        world_context: &str,
    ) -> Result<Quest> {
        let prompt = format!(
            "Design a {difficulty} {quest_type} quest for a 16-bit style game set in {world_context}. \
            Include name, description, objectives, rewards, and dialogue snippets."
        );

        generator
            .generate_structured(&prompt, TextConfig::for_game_description())
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConcept {
    pub title: String,
    pub genre: String,
    pub core_mechanics: Vec<String>,
    pub visual_style: String,
    pub target_audience: String,
    pub unique_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub rewards: Vec<String>,
    pub dialogue: HashMap<String, String>,
    pub prerequisites: Vec<String>,
}

/// Template-based text generation for consistency
pub mod templates {
    use super::*;

    pub struct TextTemplate {
        pub name: String,
        pub template: String,
        pub variables: Vec<String>,
    }

    impl TextTemplate {
        /// Create item description template
        pub fn item_description() -> Self {
            Self {
                name: "item_description".to_string(),
                template: "A {quality} {item_type} {origin}. {effect} {lore}".to_string(),
                variables: vec![
                    "quality".to_string(),
                    "item_type".to_string(),
                    "origin".to_string(),
                    "effect".to_string(),
                    "lore".to_string(),
                ],
            }
        }

        /// Create NPC dialogue template
        pub fn npc_dialogue() -> Self {
            Self {
                name: "npc_dialogue".to_string(),
                template: "{greeting} {mood_indicator} {main_message} {call_to_action}".to_string(),
                variables: vec![
                    "greeting".to_string(),
                    "mood_indicator".to_string(),
                    "main_message".to_string(),
                    "call_to_action".to_string(),
                ],
            }
        }

        /// Fill template with generated content
        pub async fn fill(&self, generator: &TextGenerator, context: &str) -> Result<String> {
            let mut filled = self.template.clone();

            for var in &self.variables {
                let prompt = format!(
                    "For a 16-bit game {}, generate a short phrase for '{}' to fill this template: {}",
                    context, var, self.template
                );

                let value = generator
                    .generate(
                        &prompt,
                        TextConfig {
                            max_tokens: 50,
                            temperature: 0.7,
                            ..Default::default()
                        },
                    )
                    .await?;

                filled = filled.replace(&format!("{{{var}}}"), value.trim());
            }

            Ok(filled)
        }
    }
}
