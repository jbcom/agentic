//! Core blending functionality for vintage game combinations
//!
//! This crate provides types and algorithms used both at build time
//! (for pre-computing metadata) and runtime (for real-time blending).

pub mod graph;
pub mod metadata;
pub mod similarity;
pub mod types;

pub use graph::GameGraph;
pub use metadata::MetadataBuilder;
pub use similarity::SimilarityEngine;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_vector_creation() {
        let vector = FeatureVector::new();
        assert_eq!(vector.genre_weights.len(), 0);
    }
}
