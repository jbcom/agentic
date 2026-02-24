use crate::wizard::{AppDirectories, AppMode, SwitchModeEvent, config::ProjectConfig};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Resource, Default)]
pub struct ListModeState {
    pub projects: Vec<(Uuid, ProjectConfig, PathBuf)>,
    pub selected_index: usize,
    pub error: Option<String>,
}

pub fn draw_list_ui(
    mut contexts: EguiContexts,
    directories: Res<AppDirectories>,
    mut state: ResMut<ListModeState>,
    mut switch_mode_events: EventWriter<SwitchModeEvent>,
) {
    // Load projects on first run
    if state.projects.is_empty() {
        match load_projects(&directories.base_dir) {
            Ok(projects) => state.projects = projects,
            Err(e) => state.error = Some(format!("Failed to load projects: {e}")),
        }
    }

    // Create simple tab state
    static mut SELECTED_TAB: &str = "details";

    // Top panel
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading("Browse Projects");
            ui.separator();
            ui.label(format!(
                "Found {} projects in: {}",
                state.projects.len(),
                directories.base_dir.display()
            ));
        });
    }

    // Bottom panel with navigation
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::TopBottomPanel::bottom("navigation").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("◀ Previous").clicked() && state.selected_index > 0 {
                    state.selected_index -= 1;
                }

                ui.separator();

                if ui.button("Edit").clicked()
                    && let Some((uuid, _, path)) = state.projects.get(state.selected_index)
                {
                    info!("Switching to edit mode for project: {}", uuid);
                    // Send event to switch to Generate mode with this project
                    switch_mode_events.write(SwitchModeEvent {
                        new_mode: AppMode::Generate,
                        project_path: Some(path.clone()),
                    });
                }

                ui.separator();

                if ui.button("Next ▶").clicked()
                    && state.selected_index < state.projects.len().saturating_sub(1)
                {
                    state.selected_index += 1;
                }
            });
        });
    }

    // Project list on top
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::TopBottomPanel::top("project_list").show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut new_index = None;
                    for (i, (_, config, _)) in state.projects.iter().enumerate() {
                        let is_selected = i == state.selected_index;
                        if ui
                            .selectable_label(is_selected, &config.basic_info.name)
                            .clicked()
                        {
                            new_index = Some(i);
                        }
                    }
                    if let Some(idx) = new_index {
                        state.selected_index = idx;
                    }
                });
            });
        });
    }

    // Main content area with tabs
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab bar
            ui.horizontal(|ui| unsafe {
                if ui
                    .selectable_label(SELECTED_TAB == "details", "Project Details")
                    .clicked()
                {
                    SELECTED_TAB = "details";
                }
                if ui
                    .selectable_label(SELECTED_TAB == "files", "Files")
                    .clicked()
                {
                    SELECTED_TAB = "files";
                }
                if ui
                    .selectable_label(SELECTED_TAB == "preview", "Preview")
                    .clicked()
                {
                    SELECTED_TAB = "preview";
                }
            });

            ui.separator();

            // Tab content
            unsafe {
                match SELECTED_TAB {
                    "details" => draw_project_details(ui, &state),
                    "files" => draw_project_files(ui, &state),
                    "preview" => draw_project_preview(ui, &state),
                    _ => {}
                }
            }

            // Error display
            if let Some(error) = &state.error {
                ui.colored_label(egui::Color32::RED, format!("Error: {error}"));
            }
        });
    }
}

fn draw_project_details(ui: &mut egui::Ui, state: &ListModeState) {
    if let Some((uuid, config, path)) = state.projects.get(state.selected_index) {
        ui.heading(&config.basic_info.name);
        ui.label(&config.basic_info.tagline);
        ui.separator();

        ui.group(|ui| {
            ui.label(format!("UUID: {uuid}"));
            ui.label(format!("Path: {}", path.display()));
            ui.label(format!("Genre: {}", config.basic_info.genre));
            // Theme and tone fields don't exist in BasicInfo
        });

        ui.separator();

        ui.collapsing("Gameplay", |ui| {
            ui.label(format!(
                "Core Mechanics: {}",
                config.gameplay.core_mechanics.join(", ")
            ));
            ui.label(format!(
                "Progression Type: {}",
                config.gameplay.progression_type
            ));
            ui.label(format!("Gameplay Loop: {}", config.gameplay.gameplay_loop));
            ui.label(format!(
                "Player Motivation: {}",
                config.gameplay.player_motivation
            ));
        });

        ui.collapsing("Visual Style", |ui| {
            ui.label(format!(
                "Reference Games: {}",
                config.visual_style.reference_games.join(", ")
            ));
            ui.label(format!("Color Mood: {}", config.visual_style.color_mood));
            ui.label(format!(
                "Sprite Size: {} px",
                config.visual_style.sprite_size
            ));
            ui.label(format!("Use Outline: {}", config.visual_style.use_outline));
        });

        ui.collapsing("Technical", |ui| {
            ui.label(format!("World Size: {}", config.technical.world_size));
            ui.label(format!(
                "Target Platforms: {}",
                config.technical.target_platforms.join(", ")
            ));
        });
    } else {
        ui.label("No project selected");
    }
}

fn draw_project_files(ui: &mut egui::Ui, state: &ListModeState) {
    if let Some((_, _, path)) = state.projects.get(state.selected_index) {
        ui.heading("Project Files");
        ui.separator();

        // List files in project directory
        if let Ok(entries) = std::fs::read_dir(path.parent().unwrap()) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    ui.label(name);
                }
            }
        } else {
            ui.label("Could not read project directory");
        }
    } else {
        ui.label("No project selected");
    }
}

fn draw_project_preview(ui: &mut egui::Ui, state: &ListModeState) {
    if let Some((_, config, _)) = state.projects.get(state.selected_index) {
        ui.heading("Preview");
        ui.separator();

        // Show a preview of what would be generated
        ui.label("This project would generate:");
        ui.group(|ui| {
            ui.label(format!("• A {} game", config.basic_info.genre));
            ui.label(format!(
                "• Inspired by {}",
                config.visual_style.reference_games.join(", ")
            ));
            ui.label(format!("• {} gameplay", config.gameplay.progression_type));
            ui.label(format!("• {} mood visuals", config.visual_style.color_mood));
        });
    } else {
        ui.label("No project selected");
    }
}

#[allow(clippy::type_complexity)]
fn load_projects(
    base_dir: &std::path::Path,
) -> Result<Vec<(Uuid, ProjectConfig, PathBuf)>, Box<dyn std::error::Error>> {
    let mut projects = Vec::new();

    // Read all directories in base dir
    for entry in std::fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a directory
        if !path.is_dir() {
            continue;
        }

        // Try to parse as UUID
        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str())
            && let Ok(uuid) = Uuid::parse_str(dir_name)
        {
            // Look for project.toml
            let config_path = path.join("project.toml");
            if config_path.exists() {
                match ProjectConfig::load(&config_path) {
                    Ok(config) => {
                        projects.push((uuid, config, config_path));
                    }
                    Err(e) => {
                        warn!("Failed to load project config at {:?}: {}", config_path, e);
                    }
                }
            }
        }
    }

    // Sort by project name
    projects.sort_by(|a, b| a.1.basic_info.name.cmp(&b.1.basic_info.name));

    Ok(projects)
}
