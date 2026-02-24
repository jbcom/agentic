use crate::wizard::pipeline::GenerationPipeline;
use crate::wizard::{
    AppDirectories, SwitchModeEvent,
    config::ConfigManager,
    state::{AppState, LogLevel, WizardStep},
    steps::{
        draw_welcome_step,
        freeform::{
            ConversationStream, FreeformModeState, render_freeform_mode, setup_freeform_mode,
        },
        guided::{GuidedModeState, render_guided_mode, setup_guided_mode},
    },
};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedTab {
    Conversation,
    Prompts,
    Assets,
    Build,
    Logs,
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: std::time::SystemTime,
    pub level: LogLevel,
    pub message: String,
}

#[allow(clippy::too_many_arguments)]
pub fn draw_generate_ui(
    mut contexts: EguiContexts,
    mut app_state: ResMut<AppState>,
    directories: Res<AppDirectories>,
    pipeline: Res<GenerationPipeline>,
    _switch_mode_events: EventWriter<SwitchModeEvent>,
    guided_state: Option<ResMut<GuidedModeState>>,
    freeform_state: Option<ResMut<FreeformModeState>>,
    stream_res: Option<ResMut<ConversationStream>>,
    commands: Commands,
    mut exit_events: EventWriter<AppExit>,
) {
    trace!("draw_generate_ui called");

    let ctx = match contexts.ctx_mut() {
        Ok(ctx) => ctx,
        Err(_) => {
            // EguiContext not ready yet - this happens in the first frame
            // Don't log as error since this is expected behavior
            trace!("EguiContext not ready yet, skipping frame");
            return;
        }
    };

    debug!(
        "Successfully got EguiContext, current wizard step: {:?}",
        app_state.wizard_step
    );

    if app_state.config_manager.is_none()
        && let Ok(config_manager) = ConfigManager::new(&directories.project_dir, None)
    {
        app_state.set_config_manager(config_manager);
    }

    // Handle exit dialog
    if app_state.show_exit_dialog {
        egui::Window::new("Exit Confirmation")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("Are you sure you want to exit?");
                ui.add_space(10.0);
                ui.label("Any unsaved progress will be lost.");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        app_state.show_exit_dialog = false;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(
                                egui::RichText::new("Exit")
                                    .color(egui::Color32::from_rgb(255, 100, 100)),
                            )
                            .clicked()
                        {
                            exit_events.write(AppExit::Success);
                        }
                    });
                });
            });
    }

    // Draw wizard steps based on current state - ONLY WELCOME → GUIDED flow
    match &app_state.wizard_step {
        WizardStep::Welcome => {
            debug!("Drawing welcome step");
            draw_wizard_frame_with_state(ctx, &mut app_state, |ui, state| {
                let wizard_mode_action = draw_welcome_step(ui, &mut state.config_manager);
                if let Some(action) = wizard_mode_action {
                    info!("Welcome action selected: {:?}", action);
                    state.set_wizard_mode(action);
                }
            });
        }
        WizardStep::GuidedMode => {
            debug!("Drawing guided mode step");
            // Guided mode handles its own UI completely
            // This is where the user browses and blends vintage games
            if let Some(guided_state) = guided_state {
                debug!("Guided state exists, rendering guided mode");
                render_guided_mode(contexts, app_state, guided_state);
            } else {
                // Need to setup guided mode resources
                warn!("No guided state found, setting up guided mode");
                setup_guided_mode(commands);

                // Show loading state for this frame
                draw_wizard_frame(ctx, &mut app_state, |ui| {
                    ui.label("Loading guided mode...");
                });
            }
        }
        WizardStep::FreeformMode => {
            debug!("Drawing freeform mode step");
            if let (Some(freeform_state), Some(stream_res)) = (freeform_state, stream_res) {
                render_freeform_mode(
                    contexts,
                    app_state,
                    freeform_state,
                    commands,
                    pipeline,
                    stream_res,
                );
            } else {
                warn!("No freeform state found, setting up freeform mode");
                setup_freeform_mode(commands);

                draw_wizard_frame(ctx, &mut app_state, |ui| {
                    ui.label("Loading freeform mode...");
                });
            }
        }
        WizardStep::Complete => {
            draw_wizard_frame_with_state(ctx, &mut app_state, |ui, state| {
                ui.heading("🎉 Export Complete!");
                ui.separator();

                if let Some(export) = &state.guided_export {
                    ui.label("Successfully exported game configuration");
                    ui.label(format!("Blend: {}", export.blend_name));

                    ui.separator();

                    if ui.button("Start New Blend").clicked() {
                        state.set_wizard_step(WizardStep::Welcome);
                        state.guided_export = None;
                    }
                } else {
                    ui.label("Export completed successfully!");

                    if ui.button("Return to Welcome").clicked() {
                        state.set_wizard_step(WizardStep::Welcome);
                    }
                }
            });
        }
        _ => {
            warn!("Unhandled wizard step: {:?}", app_state.wizard_step);
            // Any other step that isn't implemented yet
            draw_wizard_frame_with_state(ctx, &mut app_state, |ui, state| {
                ui.heading("🚧 Under Construction");
                ui.label("This feature is not implemented yet.");
                ui.add_space(20.0);

                // Show pipeline status for debugging
                ui.separator();
                ui.label("Pipeline initialized");

                if ui.button("← Back to Welcome").clicked() {
                    state.set_wizard_step(WizardStep::Welcome);
                }
            });
        }
    }
}

fn draw_wizard_frame<T>(
    ctx: &egui::Context,
    app_state: &mut AppState,
    content: impl FnOnce(&mut egui::Ui) -> T,
) -> T {
    let mut result = None;
    egui::CentralPanel::default().show(ctx, |ui| {
        // Header
        ui.horizontal(|ui| {
            ui.heading("🎮 Vintage Game Generator");
            ui.separator();
            ui.label(app_state.get_step_title());
        });
        ui.separator();

        // Progress bar
        let progress = app_state.get_progress();
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(true);
        ui.add(progress_bar);
        ui.separator();

        // Main content area
        egui::ScrollArea::vertical().show(ui, |ui| {
            result = Some(content(ui));
        });

        ui.separator();

        // Navigation
        ui.horizontal(|ui| {
            if app_state.can_go_back() && ui.button("← Back").clicked() {
                app_state.go_back();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Exit").clicked() {
                    app_state.show_exit_dialog = true;
                }
            });
        });
    });
    result.unwrap()
}

fn draw_wizard_frame_with_state(
    ctx: &egui::Context,
    app_state: &mut AppState,
    content: impl FnOnce(&mut egui::Ui, &mut AppState),
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Header
        ui.horizontal(|ui| {
            ui.heading("🎮 Vintage Game Generator");
            ui.separator();
            ui.label(app_state.get_step_title());
        });
        ui.separator();

        // Progress bar
        let progress = app_state.get_progress();
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(true);
        ui.add(progress_bar);
        ui.separator();

        // Main content area
        egui::ScrollArea::vertical().show(ui, |ui| {
            content(ui, app_state);
        });

        ui.separator();

        // Navigation
        ui.horizontal(|ui| {
            if app_state.can_go_back() && ui.button("← Back").clicked() {
                app_state.go_back();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Exit").clicked() {
                    app_state.show_exit_dialog = true;
                }
            });
        });
    });
}
