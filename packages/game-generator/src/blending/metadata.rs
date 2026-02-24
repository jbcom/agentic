//! Metadata builder for pre-computing game feature vectors
//!
//! Used by build.rs to generate metadata at compile time

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::types::{
    FeatureVector, GameMetadata, STANDARD_GENRES, STANDARD_MECHANICS, get_era_category,
};

/// Builder for creating game metadata from raw data
pub struct MetadataBuilder {
    genre_indices: HashMap<String, usize>,
    mechanic_indices: HashMap<String, usize>,
}

impl MetadataBuilder {
    pub fn new() -> Self {
        let mut genre_indices = HashMap::new();
        for (i, genre) in STANDARD_GENRES.iter().enumerate() {
            genre_indices.insert(genre.to_lowercase(), i);
        }

        let mut mechanic_indices = HashMap::new();
        for (i, mechanic) in STANDARD_MECHANICS.iter().enumerate() {
            mechanic_indices.insert(mechanic.to_lowercase(), i);
        }

        Self {
            genre_indices,
            mechanic_indices,
        }
    }

    /// Build metadata from a JSON game object
    pub fn build_from_json(&self, game_data: &Value) -> Result<GameMetadata> {
        let game_id = game_data["guid"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing game guid"))?
            .to_string();

        let name = game_data["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing game name"))?
            .to_string();

        let year = game_data["year"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Missing game year"))? as u32;

        // Build feature vector
        let feature_vector = self.build_feature_vector(game_data)?;

        // Extract mechanic tags
        let mechanic_tags = self.extract_mechanic_tags(game_data);

        // Compute genre affinities
        let genre_affinities = self.compute_genre_affinities(game_data);

        Ok(GameMetadata {
            game_id,
            name,
            year,
            feature_vector,
            common_pairings: HashMap::new(), // Will be populated later
            genre_affinities,
            mechanic_tags,
            mood_tags: Vec::new(), // Will be populated by AI analysis
            era_category: get_era_category(year),
        })
    }

    /// Build feature vector from game data
    fn build_feature_vector(&self, game_data: &Value) -> Result<FeatureVector> {
        // Initialize genre weights
        let mut genre_weights = vec![0.0; STANDARD_GENRES.len()];

        // Extract genre from game data
        if let Some(genre_str) = game_data["genre"].as_str() {
            let genre_lower = genre_str.to_lowercase();
            if let Some(&idx) = self.genre_indices.get(&genre_lower) {
                genre_weights[idx] = 1.0;

                // Add sub-genre weights based on common patterns (case-insensitive)
                match genre_lower.as_str() {
                    "action" => {
                        if let Some(&idx) = self.genre_indices.get("platform") {
                            genre_weights[idx] = 0.3;
                        }
                    }
                    "rpg" => {
                        if let Some(&idx) = self.genre_indices.get("adventure") {
                            genre_weights[idx] = 0.5;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Initialize mechanic flags
        let mut mechanic_flags = vec![false; STANDARD_MECHANICS.len()];

        // Infer mechanics from genre and description
        if let Some(genre) = game_data["genre"].as_str() {
            self.infer_mechanics_from_genre(genre, &mut mechanic_flags);
        }

        // Determine platform generation
        let platform_generation = self.determine_platform_generation(game_data);

        // Calculate complexity based on genre and year
        let complexity = self.calculate_complexity(game_data);

        // Determine action/strategy balance
        let action_strategy_balance = self.calculate_action_strategy_balance(game_data);

        // Determine single/multiplayer balance
        let single_multi_balance = self.calculate_single_multi_balance(game_data);

        Ok(FeatureVector {
            genre_weights,
            mechanic_flags,
            platform_generation,
            complexity,
            action_strategy_balance,
            single_multi_balance,
            semantic_embedding: None, // Will be populated by AI analysis
        })
    }

    /// Infer mechanics from genre (case-insensitive)
    fn infer_mechanics_from_genre(&self, genre: &str, mechanic_flags: &mut [bool]) {
        match genre.to_lowercase().as_str() {
            "action" => {
                self.set_mechanic_flag("Combat", mechanic_flags, true);
                self.set_mechanic_flag("Real-Time", mechanic_flags, true);
            }
            "rpg" => {
                self.set_mechanic_flag("Character Progression", mechanic_flags, true);
                self.set_mechanic_flag("Exploration", mechanic_flags, true);
                self.set_mechanic_flag("Story Choices", mechanic_flags, true);
            }
            "strategy" => {
                self.set_mechanic_flag("Resource Management", mechanic_flags, true);
                self.set_mechanic_flag("Turn-Based", mechanic_flags, true);
            }
            "platform" => {
                self.set_mechanic_flag("Platform Jumping", mechanic_flags, true);
                self.set_mechanic_flag("Collection", mechanic_flags, true);
            }
            "puzzle" => {
                self.set_mechanic_flag("Puzzle Solving", mechanic_flags, true);
            }
            "shooter" => {
                self.set_mechanic_flag("Combat", mechanic_flags, true);
                self.set_mechanic_flag("Real-Time", mechanic_flags, true);
            }
            _ => {}
        }
    }

    /// Set a mechanic flag by name
    fn set_mechanic_flag(&self, mechanic: &str, flags: &mut [bool], value: bool) {
        if let Some(&idx) = self.mechanic_indices.get(&mechanic.to_lowercase()) {
            flags[idx] = value;
        }
    }

    /// Determine platform generation from platforms list
    fn determine_platform_generation(&self, game_data: &Value) -> u8 {
        if let Some(platforms) = game_data["platforms"].as_array() {
            for platform in platforms {
                if let Some(platform_str) = platform.as_str() {
                    match platform_str {
                        p if p.contains("Arcade") => return 1,
                        p if p.contains("NES") || p.contains("Master System") => return 2,
                        p if p.contains("SNES") || p.contains("Genesis") => return 3,
                        p if p.contains("PlayStation") || p.contains("Saturn") => return 4,
                        _ => {}
                    }
                }
            }
        }

        // Default based on year
        let year = game_data["year"].as_u64().unwrap_or(1985) as u32;
        match year {
            1980..=1983 => 1,
            1984..=1987 => 2,
            1988..=1991 => 3,
            1992..=1995 => 4,
            _ => 3,
        }
    }

    /// Calculate game complexity (case-insensitive genre matching)
    fn calculate_complexity(&self, game_data: &Value) -> f32 {
        let genre = game_data["genre"]
            .as_str()
            .unwrap_or("Action")
            .to_lowercase();
        let year = game_data["year"].as_u64().unwrap_or(1985) as u32;

        // Base complexity from genre
        let base_complexity = match genre.as_str() {
            "strategy" | "rpg" | "simulation" => 0.8,
            "adventure" | "fighting" => 0.6,
            "action" | "platform" | "shooter" => 0.4,
            "puzzle" | "sports" => 0.3,
            _ => 0.5,
        };

        // Adjust for era (games got more complex over time)
        let era_modifier = ((year as f32 - 1980.0) / 15.0).min(0.2);

        (base_complexity + era_modifier).min(1.0)
    }

    /// Calculate action vs strategy balance (case-insensitive genre matching)
    fn calculate_action_strategy_balance(&self, game_data: &Value) -> f32 {
        let genre = game_data["genre"]
            .as_str()
            .unwrap_or("Action")
            .to_lowercase();

        match genre.as_str() {
            "action" | "shooter" | "platform" => -0.8,
            "fighting" | "racing" => -0.6,
            "sports" => -0.4,
            "adventure" => 0.0,
            "rpg" => 0.2,
            "puzzle" => 0.4,
            "simulation" => 0.6,
            "strategy" => 0.8,
            _ => 0.0,
        }
    }

    /// Calculate single vs multiplayer balance (case-insensitive genre matching)
    fn calculate_single_multi_balance(&self, game_data: &Value) -> f32 {
        let genre = game_data["genre"]
            .as_str()
            .unwrap_or("Action")
            .to_lowercase();

        // Most vintage games were single-player focused
        match genre.as_str() {
            "fighting" | "sports" => 0.8, // Multiplayer-focused
            "racing" => 0.4,
            "action" | "platform" => -0.4,
            "rpg" | "adventure" | "strategy" => -0.8, // Single-player focused
            _ => -0.5,
        }
    }

    /// Extract mechanic tags from game data (case-insensitive genre matching)
    fn extract_mechanic_tags(&self, game_data: &Value) -> Vec<String> {
        let mut tags = Vec::new();

        if let Some(genre) = game_data["genre"].as_str() {
            match genre.to_lowercase().as_str() {
                "action" => {
                    tags.extend_from_slice(&["Combat".to_string(), "Real-Time".to_string()])
                }
                "rpg" => tags.extend_from_slice(&[
                    "Character Progression".to_string(),
                    "Exploration".to_string(),
                ]),
                "strategy" => tags.extend_from_slice(&[
                    "Resource Management".to_string(),
                    "Turn-Based".to_string(),
                ]),
                "platform" => tags
                    .extend_from_slice(&["Platform Jumping".to_string(), "Collection".to_string()]),
                _ => {}
            }
        }

        tags
    }

    /// Compute genre affinities (case-insensitive genre matching)
    fn compute_genre_affinities(&self, game_data: &Value) -> HashMap<String, f32> {
        let mut affinities = HashMap::new();

        if let Some(genre) = game_data["genre"].as_str() {
            affinities.insert(genre.to_string(), 1.0);

            // Add related genres (case-insensitive matching)
            match genre.to_lowercase().as_str() {
                "action" => {
                    affinities.insert("Platform".to_string(), 0.5);
                    affinities.insert("Shooter".to_string(), 0.6);
                }
                "rpg" => {
                    affinities.insert("Adventure".to_string(), 0.7);
                    affinities.insert("Strategy".to_string(), 0.4);
                }
                "platform" => {
                    affinities.insert("Action".to_string(), 0.6);
                    affinities.insert("Puzzle".to_string(), 0.3);
                }
                _ => {}
            }
        }

        affinities
    }

    /// Update common pairings based on analysis
    pub fn update_common_pairings(&self, metadata: &mut HashMap<String, GameMetadata>) {
        // This would be called after all metadata is built to populate common pairings
        // For now, we'll add some hardcoded common pairings

        // First, collect all the pairings
        let mut all_pairings: HashMap<String, Vec<(String, f32)>> = HashMap::new();

        // Clone the metadata keys to avoid borrowing issues
        let game_ids: Vec<String> = metadata.keys().cloned().collect();

        for game_id in &game_ids {
            let mut pairings = Vec::new();

            if let Some(game_meta) = metadata.get(game_id) {
                for other_id in &game_ids {
                    if game_id != other_id
                        && let Some(other_meta) = metadata.get(other_id)
                    {
                        let compatibility = game_meta
                            .feature_vector
                            .similarity(&other_meta.feature_vector);

                        // Only store high compatibility pairings
                        if compatibility > 0.7 {
                            pairings.push((other_id.clone(), compatibility));
                        }
                    }
                }
            }

            // Keep only top 10 pairings
            pairings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            pairings.truncate(10);

            all_pairings.insert(game_id.clone(), pairings);
        }

        // Now update the metadata with the collected pairings
        for (game_id, pairings) in all_pairings {
            if let Some(game_meta) = metadata.get_mut(&game_id) {
                game_meta.common_pairings = pairings.into_iter().collect();
            }
        }
    }
}

impl Default for MetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}
