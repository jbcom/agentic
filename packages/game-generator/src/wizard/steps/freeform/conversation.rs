//! AI conversation interface for freeform mode

use super::{
    ConversationEntry, ConversationRole, ConversationStream, ConversationStreamEvent,
    FreeformModeState,
};
use crate::wizard::pipeline::GenerationPipeline;
use crate::wizard::state::AppState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use futures::StreamExt;

/// Render the AI conversation interface
pub fn render_conversation(
    mut contexts: EguiContexts,
    mut app_state: ResMut<AppState>,
    mut freeform_state: ResMut<FreeformModeState>,
    _commands: Commands,
    pipeline: Res<GenerationPipeline>,
    mut stream_res: ResMut<ConversationStream>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        // Header
        ui.horizontal(|ui| {
            ui.heading("🤖 AI Game Design Conversation");
            ui.separator();
            ui.label("Let's bring your game to life!");
        });
        ui.separator();

        // Context summary
        if !freeform_state.conversation.context_summary.is_empty() {
            ui.group(|ui| {
                ui.label("Current Context:");
                ui.label(&freeform_state.conversation.context_summary);
            });
            ui.separator();
        }

        // Conversation history
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(ui.available_height() - 100.0)
            .show(ui, |ui| {
                for entry in &freeform_state.conversation.history {
                    render_conversation_entry(ui, entry);
                    ui.add_space(10.0);
                }

                // Show processing indicator
                if freeform_state.conversation.is_processing
                    && !freeform_state.conversation.is_streaming
                {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("AI is thinking...");
                    });
                }

                // Show error if any
                if let Some(error) = &freeform_state.conversation.error_message {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                }
            });

        ui.separator();

        // Input area
        ui.horizontal(|ui| {
            let response = ui.text_edit_multiline(&mut freeform_state.conversation.current_input);

            // Focus on the text input
            if response.lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                && !freeform_state.conversation.current_input.trim().is_empty()
                && !freeform_state.conversation.is_processing
            {
                send_message(&mut freeform_state, &pipeline, stream_res.reborrow());
            }

            ui.vertical(|ui| {
                if freeform_state.conversation.is_processing {
                    if ui.button("Cancel").clicked() {
                        freeform_state.conversation.is_processing = false;
                        freeform_state.conversation.is_streaming = false;
                        stream_res.receiver = None;
                    }
                } else if ui.button("Send").clicked()
                    && !freeform_state.conversation.current_input.trim().is_empty()
                {
                    send_message(&mut freeform_state, &pipeline, stream_res.reborrow());
                }

                if ui.button("Export").clicked() {
                    // TODO: Export conversation and config
                    info!("Exporting freeform configuration...");
                }
            });
        });

        // Navigation
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("← Back to Review").clicked() {
                freeform_state.current_step = super::FreeformStep::Review;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Generate Game →").clicked() {
                    // TODO: Start generation
                    app_state.set_wizard_step(crate::wizard::state::WizardStep::Complete);
                }
            });
        });
    });
}

fn render_conversation_entry(ui: &mut egui::Ui, entry: &ConversationEntry) {
    let (icon, color) = match entry.role {
        ConversationRole::User => ("👤", egui::Color32::from_rgb(100, 150, 255)),
        ConversationRole::Assistant => ("🤖", egui::Color32::from_rgb(100, 255, 150)),
        ConversationRole::System => ("⚙️", egui::Color32::from_rgb(200, 200, 200)),
    };

    ui.horizontal(|ui| {
        ui.label(icon);
        ui.colored_label(color, &entry.content);
    });

    if let Some(metadata) = &entry.metadata {
        ui.indent("metadata", |ui| {
            ui.label(format!("Topic: {}", metadata.topic));
            if !metadata.decisions_made.is_empty() {
                ui.label("Decisions made:");
                for decision in &metadata.decisions_made {
                    ui.label(format!("  • {}", decision));
                }
            }
        });
    }
}

fn send_message(
    freeform_state: &mut FreeformModeState,
    pipeline: &GenerationPipeline,
    mut stream_res: Mut<ConversationStream>,
) {
    let message = freeform_state.conversation.current_input.trim().to_string();

    // Add user message to history
    freeform_state.conversation.history.push(ConversationEntry {
        role: ConversationRole::User,
        content: message.clone(),
        timestamp: std::time::SystemTime::now(),
        metadata: None,
    });

    // Clear input
    freeform_state.conversation.current_input.clear();

    // Set processing state
    freeform_state.conversation.is_processing = true;
    freeform_state.conversation.is_streaming = true;

    // Create channel for streaming
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    stream_res.receiver = Some(rx);

    // Get a clone of the generator
    let generator_arc = pipeline.generator.clone();
    let runtime = pipeline.runtime.clone();
    let conversation_id = freeform_state.conversation.conversation_id.clone();

    // Spawn async task for streaming
    runtime.spawn(async move {
        let generator_lock = generator_arc.lock().await;
        if let Some(generator) = generator_lock.as_ref() {
            // Start or continue conversation
            if let Some(conversation_id_ref) = &conversation_id {
                let result = generator
                    .continue_game_design_conversation_stream(conversation_id_ref, &message)
                    .await;

                match result {
                    Ok(stream) => {
                        futures::pin_mut!(stream);
                        while let Some(token_result) = stream.next().await {
                            match token_result {
                                Ok(token) => {
                                    let _ = tx.send(ConversationStreamEvent::Token(token));
                                }
                                Err(e) => {
                                    let _ = tx.send(ConversationStreamEvent::Error(e.to_string()));
                                    break;
                                }
                            }
                        }
                        let _ = tx.send(ConversationStreamEvent::Finished);
                    }
                    Err(e) => {
                        let _ = tx.send(ConversationStreamEvent::Error(e.to_string()));
                    }
                }
            } else {
                // If no conversation ID, start a new one first
                match generator.start_game_design_conversation(&message).await {
                    Ok((_new_id, initial_response)) => {
                        // Send the initial response
                        let _ = tx.send(ConversationStreamEvent::Token(initial_response));
                        let _ = tx.send(ConversationStreamEvent::Finished);
                    }
                    Err(e) => {
                        let _ = tx.send(ConversationStreamEvent::Error(e.to_string()));
                    }
                }
            }
        } else {
            let _ = tx.send(ConversationStreamEvent::Error(
                "AI Generator not initialized".to_string(),
            ));
        }
    });
}

/// System to process streaming conversation events
pub fn process_conversation_stream(
    mut freeform_state: ResMut<FreeformModeState>,
    mut stream_res: ResMut<ConversationStream>,
) {
    let receiver_ref = match &mut stream_res.receiver {
        Some(rx) => rx,
        None => return,
    };

    while let Ok(event) = receiver_ref.try_recv() {
        match event {
            ConversationStreamEvent::Token(token) => {
                // If the last message is from Assistant and we are streaming, append to it
                // Otherwise, create a new Assistant message
                if freeform_state.conversation.is_streaming {
                    if let Some(last_entry) = freeform_state.conversation.history.last_mut() {
                        if last_entry.role == ConversationRole::Assistant {
                            last_entry.content.push_str(&token);
                        } else {
                            freeform_state.conversation.history.push(ConversationEntry {
                                role: ConversationRole::Assistant,
                                content: token,
                                timestamp: std::time::SystemTime::now(),
                                metadata: None,
                            });
                        }
                    } else {
                        freeform_state.conversation.history.push(ConversationEntry {
                            role: ConversationRole::Assistant,
                            content: token,
                            timestamp: std::time::SystemTime::now(),
                            metadata: None,
                        });
                    }
                }
            }
            ConversationStreamEvent::Finished => {
                freeform_state.conversation.is_processing = false;
                freeform_state.conversation.is_streaming = false;
                stream_res.receiver = None;
                return;
            }
            ConversationStreamEvent::Error(e) => {
                freeform_state.conversation.error_message = Some(e);
                freeform_state.conversation.is_processing = false;
                freeform_state.conversation.is_streaming = false;
                stream_res.receiver = None;
                return;
            }
        }
    }
}
