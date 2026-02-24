use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Parser;
use std::path::PathBuf;
use uuid::Uuid;
use game_generator::ai_client::AiConfig;
use game_generator::wizard::{AppDirectories, AppMode, WizardPlugin};

#[derive(Parser, Debug)]
#[command(author, version, about = "AI-powered vintage game generator", long_about = None)]
struct Args {
    /// Base directory for all projects (defaults to XDG config dir)
    #[arg(short = 'b', long = "base-dir")]
    base_dir: Option<PathBuf>,

    /// Project directory (defaults to new UUID in base dir)
    #[arg(short = 'p', long = "project-dir")]
    project_dir: Option<PathBuf>,

    /// Config file path (defaults to project.toml in project dir)
    #[arg(short = 'c', long = "config-file")]
    config_file: Option<PathBuf>,

    /// Run in list mode to browse existing projects
    #[arg(short = 'l', long = "list", conflicts_with_all = &["project_dir", "config_file"])]
    list_mode: bool,

    /// Run in generate mode (default)
    #[arg(short = 'g', long = "generate", conflicts_with = "list_mode")]
    generate_mode: bool,

    // AI Configuration
    /// Text generation model (e.g., gpt-4, gpt-3.5-turbo, claude-3-opus)
    #[arg(long = "text-model", default_value = "gpt-4")]
    text_model: String,

    /// Image generation model (e.g., dall-e-3, dall-e-2)
    #[arg(long = "image-model", default_value = "dall-e-3")]
    image_model: String,

    /// Audio generation model (for future use)
    #[arg(long = "audio-model", default_value = "tts-1")]
    audio_model: String,

    /// Temperature for text generation (0.0-2.0)
    #[arg(long = "temperature", default_value = "0.8")]
    temperature: f32,

    /// Top-p for text generation (0.0-1.0)
    #[arg(long = "top-p", default_value = "0.95")]
    top_p: f32,

    /// Maximum tokens for text generation
    #[arg(long = "max-tokens", default_value = "2000")]
    max_tokens: u32,

    /// Frequency penalty for text generation (-2.0 to 2.0)
    #[arg(long = "frequency-penalty", default_value = "0.0")]
    frequency_penalty: f32,

    /// Presence penalty for text generation (-2.0 to 2.0)
    #[arg(long = "presence-penalty", default_value = "0.0")]
    presence_penalty: f32,

    /// Image quality (standard or hd for DALL-E 3)
    #[arg(long = "image-quality", default_value = "standard")]
    image_quality: String,

    /// Image size (1024x1024, 1792x1024, 1024x1792 for DALL-E 3)
    #[arg(long = "image-size", default_value = "1024x1024")]
    image_size: String,

    /// AI provider (openai, anthropic)
    #[arg(long = "ai-provider", default_value = "openai")]
    ai_provider: String,

    /// Enable AI response caching (default: true)
    #[arg(long = "cache", default_value = "true")]
    cache: bool,

    /// AI request timeout in seconds
    #[arg(long = "ai-timeout", default_value = "120")]
    ai_timeout: u64,
}

// Create AiConfig from command line args
fn create_ai_config(args: &Args) -> AiConfig {
    AiConfig {
        // Model Selection
        text_model: args.text_model.clone(),
        image_model: args.image_model.clone(),
        audio_model: args.audio_model.clone(),
        embedding_model: "text-embedding-3-small".to_string(),

        // Generation Parameters
        temperature: args.temperature,
        top_p: args.top_p,
        max_tokens: args.max_tokens,
        frequency_penalty: args.frequency_penalty,
        presence_penalty: args.presence_penalty,

        // Image Parameters
        image_quality: args.image_quality.clone(),
        image_size: args.image_size.clone(),

        // Provider Settings
        ai_provider: args.ai_provider.clone(),

        // Cache and Performance
        cache_enabled: args.cache,
        cache_ttl: 3600 * 24 * 7, // 1 week
        timeout_secs: args.ai_timeout,
        optimize_costs: true,
        max_concurrent: 5,
    }
}

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Create AI configuration from args
    let ai_config = create_ai_config(&args);

    // Determine mode
    let mode = if args.list_mode {
        AppMode::List
    } else {
        AppMode::Generate // Default mode
    };

    // Get base directory
    let base_dir = args.base_dir.unwrap_or_else(|| {
        dirs::config_dir()
            .expect("Could not find config directory")
            .join("vintage_game_generator")
    });

    // For generate mode, determine project directory
    let (project_dir, config_file) = match mode {
        AppMode::Generate => {
            let proj_dir = args.project_dir.unwrap_or_else(|| {
                // Generate new UUID for project
                let uuid = Uuid::new_v4();
                base_dir.join(uuid.to_string())
            });

            let conf_file = args
                .config_file
                .unwrap_or_else(|| proj_dir.join("project.toml"));

            (proj_dir, conf_file)
        }
        AppMode::List => {
            // In list mode, we don't need a specific project directory
            (base_dir.clone(), base_dir.join("placeholder.toml"))
        }
    };

    // Create app directories resource
    let directories = AppDirectories {
        base_dir: base_dir.clone(),
        project_dir: project_dir.clone(),
        config_file: Some(config_file),
        prompts_dir: project_dir.join("prompts"),
        assets_dir: project_dir.join("assets"),
        mode,
    };

    // Print startup info
    println!("Vintage Game Generator");
    println!("====================");
    println!("Mode: {mode:?}");
    let base_display = base_dir.display();
    println!("Base Directory: {base_display}");
    if matches!(mode, AppMode::Generate) {
        let proj_display = project_dir.display();
        println!("Project Directory: {proj_display}");
    }
    println!();
    let provider = &ai_config.ai_provider;
    let text_model = if ai_config.text_model.is_empty() {
        "(default)"
    } else {
        &ai_config.text_model
    };
    let temperature = ai_config.temperature;
    let cache_status = if ai_config.cache_enabled {
        "enabled"
    } else {
        "disabled"
    };
    println!("AI Configuration:");
    println!("  Provider: {provider}");
    println!("  Text Model: {text_model}");
    println!("  Temperature: {temperature}");
    println!("  Cache: {cache_status}");
    println!();

    // Setup Bevy app
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: match mode {
                AppMode::Generate => "Vintage Game Generator".to_string(),
                AppMode::List => "Vintage Game Generator - Browse Projects".to_string(),
            },
            resolution: (1280.0, 800.0).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EguiPlugin::default())
    .insert_resource(mode)
    .insert_resource(directories)
    .insert_resource(ai_config);

    // Add mode-specific resources
    match mode {
        AppMode::List => {
            app.insert_resource(
                game_generator::wizard::list_mode::ListModeState::default(),
            );
        }
        AppMode::Generate => {
            // Generate mode resources are added by WizardPlugin
        }
    }

    app.add_plugins(WizardPlugin).run();
}
