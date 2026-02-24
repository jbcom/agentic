use crate::wizard::steps::guided::types::{BlendResult, GuidedModeState};
use std::collections::{HashMap, HashSet};
use vintage_blending_core::{
    CompatibilityEdge,
    graph::{BlendPath, GameGraph},
    similarity::SimilarityEngine,
};

use super::analysis::{analyze_conflicts, analyze_synergies, generate_recommendations};
use super::metadata::{build_game_metadata, determine_art_styles};

/// Create a blend from selected games using the blending core
pub fn create_blend(state: &mut GuidedModeState) {
    let selected_games: Vec<_> = state.selected_games.values().cloned().collect();

    if selected_games.len() < 2 {
        return;
    }

    // Build metadata for each game
    let mut game_metadata = HashMap::new();

    for game in &selected_games {
        let metadata = build_game_metadata(game);
        game_metadata.insert(game.id.to_string(), metadata);
    }

    // Create game graph
    let graph = GameGraph::new(game_metadata.clone()).unwrap();

    // Calculate compatibility between all pairs
    let engine = SimilarityEngine::new();
    let mut edges = Vec::new();

    for i in 0..selected_games.len() {
        for j in i + 1..selected_games.len() {
            let game1 = &selected_games[i];
            let game2 = &selected_games[j];
            let meta1 = &game_metadata[&game1.id.to_string()];
            let meta2 = &game_metadata[&game2.id.to_string()];

            let compatibility = engine.compute_similarity(meta1, meta2);

            edges.push(CompatibilityEdge {
                weight: compatibility,
                synergies: analyze_synergies(game1, game2, meta1, meta2),
                conflicts: analyze_conflicts(game1, game2, meta1, meta2),
            });
        }
    }

    // Find optimal blend path
    let game_ids: Vec<String> = selected_games.iter().map(|g| g.id.to_string()).collect();
    let blend_path = graph.find_blend_path(&game_ids).unwrap_or_else(|_| {
        // Fallback: create a simple path through all games
        BlendPath {
            games: game_ids.clone(),
            total_compatibility: edges.iter().map(|e| e.weight).sum(),
            synergies: edges.iter().flat_map(|e| e.synergies.clone()).collect(),
            conflicts: edges.iter().flat_map(|e| e.conflicts.clone()).collect(),
        }
    });

    // Generate blend result
    let blend_result = generate_blend_result(&selected_games, &game_metadata, &blend_path);
    state.blend_result = Some(blend_result);
}

/// Generate the final blend result
fn generate_blend_result(
    games: &[&crate::vintage_games::TimelineGame],
    metadata: &HashMap<String, vintage_blending_core::GameMetadata>,
    blend_path: &BlendPath,
) -> BlendResult {
    // Aggregate genres with weights
    let mut genre_weights = HashMap::new();
    for game in games {
        let meta = &metadata[&game.id.to_string()];
        // For now, we'll use the game's genre affinity as a simple weight
        // Use the genre affinities from metadata
        for (genre, weight) in &meta.genre_affinities {
            *genre_weights.entry(genre.clone()).or_insert(0.0) += weight;
        }

        // If no affinities, use primary genre
        if meta.genre_affinities.is_empty() {
            // Fallback: use the game's primary genre
            *genre_weights.entry(game.genre.to_string()).or_insert(0.0) += 1.0;
        }
    }

    // Normalize genre weights
    let total_weight: f32 = genre_weights.values().sum();
    for weight in genre_weights.values_mut() {
        *weight /= total_weight;
    }

    // Collect all mechanics
    let mut all_mechanics = HashSet::new();
    for game in games {
        let meta = &metadata[&game.id.to_string()];
        all_mechanics.extend(meta.mechanic_tags.iter().cloned());
    }

    // Generate blend name
    let blend_name = generate_blend_name(games);

    // Calculate average complexity and balance
    let avg_complexity = games
        .iter()
        .map(|g| metadata[&g.id.to_string()].feature_vector.complexity)
        .sum::<f32>()
        / games.len() as f32;

    let avg_balance = games
        .iter()
        .map(|g| {
            metadata[&g.id.to_string()]
                .feature_vector
                .action_strategy_balance
        })
        .sum::<f32>()
        / games.len() as f32;

    // Extract synergies and conflicts from the blend path
    let synergies = blend_path
        .synergies
        .iter()
        .map(|s| {
            crate::wizard::steps::guided::types::Synergy {
                game1: s.type_name.clone(), // We'll use type_name as a placeholder
                game2: String::new(),
                description: s.description.clone(),
                strength: s.strength,
            }
        })
        .collect();

    let conflicts = blend_path
        .conflicts
        .iter()
        .map(|c| {
            crate::wizard::steps::guided::types::Conflict {
                game1: c.type_name.clone(), // We'll use type_name as a placeholder
                game2: String::new(),
                conflict_type: c.type_name.clone(),
                resolution: c.resolution_hint.clone(),
            }
        })
        .collect();

    // Generate recommendations
    let recommendations = generate_recommendations(&genre_weights, &all_mechanics, avg_complexity);

    // Determine art styles
    let art_styles = determine_art_styles(games);

    BlendResult {
        name: blend_name,
        description: generate_blend_description(games, &genre_weights),
        blend_path: blend_path.clone(),
        genres: genre_weights,
        mechanics: all_mechanics,
        art_styles,
        complexity_score: avg_complexity,
        action_strategy_balance: avg_balance,
        synergies,
        conflicts,
        recommended_features: recommendations,
    }
}

/// Generate a creative blend name
fn generate_blend_name(games: &[&crate::vintage_games::TimelineGame]) -> String {
    if games.len() == 2 {
        format!("{} × {}", games[0].name, games[1].name)
    } else {
        let first = &games[0].name;
        let last = &games[games.len() - 1].name;
        format!("{} meets {} (+{})", first, last, games.len() - 2)
    }
}

/// Generate blend description
fn generate_blend_description(
    games: &[&crate::vintage_games::TimelineGame],
    genres: &HashMap<String, f32>,
) -> String {
    let dominant_genre = genres
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(g, _)| g)
        .unwrap();

    let years: Vec<i32> = games.iter().map(|g| g.year).collect();
    let min_year = years.iter().min().unwrap();
    let max_year = years.iter().max().unwrap();

    format!(
        "A {} experience blending {} classic games from {}-{}, combining the best elements of each era",
        dominant_genre.to_lowercase(),
        games.len(),
        min_year,
        max_year
    )
}
