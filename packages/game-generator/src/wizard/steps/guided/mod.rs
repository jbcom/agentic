// Re-export the comprehensive implementation modules
pub mod blend;
pub mod game_card;
pub mod timeline;
pub mod types;

// Re-export key types and functions
pub use blend::{
    create_blend, export_blend_to_config, render_blend_ui, render_blend_visualization,
    render_export_ui,
};
pub use game_card::render_game_card;
pub use timeline::render_timeline;
pub use types::{BlendResult, Conflict, GuidedModeExport, GuidedModeState, SourceGame, Synergy};

use crate::wizard::state::{AppState, WizardStep};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

/// Set up guided mode resources when entering this step
pub fn setup_guided_mode(mut commands: Commands) {
    commands.insert_resource(GuidedModeState::default());
}

/// Clean up guided mode resources when leaving this step
pub fn cleanup_guided_mode(mut commands: Commands) {
    commands.remove_resource::<GuidedModeState>();
}

/// Main render function for guided mode
pub fn render_guided_mode(
    mut contexts: EguiContexts,
    mut app_state: ResMut<AppState>,
    mut guided_state: ResMut<GuidedModeState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        // Header
        ui.horizontal(|ui| {
            ui.heading("🎮 Guided Mode - Blend Vintage Games");
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
            // Step-based UI
            match guided_state.current_step {
                0 => {
                    // Timeline browsing
                    ui.label("Browse games by decade and select ones to blend:");
                    ui.separator();

                    render_timeline(ui, &mut guided_state);

                    // Show selected games count
                    if !guided_state.selected_games.is_empty() {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "Selected {} games",
                                guided_state.selected_games.len()
                            ));

                            if guided_state.selected_games.len() >= 2 {
                                if ui.button("🔀 Blend Games").clicked() {
                                    // Create the blend
                                    create_blend(&mut guided_state);
                                    guided_state.current_step = 1;
                                }
                            } else {
                                ui.label("(Select at least 2 games to blend)");
                            }
                        });
                    }
                }
                1 => {
                    // Blend visualization and export
                    if guided_state.blend_result.is_some() {
                        render_blend_ui(ui, &mut guided_state);

                        ui.separator();
                        if ui.button("⬅ Back to Selection").clicked() {
                            guided_state.current_step = 0;
                            guided_state.blend_result = None;
                        }

                        if ui.button("✅ Export Configuration").clicked()
                            && let Some(export) = export_blend_to_config(&guided_state)
                        {
                            // Store the export in app state
                            app_state.set_guided_export(export);
                            app_state.set_wizard_step(WizardStep::Complete);
                        }
                    } else {
                        // No blend result, go back
                        guided_state.current_step = 0;
                    }
                }
                _ => {
                    ui.label("Invalid step");
                }
            }

            // Selected games sidebar
            if !guided_state.selected_games.is_empty() {
                ui.separator();
                let mut games_to_remove = Vec::new();
                egui::CollapsingHeader::new("Selected Games")
                    .default_open(true)
                    .show(ui, |ui| {
                        let game_list: Vec<(u32, String, i32)> = guided_state
                            .selected_games
                            .iter()
                            .map(|(id, game)| (*id, game.name.to_string(), game.year))
                            .collect();

                        for (id, name, year) in game_list {
                            ui.horizontal(|ui| {
                                if ui.small_button("❌").clicked() {
                                    games_to_remove.push(id);
                                }
                                ui.label(&name);
                                ui.label(format!("({year})"));
                            });
                        }
                    });

                // Apply deferred removals
                for id in games_to_remove {
                    guided_state.toggle_game_selection(id);
                }
            }
        });

        ui.separator();

        // Navigation
        ui.horizontal(|ui| {
            if app_state.can_go_back() && ui.button("← Back").clicked() {
                app_state.go_back();
                // Cleanup is handled by the cleanup_guided_mode system
                // We just need to switch the wizard step
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Exit").clicked() {
                    app_state.show_exit_dialog = true;
                }
            });
        });
    });
}
