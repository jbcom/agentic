// lib.rs
//
// Crate-level lint configuration.
// We enable clippy::pedantic at the workspace level but selectively allow lints
// that are too noisy for this codebase (generated code, prototype code, etc.).
#![allow(
    // Documentation lints -- the codebase has many public functions without full doc sections
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    // must_use -- pervasive across builder patterns and accessors
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    // Casting precision/truncation -- common in game code with f32/u32/i32 conversions
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    // Naming -- generated code uses r#"..."# patterns and similar variable names
    clippy::needless_raw_string_hashes,
    clippy::similar_names,
    // Style preferences that conflict with existing code patterns
    clippy::redundant_else,
    clippy::redundant_closure_for_method_calls,
    clippy::unreadable_literal,
    clippy::items_after_statements,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::unused_self,
    clippy::unused_async,
    clippy::unnecessary_wraps,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::single_match_else,
    // Struct/function complexity -- existing code has large structs and long functions
    clippy::too_many_lines,
    clippy::struct_excessive_bools,
    // String formatting preferences
    clippy::uninlined_format_args,
    clippy::format_push_string,
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    // Collection types -- HashMap/HashSet generic hasher lints
    clippy::implicit_hasher,
    // Iterator and option/result method preferences
    clippy::manual_map,
    clippy::option_if_let_else,
    clippy::explicit_iter_loop,
    clippy::cloned_instead_of_copied,
    clippy::default_trait_access,
    clippy::if_not_else,
    clippy::let_underscore_untyped,
    clippy::map_unwrap_or,
    clippy::doc_markdown,
    clippy::manual_let_else,
    clippy::useless_conversion,
    clippy::needless_raw_strings,
    // Misc pedantic
    clippy::fn_params_excessive_bools,
    // clippy::match_on_vec_items removed in newer clippy, replaced by indexing_slicing
    clippy::stable_sort_primitive,
    clippy::inefficient_to_string,
    clippy::redundant_clone,
    clippy::semicolon_if_nothing_returned,
    clippy::implicit_clone,
    clippy::assigning_clones,
    clippy::manual_string_new,
    clippy::elidable_lifetime_names,
    clippy::ignored_unit_patterns,
)]

pub mod ai_client;
pub mod blending;
pub mod build_tools;
pub mod combat;
pub mod metaprompts;
pub mod vintage_games;
pub mod wizard;

pub use metaprompts::{GameConfig, GameGenerator, GenerationPhase, GenerationProgress};

pub use wizard::{AppDirectories, AppMode, AppState, WizardPlugin};

// Re-export common types for Tauri frontend
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub genre: String,
    pub setting: String,
    pub art_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    pub include_voice_acting: bool,
    pub include_orchestral_music: bool,
    pub target_playtime_hours: u32,
    pub difficulty_options: Vec<String>,
    pub accessibility_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub phase: String,
    pub step: String,
    pub progress: f32,
    pub message: String,
    pub artifact: Option<GeneratedArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedArtifact {
    pub artifact_type: String,
    pub name: String,
    pub path: Option<String>,
    pub preview: Option<String>, // Base64 encoded preview for images
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Generation failed: {0}")]
    GenerationFailed(String),
}

pub type Result<T> = std::result::Result<T, GeneratorError>;
