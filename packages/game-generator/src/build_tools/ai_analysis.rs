//! AI-powered game analysis during build time

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tiktoken_rs::{CoreBPE, get_bpe_from_model};
use vintage_ai_client::{
    AiService,
    text::{TextConfig, TextGenerator},
};

/// AI-analyzed game metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedGameMetadata {
    pub id: u32,
    pub name: String,
    pub year: i32,
    pub original_genre: String,
    pub platforms: Vec<String>,
    pub developer: Option<String>,
    pub deck: Option<String>,

    // AI-analyzed fields
    pub themes: Vec<String>,
    pub narrative_elements: Vec<String>,
    pub mechanics: Vec<GameMechanic>,
    pub mood_tags: Vec<String>,
    pub innovation_aspects: Vec<String>,
    pub cultural_impact: String,
    pub design_philosophy: String,
    pub player_experience: String,
    pub difficulty_curve: String,
    pub replayability_factors: Vec<String>,
    pub artistic_style: String,
    pub audio_design: String,
    pub pacing: String,
    pub target_audience: Vec<String>,
    pub unique_features: Vec<String>,
    pub influenced_by: Vec<String>,
    pub influenced_games: Vec<String>,
    pub genre_blend: Vec<(String, f32)>, // Genre with weight
    pub era_significance: String,
    pub technical_achievements: Vec<String>,
    pub memorable_moments: Vec<String>,
    pub core_loop: String,
    pub progression_system: String,
    pub social_features: Vec<String>,
    pub accessibility_notes: Vec<String>,

    // Semantic embeddings for similarity
    pub theme_embeddings: Vec<f32>,
    pub mechanic_embeddings: Vec<f32>,
    pub narrative_embeddings: Vec<f32>,
    pub overall_embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMechanic {
    pub name: String,
    pub description: String,
    pub importance: f32,       // 0.0 to 1.0
    pub innovation_level: f32, // 0.0 to 1.0
}

pub struct AIAnalyzer {
    bpe: CoreBPE,
    text_generator: TextGenerator,
}

impl AIAnalyzer {
    /// Create a new AI analyzer.
    /// Note: Uses OPENAI_API_KEY environment variable for authentication.
    /// The api_key parameter can be used to set this env var if not already set.
    pub fn new(api_key: String) -> Result<Self> {
        // Set the API key in environment if not already set
        // SAFETY: This is called during single-threaded initialization before
        // any async operations begin, so no concurrent access is possible.
        if std::env::var("OPENAI_API_KEY").is_err() {
            unsafe {
                std::env::set_var("OPENAI_API_KEY", &api_key);
            }
        }

        let bpe = get_bpe_from_model("gpt-4")?;
        let ai_service = AiService::from_env()?;
        let text_generator = ai_service.text();
        Ok(Self {
            bpe,
            text_generator,
        })
    }

    /// Analyze all games in batches using intelligent prompt chunking
    pub async fn analyze_games(
        &self,
        games: &[Value],
        batch_size: usize,
    ) -> Result<Vec<EnrichedGameMetadata>> {
        println!("Starting AI analysis of {} games...", games.len());

        let mut all_enriched = Vec::new();

        // Process games in batches
        for (batch_idx, batch) in games.chunks(batch_size).enumerate() {
            println!(
                "Processing batch {}/{}",
                batch_idx + 1,
                games.len().div_ceil(batch_size)
            );

            let enriched_batch = self.analyze_batch(batch).await?;
            all_enriched.extend(enriched_batch);

            // Rate limiting pause between batches
            if batch_idx < games.chunks(batch_size).len() - 1 {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }

        Ok(all_enriched)
    }

    /// Analyze a batch of games in a single prompt
    fn analyze_batch<'a>(
        &'a self,
        games: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<EnrichedGameMetadata>>> + Send + 'a>,
    > {
        Box::pin(async move {
            // Build the analysis prompt
            let prompt = self.build_batch_prompt(games)?;

            // Check token count and split if needed
            let token_count = self.count_tokens(&prompt);
            println!("  Batch prompt tokens: {token_count}");

            if token_count > 100000 {
                // Leave room for response
                // Split into smaller batches
                let mid = games.len() / 2;
                let mut results = Vec::new();
                results.extend(self.analyze_batch(&games[..mid]).await?);
                results.extend(self.analyze_batch(&games[mid..]).await?);
                return Ok(results);
            }

            // Send to AI for analysis
            let response = self.send_analysis_request(&prompt).await?;

            // Parse the structured response
            let enriched = self.parse_analysis_response(&response, games)?;

            Ok(enriched)
        })
    }

    /// Build a comprehensive analysis prompt for a batch of games
    fn build_batch_prompt(&self, games: &[Value]) -> Result<String> {
        let template = include_str!("../templates/ai_analysis/batch_analysis.jinja");

        let mut env = minijinja::Environment::new();
        env.add_template("batch_analysis", template)?;

        let tmpl = env.get_template("batch_analysis")?;

        // Convert games to a simpler format for the template
        let game_data: Vec<HashMap<String, Value>> = games
            .iter()
            .map(|g| {
                let mut map = HashMap::new();
                map.insert(
                    "id".to_string(),
                    g.get("id").cloned().unwrap_or(Value::Null),
                );
                map.insert(
                    "name".to_string(),
                    g.get("name").cloned().unwrap_or(Value::Null),
                );
                map.insert(
                    "year".to_string(),
                    g.get("year").cloned().unwrap_or(Value::Null),
                );
                map.insert(
                    "genre".to_string(),
                    g.get("genre").cloned().unwrap_or(Value::Null),
                );
                map.insert(
                    "platforms".to_string(),
                    g.get("platforms").cloned().unwrap_or(Value::Null),
                );
                map.insert(
                    "developer".to_string(),
                    g.get("developer").cloned().unwrap_or(Value::Null),
                );
                map.insert(
                    "deck".to_string(),
                    g.get("deck").cloned().unwrap_or(Value::Null),
                );
                map
            })
            .collect();

        let context = minijinja::context! {
            games => game_data,
            analysis_date => chrono::Utc::now().format("%Y-%m-%d").to_string(),
        };

        Ok(tmpl.render(context)?)
    }

    /// Count tokens in a string
    fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }

    /// Send analysis request to AI
    async fn send_analysis_request(&self, prompt: &str) -> Result<String> {
        let system_prompt = "You are a video game historian and design analyst. Analyze vintage games with deep insight into their design, cultural impact, and innovations. Always respond with valid JSON.";

        let config = TextConfig {
            model: "gpt-4-turbo".to_string(),
            system_prompt: Some(system_prompt.to_string()),
            temperature: 0.7,
            max_tokens: 50000,
            ..Default::default()
        };

        // Add JSON instruction to the prompt
        let json_prompt = format!("{prompt}\n\nIMPORTANT: Respond ONLY with valid JSON.");

        let response = self.text_generator.generate(&json_prompt, config).await?;
        Ok(response)
    }

    /// Parse the AI analysis response
    fn parse_analysis_response(
        &self,
        response: &str,
        original_games: &[Value],
    ) -> Result<Vec<EnrichedGameMetadata>> {
        let analysis: Value = serde_json::from_str(response)?;

        let games_analysis = analysis["games"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("No games array in response"))?;

        // Create a map of original games by ID for efficient lookup
        let original_map: std::collections::HashMap<u32, &Value> = original_games
            .iter()
            .filter_map(|g| {
                let id = g.get("id").and_then(|v| v.as_u64())? as u32;
                Some((id, g))
            })
            .collect();

        let mut enriched_games = Vec::new();

        // Match by game ID instead of index position
        for game_analysis in games_analysis.iter() {
            // Try to get the game ID from the analysis response
            let analysis_id = game_analysis
                .get("id")
                .and_then(|v| v.as_u64())
                .or_else(|| game_analysis.get("game_id").and_then(|v| v.as_u64()))
                .map(|id| id as u32);

            if let Some(id) = analysis_id {
                if let Some(original) = original_map.get(&id) {
                    let enriched = self.merge_analysis_with_original(original, game_analysis)?;
                    enriched_games.push(enriched);
                } else {
                    eprintln!("Warning: AI returned analysis for unknown game ID {id}");
                }
            } else {
                eprintln!("Warning: AI response missing game ID, skipping entry");
            }
        }

        // Warn if we didn't get all games back
        if enriched_games.len() < original_games.len() {
            eprintln!(
                "Warning: Only received {} of {} game analyses from AI",
                enriched_games.len(),
                original_games.len()
            );
        }

        Ok(enriched_games)
    }

    /// Merge AI analysis with original game data
    fn merge_analysis_with_original(
        &self,
        original: &Value,
        analysis: &Value,
    ) -> Result<EnrichedGameMetadata> {
        // Extract original fields
        let id = original.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

        let name = original
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let year = original
            .get("year")
            .and_then(|v| v.as_i64())
            .unwrap_or(1980) as i32;

        let original_genre = original
            .get("genre")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let platforms = original
            .get("platforms")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| p.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let developer = original
            .get("developer")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let deck = original
            .get("deck")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Extract AI-analyzed fields
        let themes = self.extract_string_array(analysis, "themes");
        let narrative_elements = self.extract_string_array(analysis, "narrative_elements");
        let mood_tags = self.extract_string_array(analysis, "mood_tags");
        let innovation_aspects = self.extract_string_array(analysis, "innovation_aspects");
        let replayability_factors = self.extract_string_array(analysis, "replayability_factors");
        let target_audience = self.extract_string_array(analysis, "target_audience");
        let unique_features = self.extract_string_array(analysis, "unique_features");
        let influenced_by = self.extract_string_array(analysis, "influenced_by");
        let influenced_games = self.extract_string_array(analysis, "influenced_games");
        let technical_achievements = self.extract_string_array(analysis, "technical_achievements");
        let memorable_moments = self.extract_string_array(analysis, "memorable_moments");
        let social_features = self.extract_string_array(analysis, "social_features");
        let accessibility_notes = self.extract_string_array(analysis, "accessibility_notes");

        // Extract mechanics
        let mechanics = self.extract_mechanics(analysis);

        // Extract genre blend
        let genre_blend = self.extract_genre_blend(analysis);

        // Extract single string fields
        let cultural_impact = analysis
            .get("cultural_impact")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let design_philosophy = analysis
            .get("design_philosophy")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let player_experience = analysis
            .get("player_experience")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let difficulty_curve = analysis
            .get("difficulty_curve")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let artistic_style = analysis
            .get("artistic_style")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let audio_design = analysis
            .get("audio_design")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let pacing = analysis
            .get("pacing")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let era_significance = analysis
            .get("era_significance")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let core_loop = analysis
            .get("core_loop")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let progression_system = analysis
            .get("progression_system")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Extract embeddings
        let theme_embeddings = self.extract_float_array(analysis, "theme_embeddings");
        let mechanic_embeddings = self.extract_float_array(analysis, "mechanic_embeddings");
        let narrative_embeddings = self.extract_float_array(analysis, "narrative_embeddings");
        let overall_embedding = self.extract_float_array(analysis, "overall_embedding");

        Ok(EnrichedGameMetadata {
            id,
            name,
            year,
            original_genre,
            platforms,
            developer,
            deck,
            themes,
            narrative_elements,
            mechanics,
            mood_tags,
            innovation_aspects,
            cultural_impact,
            design_philosophy,
            player_experience,
            difficulty_curve,
            replayability_factors,
            artistic_style,
            audio_design,
            pacing,
            target_audience,
            unique_features,
            influenced_by,
            influenced_games,
            genre_blend,
            era_significance,
            technical_achievements,
            memorable_moments,
            core_loop,
            progression_system,
            social_features,
            accessibility_notes,
            theme_embeddings,
            mechanic_embeddings,
            narrative_embeddings,
            overall_embedding,
        })
    }

    fn extract_string_array(&self, obj: &Value, key: &str) -> Vec<String> {
        obj.get(key)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn extract_float_array(&self, obj: &Value, key: &str) -> Vec<f32> {
        obj.get(key)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_f64())
                    .map(|f| f as f32)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn extract_mechanics(&self, obj: &Value) -> Vec<GameMechanic> {
        obj.get("mechanics")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| {
                        let name = m.get("name")?.as_str()?.to_string();
                        let description = m.get("description")?.as_str()?.to_string();
                        let importance = m.get("importance")?.as_f64()? as f32;
                        let innovation_level = m.get("innovation_level")?.as_f64()? as f32;

                        Some(GameMechanic {
                            name,
                            description,
                            importance,
                            innovation_level,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn extract_genre_blend(&self, obj: &Value) -> Vec<(String, f32)> {
        obj.get("genre_blend")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|g| {
                        let genre = g.get("genre")?.as_str()?.to_string();
                        let weight = g.get("weight")?.as_f64()? as f32;
                        Some((genre, weight))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Generate embeddings using OpenAI's embeddings API
pub async fn generate_embeddings(text: &str, ai_service: &AiService) -> Result<Vec<f32>> {
    ai_service
        .embeddings()
        .generate(text, &Default::default())
        .await
}

/// Generate embeddings for multiple texts in batch
pub async fn generate_embeddings_batch(
    texts: Vec<&str>,
    ai_service: &AiService,
) -> Result<Vec<Vec<f32>>> {
    ai_service
        .embeddings()
        .generate_batch(texts, &Default::default())
        .await
}
