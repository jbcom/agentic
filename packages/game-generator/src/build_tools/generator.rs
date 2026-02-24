//! Main game data generator orchestrator

use crate::{
    ai_analysis::{AIAnalyzer, EnrichedGameMetadata},
    api::GiantBombClient,
    graph::GraphBuilder,
    images::ImageDownloader,
    templates::TemplateProcessor,
    types::*,
};
use anyhow::Result;
use std::path::Path;

pub struct GameDataGenerator {
    api_key: String,
    openai_api_key: Option<String>,
    timeline_start: i32,
    timeline_end: i32,
}

impl GameDataGenerator {
    pub fn new(api_key: String, timeline_start: i32, timeline_end: i32) -> Self {
        // Check for OpenAI API key in environment
        let openai_api_key = std::env::var("OPENAI_API_KEY").ok();

        Self {
            api_key,
            openai_api_key,
            timeline_start,
            timeline_end,
        }
    }

    /// Create with explicit OpenAI API key
    pub fn with_openai_key(
        api_key: String,
        openai_api_key: String,
        timeline_start: i32,
        timeline_end: i32,
    ) -> Self {
        Self {
            api_key,
            openai_api_key: Some(openai_api_key),
            timeline_start,
            timeline_end,
        }
    }

    /// Run the complete generation process
    pub async fn generate(&self) -> Result<()> {
        // Check if we need to generate
        if self.is_already_generated() {
            println!("Vintage game timeline already exists and validated, skipping generation");
            return Ok(());
        }

        println!(
            "Building vintage game timeline ({}-{})...",
            self.timeline_start, self.timeline_end
        );

        // Create API client
        let client = GiantBombClient::new(self.api_key.clone())?;

        // 1. Fetch platform information
        let platforms = client.fetch_platforms()?;

        // 2. Fetch games timeline
        let timeline = client.fetch_timeline_games(self.timeline_start, self.timeline_end)?;

        // 3. Enhance games with detailed images
        let enhanced_games = client.enhance_games_with_images(timeline)?;

        // 4. Convert to JSON format for templates
        let mut timeline_games = self.convert_to_json(&enhanced_games)?;

        println!(
            "Built timeline with {} exemplar games",
            timeline_games.len()
        );

        // 5. AI Analysis (REQUIRED)
        let openai_key = self.openai_api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!(
                "OPENAI_API_KEY not found in environment. AI analysis is REQUIRED for high-quality game metadata. Please set it in your .env or .env.local file."
            ))?;

        println!("Running AI analysis on game collection...");
        let analyzer = AIAnalyzer::new(openai_key.clone())?;

        // Analyze games in batches
        let enriched_metadata = analyzer.analyze_games(&timeline_games, 10).await?;

        // Merge enriched metadata back into timeline_games
        self.merge_enriched_metadata(&mut timeline_games, &enriched_metadata)?;

        // 6. Download game cover images
        let image_downloader = ImageDownloader::new("assets/wizard/game_covers")?;
        image_downloader.download_game_covers(&timeline_games)?;

        // 7. Pre-compute game similarity graph with enriched metadata
        let graph_data =
            GraphBuilder::build_enriched_game_graph(&timeline_games, &enriched_metadata)?;

        // 8. Generate Rust modules from templates
        let template_processor =
            TemplateProcessor::new("templates/giantbomb", "src/vintage_games")?;

        template_processor.generate_modules(
            &timeline_games,
            &platforms,
            &graph_data,
            self.timeline_start,
            self.timeline_end,
        )?;

        // Validate that all required files were generated
        self.validate_generation()?;

        println!("Vintage game timeline successfully generated!");
        Ok(())
    }

    /// Check if vintage game data is already generated and valid
    fn is_already_generated(&self) -> bool {
        // Check all required module files
        let required_modules = [
            "src/vintage_games/mod.rs",
            "src/vintage_games/games.rs",
            "src/vintage_games/platforms.rs",
            "src/vintage_games/eras.rs",
            "src/vintage_games/graph.rs",
        ];

        for module in &required_modules {
            if !Path::new(module).exists() {
                return false;
            }
        }

        // Check that game covers directory exists and has images
        let covers_dir = Path::new("assets/wizard/game_covers");
        if !covers_dir.exists() {
            return false;
        }

        // Check if there are any images

        std::fs::read_dir(covers_dir)
            .map(|entries| {
                entries.filter_map(|entry| entry.ok()).any(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "jpg" || ext == "jpeg" || ext == "png")
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    }

    /// Validate that all required files and directories were created
    fn validate_generation(&self) -> Result<()> {
        // Check that all module files exist
        let required_modules = [
            "src/vintage_games/mod.rs",
            "src/vintage_games/games.rs",
            "src/vintage_games/platforms.rs",
            "src/vintage_games/eras.rs",
            "src/vintage_games/graph.rs",
        ];

        for module in &required_modules {
            if !Path::new(module).exists() {
                anyhow::bail!(
                    "FATAL: Failed to generate {module}! The wizard will not function without this module."
                );
            }
        }

        // Check that game covers directory exists and has images
        let covers_dir = Path::new("assets/wizard/game_covers");
        if !covers_dir.exists() {
            anyhow::bail!(
                "FATAL: Failed to create game covers directory! The guided mode requires game images."
            );
        }

        let image_count = std::fs::read_dir(covers_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "jpg" || ext == "jpeg" || ext == "png")
                    .unwrap_or(false)
            })
            .count();

        if image_count == 0 {
            anyhow::bail!(
                "FATAL: No game cover images were downloaded! The guided mode requires at least one game image."
            );
        }

        println!(
            "✓ Validated {} modules and {} game cover images",
            required_modules.len(),
            image_count
        );
        Ok(())
    }

    /// Convert enhanced games to JSON format for templates
    fn convert_to_json(
        &self,
        enhanced_games: &[(i32, String, Game)],
    ) -> Result<Vec<serde_json::Value>> {
        let mut timeline_games = Vec::new();

        for (year, genre, game) in enhanced_games {
            let mut game_data = serde_json::Map::new();

            game_data.insert("year".to_string(), serde_json::json!(year));
            game_data.insert("genre".to_string(), serde_json::json!(genre));
            game_data.insert("id".to_string(), serde_json::json!(game.id));
            game_data.insert("guid".to_string(), serde_json::json!(game.guid));
            game_data.insert("name".to_string(), serde_json::json!(game.name));
            game_data.insert("deck".to_string(), serde_json::json!(game.deck));
            game_data.insert(
                "platforms".to_string(),
                serde_json::json!(
                    game.platforms
                        .as_ref()
                        .map(|p| p.iter().map(|platform| &platform.name).collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
            game_data.insert(
                "developers".to_string(),
                serde_json::json!(
                    game.developers
                        .as_ref()
                        .and_then(|d| d.first())
                        .map(|dev| &dev.name)
                ),
            );

            // Include all image URLs
            if let Some(image) = &game.image {
                game_data.insert(
                    "image_icon_url".to_string(),
                    serde_json::json!(image.icon_url),
                );
                game_data.insert(
                    "image_medium_url".to_string(),
                    serde_json::json!(image.medium_url),
                );
                game_data.insert(
                    "image_screen_url".to_string(),
                    serde_json::json!(image.screen_url),
                );
                game_data.insert(
                    "image_screen_large_url".to_string(),
                    serde_json::json!(image.screen_large_url),
                );
                game_data.insert(
                    "image_small_url".to_string(),
                    serde_json::json!(image.small_url),
                );
                game_data.insert(
                    "image_super_url".to_string(),
                    serde_json::json!(image.super_url),
                );
                game_data.insert(
                    "image_thumb_url".to_string(),
                    serde_json::json!(image.thumb_url),
                );
                game_data.insert(
                    "image_tiny_url".to_string(),
                    serde_json::json!(image.tiny_url),
                );
                game_data.insert(
                    "image_original_url".to_string(),
                    serde_json::json!(image.original_url),
                );
            }

            game_data.insert(
                "site_url".to_string(),
                serde_json::json!(game.site_detail_url.as_ref().unwrap_or(&"".to_string())),
            );

            timeline_games.push(serde_json::Value::Object(game_data));
        }

        Ok(timeline_games)
    }

    /// Merge enriched metadata back into the timeline games
    fn merge_enriched_metadata(
        &self,
        timeline_games: &mut [serde_json::Value],
        enriched: &[EnrichedGameMetadata],
    ) -> Result<()> {
        // Create a map of enriched metadata by game ID
        let enriched_map: std::collections::HashMap<u32, &EnrichedGameMetadata> =
            enriched.iter().map(|e| (e.id, e)).collect();

        // Merge enriched data into timeline games
        for game in timeline_games.iter_mut() {
            if let Some(game_obj) = game.as_object_mut()
                && let Some(id) = game_obj.get("id").and_then(|v| v.as_u64())
                && let Some(enriched_data) = enriched_map.get(&(id as u32))
            {
                // Convert enriched metadata to JSON value and merge
                let enriched_json = serde_json::to_value(enriched_data)?;

                // Merge all fields from enriched data into the game object
                if let Some(enriched_obj) = enriched_json.as_object() {
                    // Skip fields we don't want to override from the original
                    let skip_fields = [
                        "id",
                        "name",
                        "year",
                        "original_genre",
                        "platforms",
                        "developer",
                        "deck",
                    ];

                    for (key, value) in enriched_obj {
                        if !skip_fields.contains(&key.as_str()) {
                            game_obj.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
