// wizard.rs - Constrained visual interface for the metaprompt generator
// Following Rust 2024 module pattern - no mod.rs needed

use bevy::prelude::*;
use bevy_egui::EguiContexts;

// Submodules in wizard/ directory
pub mod config;
pub mod directories;
pub mod generate_mode;
pub mod image_loader;
pub mod list_mode;
pub mod mode;
pub mod overlay;
pub mod pipeline;
pub mod state;
pub mod steps;
pub mod watchers;

pub use directories::AppDirectories;
pub use mode::{AppMode, SwitchModeEvent};
pub use pipeline::GenerationPipeline;
pub use state::AppState;

pub struct WizardPlugin;

impl Plugin for WizardPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing WizardPlugin");

        // Add app state and pipeline
        app.insert_resource(AppState::new())
            .insert_resource(GenerationPipeline::new())
            .insert_resource(watchers::ConfigModificationTracker::default())
            .add_event::<SwitchModeEvent>()
            .add_systems(Startup, setup_app)
            .add_systems(Update, handle_mode_switch);

        // Add mode-specific systems with run conditions
        // Important: UI systems are added in Update set, which runs after EguiPlugin setup
        app.add_systems(
            Update,
            (
                apply_theme,
                generate_mode::draw_generate_ui.run_if(in_mode(AppMode::Generate)),
                watchers::check_prompt_changes.run_if(in_mode(AppMode::Generate)),
                pipeline::process_generation_queue.run_if(in_mode(AppMode::Generate)),
                steps::freeform::process_conversation_stream.run_if(in_mode(AppMode::Generate)),
                list_mode::draw_list_ui.run_if(in_mode(AppMode::List)),
            ),
        );

        info!("WizardPlugin setup complete");
    }
}

/// Run condition to check if we're in a specific mode
fn in_mode(mode: AppMode) -> impl Fn(Res<AppMode>) -> bool {
    move |current_mode: Res<AppMode>| *current_mode == mode
}

/// Handle mode switch events
fn handle_mode_switch(
    mut events: EventReader<SwitchModeEvent>,
    mut mode: ResMut<AppMode>,
    mut directories: ResMut<AppDirectories>,
    mut windows: Query<&mut Window>,
) {
    for event in events.read() {
        info!("Switching from {:?} to {:?}", *mode, event.new_mode);

        // Update the mode
        *mode = event.new_mode;

        // Update window title
        if let Ok(mut window) = windows.single_mut() {
            window.title = match event.new_mode {
                AppMode::Generate => "Vintage Game Generator".to_string(),
                AppMode::List => "Vintage Game Generator - Browse Projects".to_string(),
            };
        }

        // If switching to Generate mode with a specific project
        if let (AppMode::Generate, Some(project_path)) = (event.new_mode, &event.project_path) {
            info!("Loading project from: {:?}", project_path);

            // Update directories to point to the selected project
            let project_dir = project_path.parent().unwrap_or(project_path).to_path_buf();

            directories.project_dir = project_dir.clone();
            directories.config_file = Some(project_path.to_path_buf());
            directories.prompts_dir = project_dir.join("prompts");
            directories.assets_dir = project_dir.join("assets");

            // Ensure the project directories exist
            if let Err(e) = directories.ensure_directories_exist() {
                error!("Failed to create project directories: {}", e);
            }
        }
    }
}

fn setup_app(_commands: Commands, directories: Res<AppDirectories>, mode: Res<AppMode>) {
    info!("AI RPG Generator starting up in {:?} mode", mode);
    info!("Base dir: {:?}", directories.base_dir);

    match *mode {
        AppMode::Generate => {
            info!("Project dir: {:?}", directories.project_dir);
            info!("Prompts dir: {:?}", directories.prompts_dir);
            info!("Assets dir: {:?}", directories.assets_dir);
        }
        AppMode::List => {
            info!("Browsing projects in: {:?}", directories.base_dir);
        }
    }

    // Create directories if they don't exist
    if let Err(e) = directories.ensure_directories_exist() {
        error!("Failed to create directories: {}", e);
    }
}

fn apply_theme(mut contexts: EguiContexts) {
    match contexts.ctx_mut() {
        Ok(ctx) => {
            trace!("Applying Catppuccin Mocha theme to egui context");
            // Apply Catppuccin Mocha theme for a modern, soothing look
            catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        }
        Err(_) => {
            // This is expected in the first frame before EguiPlugin initializes
            trace!("EguiContext not ready for theme application yet");
        }
    }
}
