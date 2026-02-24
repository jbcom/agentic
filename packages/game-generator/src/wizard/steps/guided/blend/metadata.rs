use crate::vintage_games::TimelineGame;
use std::collections::HashMap;
use vintage_blending_core::types::{
    FeatureVector, GameMetadata, STANDARD_GENRES, STANDARD_MECHANICS, get_era_category,
};

/// Build metadata for a game based on its attributes
pub fn build_game_metadata(game: &TimelineGame) -> GameMetadata {
    let genre_weights = build_genre_weights(game);
    let mechanic_flags = build_mechanic_flags(game);

    // Calculate complexity based on genre and year
    let complexity = calculate_complexity(game);

    // Determine action/strategy balance
    let action_strategy = calculate_action_strategy_balance(game);

    // Platform generation (1-5 based on era)
    let platform_generation = match game.year {
        1980..=1983 => 1,
        1984..=1986 => 2,
        1987..=1989 => 3,
        1990..=1992 => 4,
        1993..=1995 => 5,
        _ => 3,
    };

    // Single vs Multi balance
    let single_multi = if game.platforms.iter().any(|p| p.contains("Arcade")) {
        0.5 // Arcade games often had 2-player modes
    } else {
        -0.8 // Most vintage games were single-player focused
    };

    let feature_vector = FeatureVector {
        genre_weights,
        mechanic_flags,
        platform_generation,
        complexity,
        action_strategy_balance: action_strategy,
        single_multi_balance: single_multi,
        semantic_embedding: None, // Would be populated by AI embeddings
    };

    // Build genre affinities
    let mut genre_affinities = HashMap::new();
    genre_affinities.insert(game.genre.to_string(), 1.0);

    // Build mechanic tags
    let mechanic_tags = determine_game_mechanics(game);

    GameMetadata {
        game_id: game.id.to_string(),
        name: game.name.to_string(),
        year: game.year as u32,
        feature_vector,
        common_pairings: HashMap::new(), // Would be populated from data
        genre_affinities,
        mechanic_tags,
        era_category: get_era_category(game.year as u32),
        mood_tags: determine_mood_tags(game), // Infer mood from genre and era
    }
}

/// Build genre weights vector based on standard genres
fn build_genre_weights(game: &TimelineGame) -> Vec<f32> {
    let mut weights = vec![0.0f32; STANDARD_GENRES.len()];

    // Find primary genre index
    if let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == game.genre) {
        weights[idx] = 1.0;
    }

    // Add secondary genre weights based on description
    if let Some(deck) = game.deck {
        let deck_lower = deck.to_lowercase();

        if deck_lower.contains("puzzle")
            && let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == "Puzzle")
        {
            weights[idx] = weights[idx].max(0.3f32);
        }
        if deck_lower.contains("adventure")
            && let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == "Adventure")
        {
            weights[idx] = weights[idx].max(0.3f32);
        }
        if deck_lower.contains("action")
            && let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == "Action")
        {
            weights[idx] = weights[idx].max(0.3f32);
        }
        if deck_lower.contains("strategy")
            && let Some(idx) = STANDARD_GENRES.iter().position(|&g| g == "Strategy")
        {
            weights[idx] = weights[idx].max(0.3f32);
        }
    }

    // Normalize weights
    let sum: f32 = weights.iter().sum();
    if sum > 0.0 {
        for w in &mut weights {
            *w /= sum;
        }
    }

    weights
}

/// Build mechanic flags vector based on standard mechanics
fn build_mechanic_flags(game: &TimelineGame) -> Vec<bool> {
    let mut flags = vec![false; STANDARD_MECHANICS.len()];
    let mechanics = determine_game_mechanics(game);

    for (idx, &mechanic) in STANDARD_MECHANICS.iter().enumerate() {
        // Map our mechanics to standard ones
        flags[idx] = match mechanic {
            "Combat" => {
                mechanics.iter().any(|m| m == "real_time_combat")
                    || game.genre == "Action"
                    || game.genre == "Shooter"
            }
            "Exploration" => {
                mechanics.iter().any(|m| m == "exploration") || game.genre == "Adventure"
            }
            "Puzzle Solving" => {
                mechanics.iter().any(|m| m == "problem_solving") || game.genre == "Puzzle"
            }
            "Platform Jumping" => {
                mechanics.iter().any(|m| m == "jumping") || game.genre == "Platformer"
            }
            "Resource Management" => {
                mechanics.iter().any(|m| m == "resource_management") || game.genre == "Strategy"
            }
            "Character Progression" => {
                mechanics.iter().any(|m| m == "character_progression")
                    || game.genre == "Role-Playing"
            }
            "Story Choices" => {
                mechanics.iter().any(|m| m == "story_driven") || game.genre == "Adventure"
            }
            "Time Pressure" => mechanics.iter().any(|m| m == "arcade_style") || game.year <= 1985,
            "Collection" => {
                game.genre == "Platformer" || mechanics.iter().any(|m| m == "inventory")
            }
            "Stealth" => false, // Rare in vintage games
            "Multiplayer" => game.platforms.iter().any(|p| p.contains("Arcade")),
            "Turn-Based" => {
                game.genre == "Strategy" || (game.genre == "Role-Playing" && game.year <= 1990)
            }
            "Real-Time" => !flags[11] && (game.genre == "Action" || game.genre == "Shooter"),
            "Physics-Based" => false,         // Rare in vintage games
            "Procedural Generation" => false, // Rare in vintage games
            _ => false,
        };
    }

    flags
}

/// Determine game mechanics based on genre and description
pub fn determine_game_mechanics(game: &TimelineGame) -> Vec<String> {
    let mut mechanics = Vec::new();

    // Base mechanics by genre
    match game.genre {
        "Action" => {
            mechanics.push("real_time_combat".to_string());
            mechanics.push("reflexes".to_string());
        }
        "Role-Playing" => {
            mechanics.push("character_progression".to_string());
            mechanics.push("stats_management".to_string());
            mechanics.push("inventory".to_string());
        }
        "Strategy" => {
            mechanics.push("resource_management".to_string());
            mechanics.push("tactical_planning".to_string());
        }
        "Puzzle" => {
            mechanics.push("problem_solving".to_string());
            mechanics.push("pattern_recognition".to_string());
        }
        "Adventure" => {
            mechanics.push("exploration".to_string());
            mechanics.push("story_driven".to_string());
        }
        "Platform" | "Platformer" => {
            mechanics.push("jumping".to_string());
            mechanics.push("precision_movement".to_string());
        }
        "Shooter" => {
            mechanics.push("aiming".to_string());
            mechanics.push("reflexes".to_string());
        }
        "Sports" => {
            mechanics.push("timing".to_string());
            mechanics.push("team_management".to_string());
        }
        "Racing" => {
            mechanics.push("speed_control".to_string());
            mechanics.push("track_navigation".to_string());
        }
        _ => {}
    }

    // Era-specific mechanics
    if game.year <= 1985 {
        mechanics.push("arcade_style".to_string());
        mechanics.push("high_score_chase".to_string());
    } else if game.year >= 1990 {
        mechanics.push("save_system".to_string());
    }

    // Platform-specific mechanics
    if game.platforms.iter().any(|p| p.contains("Game Boy")) {
        mechanics.push("portable_friendly".to_string());
    }

    mechanics
}

/// Calculate game complexity (0.0 to 1.0)
pub fn calculate_complexity(game: &TimelineGame) -> f32 {
    let base = match game.genre {
        "Role-Playing" => 0.8,
        "Strategy" => 0.7,
        "Adventure" => 0.6,
        "Puzzle" => 0.5,
        "Action" => 0.4,
        "Platform" | "Platformer" => 0.3,
        "Sports" => 0.3,
        "Racing" => 0.3,
        "Shooter" => 0.4,
        _ => 0.5,
    };

    // Complexity increases over time
    let year_factor = (game.year - 1980) as f32 / 15.0 * 0.2;
    (base + year_factor).min(1.0)
}

/// Calculate action vs strategy balance (-1.0 = pure strategy, 1.0 = pure action)
pub fn calculate_action_strategy_balance(game: &TimelineGame) -> f32 {
    match game.genre {
        "Action" | "Shooter" => 0.8,
        "Platform" | "Platformer" | "Racing" => 0.6,
        "Sports" => 0.4,
        "Adventure" => 0.0,
        "Puzzle" => -0.4,
        "Role-Playing" => -0.2,
        "Strategy" => -0.8,
        _ => 0.0,
    }
}

/// Determine mood tags based on game attributes
fn determine_mood_tags(game: &TimelineGame) -> Vec<String> {
    let mut mood_tags = Vec::new();

    // Genre-based moods
    match game.genre {
        "Action" => {
            mood_tags.push("Fast-paced".to_string());
            mood_tags.push("Intense".to_string());
        }
        "Role-Playing" => {
            mood_tags.push("Epic".to_string());
            mood_tags.push("Immersive".to_string());
        }
        "Strategy" => {
            mood_tags.push("Thoughtful".to_string());
            mood_tags.push("Tactical".to_string());
        }
        "Puzzle" => {
            mood_tags.push("Relaxing".to_string());
            mood_tags.push("Cerebral".to_string());
        }
        "Adventure" => {
            mood_tags.push("Exploratory".to_string());
            mood_tags.push("Narrative".to_string());
        }
        "Platform" | "Platformer" => {
            mood_tags.push("Cheerful".to_string());
            mood_tags.push("Challenging".to_string());
        }
        "Shooter" => {
            mood_tags.push("Adrenaline".to_string());
            mood_tags.push("Competitive".to_string());
        }
        "Sports" => {
            mood_tags.push("Competitive".to_string());
            mood_tags.push("Energetic".to_string());
        }
        "Racing" => {
            mood_tags.push("Thrilling".to_string());
            mood_tags.push("Speed".to_string());
        }
        _ => {}
    }

    // Era-based moods
    if game.year <= 1985 {
        mood_tags.push("Arcade".to_string());
        mood_tags.push("Retro".to_string());
    } else if game.year >= 1992 {
        mood_tags.push("16-bit Era".to_string());
    }

    mood_tags
}

/// Determine art styles based on games
pub fn determine_art_styles(games: &[&TimelineGame]) -> Vec<String> {
    let mut styles = Vec::new();

    // Determine primary style based on era
    let avg_year = games.iter().map(|g| g.year).sum::<i32>() / games.len() as i32;

    if avg_year <= 1985 {
        styles.push("8-bit pixel art".to_string());
        styles.push("Limited color palette".to_string());
    } else if avg_year <= 1991 {
        styles.push("16-bit pixel art".to_string());
        styles.push("Vibrant colors".to_string());
    } else {
        styles.push("High-color pixel art".to_string());
        styles.push("Detailed sprites".to_string());
    }

    // Add genre-specific styles
    for game in games {
        match game.genre {
            "Role-Playing" => styles.push("Top-down or isometric view".to_string()),
            "Platform" | "Platformer" => styles.push("Side-scrolling perspective".to_string()),
            "Adventure" => styles.push("Detailed backgrounds".to_string()),
            "Racing" => styles.push("Pseudo-3D perspective".to_string()),
            _ => {}
        }
    }

    styles.sort();
    styles.dedup();
    styles
}
