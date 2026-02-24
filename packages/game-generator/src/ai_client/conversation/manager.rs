//! Core conversation manager implementation

use anyhow::{Context, Result};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use chrono::Utc;
use futures::Stream;
use futures::StreamExt;
use minijinja::Environment;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AiGenerator, tokens::TokenCounter};

use super::types::*;

/// Manages ongoing conversations with context
#[derive(Clone)]
pub struct ConversationManager {
    pub(crate) client: Arc<Client<OpenAIConfig>>,
    pub(crate) token_counter: Arc<Mutex<TokenCounter>>,
    pub(crate) conversations: Arc<Mutex<HashMap<String, Conversation>>>,
    pub(crate) template_env: Arc<Mutex<Option<Environment<'static>>>>,
    pub(crate) templates_dir: Option<PathBuf>,
}

impl ConversationManager {
    /// Create a new conversation manager
    pub fn new(client: Arc<Client<OpenAIConfig>>, token_counter: Arc<Mutex<TokenCounter>>) -> Self {
        Self {
            client,
            token_counter,
            conversations: Arc::new(Mutex::new(HashMap::new())),
            template_env: Arc::new(Mutex::new(None)),
            templates_dir: None,
        }
    }

    /// Initialize template environment for game generation
    pub async fn init_templates(&mut self, templates_dir: PathBuf) -> Result<()> {
        let mut env = Environment::new();

        // Load all templates from directory
        for entry in std::fs::read_dir(&templates_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension() == Some(std::ffi::OsStr::new("jinja")) {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Invalid template name"))?;

                let content = std::fs::read_to_string(&path)?;
                env.add_template_owned(name.to_string(), content)?;
            }
        }

        *self.template_env.lock().await = Some(env);
        self.templates_dir = Some(templates_dir);
        Ok(())
    }

    /// Start a new conversation
    pub async fn start_conversation(
        &self,
        title: String,
        context: ConversationContext,
    ) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let mut messages = VecDeque::new();

        // Add system message if provided
        if let Some(system_prompt) = &context.system_prompt {
            messages.push_back(ConversationMessage {
                role: MessageRole::System,
                content: system_prompt.clone(),
                timestamp: now,
                tokens: self.estimate_tokens(system_prompt).await?,
            });
        }

        let conversation = Conversation {
            id: id.clone(),
            title,
            messages,
            context,
            created_at: now,
            updated_at: now,
            total_tokens: 0,
        };

        self.conversations
            .lock()
            .await
            .insert(id.clone(), conversation);

        Ok(id)
    }

    /// Send a message and get response
    pub async fn send_message(&self, conversation_id: &str, message: String) -> Result<String> {
        self.send_message_with_config(conversation_id, message, None)
            .await
    }

    /// Send a message and get a streaming response
    pub async fn send_message_stream(
        &self,
        conversation_id: &str,
        message: String,
    ) -> Result<impl Stream<Item = Result<String>> + use<'_>> {
        self.send_message_stream_with_config(conversation_id, message, None)
            .await
    }

    /// Send a message with custom configuration
    pub async fn send_message_with_config(
        &self,
        conversation_id: &str,
        message: String,
        config: Option<MessageConfig>,
    ) -> Result<String> {
        let mut conversations = self.conversations.lock().await;
        let conversation = conversations
            .get_mut(conversation_id)
            .ok_or_else(|| anyhow::anyhow!("Conversation not found"))?;

        // Add user message
        let user_tokens = self.estimate_tokens(&message).await?;
        conversation.messages.push_back(ConversationMessage {
            role: MessageRole::User,
            content: message.clone(),
            timestamp: Utc::now(),
            tokens: user_tokens,
        });

        // Prepare messages for API
        let api_messages = self.prepare_api_messages(conversation)?;

        // Create request with optional custom config
        let config = config.unwrap_or_default();
        let request = CreateChatCompletionRequestArgs::default()
            .model(config.model.as_str())
            .messages(api_messages)
            .temperature(config.temperature)
            .max_tokens(config.max_tokens)
            .build()?;

        // Make API call
        let response = self
            .client
            .chat()
            .create(request)
            .await
            .context("Failed to get conversation response")?;

        // Extract response
        let assistant_message = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .unwrap_or_default();

        // Track tokens
        let model_name = config.model.as_str();
        if let Some(usage) = response.usage {
            self.token_counter
                .lock()
                .await
                .record_usage(
                    model_name,
                    usage.prompt_tokens as usize,
                    usage.completion_tokens as usize,
                )
                .await?;

            conversation.total_tokens += usage.total_tokens as usize;
        }

        // Add assistant message
        let assistant_tokens = self.estimate_tokens(&assistant_message).await?;
        conversation.messages.push_back(ConversationMessage {
            role: MessageRole::Assistant,
            content: assistant_message.clone(),
            timestamp: Utc::now(),
            tokens: assistant_tokens,
        });

        // Trim context if needed
        self.trim_context(conversation);

        conversation.updated_at = Utc::now();

        Ok(assistant_message)
    }

    /// Send a message with custom configuration and get a streaming response
    pub async fn send_message_stream_with_config(
        &self,
        conversation_id: &str,
        message: String,
        config: Option<MessageConfig>,
    ) -> Result<impl Stream<Item = Result<String>> + use<'_>> {
        let mut conversations = self.conversations.lock().await;
        let conversation = conversations
            .get_mut(conversation_id)
            .ok_or_else(|| anyhow::anyhow!("Conversation not found"))?;

        // Add user message
        let user_tokens = self.estimate_tokens(&message).await?;
        conversation.messages.push_back(ConversationMessage {
            role: MessageRole::User,
            content: message.clone(),
            timestamp: Utc::now(),
            tokens: user_tokens,
        });

        // Prepare messages for API
        let api_messages = self.prepare_api_messages(conversation)?;

        // Create request with optional custom config
        let config = config.unwrap_or_default();
        let request = CreateChatCompletionRequestArgs::default()
            .model(config.model.as_str())
            .messages(api_messages)
            .temperature(config.temperature)
            .max_tokens(config.max_tokens)
            .stream(true)
            .build()?;

        let mut stream = self.client.chat().create_stream(request).await?;

        let conversation_id = conversation_id.to_string();
        let conversations_arc = self.conversations.clone();
        let _token_counter = self.token_counter.clone();
        let _model_name = config.model.clone();

        Ok(async_stream::try_stream! {
            let mut full_response = String::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        if let Some(choice) = response.choices.first()
                            && let Some(content) = &choice.delta.content {
                                full_response.push_str(content);
                                yield content.clone();
                            }
                    }
                    Err(e) => {
                        Err(anyhow::anyhow!("Stream error: {}", e))?;
                    }
                }
            }

            // After stream completes, update the conversation history
            let mut convs = conversations_arc.lock().await;
            if let Some(conv) = convs.get_mut(&conversation_id) {
                // Approximate token count (4 chars = 1 token)
                let assistant_tokens = full_response.len() / 4;

                conv.messages.push_back(ConversationMessage {
                    role: MessageRole::Assistant,
                    content: full_response,
                    timestamp: Utc::now(),
                    tokens: assistant_tokens,
                });

                // Trim context
                let max_messages = conv.context.max_context_messages;
                let system_count = conv.messages.iter().filter(|m| matches!(m.role, MessageRole::System)).count();
                let max_non_system = max_messages.saturating_sub(system_count);
                let non_system_count = conv.messages.iter().filter(|m| !matches!(m.role, MessageRole::System)).count();

                if non_system_count > max_non_system {
                    let to_remove = non_system_count - max_non_system;
                    let mut removed = 0;
                    while removed < to_remove {
                        for i in 0..conv.messages.len() {
                            if !matches!(conv.messages[i].role, MessageRole::System) {
                                conv.messages.remove(i);
                                removed += 1;
                                break;
                            }
                        }
                    }
                }

                conv.updated_at = Utc::now();
            }
        })
    }

    /// Update conversation context (e.g., after blend changes)
    pub async fn update_context(
        &self,
        conversation_id: &str,
        context: ConversationContext,
    ) -> Result<()> {
        let mut conversations = self.conversations.lock().await;
        if let Some(conversation) = conversations.get_mut(conversation_id) {
            conversation.context = context;
            conversation.updated_at = Utc::now();
        }
        Ok(())
    }

    /// Get conversation history
    pub async fn get_conversation(&self, conversation_id: &str) -> Result<Conversation> {
        let conversations = self.conversations.lock().await;
        conversations
            .get(conversation_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Conversation not found"))
    }

    /// List all conversations
    pub async fn list_conversations(&self) -> Result<Vec<ConversationSummary>> {
        let conversations = self.conversations.lock().await;
        let mut summaries: Vec<_> = conversations
            .values()
            .map(|conv| ConversationSummary {
                id: conv.id.clone(),
                title: conv.title.clone(),
                created_at: conv.created_at,
                updated_at: conv.updated_at,
                message_count: conv.messages.len(),
                total_tokens: conv.total_tokens,
            })
            .collect();

        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(summaries)
    }

    /// Delete a conversation
    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<()> {
        self.conversations.lock().await.remove(conversation_id);
        Ok(())
    }

    /// Prepare messages for API call
    fn prepare_api_messages(
        &self,
        conversation: &Conversation,
    ) -> Result<Vec<ChatCompletionRequestMessage>> {
        let mut messages = Vec::new();

        // Add system message if exists
        if let Some(system_msg) = conversation
            .messages
            .iter()
            .find(|m| matches!(m.role, MessageRole::System))
        {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_msg.content.as_str())
                    .build()?
                    .into(),
            );
        }

        // Add context summary if needed
        if let Some(game_context) = &conversation.context.game_concept {
            let context_summary = format!(
                "Current game concept: {} ({}). Inspirations: {}",
                game_context.title,
                game_context.genre,
                game_context.inspirations.join(", ")
            );

            if let Some(blend) = &game_context.current_blend {
                let blend_summary = format!(
                    "\nCurrent blend: {} with dominant attributes: {}",
                    blend.selected_games.join(" + "),
                    blend.dominant_attributes.join(", ")
                );
                messages.push(
                    ChatCompletionRequestSystemMessageArgs::default()
                        .content(format!("{context_summary}{blend_summary}").as_str())
                        .build()?
                        .into(),
                );
            }
        }

        // Add conversation messages (skip system messages as they're already added)
        for msg in conversation
            .messages
            .iter()
            .filter(|m| !matches!(m.role, MessageRole::System))
            .take(conversation.context.max_context_messages)
        {
            match msg.role {
                MessageRole::User => {
                    messages.push(
                        ChatCompletionRequestUserMessageArgs::default()
                            .content(msg.content.as_str())
                            .build()?
                            .into(),
                    );
                }
                MessageRole::Assistant => {
                    messages.push(
                        ChatCompletionRequestAssistantMessageArgs::default()
                            .content(msg.content.as_str())
                            .build()?
                            .into(),
                    );
                }
                _ => {}
            }
        }

        Ok(messages)
    }

    /// Trim conversation context to stay within limits
    fn trim_context(&self, conversation: &mut Conversation) {
        let max_messages = conversation.context.max_context_messages;

        // Keep system message + last N messages
        let system_count = conversation
            .messages
            .iter()
            .filter(|m| matches!(m.role, MessageRole::System))
            .count();

        let max_non_system = max_messages.saturating_sub(system_count);

        // Count non-system messages
        let non_system_count = conversation
            .messages
            .iter()
            .filter(|m| !matches!(m.role, MessageRole::System))
            .count();

        // Remove oldest non-system messages if we exceed the limit
        if non_system_count > max_non_system {
            let to_remove = non_system_count - max_non_system;
            let mut removed = 0;

            // Remove from the front (oldest messages)
            while removed < to_remove {
                for i in 0..conversation.messages.len() {
                    if !matches!(conversation.messages[i].role, MessageRole::System) {
                        conversation.messages.remove(i);
                        removed += 1;
                        break;
                    }
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl AiGenerator for ConversationManager {
    async fn estimate_tokens(&self, request: &str) -> Result<usize> {
        let counter = self.token_counter.lock().await;
        counter.count_tokens(request, "gpt-4-turbo")
    }

    async fn estimate_cost(&self, request: &str) -> Result<f64> {
        let counter = self.token_counter.lock().await;
        counter.estimate_cost(request, "gpt-4-turbo", 2000)
    }

    async fn is_cached(&self, _key: &str) -> bool {
        // Conversations are not typically cached
        false
    }

    async fn clear_cache(&self, _key: &str) -> Result<()> {
        // No cache to clear for conversations
        Ok(())
    }
}
