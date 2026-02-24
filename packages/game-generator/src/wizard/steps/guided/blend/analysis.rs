use crate::vintage_games::TimelineGame;
use crate::wizard::steps::guided::types::{Conflict, Synergy};
use std::collections::HashMap;
use vintage_blending_core::types::{
    Conflict as CoreConflict, GameMetadata, Synergy as CoreSynergy,
};

/// Analyze synergies between two games
pub fn analyze_synergies(
    game1: &TimelineGame,
    game2: &TimelineGame,
    meta1: &GameMetadata,
    meta2: &GameMetadata,
) -> Vec<CoreSynergy> {
    let mut synergies = Vec::new();

    // Genre synergies
    if game1.genre == game2.genre {
        synergies.push(CoreSynergy {
            type_name: "Genre Match".to_string(),
            description: format!("Both games share {} genre expertise", game1.genre),
            strength: 0.8,
        });
    }

    // Era synergies
    if (game1.year - game2.year).abs() <= 2 {
        synergies.push(CoreSynergy {
            type_name: "Era Match".to_string(),
            description: "Games from similar era share technical constraints".to_string(),
            strength: 0.6,
        });
    }

    // Platform synergies
    let shared_platforms: Vec<_> = game1
        .platforms
        .iter()
        .filter(|p| game2.platforms.contains(p))
        .cloned()
        .collect();

    if !shared_platforms.is_empty() {
        synergies.push(CoreSynergy {
            type_name: "Platform Match".to_string(),
            description: format!("Both released on: {}", shared_platforms.join(", ")),
            strength: 0.5,
        });
    }

    // Complexity synergy
    let complexity_diff = (meta1.feature_vector.complexity - meta2.feature_vector.complexity).abs();
    if complexity_diff < 0.2 {
        synergies.push(CoreSynergy {
            type_name: "Complexity Match".to_string(),
            description: "Similar complexity levels ensure consistent experience".to_string(),
            strength: 0.7,
        });
    }

    // Mechanic overlaps
    let mut shared_mechanics = 0;
    for i in 0..meta1.mechanic_tags.len().min(meta2.mechanic_tags.len()) {
        if meta1.mechanic_tags.contains(&meta2.mechanic_tags[i]) {
            shared_mechanics += 1;
        }
    }

    if shared_mechanics > 0 {
        synergies.push(CoreSynergy {
            type_name: "Mechanic Overlap".to_string(),
            description: format!("{shared_mechanics} shared game mechanics"),
            strength: 0.6,
        });
    }

    synergies
}

/// Analyze conflicts between two games
pub fn analyze_conflicts(
    game1: &TimelineGame,
    game2: &TimelineGame,
    meta1: &GameMetadata,
    meta2: &GameMetadata,
) -> Vec<CoreConflict> {
    let mut conflicts = Vec::new();

    // Complexity mismatch
    let complexity_diff = (meta1.feature_vector.complexity - meta2.feature_vector.complexity).abs();
    if complexity_diff > 0.5 {
        conflicts.push(CoreConflict {
            type_name: "Complexity Mismatch".to_string(),
            description: "Large complexity gap may create uneven experience".to_string(),
            severity: complexity_diff,
            resolution_hint: "Implement difficulty modes or gradual complexity ramp".to_string(),
        });
    }

    // Action/strategy mismatch
    let balance_diff = (meta1.feature_vector.action_strategy_balance
        - meta2.feature_vector.action_strategy_balance)
        .abs();
    if balance_diff > 1.0 {
        conflicts.push(CoreConflict {
            type_name: "Gameplay Style Conflict".to_string(),
            description: "Conflicting pace: action vs strategy focus".to_string(),
            severity: balance_diff / 2.0,
            resolution_hint: "Create distinct gameplay modes or hybrid mechanics".to_string(),
        });
    }

    // Era gap
    if (game1.year - game2.year).abs() > 10 {
        conflicts.push(CoreConflict {
            type_name: "Era Gap".to_string(),
            description: "Large era gap may create inconsistent expectations".to_string(),
            severity: 0.4,
            resolution_hint: "Use modern QoL features while preserving retro charm".to_string(),
        });
    }

    // Genre conflicts
    let conflicting_genres = [
        ("Action", "Strategy"),
        ("Puzzle", "Action"),
        ("Racing", "Role-Playing"),
    ];

    for (genre1, genre2) in &conflicting_genres {
        if (game1.genre == *genre1 && game2.genre == *genre2)
            || (game1.genre == *genre2 && game2.genre == *genre1)
        {
            conflicts.push(CoreConflict {
                type_name: "Genre Conflict".to_string(),
                description: format!(
                    "{genre1} and {genre2} have very different player expectations"
                ),
                severity: 0.6,
                resolution_hint: "Clearly communicate genre blend in game description".to_string(),
            });
        }
    }

    conflicts
}

/// Extract synergies from blend path with game names
pub fn extract_synergies(
    edges: &[vintage_blending_core::CompatibilityEdge],
    games: &[&TimelineGame],
) -> Vec<Synergy> {
    // Since edges don't have game IDs, we'll extract synergies from the edges themselves
    let mut all_synergies = Vec::new();

    for (i, edge) in edges.iter().enumerate() {
        // For each edge's synergies, create our Synergy type
        for syn in &edge.synergies {
            // Try to infer game names from position (this is a simplification)
            let (game1_name, game2_name) = if i < games.len() - 1 {
                (games[i].name, games[i + 1].name)
            } else {
                (games[0].name, games[games.len() - 1].name)
            };

            all_synergies.push(Synergy {
                game1: game1_name.to_string(),
                game2: game2_name.to_string(),
                description: syn.description.clone(),
                strength: syn.strength,
            });
        }
    }

    all_synergies
}

/// Extract conflicts from blend path with game names
pub fn extract_conflicts(
    edges: &[vintage_blending_core::CompatibilityEdge],
    games: &[&TimelineGame],
) -> Vec<Conflict> {
    // Since edges don't have game IDs, we'll extract conflicts from the edges themselves
    let mut all_conflicts = Vec::new();

    for (i, edge) in edges.iter().enumerate() {
        // For each edge's conflicts, create our Conflict type
        for conf in &edge.conflicts {
            // Try to infer game names from position (this is a simplification)
            let (game1_name, game2_name) = if i < games.len() - 1 {
                (games[i].name, games[i + 1].name)
            } else {
                (games[0].name, games[games.len() - 1].name)
            };

            all_conflicts.push(Conflict {
                game1: game1_name.to_string(),
                game2: game2_name.to_string(),
                conflict_type: conf.type_name.clone(),
                resolution: conf.resolution_hint.clone(),
            });
        }
    }

    all_conflicts
}

/// Suggest resolution for a conflict
pub fn suggest_resolution(conflict: &str) -> String {
    if conflict.contains("complexity") {
        "Implement difficulty modes or gradual complexity ramp".to_string()
    } else if conflict.contains("pace") || conflict.contains("action vs strategy") {
        "Create distinct gameplay modes or hybrid mechanics".to_string()
    } else if conflict.contains("era") {
        "Use modern QoL features while preserving retro charm".to_string()
    } else if conflict.contains("expectations") {
        "Clearly communicate genre blend in game description".to_string()
    } else {
        "Balance conflicting elements through player choice".to_string()
    }
}

/// Generate feature recommendations based on blend
pub fn generate_recommendations(
    genres: &HashMap<String, f32>,
    mechanics: &std::collections::HashSet<String>,
    complexity: f32,
) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Genre-based recommendations
    if genres.get("Role-Playing").unwrap_or(&0.0) > &0.3 {
        recommendations.push("Character customization system".to_string());
        recommendations.push("Quest system with branching paths".to_string());
    }

    if genres.get("Action").unwrap_or(&0.0) > &0.3 {
        recommendations.push("Responsive combat with combo system".to_string());
        recommendations.push("Boss battles with pattern learning".to_string());
    }

    if genres.get("Strategy").unwrap_or(&0.0) > &0.3 {
        recommendations.push("Resource management layer".to_string());
        recommendations.push("Strategic planning phases".to_string());
    }

    if genres.get("Puzzle").unwrap_or(&0.0) > &0.3 {
        recommendations.push("Environmental puzzles integrated into levels".to_string());
    }

    // Mechanic-based recommendations
    if mechanics.contains("exploration") {
        recommendations.push("Hidden areas and secrets to discover".to_string());
        recommendations.push("Metroidvania-style ability gating".to_string());
    }

    if mechanics.contains("character_progression") {
        recommendations.push("Skill trees or ability unlocks".to_string());
        recommendations.push("Experience point system".to_string());
    }

    if mechanics.contains("high_score_chase") {
        recommendations.push("Score multiplier system".to_string());
        recommendations.push("Online leaderboards".to_string());
    }

    // Complexity-based recommendations
    if complexity > 0.7 {
        recommendations.push("In-depth tutorial system".to_string());
        recommendations.push("Codex or journal for tracking information".to_string());
        recommendations.push("Hint system for stuck players".to_string());
    } else if complexity < 0.3 {
        recommendations.push("Pick-up-and-play design".to_string());
        recommendations.push("Visual feedback over text explanations".to_string());
        recommendations.push("Intuitive controls".to_string());
    }

    // Balance recommendations
    if mechanics.contains("real_time_combat") && mechanics.contains("tactical_planning") {
        recommendations.push("Pause-and-plan tactical mode".to_string());
    }

    recommendations
}
