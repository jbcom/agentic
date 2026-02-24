//! Weighted graph implementation for game blending
//!
//! Uses petgraph for efficient graph operations

use anyhow::Result;
use petgraph::Undirected;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{CompatibilityEdge, Conflict, GameMetadata, Synergy};

/// Weighted graph of game relationships
pub struct GameGraph {
    /// The actual graph structure
    graph: Graph<String, f32, Undirected>,
    /// Node index lookup by game ID
    node_lookup: HashMap<String, NodeIndex>,
    /// Pre-computed metadata
    metadata: HashMap<String, GameMetadata>,
}

impl GameGraph {
    /// Create a new graph from metadata
    pub fn new(metadata: HashMap<String, GameMetadata>) -> Result<Self> {
        // Use undirected graph since game compatibility is symmetric
        let mut graph = Graph::new_undirected();
        let mut node_lookup = HashMap::new();

        // Add nodes
        for game_id in metadata.keys() {
            let idx = graph.add_node(game_id.clone());
            node_lookup.insert(game_id.clone(), idx);
        }

        // Add edges with compatibility weights
        let game_ids: Vec<_> = metadata.keys().cloned().collect();
        for i in 0..game_ids.len() {
            for j in (i + 1)..game_ids.len() {
                let game1_id = &game_ids[i];
                let game2_id = &game_ids[j];

                if let (Some(meta1), Some(meta2)) = (metadata.get(game1_id), metadata.get(game2_id))
                {
                    let compatibility = meta1.feature_vector.similarity(&meta2.feature_vector);

                    // Only add edges for games with meaningful compatibility
                    if compatibility > 0.1 {
                        let idx1 = node_lookup[game1_id];
                        let idx2 = node_lookup[game2_id];
                        graph.add_edge(idx1, idx2, compatibility);
                    }
                }
            }
        }

        Ok(Self {
            graph,
            node_lookup,
            metadata,
        })
    }

    /// Find the best blend path between multiple games
    pub fn find_blend_path(&self, game_ids: &[String]) -> Result<BlendPath> {
        if game_ids.len() < 2 {
            anyhow::bail!("Need at least 2 games to blend");
        }

        // Create a subgraph with just the selected games (undirected for symmetric compatibility)
        let mut subgraph = Graph::new_undirected();
        let mut sub_lookup = HashMap::new();

        // Add nodes
        for game_id in game_ids {
            if !self.node_lookup.contains_key(game_id) {
                anyhow::bail!("Game {game_id} not found in graph");
            }
            let idx = subgraph.add_node(game_id.clone());
            sub_lookup.insert(game_id.clone(), idx);
        }

        // Add edges from original graph with negated weights for max spanning tree
        // We negate weights because min_spanning_tree finds minimum, but we want maximum compatibility
        for i in 0..game_ids.len() {
            for j in (i + 1)..game_ids.len() {
                let game1_id = &game_ids[i];
                let game2_id = &game_ids[j];

                let idx1 = self.node_lookup[game1_id];
                let idx2 = self.node_lookup[game2_id];

                if let Some(edge) = self.graph.find_edge(idx1, idx2) {
                    let weight = self.graph[edge];
                    let sub_idx1 = sub_lookup[game1_id];
                    let sub_idx2 = sub_lookup[game2_id];
                    // Negate weight to convert min spanning tree to max spanning tree
                    subgraph.add_edge(sub_idx1, sub_idx2, -weight);
                }
            }
        }

        // Find minimum spanning tree (with negated weights, this gives us max spanning tree)
        let mst = min_spanning_tree(&subgraph);
        let mst_graph = Graph::<String, f32>::from_elements(mst);

        // Calculate total compatibility (negate back to get positive values)
        let total_compatibility: f32 = mst_graph.edge_weights().map(|w| -w).sum();

        // Analyze synergies and conflicts
        let mut synergies = Vec::new();
        let mut conflicts = Vec::new();

        for edge in mst_graph.edge_indices() {
            let (src, dst) = mst_graph.edge_endpoints(edge).unwrap();
            let game1_id = &mst_graph[src];
            let game2_id = &mst_graph[dst];

            let edge_analysis = self.analyze_edge(game1_id, game2_id)?;
            synergies.extend(edge_analysis.synergies);
            conflicts.extend(edge_analysis.conflicts);
        }

        Ok(BlendPath {
            games: game_ids.to_vec(),
            total_compatibility,
            synergies,
            conflicts,
        })
    }

    /// Analyze compatibility between two specific games
    pub fn analyze_edge(&self, game1_id: &str, game2_id: &str) -> Result<CompatibilityEdge> {
        let meta1 = self
            .metadata
            .get(game1_id)
            .ok_or_else(|| anyhow::anyhow!("Game {game1_id} not found"))?;
        let meta2 = self
            .metadata
            .get(game2_id)
            .ok_or_else(|| anyhow::anyhow!("Game {game2_id} not found"))?;

        let weight = meta1.feature_vector.similarity(&meta2.feature_vector);

        // Analyze synergies
        let mut synergies = Vec::new();

        // Era synergy
        if meta1.era_category == meta2.era_category {
            synergies.push(Synergy {
                type_name: "Era Match".to_string(),
                description: format!("Both games are from the {}", meta1.era_category),
                strength: 0.8,
            });
        }

        // Mechanic synergies
        for tag1 in &meta1.mechanic_tags {
            for tag2 in &meta2.mechanic_tags {
                if tag1 == tag2 {
                    synergies.push(Synergy {
                        type_name: "Shared Mechanic".to_string(),
                        description: format!("Both games feature {tag1}"),
                        strength: 0.6,
                    });
                }
            }
        }

        // Analyze conflicts
        let mut conflicts = Vec::new();

        // Complexity conflict
        let complexity_diff =
            (meta1.feature_vector.complexity - meta2.feature_vector.complexity).abs();
        if complexity_diff > 0.5 {
            // Always describe the more complex game first
            let (more_complex, less_complex) =
                if meta1.feature_vector.complexity > meta2.feature_vector.complexity {
                    (meta1.name.clone(), meta2.name.clone())
                } else {
                    (meta2.name.clone(), meta1.name.clone())
                };
            conflicts.push(Conflict {
                type_name: "Complexity Mismatch".to_string(),
                description: format!("{more_complex} is much more complex than {less_complex}"),
                severity: complexity_diff,
                resolution_hint: "Consider adjusting difficulty curves or adding tutorial layers"
                    .to_string(),
            });
        }

        // Action/Strategy balance conflict
        let balance_diff = (meta1.feature_vector.action_strategy_balance
            - meta2.feature_vector.action_strategy_balance)
            .abs();
        if balance_diff > 1.0 {
            conflicts.push(Conflict {
                type_name: "Gameplay Style Conflict".to_string(),
                description: "One game is action-focused while the other is strategy-focused"
                    .to_string(),
                severity: balance_diff / 2.0,
                resolution_hint:
                    "Blend by creating strategic action sequences or real-time strategy elements"
                        .to_string(),
            });
        }

        Ok(CompatibilityEdge {
            weight,
            synergies,
            conflicts,
        })
    }

    /// Get metadata for a specific game
    pub fn get_metadata(&self, game_id: &str) -> Option<&GameMetadata> {
        self.metadata.get(game_id)
    }

    /// Find games most compatible with a given game
    pub fn find_compatible_games(&self, game_id: &str, limit: usize) -> Vec<(String, f32)> {
        if let Some(&node_idx) = self.node_lookup.get(game_id) {
            let mut compatibilities = Vec::new();

            for edge in self.graph.edges(node_idx) {
                let other_idx = edge.target();
                let other_id = &self.graph[other_idx];
                let weight = edge.weight();
                compatibilities.push((other_id.clone(), *weight));
            }

            compatibilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            compatibilities.truncate(limit);
            compatibilities
        } else {
            Vec::new()
        }
    }
}

/// Result of finding an optimal blend path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendPath {
    pub games: Vec<String>,
    pub total_compatibility: f32,
    pub synergies: Vec<Synergy>,
    pub conflicts: Vec<Conflict>,
}
