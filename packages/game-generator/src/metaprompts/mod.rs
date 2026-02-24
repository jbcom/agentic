// Module declarations for metaprompts
pub mod conversation;
pub mod generator;
pub mod types;
pub mod validation;
pub mod watcher;

// Re-exports for convenience
pub use conversation::{SimpleMessage, WizardConversationState};
pub use generator::{
    ConversationMessage, ConversationState, GameGenerator, GenerationPhase, GenerationProgress,
};
pub use types::{ArtStyle, ColorPalette, GameConfig, WorldConfig};
pub use validation::{PromptValidator, ValidationResult};
pub use watcher::{GenerationQueue, PromptWatcher};
