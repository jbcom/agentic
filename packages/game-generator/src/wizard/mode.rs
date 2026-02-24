use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Generate,
    List,
}

#[derive(Event)]
pub struct SwitchModeEvent {
    pub new_mode: AppMode,
    pub project_path: Option<PathBuf>,
}
