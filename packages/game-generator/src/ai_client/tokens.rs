//! Token counting and cost estimation for AI operations
//!
//! Uses tiktoken-rs for accurate token counting
//! Supports all OpenAI models and their pricing

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tiktoken_rs::{CoreBPE, cl100k_base, p50k_base, r50k_base};
use tokio::sync::Mutex;

/// Token counter for tracking usage and costs
pub struct TokenCounter {
    /// Token encoders for different models
    encoders: HashMap<String, Arc<CoreBPE>>,
    /// Usage statistics
    stats: Arc<Mutex<TokenStats>>,
    /// Model pricing information
    pricing: ModelPricing,
}

#[derive(Debug, Clone, Default)]
pub struct TokenStats {
    /// Total tokens used for prompts
    pub prompt_tokens: u64,
    /// Total tokens used for completions
    pub completion_tokens: u64,
    /// Total tokens for embeddings
    pub embedding_tokens: u64,
    /// Total tokens for images (estimated)
    pub image_tokens: u64,
    /// Total cost in USD
    pub total_cost: f64,
    /// Cost breakdown by model
    pub cost_by_model: HashMap<String, f64>,
    /// Token breakdown by model
    pub tokens_by_model: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct ModelPricing {
    /// Pricing per model
    models: HashMap<String, ModelCost>,
}

#[derive(Debug, Clone)]
pub struct ModelCost {
    /// Cost per 1K prompt tokens in USD
    pub prompt_cost_per_1k: f64,
    /// Cost per 1K completion tokens in USD
    pub completion_cost_per_1k: f64,
    /// Cost per image generation (for DALL-E)
    pub image_cost: Option<f64>,
    /// Cost per 1K embedding tokens
    pub embedding_cost_per_1k: Option<f64>,
}

impl Default for ModelPricing {
    fn default() -> Self {
        let mut models = HashMap::new();

        // GPT-4 pricing
        models.insert(
            "gpt-4".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.03,
                completion_cost_per_1k: 0.06,
                image_cost: None,
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "gpt-4-turbo".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.01,
                completion_cost_per_1k: 0.03,
                image_cost: None,
                embedding_cost_per_1k: None,
            },
        );

        // GPT-3.5 pricing
        models.insert(
            "gpt-3.5-turbo".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0005,
                completion_cost_per_1k: 0.0015,
                image_cost: None,
                embedding_cost_per_1k: None,
            },
        );

        // DALL-E pricing
        models.insert(
            "dall-e-3-1024x1024-standard".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.04),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1024x1024-hd".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.08),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1792x1024-standard".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.08),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1024x1792-standard".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.08),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1792x1024-hd".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.12),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1024x1792-hd".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.12),
                embedding_cost_per_1k: None,
            },
        );

        // Additional sizes (mapping to similar pricing)
        models.insert(
            "dall-e-3-1536x1024-standard".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.08),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1536x1024-hd".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.12),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1024x1536-standard".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.08),
                embedding_cost_per_1k: None,
            },
        );

        models.insert(
            "dall-e-3-1024x1536-hd".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: Some(0.12),
                embedding_cost_per_1k: None,
            },
        );

        // Embedding models
        models.insert(
            "text-embedding-3-small".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: None,
                embedding_cost_per_1k: Some(0.00002),
            },
        );

        models.insert(
            "text-embedding-3-large".to_string(),
            ModelCost {
                prompt_cost_per_1k: 0.0,
                completion_cost_per_1k: 0.0,
                image_cost: None,
                embedding_cost_per_1k: Some(0.00013),
            },
        );

        Self { models }
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenCounter {
    /// Create a new token counter
    pub fn new() -> Self {
        let mut encoders = HashMap::new();

        // Initialize encoders for different model families
        encoders.insert("cl100k_base".to_string(), Arc::new(cl100k_base().unwrap()));
        encoders.insert("p50k_base".to_string(), Arc::new(p50k_base().unwrap()));
        encoders.insert("r50k_base".to_string(), Arc::new(r50k_base().unwrap()));

        Self {
            encoders,
            stats: Arc::new(Mutex::new(TokenStats::default())),
            pricing: ModelPricing::default(),
        }
    }

    /// Count tokens for a given text and model
    pub fn count_tokens(&self, text: &str, model: &str) -> Result<usize> {
        let encoder = self.get_encoder_for_model(model)?;
        let tokens = encoder.encode_with_special_tokens(text);
        Ok(tokens.len())
    }

    /// Estimate tokens for an image
    pub fn estimate_image_tokens(&self, width: u32, height: u32) -> usize {
        // Rough estimation based on image dimensions
        // OpenAI doesn't provide exact token counts for images
        let pixels = width * height;
        let base_tokens = 85; // Base tokens for any image
        let dimension_tokens = (pixels as f64 / 750.0).ceil() as usize;
        base_tokens + dimension_tokens
    }

    /// Record token usage
    pub async fn record_usage(
        &self,
        model: &str,
        prompt_tokens: usize,
        completion_tokens: usize,
    ) -> Result<()> {
        let mut stats = self.stats.lock().await;

        // Update token counts
        stats.prompt_tokens += prompt_tokens as u64;
        stats.completion_tokens += completion_tokens as u64;

        // Update per-model stats
        *stats.tokens_by_model.entry(model.to_string()).or_insert(0) +=
            (prompt_tokens + completion_tokens) as u64;

        // Calculate cost
        if let Some(pricing) = self.pricing.models.get(model) {
            let prompt_cost = (prompt_tokens as f64 / 1000.0) * pricing.prompt_cost_per_1k;
            let completion_cost =
                (completion_tokens as f64 / 1000.0) * pricing.completion_cost_per_1k;
            let total_cost = prompt_cost + completion_cost;

            stats.total_cost += total_cost;
            *stats.cost_by_model.entry(model.to_string()).or_insert(0.0) += total_cost;
        }

        Ok(())
    }

    /// Record image generation
    pub async fn record_image_generation(
        &self,
        model: &str,
        width: u32,
        height: u32,
        count: usize,
    ) -> Result<()> {
        let mut stats = self.stats.lock().await;

        if let Some(pricing) = self.pricing.models.get(model)
            && let Some(image_cost) = pricing.image_cost
        {
            let total_cost = image_cost * count as f64;
            stats.total_cost += total_cost;
            *stats.cost_by_model.entry(model.to_string()).or_insert(0.0) += total_cost;
            stats.image_tokens += self.estimate_image_tokens(width, height) as u64 * count as u64;
        }

        Ok(())
    }

    /// Record embedding usage
    pub async fn record_embedding(&self, model: &str, tokens: usize) -> Result<()> {
        let mut stats = self.stats.lock().await;

        stats.embedding_tokens += tokens as u64;

        if let Some(pricing) = self.pricing.models.get(model)
            && let Some(embedding_cost) = pricing.embedding_cost_per_1k
        {
            let total_cost = (tokens as f64 / 1000.0) * embedding_cost;
            stats.total_cost += total_cost;
            *stats.cost_by_model.entry(model.to_string()).or_insert(0.0) += total_cost;
        }

        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> TokenStats {
        self.stats.lock().await.clone()
    }

    /// Reset statistics
    pub async fn reset_stats(&self) {
        *self.stats.lock().await = TokenStats::default();
    }

    /// Estimate cost for a request before making it
    pub fn estimate_cost(
        &self,
        model: &str,
        prompt: &str,
        max_completion_tokens: usize,
    ) -> Result<f64> {
        let prompt_tokens = self.count_tokens(prompt, model)?;

        if let Some(pricing) = self.pricing.models.get(model) {
            let prompt_cost = (prompt_tokens as f64 / 1000.0) * pricing.prompt_cost_per_1k;
            let completion_cost =
                (max_completion_tokens as f64 / 1000.0) * pricing.completion_cost_per_1k;
            Ok(prompt_cost + completion_cost)
        } else {
            Ok(0.0)
        }
    }

    /// Get the appropriate encoder for a model
    fn get_encoder_for_model(&self, model: &str) -> Result<&Arc<CoreBPE>> {
        // Determine which encoder to use based on model name
        let encoder_name = if model.starts_with("gpt-4") || model.starts_with("gpt-3.5-turbo") {
            "cl100k_base"
        } else if model.contains("davinci") || model.contains("curie") {
            "p50k_base"
        } else {
            "cl100k_base" // Default to newest encoder
        };

        self.encoders
            .get(encoder_name)
            .context(format!("No encoder found for model: {model}"))
    }
}

/// Token optimization strategies
pub struct TokenOptimizer {
    /// Maximum context window sizes by model
    context_windows: HashMap<String, usize>,
}

impl Default for TokenOptimizer {
    fn default() -> Self {
        let mut context_windows = HashMap::new();

        context_windows.insert("gpt-4".to_string(), 8192);
        context_windows.insert("gpt-4-32k".to_string(), 32768);
        context_windows.insert("gpt-4-turbo".to_string(), 128000);
        context_windows.insert("gpt-3.5-turbo".to_string(), 16384);
        context_windows.insert("gpt-3.5-turbo-16k".to_string(), 16384);

        Self { context_windows }
    }
}

impl TokenOptimizer {
    /// Optimize a prompt to fit within token limits
    pub fn optimize_prompt(
        &self,
        prompt: &str,
        model: &str,
        max_completion_tokens: usize,
        counter: &TokenCounter,
    ) -> Result<String> {
        let max_context = self.context_windows.get(model).copied().unwrap_or(4096);
        let current_tokens = counter.count_tokens(prompt, model)?;

        if current_tokens + max_completion_tokens <= max_context {
            return Ok(prompt.to_string());
        }

        // Need to truncate - implement smart truncation
        let available_tokens = max_context - max_completion_tokens - 100; // Safety margin
        self.truncate_to_token_limit(prompt, available_tokens, model, counter)
    }

    /// Truncate text to fit within token limit
    fn truncate_to_token_limit(
        &self,
        text: &str,
        max_tokens: usize,
        model: &str,
        counter: &TokenCounter,
    ) -> Result<String> {
        // Binary search for the right truncation point
        let chars: Vec<char> = text.chars().collect();
        let mut left = 0;
        let mut right = chars.len();
        let mut best_fit = String::new();

        while left < right {
            let mid = (left + right).div_ceil(2);
            let truncated: String = chars[..mid].iter().collect();
            let tokens = counter.count_tokens(&truncated, model)?;

            if tokens <= max_tokens {
                best_fit = truncated;
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        Ok(best_fit)
    }

    /// Split text into chunks that fit within token limits
    pub fn chunk_text(
        &self,
        text: &str,
        max_tokens_per_chunk: usize,
        model: &str,
        counter: &TokenCounter,
    ) -> Result<Vec<String>> {
        let mut chunks = Vec::new();
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        let mut current_chunk = String::new();
        let mut current_tokens = 0;

        for paragraph in paragraphs {
            let paragraph_tokens = counter.count_tokens(paragraph, model)?;

            if current_tokens + paragraph_tokens > max_tokens_per_chunk && !current_chunk.is_empty()
            {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
                current_tokens = 0;
            }

            if paragraph_tokens > max_tokens_per_chunk {
                // Split large paragraph
                let sub_chunks =
                    self.split_paragraph(paragraph, max_tokens_per_chunk, model, counter)?;
                for sub_chunk in sub_chunks {
                    chunks.push(sub_chunk);
                }
            } else {
                if !current_chunk.is_empty() {
                    current_chunk.push_str("\n\n");
                }
                current_chunk.push_str(paragraph);
                current_tokens += paragraph_tokens;
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        Ok(chunks)
    }

    fn split_paragraph(
        &self,
        paragraph: &str,
        max_tokens: usize,
        model: &str,
        counter: &TokenCounter,
    ) -> Result<Vec<String>> {
        let sentences: Vec<&str> = paragraph.split(". ").collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_tokens = 0;

        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_with_period = if i < sentences.len() - 1 {
                format!("{sentence}. ")
            } else {
                sentence.to_string()
            };

            let sentence_tokens = counter.count_tokens(&sentence_with_period, model)?;

            if current_tokens + sentence_tokens > max_tokens && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
                current_tokens = 0;
            }

            current_chunk.push_str(&sentence_with_period);
            current_tokens += sentence_tokens;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        Ok(chunks)
    }
}

/// Cost optimization suggestions
#[derive(Debug, Clone)]
pub struct CostOptimizationSuggestion {
    pub description: String,
    pub potential_savings: f64,
    pub implementation_difficulty: Difficulty,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Analyze usage and provide cost optimization suggestions
pub async fn analyze_usage_for_optimizations(
    stats: &TokenStats,
) -> Vec<CostOptimizationSuggestion> {
    let mut suggestions = Vec::new();

    // Check if using expensive models for simple tasks
    if let Some(gpt4_cost) = stats.cost_by_model.get("gpt-4")
        && *gpt4_cost > stats.total_cost * 0.5
    {
        suggestions.push(CostOptimizationSuggestion {
            description: "Consider using GPT-3.5-turbo for simpler tasks".to_string(),
            potential_savings: gpt4_cost * 0.7,
            implementation_difficulty: Difficulty::Easy,
        });
    }

    // Check prompt efficiency
    let avg_prompt_length = if stats.prompt_tokens > 0 {
        stats.prompt_tokens as f64 / stats.tokens_by_model.len() as f64
    } else {
        0.0
    };

    if avg_prompt_length > 1000.0 {
        suggestions.push(CostOptimizationSuggestion {
            description: "Optimize prompts to be more concise".to_string(),
            potential_savings: stats.total_cost * 0.2,
            implementation_difficulty: Difficulty::Medium,
        });
    }

    // Suggest caching for repeated requests
    suggestions.push(CostOptimizationSuggestion {
        description: "Enable caching for repeated AI requests".to_string(),
        potential_savings: stats.total_cost * 0.3,
        implementation_difficulty: Difficulty::Easy,
    });

    suggestions
}
