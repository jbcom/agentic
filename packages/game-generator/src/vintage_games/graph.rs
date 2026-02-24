//! Graph building for vintage_blending_core integration

use super::eras::{Era, era_for_year};
use super::games::{TIMELINE_GAMES, TimelineGame};

use petgraph::graph::Graph;
use std::collections::HashMap;
use vintage_blending_core::{GameMetadata, similarity::SimilarityEngine};

/// Node representation for the game graph
#[derive(Debug, Clone)]
pub struct GameNode {
    pub game: &'static TimelineGame,
    pub metadata: GameMetadata,
}

/// Build a weighted graph of all games for blending operations
pub fn build_game_graph() -> Graph<GameNode, f32> {
    let mut graph = Graph::new();
    let mut nodes = Vec::new();

    // First pass: Create nodes for all games
    for game in TIMELINE_GAMES.iter() {
        let metadata = game_to_metadata(game);
        let node = GameNode { game, metadata };
        nodes.push(node);
    }

    // Add all nodes to the graph
    let node_ids: Vec<_> = nodes
        .iter()
        .map(|node| graph.add_node(node.clone()))
        .collect();

    // Second pass: Calculate similarities and add edges
    for (i, node_i) in nodes.iter().enumerate() {
        for (j, node_j) in nodes.iter().enumerate().skip(i + 1) {
            let similarity = calculate_similarity(&node_i.metadata, &node_j.metadata);

            // Only add edges for games with meaningful similarity (> 0.1)
            if similarity > 0.1 {
                graph.add_edge(node_ids[i], node_ids[j], similarity);
            }
        }
    }

    graph
}

/// Convert a TimelineGame to GameMetadata for similarity calculations
fn game_to_metadata(game: &TimelineGame) -> GameMetadata {
    use vintage_blending_core::types::{STANDARD_GENRES, STANDARD_MECHANICS, get_era_category};

    // Create feature vector
    let mut genre_weights = vec![0.0; STANDARD_GENRES.len()];
    let mut mechanic_flags = vec![false; STANDARD_MECHANICS.len()];
    let mut mechanic_tags = Vec::new();
    let mut mood_tags = Vec::new();

    // Set genre weight
    if let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == game.genre) {
        genre_weights[idx] = 1.0;
    }

    // Determine platform generation
    let platform_generation = match game.year {
        1980..=1983 => 1,
        1984..=1987 => 2,
        1988..=1991 => 3,
        1992..=1995 => 4,
        _ => 3,
    };

    // Calculate complexity
    let complexity = match game.genre {
        "Strategy" | "RPG" | "Simulation" => 0.8,
        "Adventure" | "Fighting" => 0.6,
        "Action" | "Platform" | "Shooter" => 0.4,
        "Puzzle" | "Sports" => 0.3,
        _ => 0.5,
    };

    // Calculate action vs strategy balance
    let action_strategy_balance = match game.genre {
        "Action" | "Shooter" | "Platform" => -0.8,
        "Fighting" | "Racing" => -0.6,
        "Sports" => -0.4,
        "Adventure" => 0.0,
        "RPG" => 0.2,
        "Puzzle" => 0.4,
        "Simulation" => 0.6,
        "Strategy" => 0.8,
        _ => 0.0,
    };

    // Calculate single vs multiplayer balance
    let single_multi_balance = match game.genre {
        "Fighting" | "Sports" => 0.8,
        "Racing" => 0.4,
        "Action" | "Platform" => -0.4,
        "RPG" | "Adventure" | "Strategy" => -0.8,
        _ => -0.5,
    };

    // Infer mechanics and mood from genre and era
    if let Some(era) = era_for_year(game.year) {
        match era {
            Era::ArcadeGoldenAge => {
                mechanic_tags.push("High Score".to_string());
                mechanic_tags.push("Lives System".to_string());
                mood_tags.push("Arcade".to_string());
                mood_tags.push("Fast-paced".to_string());
            }
            Era::EarlyConsole => {
                if game.genre.contains("RPG") {
                    mechanic_tags.push("Save System".to_string());
                    mechanic_tags.push("Character Progression".to_string());
                }
                mood_tags.push("Home Console".to_string());
            }
            Era::Late8BitEarly16 => {
                mechanic_tags.push("Pixel Perfect".to_string());
                mood_tags.push("Retro".to_string());
            }
            Era::Peak16Bit => {
                mechanic_tags.push("Advanced Graphics".to_string());
                mood_tags.push("16-bit".to_string());
            }
        }
    }

    // Set mechanic flags based on genre
    match game.genre {
        "Action" => {
            if let Some(idx) = STANDARD_MECHANICS.iter().position(|&m| m == "Combat") {
                mechanic_flags[idx] = true;
            }
            if let Some(idx) = STANDARD_MECHANICS.iter().position(|&m| m == "Real-Time") {
                mechanic_flags[idx] = true;
            }
            mechanic_tags.push("Real-time Combat".to_string());
        }
        "RPG" | "Role-Playing" => {
            if let Some(idx) = STANDARD_MECHANICS
                .iter()
                .position(|&m| m == "Character Progression")
            {
                mechanic_flags[idx] = true;
            }
            if let Some(idx) = STANDARD_MECHANICS.iter().position(|&m| m == "Exploration") {
                mechanic_flags[idx] = true;
            }
            mechanic_tags.extend_from_slice(&[
                "Stats".to_string(),
                "Inventory".to_string(),
                "Quests".to_string(),
            ]);
        }
        "Strategy" => {
            if let Some(idx) = STANDARD_MECHANICS.iter().position(|&m| m == "Turn-Based") {
                mechanic_flags[idx] = true;
            }
            if let Some(idx) = STANDARD_MECHANICS
                .iter()
                .position(|&m| m == "Resource Management")
            {
                mechanic_flags[idx] = true;
            }
        }
        "Platform" => {
            if let Some(idx) = STANDARD_MECHANICS
                .iter()
                .position(|&m| m == "Platform Jumping")
            {
                mechanic_flags[idx] = true;
            }
            if let Some(idx) = STANDARD_MECHANICS.iter().position(|&m| m == "Collection") {
                mechanic_flags[idx] = true;
            }
        }
        "Puzzle" => {
            if let Some(idx) = STANDARD_MECHANICS
                .iter()
                .position(|&m| m == "Puzzle Solving")
            {
                mechanic_flags[idx] = true;
            }
            mechanic_tags.push("Pattern Recognition".to_string());
        }
        _ => {}
    }

    // Create genre affinities
    let mut genre_affinities = HashMap::new();
    genre_affinities.insert(game.genre.to_string(), 1.0);

    // Add related genres
    match game.genre {
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

    let feature_vector = vintage_blending_core::FeatureVector {
        genre_weights,
        mechanic_flags,
        platform_generation,
        complexity,
        action_strategy_balance,
        single_multi_balance,
        semantic_embedding: None,
    };

    GameMetadata {
        game_id: game.id.to_string(),
        name: game.name.to_string(),
        year: game.year as u32,
        feature_vector,
        common_pairings: HashMap::new(), // Will be populated later
        genre_affinities,
        mechanic_tags,
        era_category: get_era_category(game.year as u32),
        mood_tags,
    }
}

/// Calculate similarity between two games based on their metadata
fn calculate_similarity(game1: &GameMetadata, game2: &GameMetadata) -> f32 {
    let sim_engine = SimilarityEngine::new();
    sim_engine.compute_similarity(game1, game2)
}

/// Find the most similar games to a given game
pub fn find_similar_games(game_id: u32, count: usize) -> Vec<(&'static TimelineGame, f32)> {
    let target_game = match TIMELINE_GAMES.iter().find(|g| g.id == game_id) {
        Some(game) => game,
        None => return Vec::new(),
    };

    let target_metadata = game_to_metadata(target_game);
    let sim_engine = SimilarityEngine::new();

    let mut similarities: Vec<(&'static TimelineGame, f32)> = TIMELINE_GAMES
        .iter()
        .filter(|g| g.id != game_id)
        .map(|game| {
            let metadata = game_to_metadata(game);
            let score = sim_engine.compute_similarity(&target_metadata, &metadata);
            (game, score)
        })
        .collect();

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    similarities.truncate(count);

    similarities
}

/// Create a subgraph containing only games from specific eras
pub fn build_era_subgraph(eras: &[Era]) -> Graph<GameNode, f32> {
    let mut graph = Graph::new();
    let mut nodes = Vec::new();

    // Filter games by era
    let era_games: Vec<_> = TIMELINE_GAMES
        .iter()
        .filter(|game| {
            if let Some(game_era) = era_for_year(game.year) {
                eras.contains(&game_era)
            } else {
                false
            }
        })
        .collect();

    // Create nodes
    for game in era_games.iter() {
        let metadata = game_to_metadata(game);
        let node = GameNode { game, metadata };
        nodes.push(node);
    }

    // Add nodes and edges
    let node_ids: Vec<_> = nodes
        .iter()
        .map(|node| graph.add_node(node.clone()))
        .collect();

    for (i, node_i) in nodes.iter().enumerate() {
        for (j, node_j) in nodes.iter().enumerate().skip(i + 1) {
            let similarity = calculate_similarity(&node_i.metadata, &node_j.metadata);
            if similarity > 0.1 {
                graph.add_edge(node_ids[i], node_ids[j], similarity);
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_building() {
        let graph = build_game_graph();
        assert!(graph.node_count() > 0);
    }

    #[test]
    fn test_metadata_conversion() {
        if let Some(game) = TIMELINE_GAMES.first() {
            let metadata = game_to_metadata(game);
            assert_eq!(metadata.name, game.name);
            assert_eq!(metadata.game_id, game.id.to_string());
            assert_eq!(metadata.year, game.year as u32);
            assert!(!metadata.genre_affinities.is_empty());
        }
    }
}
