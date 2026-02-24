//! Game similarity graph pre-computation

use crate::ai_analysis::EnrichedGameMetadata;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use vintage_blending_core::{
    FeatureVector, GameMetadata, STANDARD_GENRES, STANDARD_MECHANICS, get_era_category,
};

pub struct GraphBuilder;

impl GraphBuilder {
    /// Build game graph with AI-enriched metadata for better similarity calculations
    pub fn build_enriched_game_graph(
        timeline_games: &[Value],
        enriched_metadata: &[EnrichedGameMetadata],
    ) -> Result<Value> {
        println!("Pre-computing enriched game similarity graph...");

        // Create a map of enriched metadata by game ID
        let enriched_map: HashMap<u32, &EnrichedGameMetadata> =
            enriched_metadata.iter().map(|e| (e.id, e)).collect();

        let mut metadata_list = Vec::new();

        // Convert JSON games to GameMetadata with enriched data
        for game in timeline_games {
            let game_id = game.get("id").and_then(|v| v.as_u64()).map(|id| id as u32);

            let mut metadata = Self::json_to_metadata(game)?;

            // Enhance with AI analysis if available
            if let Some(id) = game_id
                && let Some(enriched) = enriched_map.get(&id)
            {
                // Use the overall embedding for the feature vector
                if !enriched.overall_embedding.is_empty() {
                    metadata.feature_vector.semantic_embedding =
                        Some(enriched.overall_embedding.clone());
                }

                // Add enriched mechanic tags
                for mechanic in &enriched.mechanics {
                    metadata.mechanic_tags.push(mechanic.name.clone());
                }
                metadata.mechanic_tags.sort();
                metadata.mechanic_tags.dedup();

                // Update genre affinities based on AI analysis
                for (genre_name, weight) in &enriched.genre_blend {
                    metadata
                        .genre_affinities
                        .insert(genre_name.clone(), *weight);
                }

                // Add mood tags as additional features
                for mood in &enriched.mood_tags {
                    metadata.mood_tags.push(mood.clone());
                }
            }

            metadata_list.push(metadata);
        }

        // Calculate similarities with enhanced data
        Self::compute_similarity_graph(metadata_list, timeline_games)
    }

    /// Pre-compute game similarity graph (without AI enrichment)
    pub fn build_game_graph(timeline_games: &[Value]) -> Result<Value> {
        println!("Pre-computing game similarity graph...");

        let mut metadata_list = Vec::new();

        // Convert JSON games to GameMetadata
        for game in timeline_games {
            let metadata = Self::json_to_metadata(game)?;
            metadata_list.push(metadata);
        }

        // Handle empty game list
        if metadata_list.is_empty() {
            println!("  No games to build graph from");
            return Ok(serde_json::json!({
                "node_count": 0,
                "edge_count": 0,
                "average_similarity": 0.0,
                "top_hubs": [],
                "edges": [],
                "metadata": [],
            }));
        }

        // Calculate similarities between all games
        let mut similarities: Vec<Vec<f32>> =
            vec![vec![0.0; metadata_list.len()]; metadata_list.len()];
        let mut edge_count = 0;
        let total_comparisons = if metadata_list.len() > 1 {
            (metadata_list.len() * (metadata_list.len() - 1)) / 2
        } else {
            0
        };
        let mut comparisons_done = 0;

        for i in 0..metadata_list.len() {
            for j in (i + 1)..metadata_list.len() {
                let similarity = metadata_list[i]
                    .feature_vector
                    .similarity(&metadata_list[j].feature_vector);
                similarities[i][j] = similarity;
                similarities[j][i] = similarity;

                // Count edges with meaningful similarity
                if similarity > 0.1 {
                    edge_count += 1;
                }

                comparisons_done += 1;
                if comparisons_done % 100 == 0 {
                    println!("  Progress: {comparisons_done}/{total_comparisons} comparisons");
                }
            }
        }

        println!(
            "  Created graph with {} nodes and {} edges",
            metadata_list.len(),
            edge_count
        );

        // Calculate graph statistics
        let avg_similarity = if edge_count > 0 {
            let total_similarity = similarities
                .iter()
                .enumerate()
                .flat_map(|(i, row)| row.iter().skip(i + 1).filter(|&&sim| sim > 0.1))
                .sum::<f32>();
            total_similarity / edge_count as f32
        } else {
            0.0
        };

        // Find most connected games (hubs)
        let mut connection_counts: Vec<(usize, u32)> = similarities
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let connections = row.iter().filter(|&&sim| sim > 0.1).count() as u32;
                (idx, connections)
            })
            .collect();

        connection_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        let top_hubs: Vec<Value> = connection_counts
            .iter()
            .take(5)
            .map(|&(idx, count)| {
                let game = &timeline_games[idx];
                serde_json::json!({
                    "name": game.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    "year": game.get("year").and_then(|v| v.as_i64()).unwrap_or(0),
                    "connections": count
                })
            })
            .collect();

        // Convert similarities to a format suitable for the template
        let mut similarity_edges = Vec::new();
        for i in 0..metadata_list.len() {
            for j in (i + 1)..metadata_list.len() {
                let sim = similarities[i][j];
                if sim > 0.1 {
                    // Extract numeric IDs from game_id strings
                    let game1_id = metadata_list[i].game_id.parse::<u32>().unwrap_or(0);
                    let game2_id = metadata_list[j].game_id.parse::<u32>().unwrap_or(0);

                    similarity_edges.push(serde_json::json!({
                        "game1_id": game1_id,
                        "game2_id": game2_id,
                        "similarity": sim
                    }));
                }
            }
        }

        // Get hub game IDs (just the numeric IDs)
        let hub_game_ids: Vec<u32> = connection_counts
            .iter()
            .take(10) // Top 10 hubs
            .filter_map(|&(idx, _)| metadata_list[idx].game_id.parse::<u32>().ok())
            .collect();

        // Serialize graph data for use at runtime
        let graph_data = serde_json::json!({
            "node_count": metadata_list.len(),
            "edge_count": edge_count,
            "average_similarity": avg_similarity,
            "top_hubs": top_hubs,
            "hub_game_ids": hub_game_ids,
            "edges": similarity_edges,
            "metadata": metadata_list.into_iter().map(|m| {
                serde_json::json!({
                    "game_id": m.game_id,
                    "name": m.name,
                    "year": m.year,
                    "era_category": m.era_category,
                    "mechanic_tags": m.mechanic_tags,
                    "genre_affinities": m.genre_affinities,
                })
            }).collect::<Vec<_>>(),
        });

        Ok(graph_data)
    }

    /// Convert JSON game data to GameMetadata for similarity calculation
    fn json_to_metadata(game: &Value) -> Result<GameMetadata> {
        let game_id = game
            .get("id")
            .and_then(|v| v.as_u64())
            .map(|id| id.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let name = game
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let year = game.get("year").and_then(|v| v.as_i64()).unwrap_or(1980) as u32;

        let genre = game
            .get("genre")
            .and_then(|v| v.as_str())
            .unwrap_or("Action")
            .to_string();

        let platforms: Vec<String> = game
            .get("platforms")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| p.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        // Build feature vector
        let mut feature_vector = FeatureVector::new();

        // Set genre weights
        feature_vector.genre_weights = vec![0.0; STANDARD_GENRES.len()];
        for (i, &standard_genre) in STANDARD_GENRES.iter().enumerate() {
            if genre.eq_ignore_ascii_case(standard_genre) {
                feature_vector.genre_weights[i] = 1.0;

                // Add sub-genre weights
                match standard_genre {
                    "Action" => {
                        if let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == "Platform") {
                            feature_vector.genre_weights[idx] = 0.3;
                        }
                    }
                    "RPG" => {
                        if let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == "Adventure") {
                            feature_vector.genre_weights[idx] = 0.5;
                        }
                    }
                    _ => {}
                }
                break;
            }
        }

        // Set mechanic flags based on genre
        feature_vector.mechanic_flags = vec![false; STANDARD_MECHANICS.len()];
        Self::set_mechanics_from_genre(&genre, &mut feature_vector.mechanic_flags);

        // Determine platform generation (use platforms if available, otherwise use year)
        feature_vector.platform_generation =
            Self::platform_generation_from_platforms(&platforms, year);

        // Calculate complexity based on genre and year
        feature_vector.complexity = Self::calculate_complexity(&genre, year);

        // Calculate action/strategy balance
        feature_vector.action_strategy_balance = Self::calculate_action_strategy_balance(&genre);

        // Calculate single/multi balance
        feature_vector.single_multi_balance = Self::calculate_single_multi_balance(&genre);

        // Build genre affinities
        let mut genre_affinities = HashMap::new();
        genre_affinities.insert(genre.clone(), 1.0);

        // Add related genres
        match genre.as_str() {
            "Action" => {
                genre_affinities.insert("Platform".to_string(), 0.5);
                genre_affinities.insert("Shooter".to_string(), 0.6);
            }
            "RPG" => {
                genre_affinities.insert("Adventure".to_string(), 0.7);
                genre_affinities.insert("Strategy".to_string(), 0.4);
            }
            "Platform" => {
                genre_affinities.insert("Action".to_string(), 0.6);
                genre_affinities.insert("Puzzle".to_string(), 0.3);
            }
            _ => {}
        }

        // Extract mechanic tags
        let mechanic_tags = Self::extract_mechanic_tags(&genre);

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

    fn set_mechanics_from_genre(genre: &str, mechanic_flags: &mut [bool]) {
        let set_mechanic = |name: &str, flags: &mut [bool]| {
            if let Some(idx) = STANDARD_MECHANICS.iter().position(|&m| m == name) {
                flags[idx] = true;
            }
        };

        match genre {
            "Action" => {
                set_mechanic("Combat", mechanic_flags);
                set_mechanic("Real-Time", mechanic_flags);
            }
            "RPG" => {
                set_mechanic("Character Progression", mechanic_flags);
                set_mechanic("Exploration", mechanic_flags);
                set_mechanic("Story Choices", mechanic_flags);
            }
            "Strategy" => {
                set_mechanic("Resource Management", mechanic_flags);
                set_mechanic("Turn-Based", mechanic_flags);
            }
            "Platform" => {
                set_mechanic("Platform Jumping", mechanic_flags);
                set_mechanic("Collection", mechanic_flags);
            }
            "Puzzle" => {
                set_mechanic("Puzzle Solving", mechanic_flags);
            }
            "Shooter" => {
                set_mechanic("Combat", mechanic_flags);
                set_mechanic("Real-Time", mechanic_flags);
            }
            _ => {}
        }
    }

    fn platform_generation_from_platforms(platforms: &[String], year: u32) -> u8 {
        // Check platforms first for more accurate generation determination
        // Focus on 2D-centric platforms from 1980-1995
        for platform in platforms {
            // Generation 1: Early 80s arcade and home consoles
            if platform.contains("Arcade")
                || platform.contains("Atari 2600")
                || platform.contains("Intellivision")
            {
                return 1;
            }
            // Generation 2: 8-bit era
            else if platform.contains("NES")
                || platform.contains("Master System")
                || platform.contains("C64")
                || platform.contains("MSX")
                || platform.contains("Game Boy")
            {
                return 2;
            }
            // Generation 3: 16-bit golden age
            else if platform.contains("SNES")
                || platform.contains("Genesis")
                || platform.contains("TurboGrafx")
                || platform.contains("Neo Geo")
            {
                return 3;
            }
            // Generation 4: Late 16-bit era (still 2D focused)
            else if platform.contains("Game Gear")
                || platform.contains("Lynx")
                || platform.contains("Jaguar")
                || platform.contains("32X")
            {
                return 4;
            }
        }

        // Fallback to year-based if no known platforms
        Self::platform_generation_from_year(year)
    }

    fn platform_generation_from_year(year: u32) -> u8 {
        match year {
            1980..=1983 => 1,
            1984..=1987 => 2,
            1988..=1991 => 3,
            1992..=1995 => 4,
            _ => 3,
        }
    }

    fn calculate_complexity(genre: &str, year: u32) -> f32 {
        let base_complexity = match genre {
            "Strategy" | "RPG" | "Simulation" => 0.8,
            "Adventure" | "Fighting" => 0.6,
            "Action" | "Platform" | "Shooter" => 0.4,
            "Puzzle" | "Sports" => 0.3,
            _ => 0.5,
        };

        let era_modifier = ((year as f32 - 1980.0) / 15.0).min(0.2);
        (base_complexity + era_modifier).min(1.0)
    }

    fn calculate_action_strategy_balance(genre: &str) -> f32 {
        match genre {
            "Action" | "Shooter" | "Platform" => -0.8,
            "Fighting" | "Racing" => -0.6,
            "Sports" => -0.4,
            "Adventure" => 0.0,
            "RPG" => 0.2,
            "Puzzle" => 0.4,
            "Simulation" => 0.6,
            "Strategy" => 0.8,
            _ => 0.0,
        }
    }

    fn calculate_single_multi_balance(genre: &str) -> f32 {
        match genre {
            "Fighting" | "Sports" => 0.8,
            "Racing" => 0.4,
            "Action" | "Platform" => -0.4,
            "RPG" | "Adventure" | "Strategy" => -0.8,
            _ => -0.5,
        }
    }

    fn extract_mechanic_tags(genre: &str) -> Vec<String> {
        let mut tags = Vec::new();

        match genre {
            "Action" => tags.extend_from_slice(&["Combat".to_string(), "Real-Time".to_string()]),
            "RPG" => tags.extend_from_slice(&[
                "Character Progression".to_string(),
                "Exploration".to_string(),
            ]),
            "Strategy" => tags
                .extend_from_slice(&["Resource Management".to_string(), "Turn-Based".to_string()]),
            "Platform" => {
                tags.extend_from_slice(&["Platform Jumping".to_string(), "Collection".to_string()])
            }
            _ => {}
        }

        tags
    }

    /// Common similarity computation logic
    fn compute_similarity_graph(
        metadata_list: Vec<GameMetadata>,
        timeline_games: &[Value],
    ) -> Result<Value> {
        // Handle empty game list
        if metadata_list.is_empty() {
            println!("  No games to build graph from");
            return Ok(serde_json::json!({
                "node_count": 0,
                "edge_count": 0,
                "average_similarity": 0.0,
                "top_hubs": [],
                "edges": [],
                "metadata": [],
            }));
        }

        // Calculate similarities between all games
        let mut similarities: Vec<Vec<f32>> =
            vec![vec![0.0; metadata_list.len()]; metadata_list.len()];
        let mut edge_count = 0;
        let total_comparisons = if metadata_list.len() > 1 {
            (metadata_list.len() * (metadata_list.len() - 1)) / 2
        } else {
            0
        };
        let mut comparisons_done = 0;

        for i in 0..metadata_list.len() {
            for j in (i + 1)..metadata_list.len() {
                let similarity = metadata_list[i]
                    .feature_vector
                    .similarity(&metadata_list[j].feature_vector);
                similarities[i][j] = similarity;
                similarities[j][i] = similarity;

                // Count edges with meaningful similarity
                if similarity > 0.1 {
                    edge_count += 1;
                }

                comparisons_done += 1;
                if comparisons_done % 100 == 0 {
                    println!("  Progress: {comparisons_done}/{total_comparisons} comparisons");
                }
            }
        }

        println!(
            "  Created graph with {} nodes and {} edges",
            metadata_list.len(),
            edge_count
        );

        // Calculate graph statistics
        let avg_similarity = if edge_count > 0 {
            let total_similarity = similarities
                .iter()
                .enumerate()
                .flat_map(|(i, row)| row.iter().skip(i + 1).filter(|&&sim| sim > 0.1))
                .sum::<f32>();
            total_similarity / edge_count as f32
        } else {
            0.0
        };

        // Find most connected games (hubs)
        let mut connection_counts: Vec<(usize, u32)> = similarities
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let connections = row.iter().filter(|&&sim| sim > 0.1).count() as u32;
                (idx, connections)
            })
            .collect();

        connection_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        let top_hubs: Vec<Value> = connection_counts
            .iter()
            .take(5)
            .map(|&(idx, count)| {
                let game = &timeline_games[idx];
                serde_json::json!({
                    "name": game.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    "year": game.get("year").and_then(|v| v.as_i64()).unwrap_or(0),
                    "connections": count
                })
            })
            .collect();

        // Convert similarities to a format suitable for the template
        let mut similarity_edges = Vec::new();
        for i in 0..metadata_list.len() {
            for j in (i + 1)..metadata_list.len() {
                let sim = similarities[i][j];
                if sim > 0.1 {
                    // Extract numeric IDs from game_id strings
                    let game1_id = metadata_list[i].game_id.parse::<u32>().unwrap_or(0);
                    let game2_id = metadata_list[j].game_id.parse::<u32>().unwrap_or(0);

                    similarity_edges.push(serde_json::json!({
                        "game1_id": game1_id,
                        "game2_id": game2_id,
                        "similarity": sim
                    }));
                }
            }
        }

        // Get hub game IDs (just the numeric IDs)
        let hub_game_ids: Vec<u32> = connection_counts
            .iter()
            .take(10) // Top 10 hubs
            .filter_map(|&(idx, _)| metadata_list[idx].game_id.parse::<u32>().ok())
            .collect();

        // Serialize graph data for use at runtime
        let graph_data = serde_json::json!({
            "node_count": metadata_list.len(),
            "edge_count": edge_count,
            "average_similarity": avg_similarity,
            "top_hubs": top_hubs,
            "hub_game_ids": hub_game_ids,
            "edges": similarity_edges,
            "metadata": metadata_list.into_iter().map(|m| {
                serde_json::json!({
                    "game_id": m.game_id,
                    "name": m.name,
                    "year": m.year,
                    "era_category": m.era_category,
                    "mechanic_tags": m.mechanic_tags,
                    "genre_affinities": m.genre_affinities,
                    "mood_tags": m.mood_tags,
                    "overall_embedding": m.feature_vector.semantic_embedding,
                })
            }).collect::<Vec<_>>(),
        });

        Ok(graph_data)
    }
}
