use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::wizard::{AppState, AppDirectories, GenerationPipeline, state::{SelectedTab, LogLevel}, SwitchModeEvent, AppMode};
use crate::wizard::generate_flow::{WizardUI, WizardAction, WizardStep};
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestAssistantMessageArgs};

#[derive(Resource, Default)]
struct UIState {
    wizard_ui: Option<WizardUI>,
    show_wizard: bool,
    selected_tab: SelectedTab,
    input_text: String,
}

pub fn draw_app_ui(
    mut contexts: EguiContexts,
    mut app_state: ResMut<AppState>,
    directories: Res<AppDirectories>,
    pipeline: Res<GenerationPipeline>,
    mut switch_mode_events: EventWriter<SwitchModeEvent>,
    mut ui_state: ResMut<UIState>,
) {
    // Initialize wizard if needed
    if ui_state.wizard_ui.is_none() {
        let mut wizard = WizardUI::new(&directories);
        // Check if we should skip wizard (already have conversation started)
        if wizard.current_step == WizardStep::Conversation {
            ui_state.show_wizard = false;
        } else {
            ui_state.show_wizard = true;
        }
        ui_state.wizard_ui = Some(wizard);
    }

    // Handle wizard
    if ui_state.show_wizard {
        if let Some(wizard) = ui_state.wizard_ui.as_mut() {
                if let Ok(ctx) = contexts.ctx_mut() {
                    egui::CentralPanel::default().show(ctx, |ui| {
                    // Need to use the same UI type - cast or convert
                    if let Some(action) = wizard.draw(ui, &mut app_state) {
                        match action {
                            WizardAction::StartConversation => {
                                // Transition to conversation mode
                                ui_state.show_wizard = false;

                                // Store wizard configuration for AI context
                                app_state.set_project_config(wizard.config_manager.config.clone());

                                // Initialize conversation with wizard context as system message
                                let system_message = format!(
                                    "You are helping create a 16-bit style RPG game. The user has provided these specifications through a configuration wizard:\n\n{}",
                                    wizard.config_manager.config.to_ai_summary()
                                );

                                // Add opening message to conversation
                                let opening_message = format!(
                                    "I see you're creating {}, {} inspired by {}. Let's dive deeper into what makes your game unique. {}",
                                    wizard.config_manager.config.basic_info.name,
                                    wizard.config_manager.config.basic_info.tagline,
                                    wizard.config_manager.config.visual_style.reference_games.join(", "),
                                    match wizard.config_manager.config.basic_info.genre.as_str() {
                                        "Action RPG" => "What kind of combat mechanics do you envision?",
                                        "Turn-Based RPG" => "What strategic elements would you like in combat?",
                                        "Puzzle RPG" => "What types of puzzles do you want to incorporate?",
                                        _ => "What unique mechanics do you have in mind?"
                                    }
                                );

                                let conversation_state = app_state.conversation.clone();
                                pipeline.runtime.spawn(async move {
                                    let mut conv = conversation_state.lock().await;
                                    // Add system context
                                    conv.messages.push(ChatCompletionRequestMessage::System(
                                        async_openai::types::ChatCompletionRequestSystemMessage {
                                            content: async_openai::types::ChatCompletionRequestSystemMessageContent::Text(system_message),
                                            ..Default::default()
                                        }
                                    ));
                                    // Add AI opening
                                    conv.messages.push(ChatCompletionRequestMessage::Assistant(
                                        ChatCompletionRequestAssistantMessageArgs::default()
                                            .content(opening_message)
                                            .build()
                                            .unwrap()
                                    ));
                                });

                                app_state.add_log(
                                    LogLevel::Success,
                                    "Wizard completed! Starting AI conversation...".to_string()
                                );
                            }
                        }
                    }
                    });
            }
            return; // Don't show main UI while wizard is active
        }
    }

    // Main UI (after wizard completes)

        // Top bar
        if let Ok(ctx) = contexts.ctx_mut() {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("AI RPG Generator");
                    ui.separator();

                    // Phase indicator
                    ui.label(format!("Phase: {:?}", app_state.current_phase));
                    ui.separator();

                    // API Key status
                    if app_state.api_key_set {
                        ui.colored_label(egui::Color32::GREEN, "✓ API Key Set");
                    } else {
                        ui.colored_label(egui::Color32::RED, "✗ API Key Missing");
                        ui.label("Set OPENAI_API_KEY environment variable");
                    }

                    ui.separator();

                    // Generation controls
                    if app_state.generation_active {
                        if ui.button("⏸ Pause").clicked() {
                            app_state.set_generation_active(false);
                        }
                    } else {
                        if ui.button("▶ Start").clicked() {
                            app_state.set_generation_active(true);
                        }
                    }

                    ui.separator();

                    // Mode switch button
                    if ui.button("📂 Browse Projects").clicked() {
                        switch_mode_events.write(SwitchModeEvent {
                            new_mode: AppMode::List,
                            project_path: None,
                        });
                    }
            });
            });
        }

        // Tab bar
        if let Ok(ctx) = contexts.ctx_mut() {
            egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut ui_state.selected_tab, SelectedTab::Conversation, "💬 Conversation");
            ui.selectable_value(&mut ui_state.selected_tab, SelectedTab::Prompts, "📝 Prompts");
            ui.selectable_value(&mut ui_state.selected_tab, SelectedTab::Assets, "🎨 Assets");
            ui.selectable_value(&mut ui_state.selected_tab, SelectedTab::Build, "🔨 Build");
            ui.selectable_value(&mut ui_state.selected_tab, SelectedTab::Logs, "📋 Logs");
        });
            });

            // Main panel with selected tab content
            egui::CentralPanel::default().show(ctx, |ui| {
                match ui_state.selected_tab {
                    SelectedTab::Conversation => conversation_tab(ui, &mut app_state, &mut ui_state.input_text),
                    SelectedTab::Prompts => prompts_tab(ui, &mut app_state, &directories),
                    SelectedTab::Assets => assets_tab(ui, &mut app_state),
                    SelectedTab::Build => build_tab(ui, &mut app_state, &directories),
                    SelectedTab::Logs => logs_tab(ui, &mut app_state),
                }
            });
        }
}

fn conversation_tab(ui: &mut egui::Ui, app_state: &mut AppState, input_text: &mut String) {
    ui.vertical(|ui| {
        ui.heading("Metaprompt Conversation");
        ui.separator();

        // Game config display
        if let Some(config) = &app_state.game_specification {
            ui.group(|ui| {
                ui.label(format!("Game: {}", config.name));
                ui.label(format!("Genre: {}", config.genre));
                ui.label(format!("Style: {:?}", config.art_style));
            });
        }

        ui.separator();

        // Conversation history
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                // TODO: Show conversation messages from ConversationState
                ui.label("Conversation history will appear here...");
            });

        ui.separator();

        // Input area
        ui.horizontal(|ui| {
            let response = ui.text_edit_singleline(input_text);
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                // TODO: Send message
                app_state.add_log(
                    LogLevel::Info,
                    format!("User input: {}", input_text)
                );
                input_text.clear();
            }
        });
    });
}

fn prompts_tab(ui: &mut egui::Ui, app_state: &mut AppState, directories: &AppDirectories) {
    ui.vertical(|ui| {
        ui.heading("Prompt Management");
        ui.separator();

        // Show validation queue
        ui.group(|ui| {
            ui.label("Validation Queue:");
            for prompt in &app_state.prompt_validation_queue {
                ui.horizontal(|ui| {
                    if prompt.validated {
                        if prompt.errors.is_empty() {
                            ui.colored_label(egui::Color32::GREEN, "✓");
                        } else {
                            ui.colored_label(egui::Color32::RED, "✗");
                        }
                    } else {
                        ui.colored_label(egui::Color32::YELLOW, "⏳");
                    }

                    ui.label(format!("{}/{}", prompt.phase, prompt.name));

                    if !prompt.errors.is_empty() {
                        ui.collapsing("Errors", |ui| {
                            for error in &prompt.errors {
                                ui.label(format!("• {}", error));
                            }
                        });
                    }
                });
            }
        });

        ui.separator();

        // Browse prompts by phase
        ui.collapsing("Browse Prompts", |ui| {
            let phases = [
                "01_design",
                "02_style",
                "03_world",
                "04_ai_systems",
                "05_assets",
                "06_code",
                "07_dialog",
                "08_music",
                "09_integration",
            ];

            for phase in phases {
                ui.collapsing(phase, |ui| {
                    if let Ok(prompts) = directories.get_phase_prompts(phase) {
                        for prompt_path in prompts {
                            if let Some(file_name) = prompt_path.file_name() {
                                if ui.button(file_name.to_string_lossy().as_ref()).clicked() {
                                    // TODO: Open prompt in editor
                                    app_state.add_log(
                                        LogLevel::Info,
                                        format!("Opening prompt: {:?}", file_name)
                                    );
                                }
                            }
                        }
                    }
                });
            }
        });
    });
}

fn assets_tab(ui: &mut egui::Ui, _app_state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading("Generated Assets");
        ui.separator();

        // Asset gallery
        ui.label("Generated assets will appear here...");

        // TODO: Display generated images using egui::Image
        // TODO: List audio files
        // TODO: Show sprite sheets
    });
}

fn build_tab(ui: &mut egui::Ui, app_state: &mut AppState, directories: &AppDirectories) {
    ui.vertical(|ui| {
        ui.heading("Build Output");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("📁 Open Build Directory").clicked() {
                let build_dir = directories.project_dir.join("build");
                if let Err(e) = open::that(&build_dir) {
                    app_state.add_log(
                        LogLevel::Error,
                        format!("Failed to open build directory: {}", e)
                    );
                }
            }

            if ui.button("🎮 Run Game").clicked() {
                // TODO: Launch the generated game
                app_state.add_log(
                    LogLevel::Info,
                    "Launching generated game...".to_string()
                );
            }
        });

        ui.separator();

        // File tree of build directory
        ui.label("Build directory contents:");
        // TODO: Show file tree
    });
}

fn logs_tab(ui: &mut egui::Ui, app_state: &mut AppState) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.heading("Logs");
            if ui.button("Clear").clicked() {
                app_state.clear_logs();
            }
        });
        ui.separator();

        // Log display
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for log in &app_state.logs {
                    ui.horizontal(|ui| {
                        // Timestamp
                        let timestamp = chrono::DateTime::<chrono::Local>::from(log.timestamp);
                        ui.monospace(format!("{}", timestamp.format("%H:%M:%S")));

                        // Level with color
                        let (symbol, color) = match log.level {
                            LogLevel::Info => ("ℹ", egui::Color32::LIGHT_BLUE),
                            LogLevel::Warning => ("⚠", egui::Color32::YELLOW),
                            LogLevel::Error => ("✗", egui::Color32::RED),
                            LogLevel::Success => ("✓", egui::Color32::GREEN),
                        };
                        ui.colored_label(color, symbol);

                        // Message
                        ui.label(&log.message);
                    });
                }
            });
    });
}
