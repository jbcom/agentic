use crate::wizard::mode::AppMode;
use anyhow::{Context, Result};
use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone, Resource)]
pub struct AppDirectories {
    pub base_dir: PathBuf,
    pub project_dir: PathBuf,
    pub config_file: Option<PathBuf>,
    pub prompts_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub mode: AppMode,
}

impl AppDirectories {
    pub fn ensure_directories_exist(&self) -> Result<()> {
        std::fs::create_dir_all(&self.base_dir).context("Failed to create base directory")?;

        match self.mode {
            AppMode::Generate => {
                std::fs::create_dir_all(&self.project_dir)
                    .context("Failed to create project directory")?;
                std::fs::create_dir_all(&self.prompts_dir)
                    .context("Failed to create prompts directory")?;
                std::fs::create_dir_all(&self.assets_dir)
                    .context("Failed to create assets directory")?;
            }
            AppMode::List => {
                // List mode only needs the base directory
            }
        }

        // Create subdirectories for different prompt phases
        let prompt_phases = [
            "01_design",
            "02_style",
            "03_world",
            "04_ai_systems",
            "05_assets",
            "06_code",
            "07_dialog",
            "08_music",
            "09_integration",
            "generated", // For AI-generated prompts
            "validated", // For validated prompts
        ];

        for phase in prompt_phases {
            std::fs::create_dir_all(self.prompts_dir.join(phase))
                .context(format!("Failed to create prompts/{phase} directory"))?;
        }

        Ok(())
    }

    /// List all project directories (UUID subdirectories) in the base directory
    pub fn list_project_dirs(&self) -> Result<Vec<(PathBuf, Option<String>)>> {
        let mut projects = Vec::new();

        if self.base_dir.exists() {
            for entry in std::fs::read_dir(&self.base_dir)? {
                let entry = entry?;
                let path = entry.path();

                // Check if it's a directory and looks like a UUID
                if path.is_dir()
                    && let Some(dir_name) = path.file_name()
                {
                    let dir_str = dir_name.to_string_lossy();
                    // Basic UUID format check (8-4-4-4-12 hex characters)
                    if dir_str.len() == 36 && dir_str.chars().filter(|&c| c == '-').count() == 4 {
                        // Try to load the project name from project.toml
                        let config_path = path.join("project.toml");
                        let project_name = if config_path.exists() {
                            Self::read_project_name(&config_path)
                        } else {
                            None
                        };
                        projects.push((path, project_name));
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        projects.sort_by(|a, b| {
            let time_a = std::fs::metadata(&a.0)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let time_b = std::fs::metadata(&b.0)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            time_b.cmp(&time_a)
        });

        Ok(projects)
    }

    fn read_project_name(config_path: &PathBuf) -> Option<String> {
        let content = std::fs::read_to_string(config_path).ok()?;
        let value: toml::Value = toml::from_str(&content).ok()?;
        value.get("name")?.as_str().map(|s| s.to_string())
    }

    pub fn get_generated_prompt_path(&self, phase: &str, name: &str) -> PathBuf {
        self.prompts_dir
            .join("generated")
            .join(phase)
            .join(format!("{name}.jinja"))
    }

    pub fn get_validated_prompt_path(&self, phase: &str, name: &str) -> PathBuf {
        self.prompts_dir
            .join("validated")
            .join(phase)
            .join(format!("{name}.jinja"))
    }

    pub fn get_phase_prompts(&self, phase: &str) -> Result<Vec<PathBuf>> {
        let phase_dir = self.prompts_dir.join(phase);
        let mut prompts = Vec::new();

        if phase_dir.exists() {
            for entry in std::fs::read_dir(&phase_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path
                    .extension()
                    .is_some_and(|ext| ext == "jinja" || ext == "jinja2")
                {
                    prompts.push(path);
                }
            }
        }

        Ok(prompts)
    }
}
