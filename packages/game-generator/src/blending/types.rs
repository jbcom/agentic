//! Core types for game blending
//!
//! These types are used both at build time and runtime

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Feature vector for game similarity calculations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureVector {
    /// Genre weights (normalized 0.0-1.0)
    pub genre_weights: Vec<f32>,
    /// Mechanic presence flags
    pub mechanic_flags: Vec<bool>,
    /// Platform generation (1-5)
    pub platform_generation: u8,
    /// Complexity score (0.0-1.0)
    pub complexity: f32,
    /// Action vs Strategy balance (-1.0 to 1.0)
    pub action_strategy_balance: f32,
    /// Single vs Multiplayer focus (-1.0 to 1.0)
    pub single_multi_balance: f32,
    /// Semantic embedding from AI analysis (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_embedding: Option<Vec<f32>>,
}

impl FeatureVector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate cosine similarity between two vectors
    pub fn similarity(&self, other: &Self) -> f32 {
        // If both have semantic embeddings, use those for higher quality similarity
        if let (Some(embed_a), Some(embed_b)) =
            (&self.semantic_embedding, &other.semantic_embedding)
        {
            let semantic_sim = self.cosine_similarity(embed_a, embed_b);
            // Mix semantic similarity with traditional features
            let base_sim = self.base_similarity(other);

            // Weight semantic similarity heavily when available
            semantic_sim * 0.6 + base_sim * 0.4
        } else {
            self.base_similarity(other)
        }
    }

    /// Calculate similarity based only on traditional features (no embeddings)
    pub fn base_similarity(&self, other: &Self) -> f32 {
        let genre_sim = self.genre_similarity(&other.genre_weights);
        let mech_sim = self.mechanic_similarity(&other.mechanic_flags);
        let gen_sim =
            1.0 - (self.platform_generation.abs_diff(other.platform_generation) as f32 / 5.0);
        let complex_sim = 1.0 - (self.complexity - other.complexity).abs();
        let action_sim =
            1.0 - (self.action_strategy_balance - other.action_strategy_balance).abs() / 2.0;
        let multi_sim = 1.0 - (self.single_multi_balance - other.single_multi_balance).abs() / 2.0;

        // Weighted average
        genre_sim * 0.3
            + mech_sim * 0.3
            + gen_sim * 0.1
            + complex_sim * 0.1
            + action_sim * 0.1
            + multi_sim * 0.1
    }

    fn genre_similarity(&self, other: &[f32]) -> f32 {
        if self.genre_weights.is_empty() || other.is_empty() {
            return 0.5;
        }

        let mut dot_product = 0.0;
        let mut magnitude_a = 0.0;
        let mut magnitude_b = 0.0;

        for (a, b) in self.genre_weights.iter().zip(other.iter()) {
            dot_product += a * b;
            magnitude_a += a * a;
            magnitude_b += b * b;
        }

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude_a.sqrt() * magnitude_b.sqrt())
    }

    fn mechanic_similarity(&self, other: &[bool]) -> f32 {
        if self.mechanic_flags.is_empty() || other.is_empty() {
            return 0.5;
        }

        let matches = self
            .mechanic_flags
            .iter()
            .zip(other.iter())
            .filter(|(a, b)| a == b)
            .count();

        matches as f32 / self.mechanic_flags.len().min(other.len()) as f32
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.is_empty() || b.is_empty() || a.len() != b.len() {
            return 0.0;
        }

        let mut dot_product = 0.0;
        let mut magnitude_a = 0.0;
        let mut magnitude_b = 0.0;

        for i in 0..a.len() {
            dot_product += a[i] * b[i];
            magnitude_a += a[i] * a[i];
            magnitude_b += b[i] * b[i];
        }

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude_a.sqrt() * magnitude_b.sqrt())
    }
}

/// Pre-computed game metadata for efficient blending
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetadata {
    pub game_id: String,
    pub name: String,
    pub year: u32,
    pub feature_vector: FeatureVector,
    /// Pre-computed compatibility scores with common pairings
    pub common_pairings: HashMap<String, f32>,
    /// Genre affinity scores
    pub genre_affinities: HashMap<String, f32>,
    /// Mechanic tags for quick matching
    pub mechanic_tags: Vec<String>,
    /// Era category (early_80s, late_80s, early_90s, etc.)
    pub era_category: String,
    /// Mood tags from AI analysis (optional)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mood_tags: Vec<String>,
}

/// Edge data in the game graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityEdge {
    pub weight: f32,
    pub synergies: Vec<Synergy>,
    pub conflicts: Vec<Conflict>,
}

/// Synergy between game elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synergy {
    pub type_name: String,
    pub description: String,
    pub strength: f32,
}

/// Conflict between game elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub type_name: String,
    pub description: String,
    pub severity: f32,
    pub resolution_hint: String,
}

/// Resolution strategy for conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionStrategy {
    pub name: String,
    pub description: String,
    pub actions: Vec<ResolutionAction>,
    pub prerequisites: Vec<String>,
}

/// Individual resolution action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionAction {
    pub action_type: String,
    pub target: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Standard genre list for consistent indexing
pub const STANDARD_GENRES: &[&str] = &[
    "Action",
    "Adventure",
    "RPG",
    "Strategy",
    "Puzzle",
    "Platform",
    "Shooter",
    "Fighting",
    "Racing",
    "Sports",
    "Simulation",
    "Horror",
];

/// Standard mechanic list for consistent indexing
pub const STANDARD_MECHANICS: &[&str] = &[
    "Combat",
    "Exploration",
    "Puzzle Solving",
    "Platform Jumping",
    "Resource Management",
    "Character Progression",
    "Story Choices",
    "Time Pressure",
    "Collection",
    "Stealth",
    "Multiplayer",
    "Turn-Based",
    "Real-Time",
    "Physics-Based",
    "Procedural Generation",
];

/// Get era category from year
pub fn get_era_category(year: u32) -> String {
    match year {
        1980..=1983 => "early_80s".to_string(),
        1984..=1986 => "mid_80s".to_string(),
        1987..=1989 => "late_80s".to_string(),
        1990..=1992 => "early_90s".to_string(),
        1993..=1995 => "mid_90s".to_string(),
        _ => "unknown".to_string(),
    }
}
