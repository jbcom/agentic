//! Freeform Mode - AI-assisted game creation through conversation
//!
//! This mode provides an interactive conversation interface where users can
//! describe their game ideas and the AI helps design and generate the game.

use crate::wizard::pipeline::GenerationPipeline;
use crate::wizard::state::AppState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

mod conversation;
mod types;

pub use conversation::*;
pub use types::*;

/// Main entry point for rendering freeform mode
pub fn render_freeform_mode(
    contexts: EguiContexts,
    app_state: ResMut<AppState>,
    freeform_state: ResMut<FreeformModeState>,
    commands: Commands,
    pipeline: Res<GenerationPipeline>,
    stream_res: ResMut<ConversationStream>,
) {
    // Route to appropriate sub-step
    match &freeform_state.current_step {
        FreeformStep::Introduction => {
            ui_placeholder(contexts, "Introduction");
        }
        FreeformStep::BasicInfo => {
            ui_placeholder(contexts, "Basic Info");
        }
        FreeformStep::GameplayDesign => {
            ui_placeholder(contexts, "Gameplay Design");
        }
        FreeformStep::VisualStyle => {
            ui_placeholder(contexts, "Visual Style");
        }
        FreeformStep::Features => {
            ui_placeholder(contexts, "Features");
        }
        FreeformStep::TechnicalSettings => {
            ui_placeholder(contexts, "Technical Settings");
        }
        FreeformStep::Review => {
            ui_placeholder(contexts, "Review");
        }
        FreeformStep::Conversation => {
            conversation::render_conversation(
                contexts,
                app_state,
                freeform_state,
                commands,
                pipeline,
                stream_res,
            );
        }
    }
}

fn ui_placeholder(mut contexts: EguiContexts, name: &str) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(format!("{} - Under Construction", name));
        ui.label("This step is currently being rebuilt.");
    });
}

/// Setup resources for freeform mode
pub fn setup_freeform_mode(mut commands: Commands) {
    // Insert the freeform mode state
    commands.insert_resource(FreeformModeState::default());
    commands.insert_resource(ConversationStream::default());

    // Initialize AI client if needed
    if std::env::var("OPENAI_API_KEY").is_ok() {
        // Initialize AI resources
        info!("OpenAI API key found, AI conversation will be available");
    } else {
        warn!("No OpenAI API key found, AI features will be limited");
    }
}

/// Cleanup resources when leaving freeform mode
pub fn cleanup_freeform_mode(mut commands: Commands) {
    commands.remove_resource::<FreeformModeState>();
    commands.remove_resource::<ConversationStream>();
}
