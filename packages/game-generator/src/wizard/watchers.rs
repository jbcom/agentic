use crate::wizard::{
    directories::AppDirectories,
    state::{AppState, LogLevel},
};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, bounded};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub event_type: FileEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileEventType {
    Created,
    Modified,
    Removed,
}

#[derive(Resource)]
pub struct PromptWatcher {
    _watcher: notify::RecommendedWatcher,
    rx: Receiver<Result<Event, notify::Error>>,
}

impl PromptWatcher {
    pub fn new(directories: &AppDirectories) -> Result<Self, notify::Error> {
        let (tx, rx) = bounded(100);

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;

        // Watch the prompts directory
        watcher.watch(&directories.prompts_dir, RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    pub fn poll_events(&mut self) -> Vec<FileChangeEvent> {
        let mut events = Vec::new();

        while let Ok(Ok(event)) = self.rx.try_recv() {
            match event.kind {
                EventKind::Create(_) => {
                    for path in event.paths {
                        if is_prompt_file(&path) {
                            events.push(FileChangeEvent {
                                path,
                                event_type: FileEventType::Created,
                            });
                        }
                    }
                }
                EventKind::Modify(_) => {
                    for path in event.paths {
                        if is_prompt_file(&path) {
                            events.push(FileChangeEvent {
                                path,
                                event_type: FileEventType::Modified,
                            });
                        }
                    }
                }
                EventKind::Remove(_) => {
                    for path in event.paths {
                        if is_prompt_file(&path) {
                            events.push(FileChangeEvent {
                                path,
                                event_type: FileEventType::Removed,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        events
    }
}

fn is_prompt_file(path: &std::path::Path) -> bool {
    path.extension()
        .map(|ext| ext == "jinja" || ext == "jinja2")
        .unwrap_or(false)
}

/// Check for prompt file changes
pub fn check_prompt_changes(
    mut app_state: ResMut<AppState>,
    directories: Res<AppDirectories>,
    watcher_option: Option<ResMut<PromptWatcher>>,
) {
    let mut watcher = match watcher_option {
        Some(w) => w,
        None => {
            // Try to create watcher if it doesn't exist
            match PromptWatcher::new(&directories) {
                Ok(_new_watcher) => {
                    app_state.add_log(
                        LogLevel::Info,
                        "Started watching prompts directory for changes".to_string(),
                    );
                    // We can't insert the resource from here, so just return
                    return;
                }
                Err(e) => {
                    app_state.add_log(
                        LogLevel::Error,
                        format!("Failed to create file watcher: {e}"),
                    );
                    return;
                }
            }
        }
    };

    let events = watcher.poll_events();

    for event in events {
        match event.event_type {
            FileEventType::Created => {
                app_state.add_log(
                    LogLevel::Info,
                    format!("New prompt detected: {:?}", event.path.file_name()),
                );

                // Check if it's in the generated directory
                if event.path.to_string_lossy().contains("generated")
                    && let (Some(content), Some(file_name)) = (
                        std::fs::read_to_string(&event.path).ok(),
                        event.path.file_stem().and_then(|s| s.to_str()),
                    )
                {
                    // Extract phase from path
                    let phase = event
                        .path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    app_state.add_prompt_to_validate(
                        phase,
                        file_name.to_string(),
                        content,
                        event.path.clone(),
                    );

                    app_state.add_log(
                        LogLevel::Info,
                        format!("Added {file_name} to validation queue"),
                    );
                }
            }
            FileEventType::Modified => {
                app_state.add_log(
                    LogLevel::Info,
                    format!("Prompt modified: {:?}", event.path.file_name()),
                );

                // If it's a validated prompt that was modified, we might want to re-validate
                if event.path.to_string_lossy().contains("validated") {
                    app_state.add_log(
                        LogLevel::Warning,
                        format!(
                            "Validated prompt was modified: {:?}",
                            event.path.file_name()
                        ),
                    );
                }
            }
            FileEventType::Removed => {
                app_state.add_log(
                    LogLevel::Warning,
                    format!("Prompt removed: {:?}", event.path.file_name()),
                );
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct ConfigModificationTracker {
    last_modified: Option<SystemTime>,
}

/// Watch for config file changes
pub fn check_config_changes(
    mut app_state: ResMut<AppState>,
    directories: Res<AppDirectories>,
    mut tracker: ResMut<ConfigModificationTracker>,
) {
    let config_file = directories.project_dir.join("project.toml");

    if !config_file.exists() {
        return;
    }

    // Check modification time
    if let Ok(metadata) = std::fs::metadata(&config_file)
        && let Ok(modified) = metadata.modified()
    {
        // Compare with last known modification time
        if tracker.last_modified.is_none() {
            tracker.last_modified = Some(modified);
        } else if tracker.last_modified != Some(modified) {
            tracker.last_modified = Some(modified);

            app_state.add_log(
                LogLevel::Info,
                "Configuration file changed, reloading...".to_string(),
            );

            // Reload config
            if let Ok(content) = std::fs::read_to_string(&config_file) {
                match toml::from_str::<crate::metaprompts::GameConfig>(&content) {
                    Ok(config) => {
                        app_state.set_game_specification(config);
                        app_state.add_log(
                            LogLevel::Success,
                            "Configuration reloaded successfully".to_string(),
                        );
                    }
                    Err(e) => {
                        app_state.add_log(LogLevel::Error, format!("Failed to parse config: {e}"));
                    }
                }
            }
        }
    }
}
