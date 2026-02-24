//! Game generation extension methods for ConversationManager

use super::{
    manager::ConversationManager,
    starters,
    types::{GenerationPhase, GenerationProgress, MessageConfig},
};
use crate::game_types::{GameConfig, WorldData};
use anyhow::Result;
use minijinja::context;
use std::path::{Path, PathBuf};

/// Extension trait for game generation methods
#[async_trait::async_trait]
pub trait GameGenerationExt {
    /// Start a game design conversation with interactive refinement
    async fn start_game_design_conversation(
        &self,
        initial_prompt: &str,
        project_config: Option<serde_json::Value>,
    ) -> Result<(String, String)>;

    /// Continue game design conversation and check if design is complete
    async fn continue_game_design_conversation(
        &self,
        conversation_id: &str,
        user_input: &str,
    ) -> Result<(String, bool, Option<GameConfig>)>;

    /// Generate complete game with progress tracking
    async fn generate_full_game<F>(
        &self,
        config: &GameConfig,
        project_config: Option<serde_json::Value>,
        progress_callback: F,
    ) -> Result<String>
    where
        F: Fn(GenerationProgress) + Send + 'static;
}

#[async_trait::async_trait]
impl GameGenerationExt for ConversationManager {
    async fn start_game_design_conversation(
        &self,
        initial_prompt: &str,
        project_config: Option<serde_json::Value>,
    ) -> Result<(String, String)> {
        // Create conversation with game generation context
        let mut context = starters::game_generation_context(project_config.clone());

        // Load system prompt from template if available
        if let Some(env) = self.template_env.lock().await.as_ref()
            && let Ok(template) = env.get_template("01_design")
        {
            let system_prompt = if let Some(config) = &project_config {
                template.render(context!(project => config))?
            } else {
                template.render(context!())?
            };
            context.system_prompt = Some(system_prompt);
        }

        context.conversation_type = "game_design".to_string();
        let conversation_id = self
            .start_conversation(
                format!(
                    "Game Design: {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M")
                ),
                context,
            )
            .await?;

        // Send initial prompt
        let response = self
            .send_message(&conversation_id, initial_prompt.to_string())
            .await?;

        Ok((conversation_id, response))
    }

    async fn continue_game_design_conversation(
        &self,
        conversation_id: &str,
        user_input: &str,
    ) -> Result<(String, bool, Option<GameConfig>)> {
        let response = self
            .send_message(conversation_id, user_input.to_string())
            .await?;

        // Check if design is complete
        let is_complete = response.contains("DESIGN_COMPLETE");
        let mut final_config = None;

        if is_complete {
            // Extract the game config
            if let Some(env) = self.template_env.lock().await.as_ref()
                && let Ok(template) = env.get_template("03_extract_game_config")
            {
                let extraction_prompt = template.render(context!())?;

                let config_json = self
                    .send_message_with_config(
                        conversation_id,
                        extraction_prompt,
                        Some(MessageConfig {
                            model: "gpt-4-turbo".to_string(),
                            temperature: 0.3,
                            max_tokens: 4000,
                        }),
                    )
                    .await?;

                // Try to parse the JSON
                if let Ok(config) = serde_json::from_str::<GameConfig>(&config_json) {
                    final_config = Some(config);
                }
            }
        }

        Ok((response, is_complete, final_config))
    }

    async fn generate_full_game<F>(
        &self,
        config: &GameConfig,
        project_config: Option<serde_json::Value>,
        progress_callback: F,
    ) -> Result<String>
    where
        F: Fn(GenerationProgress) + Send + 'static,
    {
        let project_path = create_project_directory(&config.name)?;

        // Create a new conversation for generation
        let context = starters::game_generation_context(project_config.clone());
        let conversation_id = self
            .start_conversation(format!("Generating: {}", config.name), context)
            .await?;

        // Phase 1: Generate Style Guide
        progress_callback(GenerationProgress {
            phase: GenerationPhase::StyleGuide,
            step: "Creating visual style guide".to_string(),
            progress: 0.1,
            message: "Establishing art direction...".to_string(),
        });

        let style_guide =
            generate_style_guide(self, &conversation_id, config, project_config.as_ref()).await?;
        std::fs::write(project_path.join("STYLE_GUIDE.md"), &style_guide)?;

        // Phase 2: Generate World
        progress_callback(GenerationProgress {
            phase: GenerationPhase::WorldGeneration,
            step: "Designing world map".to_string(),
            progress: 0.2,
            message: "Creating regions and connections...".to_string(),
        });

        let world_data = generate_world(
            self,
            &conversation_id,
            config,
            &style_guide,
            project_config.as_ref(),
        )
        .await?;
        save_world_data(&project_path, &world_data)?;

        // Phase 3: Generate AI Systems
        progress_callback(GenerationProgress {
            phase: GenerationPhase::AiSystems,
            step: "Designing AI behaviors".to_string(),
            progress: 0.3,
            message: "Creating NPC state machines and behavior trees...".to_string(),
        });

        // Generate AI systems
        if let Some(env) = self.template_env.lock().await.as_ref()
            && let Ok(template) = env.get_template("04_ai_systems")
        {
            let prompt = if let Some(config) = &project_config {
                template.render(context!(config => config))?
            } else {
                template.render(context!())?
            };

            let ai_systems = self.send_message(&conversation_id, prompt).await?;
            let ai_systems_path = project_path.join("src").join("npc_ai.rs");
            std::fs::write(&ai_systems_path, ai_systems)?;
        }

        // Phase 4: Generate Assets
        progress_callback(GenerationProgress {
            phase: GenerationPhase::AssetGeneration,
            step: "Creating game assets".to_string(),
            progress: 0.4,
            message: "Generating sprites and tilesets...".to_string(),
        });

        // TODO: Implement asset generation

        // Phase 4: Generate Code
        progress_callback(GenerationProgress {
            phase: GenerationPhase::CodeGeneration,
            step: "Writing game code".to_string(),
            progress: 0.6,
            message: "Implementing game mechanics...".to_string(),
        });

        // TODO: Implement code generation

        // Phase 5: Generate Dialog
        progress_callback(GenerationProgress {
            phase: GenerationPhase::DialogWriting,
            step: "Writing dialog".to_string(),
            progress: 0.7,
            message: "Creating character conversations...".to_string(),
        });

        // TODO: Implement dialog generation

        // Phase 6: Generate Music
        progress_callback(GenerationProgress {
            phase: GenerationPhase::MusicComposition,
            step: "Composing music".to_string(),
            progress: 0.8,
            message: "Creating soundtrack...".to_string(),
        });

        // TODO: Implement music generation

        // Phase 7: Integration
        progress_callback(GenerationProgress {
            phase: GenerationPhase::Integration,
            step: "Integrating components".to_string(),
            progress: 0.9,
            message: "Putting it all together...".to_string(),
        });

        // TODO: Implement integration

        // Phase 8: Packaging
        progress_callback(GenerationProgress {
            phase: GenerationPhase::Packaging,
            step: "Packaging game".to_string(),
            progress: 1.0,
            message: "Creating final build...".to_string(),
        });

        // Copy AI toolkit if needed
        copy_ai_toolkit(&project_path)?;

        Ok(project_path.to_string_lossy().to_string())
    }
}

// Helper functions

fn copy_ai_toolkit(project_path: &Path) -> Result<()> {
    // Try different possible locations for the template
    let possible_paths = [
        PathBuf::from("vintage_game_generator/templates/bevy-ai-toolkit"),
        PathBuf::from("templates/bevy-ai-toolkit"),
        PathBuf::from("../templates/bevy-ai-toolkit"),
    ];

    let toolkit_src = possible_paths.iter().find(|p| p.exists());

    if let Some(src) = toolkit_src {
        let toolkit_dest = project_path.join("crates/bevy-ai-toolkit");
        std::fs::create_dir_all(&toolkit_dest)?;
        copy_dir_recursive(src, &toolkit_dest)?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn create_project_directory(project_name: &str) -> Result<PathBuf> {
    let sanitized_name = project_name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();

    let project_path = PathBuf::from("generated_games").join(&sanitized_name);
    std::fs::create_dir_all(&project_path)?;

    Ok(project_path)
}

async fn generate_style_guide(
    manager: &ConversationManager,
    conversation_id: &str,
    config: &GameConfig,
    project_config: Option<&serde_json::Value>,
) -> Result<String> {
    if let Some(env) = manager.template_env.lock().await.as_ref()
        && let Ok(template) = env.get_template("02_style")
    {
        let prompt = if let Some(project) = project_config {
            template.render(context!(
                project => project,
                config => config
            ))?
        } else {
            template.render(context!(config => config))?
        };

        let response = manager
            .send_message_with_config(
                conversation_id,
                prompt,
                Some(MessageConfig {
                    model: "gpt-4-turbo".to_string(),
                    temperature: 0.3,
                    max_tokens: 3000,
                }),
            )
            .await?;

        return Ok(response);
    }

    Err(anyhow::anyhow!("Style guide template not found"))
}

async fn generate_world(
    manager: &ConversationManager,
    conversation_id: &str,
    config: &GameConfig,
    style_guide: &str,
    project_config: Option<&serde_json::Value>,
) -> Result<WorldData> {
    if let Some(env) = manager.template_env.lock().await.as_ref()
        && let Ok(template) = env.get_template("03_world")
    {
        let prompt = if let Some(project) = project_config {
            template.render(context!(
                project => project,
                config => config,
                style_guide => style_guide
            ))?
        } else {
            template.render(context!(
                config => config,
                style_guide => style_guide
            ))?
        };

        let response = manager
            .send_message_with_config(
                conversation_id,
                prompt,
                Some(MessageConfig {
                    model: "gpt-4-turbo".to_string(),
                    temperature: 0.5,
                    max_tokens: 4000,
                }),
            )
            .await?;

        // Parse the response as JSON
        let world_data: WorldData = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse world data: {e}"))?;

        return Ok(world_data);
    }

    Err(anyhow::anyhow!("World generation template not found"))
}

fn save_world_data(project_path: &Path, world_data: &WorldData) -> Result<()> {
    let world_dir = project_path.join("world");
    std::fs::create_dir_all(&world_dir)?;

    // Save world data as JSON
    let world_json = serde_json::to_string_pretty(world_data)?;
    std::fs::write(world_dir.join("world_data.json"), world_json)?;

    // Create directories for regions, towns, and dungeons
    std::fs::create_dir_all(world_dir.join("regions"))?;
    std::fs::create_dir_all(world_dir.join("towns"))?;
    std::fs::create_dir_all(world_dir.join("dungeons"))?;

    Ok(())
}
