//! Template processing for code generation

use anyhow::{Context, Result};
use minijinja::Environment;
use std::fs;
use std::path::Path;

pub struct TemplateProcessor {
    template_dir: String,
    output_dir: String,
}

impl TemplateProcessor {
    pub fn new(template_dir: &str, output_dir: &str) -> Result<Self> {
        // Ensure output directory exists
        fs::create_dir_all(output_dir)?;

        Ok(Self {
            template_dir: template_dir.to_string(),
            output_dir: output_dir.to_string(),
        })
    }

    /// Generate Rust modules from templates
    pub fn generate_modules(
        &self,
        timeline_games: &[serde_json::Value],
        platforms: &[crate::types::PlatformInfo],
        graph_data: &serde_json::Value,
        timeline_start: i32,
        timeline_end: i32,
    ) -> Result<()> {
        println!("Generating Rust modules from templates...");

        // Template files to process
        let template_files = [
            ("mod.rs.jinja", "mod.rs"),
            ("games.rs.jinja", "games.rs"),
            ("platforms.rs.jinja", "platforms.rs"),
            ("eras.rs.jinja", "eras.rs"),
            ("graph.rs.jinja", "graph.rs"),
        ];

        // Create template context
        let context = serde_json::json!({
            "games": timeline_games,
            "platforms": platforms,
            "graph": graph_data,
            "timeline_start": timeline_start,
            "timeline_end": timeline_end
        });

        // Generate each module file
        for (template_file, output_file) in template_files.iter() {
            // Create a new environment for each template to avoid lifetime issues
            let mut env = Environment::new();

            let template_path = Path::new(&self.template_dir).join(template_file);
            let template_content = fs::read_to_string(&template_path).with_context(|| {
                format!("Failed to read template file: {}", template_path.display())
            })?;

            // Add and immediately use the template
            env.add_template(template_file, &template_content)?;
            let tmpl = env.get_template(template_file)?;
            let module_content = tmpl.render(&context)?;

            let output_path = Path::new(&self.output_dir).join(output_file);
            fs::write(&output_path, module_content)?;
            println!("  Generated: {}", output_path.display());
        }

        Ok(())
    }

    // JSON generation removed - all data is now compiled into Rust modules
}
