//! Predefined conversation starters

use super::types::{BlendContext, ConversationContext, GameConceptContext, GenerationPhase};
use std::collections::HashMap;

/// Start a game design conversation
pub fn game_design_context() -> ConversationContext {
    ConversationContext {
        conversation_type: "game_design".to_string(),
        game_concept: None,
        max_context_messages: 20,
        system_prompt: Some(
            "You are a creative game designer specializing in nostalgic 16-bit games. \
            Help the user explore and refine their game concepts, offering creative \
            suggestions while maintaining the authentic feel of classic gaming eras. \
            Be encouraging, specific, and reference classic games when relevant."
                .to_string(),
        ),
        generation_phase: None,
        project_config: None,
    }
}

/// Start a blend exploration conversation
pub fn blend_exploration_context(selected_games: Vec<String>) -> ConversationContext {
    ConversationContext {
        conversation_type: "blend_exploration".to_string(),
        game_concept: Some(GameConceptContext {
            title: "Untitled Blend".to_string(),
            genre: "Hybrid".to_string(),
            inspirations: selected_games.clone(),
            current_blend: Some(BlendContext {
                selected_games,
                blend_weights: HashMap::new(),
                dominant_attributes: Vec::new(),
            }),
        }),
        max_context_messages: 15,
        system_prompt: Some(
            "You are an expert at analyzing and blending classic game mechanics and styles. \
            Help the user understand how their selected games could combine into something \
            new and exciting. Focus on specific mechanics, visual styles, and gameplay loops."
                .to_string(),
        ),
        generation_phase: None,
        project_config: None,
    }
}

/// Start a technical assistance conversation
pub fn technical_assistance_context() -> ConversationContext {
    ConversationContext {
        conversation_type: "technical".to_string(),
        game_concept: None,
        max_context_messages: 10,
        system_prompt: Some(
            "You are a helpful technical assistant for game developers. \
            Provide clear, practical advice about implementing game features, \
            optimizing performance, and solving technical challenges. \
            Focus on solutions appropriate for indie developers."
                .to_string(),
        ),
        generation_phase: None,
        project_config: None,
    }
}

/// Start a game generation conversation with template support
pub fn game_generation_context(project_config: Option<serde_json::Value>) -> ConversationContext {
    ConversationContext {
        conversation_type: "game_generation".to_string(),
        game_concept: None,
        max_context_messages: 30,
        system_prompt: None, // Will be loaded from template
        generation_phase: Some(GenerationPhase::Design),
        project_config,
    }
}
