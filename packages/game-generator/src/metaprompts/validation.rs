use anyhow::{Context, Result};
use minijinja::Environment;
use serde_json::Value;
use std::path::Path;
use syn::{File as SynFile, parse_str};

pub struct PromptValidator {
    template_env: Environment<'static>,
}

impl Default for PromptValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptValidator {
    pub fn new() -> Self {
        Self {
            template_env: Environment::new(),
        }
    }

    pub async fn validate_prompt(&self, prompt_path: &Path) -> Result<ValidationResult> {
        let content = std::fs::read_to_string(prompt_path).context("Failed to read prompt file")?;

        let mut result = ValidationResult {
            path: prompt_path.to_path_buf(),
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Check if it's valid Jinja2 syntax
        match self.template_env.render_str(&content, ()) {
            Ok(_) => {}
            Err(e) => {
                result.valid = false;
                result.errors.push(format!("Template syntax error: {e}"));
            }
        }

        // Check for required variables
        if !content.contains("{{") && !content.contains("{%") {
            result
                .warnings
                .push("No template variables found - is this a static prompt?".to_string());
        }

        // Check prompt length
        let word_count = content.split_whitespace().count();
        if word_count > 4000 {
            result.warnings.push(format!(
                "Prompt is very long ({word_count} words) - consider splitting"
            ));
        }

        // Check for common issues
        if content.contains("TODO") || content.contains("FIXME") {
            result
                .warnings
                .push("Contains TODO/FIXME markers".to_string());
        }

        // Validate JSON blocks if present
        self.validate_json_blocks(&content, &mut result);

        // Validate Rust code blocks if present
        self.validate_rust_blocks(&content, &mut result);

        Ok(result)
    }

    fn validate_json_blocks(&self, content: &str, result: &mut ValidationResult) {
        // Find JSON code blocks
        let mut in_json = false;
        let mut json_content = String::new();

        for line in content.lines() {
            if line.trim() == "```json" {
                in_json = true;
                json_content.clear();
            } else if line.trim() == "```" && in_json {
                in_json = false;
                // Validate the JSON
                if let Err(e) = serde_json::from_str::<Value>(&json_content) {
                    result.errors.push(format!("Invalid JSON block: {e}"));
                }
            } else if in_json {
                json_content.push_str(line);
                json_content.push('\n');
            }
        }
    }

    fn validate_rust_blocks(&self, content: &str, result: &mut ValidationResult) {
        let mut in_rust = false;
        let mut rust_content = String::new();
        let mut line_num = 0;

        for line in content.lines() {
            line_num += 1;

            if line.trim() == "```rust" || line.trim() == "```rs" {
                in_rust = true;
                rust_content.clear();
            } else if line.trim() == "```" && in_rust {
                in_rust = false;

                // Try to parse as a complete Rust file
                match parse_str::<SynFile>(&rust_content) {
                    Ok(_) => {
                        // Valid as a complete file
                    }
                    Err(file_err) => {
                        // Try parsing as an expression
                        match syn::parse_str::<syn::Expr>(&rust_content) {
                            Ok(_) => {
                                // Valid as an expression
                            }
                            Err(_expr_err) => {
                                // Try parsing as an item (function, struct, etc.)
                                match syn::parse_str::<syn::Item>(&rust_content) {
                                    Ok(_) => {
                                        // Valid as an item
                                    }
                                    Err(_) => {
                                        // Report the original file parsing error
                                        result.warnings.push(format!(
                                            "Rust code block at line {} has syntax issues: {}",
                                            line_num - rust_content.lines().count(),
                                            file_err
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                // Check for common Rust issues
                if rust_content.contains("unwrap()") {
                    result.warnings.push("Rust code uses unwrap() - consider using ? operator or proper error handling".to_string());
                }

                if rust_content.contains("panic!") {
                    result
                        .warnings
                        .push("Rust code contains panic! - ensure this is intentional".to_string());
                }

                if rust_content.contains("unsafe") {
                    result.warnings.push(
                        "Rust code contains unsafe block - document safety requirements"
                            .to_string(),
                    );
                }
            } else if in_rust {
                rust_content.push_str(line);
                rust_content.push('\n');
            }
        }
    }

    pub async fn validate_prompt_chain(&self, prompts: &[&Path]) -> Result<ChainValidationResult> {
        let mut results = Vec::new();
        let mut chain_valid = true;

        for prompt_path in prompts {
            let result = self.validate_prompt(prompt_path).await?;
            if !result.valid {
                chain_valid = false;
            }
            results.push(result);
        }

        Ok(ChainValidationResult {
            chain_valid,
            prompt_results: results,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub path: std::path::PathBuf,
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn to_markdown(&self) -> String {
        let mut md = format!("## Validation Result: {}\n\n", self.path.display());

        if self.valid {
            md.push_str("✅ **Valid**\n\n");
        } else {
            md.push_str("❌ **Invalid**\n\n");
        }

        if !self.errors.is_empty() {
            md.push_str("### Errors\n");
            for error in &self.errors {
                md.push_str(&format!("- {error}\n"));
            }
            md.push('\n');
        }

        if !self.warnings.is_empty() {
            md.push_str("### Warnings\n");
            for warning in &self.warnings {
                md.push_str(&format!("- {warning}\n"));
            }
            md.push('\n');
        }

        md
    }
}

#[derive(Debug)]
pub struct ChainValidationResult {
    pub chain_valid: bool,
    pub prompt_results: Vec<ValidationResult>,
}
