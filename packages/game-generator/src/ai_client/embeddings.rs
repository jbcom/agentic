//! Embeddings generation module for semantic similarity
//!
//! Handles generation of vector embeddings for:
//! - Text similarity matching
//! - Semantic search
//! - Content clustering
//! - Style consistency

use anyhow::{Context, Result};
use async_openai::{Client, config::OpenAIConfig, types::embeddings::CreateEmbeddingRequestArgs};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    AiConfig, AiGenerator,
    cache::{AiCache, CachedData},
    tokens::TokenCounter,
};

/// Embeddings generator for semantic similarity
#[derive(Clone)]
pub struct EmbeddingsGenerator {
    client: Arc<Client<OpenAIConfig>>,
    cache: Arc<Mutex<AiCache>>,
    token_counter: Arc<Mutex<TokenCounter>>,
}

#[async_trait::async_trait]
impl AiGenerator for EmbeddingsGenerator {
    async fn estimate_tokens(&self, request: &str) -> Result<usize> {
        // Embeddings use tiktoken for accurate count
        let counter = self.token_counter.lock().await;
        counter.count_tokens(request, "text-embedding-3-small")
    }

    async fn estimate_cost(&self, request: &str) -> Result<f64> {
        let tokens = self.estimate_tokens(request).await?;
        // text-embedding-3-small is $0.00002 / 1k tokens
        Ok((tokens as f64 / 1000.0) * 0.00002)
    }

    async fn is_cached(&self, key: &str) -> bool {
        self.cache.lock().await.get(key).await.is_some()
    }

    async fn clear_cache(&self, key: &str) -> Result<()> {
        self.cache.lock().await.clear(key).await
    }
}

impl EmbeddingsGenerator {
    /// Create a new embeddings generator
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

    /// Generate embeddings for a single text
    pub async fn generate(&self, text: &str, config: &AiConfig) -> Result<Vec<f32>> {
        // Check cache first
        let cache_key = format!("embedding:{}:{}", config.embedding_model, text);

        if let Some(cached) = self.cache.lock().await.get(&cache_key).await
            && let CachedData::Embedding(embedding) = cached.data
        {
            return Ok(embedding);
        }

        // Determine embedding model based on config
        let model = match config.embedding_model.as_str() {
            "text-embedding-3-small" => "text-embedding-3-small",
            "text-embedding-3-large" => "text-embedding-3-large",
            "text-embedding-ada-002" => "text-embedding-ada-002",
            _ => "text-embedding-3-small", // Default
        };

        let request = CreateEmbeddingRequestArgs::default()
            .model(model)
            .input(text)
            .build()?;

        let response = self
            .client
            .embeddings()
            .create(request)
            .await
            .context("Failed to generate embedding")?;

        // Extract the embedding vector
        let embedding = response
            .data
            .first()
            .ok_or_else(|| anyhow::anyhow!("No embedding returned"))?
            .embedding
            .clone();

        // Track token usage
        self.token_counter
            .lock()
            .await
            .record_usage(
                model,
                response.usage.prompt_tokens as usize,
                0, // No completion tokens for embeddings
            )
            .await?;

        // Cache the result
        self.cache
            .lock()
            .await
            .put(
                cache_key,
                CachedData::Embedding(embedding.clone()),
                std::collections::HashMap::new(),
            )
            .await?;

        Ok(embedding)
    }

    /// Generate embeddings for multiple texts in batch
    pub async fn generate_batch(
        &self,
        texts: Vec<&str>,
        config: &AiConfig,
    ) -> Result<Vec<Vec<f32>>> {
        // OpenAI supports batch embedding requests
        let model = match config.embedding_model.as_str() {
            "text-embedding-3-small" => "text-embedding-3-small",
            "text-embedding-3-large" => "text-embedding-3-large",
            "text-embedding-ada-002" => "text-embedding-ada-002",
            _ => "text-embedding-3-small",
        };

        let request = CreateEmbeddingRequestArgs::default()
            .model(model)
            .input(texts.clone())
            .build()?;

        let response = self
            .client
            .embeddings()
            .create(request)
            .await
            .context("Failed to generate embeddings batch")?;

        // Track token usage
        self.token_counter
            .lock()
            .await
            .record_usage(model, response.usage.prompt_tokens as usize, 0)
            .await?;

        // Extract all embeddings
        let embeddings: Vec<Vec<f32>> = response.data.into_iter().map(|e| e.embedding).collect();

        // Cache individual results
        for (idx, text) in texts.iter().enumerate() {
            if let Some(embedding) = embeddings.get(idx) {
                let cache_key = format!("embedding:{}:{}", config.embedding_model, text);
                self.cache
                    .lock()
                    .await
                    .put(
                        cache_key,
                        CachedData::Embedding(embedding.clone()),
                        std::collections::HashMap::new(),
                    )
                    .await?;
            }
        }

        Ok(embeddings)
    }

    /// Calculate cosine similarity between two embeddings
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let mut dot_product = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for i in 0..a.len() {
            dot_product += a[i] * b[i];
            norm_a += a[i] * a[i];
            norm_b += b[i] * b[i];
        }

        norm_a = norm_a.sqrt();
        norm_b = norm_b.sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// Find most similar texts from a collection
    pub async fn find_similar(
        &self,
        query: &str,
        candidates: Vec<&str>,
        config: &AiConfig,
        top_k: usize,
    ) -> Result<Vec<(String, f32)>> {
        // Get query embedding
        let query_embedding = self.generate(query, config).await?;

        // Get candidate embeddings
        let candidate_embeddings = self.generate_batch(candidates.clone(), config).await?;

        // Calculate similarities
        let mut similarities: Vec<(String, f32)> = candidates
            .into_iter()
            .zip(candidate_embeddings)
            .map(|(text, embedding)| {
                let similarity = Self::cosine_similarity(&query_embedding, &embedding);
                (text.to_string(), similarity)
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return top k
        similarities.truncate(top_k);
        Ok(similarities)
    }
}

/// Configuration specifically for embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Model to use for embeddings
    pub model: String,
    /// Dimension of embeddings (if supported by model)
    pub dimensions: Option<usize>,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: "text-embedding-3-small".to_string(),
            dimensions: None, // Use model default
        }
    }
}

impl From<&AiConfig> for EmbeddingConfig {
    fn from(config: &AiConfig) -> Self {
        Self {
            model: config.text_model.clone(),
            dimensions: None,
        }
    }
}
