use crate::metaprompts::{GameGenerator, GenerationPhase};
use crate::wizard::{
    directories::AppDirectories,
    state::{AppState, LogLevel},
};
use anyhow::Result;
use bevy::prelude::*;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[derive(Clone, Resource)]
pub struct GenerationPipeline {
    pub runtime: Arc<Runtime>,
    pub generator: Arc<Mutex<Option<GameGenerator>>>,
    pub current_task: Option<GenerationTask>,
    pub rate_limiter: RateLimiter,
}

#[derive(Debug, Clone)]
pub struct GenerationTask {
    pub phase: GenerationPhase,
    pub prompt_name: String,
    pub started_at: std::time::Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub last_request: Option<std::time::Instant>,
    pub min_delay_ms: u64,
}

impl Default for GenerationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerationPipeline {
    pub fn new() -> Self {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime"),
        );

        Self {
            runtime,
            generator: Arc::new(Mutex::new(None)),
            current_task: None,
            rate_limiter: RateLimiter {
                last_request: None,
                min_delay_ms: 1000, // 1 second between requests
            },
        }
    }

    pub fn initialize_generator(
        &self,
        _api_key: String,
        _directories: &AppDirectories,
    ) -> Result<()> {
        let generator_arc = self.generator.clone();

        self.runtime.block_on(async move {
            let new_generator = GameGenerator::new().await?;
            let mut generator_lock = generator_arc.lock().await;
            *generator_lock = Some(new_generator);
            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }

    pub fn can_make_request(&self) -> bool {
        match self.rate_limiter.last_request {
            None => true,
            Some(last) => {
                let elapsed = last.elapsed().as_millis() as u64;
                elapsed >= self.rate_limiter.min_delay_ms
            }
        }
    }

    pub fn mark_request_made(&mut self) {
        self.rate_limiter.last_request = Some(std::time::Instant::now());
    }
}

/// Process the generation queue
pub fn process_generation_queue(
    mut pipeline: ResMut<GenerationPipeline>,
    mut app_state: ResMut<AppState>,
    directories: Res<AppDirectories>,
) {
    // Check if we're ready to process
    if !app_state.generation_active || !pipeline.can_make_request() {
        return;
    }

    // Check if we have a prompt to validate
    let prompt_to_validate = app_state.get_next_prompt_to_validate().cloned();
    if let Some(prompt) = prompt_to_validate {
        // Run validation
        let validation_errors = validate_prompt(&prompt.content);

        if validation_errors.is_empty() {
            // Move to validated directory
            let validated_path = directories.get_validated_prompt_path(&prompt.phase, &prompt.name);
            if let Err(e) = std::fs::create_dir_all(validated_path.parent().unwrap()) {
                app_state.add_log(
                    LogLevel::Error,
                    format!("Failed to create validated directory: {e}"),
                );
                return;
            }

            if let Err(e) = std::fs::write(&validated_path, &prompt.content) {
                app_state.add_log(
                    LogLevel::Error,
                    format!("Failed to write validated prompt: {e}"),
                );
            } else {
                app_state.add_log(
                    LogLevel::Success,
                    format!("Validated prompt: {}/{}", prompt.phase, prompt.name),
                );
            }
        } else {
            app_state.add_log(
                LogLevel::Warning,
                format!(
                    "Validation errors for {}: {:?}",
                    prompt.name, validation_errors
                ),
            );
        }

        app_state.mark_prompt_validated(&prompt.path, validation_errors);
        return;
    }

    // If all prompts are validated for current phase, advance
    let all_validated = app_state
        .prompt_validation_queue
        .iter()
        .all(|p| p.validated);

    if all_validated && !app_state.prompt_validation_queue.is_empty() {
        let current_phase = app_state.current_phase;
        app_state.add_log(
            LogLevel::Success,
            format!("Phase {current_phase:?} complete, advancing..."),
        );
        app_state.advance_phase();
        app_state.prompt_validation_queue.clear();

        // Start next phase generation
        if app_state.current_phase != GenerationPhase::Packaging {
            start_phase_generation(&mut pipeline, &mut app_state, &directories);
        }
    }
}

fn start_phase_generation(
    pipeline: &mut GenerationPipeline,
    app_state: &mut AppState,
    directories: &AppDirectories,
) {
    let current_phase = app_state.current_phase;
    app_state.add_log(
        LogLevel::Info,
        format!("Starting generation for phase: {current_phase:?}"),
    );

    // Mark that we're making a request
    pipeline.mark_request_made();

    // Map current phase to the template directory name
    let phase_dir = match current_phase {
        GenerationPhase::Design => "01_design",
        GenerationPhase::StyleGuide => "02_style",
        GenerationPhase::WorldGeneration => "03_world",
        GenerationPhase::AiSystems => "04_ai_systems",
        GenerationPhase::AssetGeneration => "05_assets",
        GenerationPhase::CodeGeneration => "06_code",
        GenerationPhase::DialogWriting => "07_dialog",
        GenerationPhase::MusicComposition => "08_music",
        GenerationPhase::Integration => "09_integration",
        _ => {
            app_state.add_log(
                LogLevel::Warning,
                format!("No template directory for phase: {current_phase:?}"),
            );
            return;
        }
    };

    // Load prompts from the phase directory and queue them for validation
    match directories.get_phase_prompts(phase_dir) {
        Ok(prompts) if !prompts.is_empty() => {
            for prompt_path in prompts {
                match std::fs::read_to_string(&prompt_path) {
                    Ok(content) => {
                        let name = prompt_path
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "unknown".to_string());
                        app_state.add_prompt_to_validate(
                            phase_dir.to_string(),
                            name,
                            content,
                            prompt_path,
                        );
                    }
                    Err(e) => {
                        app_state.add_log(
                            LogLevel::Error,
                            format!("Failed to read prompt {}: {e}", prompt_path.display()),
                        );
                    }
                }
            }
            app_state.add_log(
                LogLevel::Info,
                format!(
                    "Queued {} prompts for validation in phase {phase_dir}",
                    app_state
                        .prompt_validation_queue
                        .iter()
                        .filter(|p| p.phase == phase_dir)
                        .count()
                ),
            );
        }
        Ok(_) => {
            // No prompts found for this phase; generate a placeholder via the AI pipeline
            let generated_path = directories.get_generated_prompt_path(phase_dir, "main");
            let placeholder = format!(
                "{{% comment %}}Auto-generated placeholder for phase: {phase_dir}{{% endcomment %}}\n\
                 {{{{ game_name }}}}\n\
                 {{{{ genre }}}}\n"
            );

            if let Some(parent) = generated_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            match std::fs::write(&generated_path, &placeholder) {
                Ok(()) => {
                    app_state.add_prompt_to_validate(
                        phase_dir.to_string(),
                        "main".to_string(),
                        placeholder,
                        generated_path,
                    );
                    app_state.add_log(
                        LogLevel::Info,
                        format!("Generated placeholder prompt for phase {phase_dir}"),
                    );
                }
                Err(e) => {
                    app_state.add_log(
                        LogLevel::Error,
                        format!("Failed to write generated prompt: {e}"),
                    );
                }
            }
        }
        Err(e) => {
            app_state.add_log(
                LogLevel::Error,
                format!("Failed to list prompts for phase {phase_dir}: {e}"),
            );
        }
    }
}

fn validate_prompt(content: &str) -> Vec<String> {
    let mut errors = Vec::new();

    // Basic validation
    if content.trim().is_empty() {
        errors.push("Prompt is empty".to_string());
    }

    if content.len() < 50 {
        errors.push("Prompt seems too short".to_string());
    }

    // Check for required MinJinja syntax
    if !content.contains("{{") && !content.contains("{%") {
        errors.push("No template variables or logic found".to_string());
    }

    // Try to parse as MinJinja template
    match minijinja::Environment::new().add_template("test", content) {
        Ok(_) => {}
        Err(e) => errors.push(format!("MinJinja parse error: {e}")),
    }

    // Validate code blocks: ensure they are properly closed and check for known issues
    let fence_count = content.matches("```").count();
    if !fence_count.is_multiple_of(2) {
        errors.push("Unclosed code block detected".to_string());
    }

    // Check Rust code blocks for basic syntactic validity
    if content.contains("```rust") || content.contains("```rs") {
        let mut in_rust_block = false;
        let mut rust_content = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed == "```rust" || trimmed == "```rs" {
                in_rust_block = true;
                rust_content.clear();
            } else if trimmed == "```" && in_rust_block {
                in_rust_block = false;
                // Check for balanced braces/parens/brackets
                let open_braces = rust_content.chars().filter(|&c| c == '{').count();
                let close_braces = rust_content.chars().filter(|&c| c == '}').count();
                if open_braces != close_braces {
                    errors.push(format!(
                        "Unbalanced braces in Rust code block: {} open, {} close",
                        open_braces, close_braces
                    ));
                }
            } else if in_rust_block {
                rust_content.push_str(line);
                rust_content.push('\n');
            }
        }
    }

    // Check TOML code blocks for basic validity
    if content.contains("```toml") {
        let mut in_toml_block = false;
        let mut toml_content = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed == "```toml" {
                in_toml_block = true;
                toml_content.clear();
            } else if trimmed == "```" && in_toml_block {
                in_toml_block = false;
                if toml_content.parse::<toml::Value>().is_err() {
                    errors.push("Invalid TOML in code block".to_string());
                }
            } else if in_toml_block {
                toml_content.push_str(line);
                toml_content.push('\n');
            }
        }
    }

    errors
}
