//! Conversation management for interactive AI assistance
//!
//! Handles multi-turn conversations for:
//! - Game design discussions
//! - Iterative refinement of concepts
//! - Q&A about generated content
//! - Blend exploration and explanation
//! - Phased game generation workflows

mod game_generation;
mod manager;
mod starters;
mod types;

pub use manager::ConversationManager;
pub use starters::{
    blend_exploration_context, game_design_context, game_generation_context,
    technical_assistance_context,
};
pub use types::{
    BlendContext, Conversation, ConversationContext, ConversationMessage, ConversationSummary,
    GameConceptContext, GenerationPhase, GenerationProgress, MessageConfig, MessageRole,
};

// Re-export game generation methods
pub use game_generation::GameGenerationExt;
