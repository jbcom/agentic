//! Build tools for vintage game data generation

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

pub mod ai_analysis;
pub mod api;
pub mod generator;
pub mod graph;
pub mod images;
pub mod templates;
pub mod types;

pub use ai_analysis::{AIAnalyzer, EnrichedGameMetadata, GameMechanic};
pub use generator::GameDataGenerator;

/// Build tools configuration
pub struct VintageBuildTools {
    generator: GameDataGenerator,
}

impl VintageBuildTools {
    /// Create new build tools instance
    pub fn new(api_key: String, timeline_start: i32, timeline_end: i32) -> Self {
        Self {
            generator: GameDataGenerator::new(api_key, timeline_start, timeline_end),
        }
    }

    /// Create from environment (loads .env file from repository root)
    pub fn from_env(timeline_start: i32, timeline_end: i32) -> Result<Self> {
        // Find repository root by looking for .git directory or workspace Cargo.toml
        let repo_root = find_repository_root()?;

        // Try loading .env.local first (higher priority), then .env
        let env_local_path = repo_root.join(".env.local");
        let env_path = repo_root.join(".env");

        if env_local_path.exists() {
            dotenv::from_path(&env_local_path).ok();
        } else if env_path.exists() {
            dotenv::from_path(&env_path).ok();
        } else {
            // Try loading from current directory as fallback
            dotenv().ok();
        }

        // Get API key from environment
        let api_key = env::var("GIANTBOMB_API_KEY")
            .map_err(|_| anyhow::anyhow!(
                "GIANTBOMB_API_KEY not found in environment. Please set it in your .env or .env.local file."
            ))?;

        Ok(Self::new(api_key, timeline_start, timeline_end))
    }

    /// Run the build process
    pub async fn build(&self) -> Result<()> {
        self.generator.generate().await
    }
}

/// Find the repository root by looking for .git directory or workspace Cargo.toml
fn find_repository_root() -> Result<PathBuf> {
    let mut current = env::current_dir()?;

    loop {
        // Check for .git directory (most reliable)
        if current.join(".git").exists() {
            return Ok(current);
        }

        // Check for workspace Cargo.toml with [workspace] section
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            let contents = std::fs::read_to_string(&cargo_toml)?;
            if contents.contains("[workspace]") {
                return Ok(current);
            }
        }

        // Move up one directory
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => {
                // Reached filesystem root without finding repository markers
                anyhow::bail!(
                    "Could not find repository root. Please run from within the repository directory."
                );
            }
        }
    }
}
