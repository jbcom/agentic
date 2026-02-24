//! Image generation module optimized for 16-bit nostalgic game art
//!
//! Features:
//! - Style consistency across all generated assets
//! - Sprite sheet optimization
//! - Palette quantization and recoloring
//! - Batch generation with rate limiting
//! - Smart caching to reduce costs

use anyhow::{Context, Result};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::images::{
        CreateImageRequestArgs, Image, ImageModel, ImageQuality, ImageResponseFormat, ImageSize,
    },
};
use base64::Engine;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

use super::{
    AiConfig, AiGenerator,
    cache::{AiCache, ImageCache},
    consistency::{Color, ColorPalette, StyleManager},
    tokens::TokenCounter,
};

/// Image generator with style consistency
#[derive(Clone)]
pub struct ImageGenerator {
    client: Arc<Client<OpenAIConfig>>,
    cache: Arc<Mutex<AiCache>>,
    image_cache: ImageCache,
    token_counter: Arc<Mutex<TokenCounter>>,
    style_manager: Arc<Mutex<StyleManager>>,
    batch_semaphore: Arc<Semaphore>,
    template_env: Arc<Mutex<Environment<'static>>>,
}

/// Configuration for image generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    /// Model to use (dall-e-2 or dall-e-3)
    pub model: ImageModel,
    /// Image size
    pub size: ImageSize,
    /// Quality (standard or hd for dall-e-3)
    pub quality: ImageQuality,
    /// Number of images to generate
    pub n: u8,
    /// Response format
    pub response_format: ImageResponseFormat,
    /// Style consistency mode
    pub enforce_consistency: bool,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            model: ImageModel::DallE3,
            size: ImageSize::S1024x1024,
            quality: ImageQuality::Standard,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: true,
        }
    }
}

impl ImageConfig {
    /// Create image config from global AI config
    pub fn from_ai_config(config: &AiConfig) -> Self {
        let size = match config.image_size.as_str() {
            "1024x1024" => ImageSize::S1024x1024,
            "1792x1024" => ImageSize::S1792x1024,
            "1024x1792" => ImageSize::S1024x1792,
            _ => ImageSize::S1024x1024,
        };

        let quality = match config.image_quality.as_str() {
            "hd" => ImageQuality::HD,
            _ => ImageQuality::Standard,
        };

        let model = match config.image_model.as_str() {
            "dall-e-2" => ImageModel::DallE2,
            _ => ImageModel::DallE3,
        };

        Self {
            model,
            size,
            quality,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: config.optimize_costs,
        }
    }

    /// Get dimensions for a given size
    pub fn get_dimensions(size: &ImageSize) -> (u32, u32) {
        match size {
            ImageSize::S256x256 => (256, 256),
            ImageSize::S512x512 => (512, 512),
            ImageSize::S1024x1024 => (1024, 1024),
            ImageSize::S1792x1024 => (1792, 1024),
            ImageSize::S1024x1792 => (1024, 1792),
            ImageSize::S1536x1024 => (1536, 1024),
            ImageSize::S1024x1536 => (1024, 1536),
            _ => (1024, 1024),
        }
    }

    /// Configuration for sprite generation
    pub fn for_sprites() -> Self {
        Self {
            model: ImageModel::DallE3,
            size: ImageSize::S1024x1024,
            quality: ImageQuality::Standard,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: true,
        }
    }

    /// Configuration for background/tileset generation
    pub fn for_backgrounds() -> Self {
        Self {
            model: ImageModel::DallE3,
            size: ImageSize::S1792x1024,
            quality: ImageQuality::HD,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: true,
        }
    }

    /// Configuration for wide background generation
    pub fn for_backgrounds_wide() -> Self {
        Self {
            model: ImageModel::DallE3,
            size: ImageSize::S1792x1024,
            quality: ImageQuality::HD,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: true,
        }
    }

    /// Configuration for tall background generation
    pub fn for_backgrounds_tall() -> Self {
        Self {
            model: ImageModel::DallE3,
            size: ImageSize::S1024x1792,
            quality: ImageQuality::HD,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: true,
        }
    }

    /// Configuration for UI elements
    pub fn for_ui() -> Self {
        Self {
            model: ImageModel::DallE3,
            size: ImageSize::S1024x1024,
            quality: ImageQuality::Standard,
            n: 1,
            response_format: ImageResponseFormat::B64Json,
            enforce_consistency: true,
        }
    }
}

impl ImageGenerator {
    /// Create a new image generator
    pub fn new(
        client: Arc<Client<OpenAIConfig>>,
        cache: Arc<Mutex<AiCache>>,
        token_counter: Arc<Mutex<TokenCounter>>,
        style_manager: Arc<Mutex<StyleManager>>,
    ) -> Self {
        // Extract the inner AiCache from the Mutex
        let inner_cache = cache
            .try_lock()
            .ok()
            .map(|guard| Arc::new(guard.clone()))
            .unwrap_or_else(|| Arc::new(AiCache::new().unwrap()));
        let image_cache = ImageCache::new(inner_cache);

        let mut env = Environment::new();

        // Load all image prompt templates
        let templates = [
            (
                "style_guide",
                include_str!("../prompts/image/style_guide.jinja"),
            ),
            ("sprite", include_str!("../prompts/image/sprite.jinja")),
            ("tileset", include_str!("../prompts/image/tileset.jinja")),
        ];

        for (name, template) in templates {
            env.add_template(name, template).ok();
        }

        Self {
            client,
            cache,
            image_cache,
            token_counter,
            style_manager,
            batch_semaphore: Arc::new(Semaphore::new(3)), // Max 3 concurrent image generations
            template_env: Arc::new(Mutex::new(env)),
        }
    }

    /// Generate a style guide that establishes visual consistency
    pub async fn generate_style_guide(&self, concept: &GameConcept) -> Result<Vec<u8>> {
        let style_config = self.style_manager.lock().await.get_style().await;

        // Prepare context for template
        let context = json!({
            "genre": concept.genre,
            "max_colors": style_config.palette.max_colors,
            "character_width": style_config.sprite_specs.character_size.0,
            "character_height": style_config.sprite_specs.character_size.1,
            "tile_width": style_config.sprite_specs.tile_size.0,
            "tile_height": style_config.sprite_specs.tile_size.1,
            "shading_technique": self.format_shading(&style_config.rules.shading_technique),
            "outline_style": self.format_outline(&style_config.rules.outline_style),
            "perspective": self.format_perspective(&style_config.rules.perspective),
            "visual_inspirations": concept.visual_inspirations.join(", "),
            "mood": concept.mood,
            "style_name": style_config.style_name,
        });

        // Render template
        let env = self.template_env.lock().await;
        let template = env
            .get_template("style_guide")
            .context("Failed to get style guide template")?;
        let style_guide_prompt = template
            .render(&context)
            .context("Failed to render style guide template")?;

        // Generate with validation
        let result = self
            .generate_with_validation(
                &style_guide_prompt,
                ImageConfig::for_sprites(),
                ValidationCriteria::StyleGuide,
                5, // max attempts
            )
            .await?;

        // Extract and store style information
        self.extract_style_information(&result).await?;

        Ok(result)
    }

    /// Generate a sprite with enforced consistency
    pub async fn generate_sprite(
        &self,
        sprite_type: &str,
        description: &str,
        _style_guide: Option<&[u8]>,
    ) -> Result<Vec<u8>> {
        let style_config = self.style_manager.lock().await.get_style().await;

        // Get style-consistent description
        let styled_description = self
            .style_manager
            .lock()
            .await
            .create_style_prompt(description)
            .await?;

        // Prepare context for template
        let context = json!({
            "sprite_type": sprite_type,
            "description": styled_description,
            "max_width": style_config.sprite_specs.character_size.0 * 2,
            "max_height": style_config.sprite_specs.character_size.1 * 2,
            "max_colors": style_config.palette.max_colors,
            "shading_technique": self.format_shading(&style_config.rules.shading_technique),
            "outline_style": self.format_outline(&style_config.rules.outline_style),
            "perspective": self.format_perspective(&style_config.rules.perspective),
            "visual_style": style_config.style_name,
        });

        // Render template
        let env = self.template_env.lock().await;
        let template = env
            .get_template("sprite")
            .context("Failed to get sprite template")?;
        let prompt = template
            .render(&context)
            .context("Failed to render sprite template")?;

        // Generate with validation
        let sprite = self
            .generate_with_validation(
                &prompt,
                ImageConfig::for_sprites(),
                ValidationCriteria::Sprite(sprite_type.to_string()),
                3,
            )
            .await?;

        // Post-process for consistency
        let processed = self.enforce_palette_consistency(&sprite).await?;

        Ok(processed)
    }

    /// Generate multiple sprites as a batch
    pub async fn generate_sprite_batch(
        &self,
        requests: Vec<SpriteRequest>,
    ) -> Result<HashMap<String, Vec<u8>>> {
        let mut results = HashMap::new();
        let mut tasks = Vec::new();

        for request in requests {
            let generator = self.clone();
            let permit = self.batch_semaphore.clone().acquire_owned().await?;

            let task = tokio::spawn(async move {
                let _permit = permit; // Hold permit until done
                let result = generator
                    .generate_sprite(&request.sprite_type, &request.description, None)
                    .await;
                (request.name, result)
            });

            tasks.push(task);
        }

        // Collect results
        for task in tasks {
            let (name, result) = task.await?;
            match result {
                Ok(data) => {
                    results.insert(name, data);
                }
                Err(e) => {
                    tracing::error!("Failed to generate sprite {}: {}", name, e);
                }
            }
        }

        Ok(results)
    }

    /// Generate with validation and retry
    async fn generate_with_validation(
        &self,
        prompt: &str,
        config: ImageConfig,
        criteria: ValidationCriteria,
        max_attempts: u32,
    ) -> Result<Vec<u8>> {
        let mut best_result = None;
        let mut best_score = 0.0;

        for attempt in 0..max_attempts {
            match self.generate_single(prompt, config.clone()).await {
                Ok(data) => {
                    let validation = self.validate_image(&data, &criteria).await?;

                    if validation.passed {
                        return Ok(data);
                    }

                    if validation.score > best_score {
                        best_score = validation.score;
                        best_result = Some(data);
                    }

                    // Add feedback to prompt for next attempt
                    if attempt < max_attempts - 1 {
                        tracing::info!(
                            "Validation failed (attempt {}): {:?}",
                            attempt + 1,
                            validation.issues
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("Generation attempt {} failed: {}", attempt + 1, e);
                    if attempt == max_attempts - 1 {
                        return Err(e);
                    }
                }
            }

            // Exponential backoff
            tokio::time::sleep(tokio::time::Duration::from_millis(
                100 * (attempt + 1) as u64,
            ))
            .await;
        }

        best_result.ok_or_else(|| {
            anyhow::anyhow!("Failed to generate valid image after {max_attempts} attempts")
        })
    }

    /// Generate a single image
    pub async fn generate_single(&self, prompt: &str, config: ImageConfig) -> Result<Vec<u8>> {
        // Check cache first
        let mut params = HashMap::new();
        params.insert("model".to_string(), format!("{:?}", config.model));
        params.insert("size".to_string(), format!("{:?}", config.size));
        params.insert("quality".to_string(), format!("{:?}", config.quality));

        let cache_key = self
            .cache
            .lock()
            .await
            .generate_key("image", prompt, &params);

        if let Some(cached_data) = self
            .image_cache
            .get_image(&cache_key, super::cache::ImageFormat::Png)
            .await
        {
            return Ok(cached_data);
        }

        // Create request
        let request = CreateImageRequestArgs::default()
            .prompt(prompt)
            .model(config.model.clone())
            .n(config.n)
            .quality(config.quality.clone())
            .response_format(config.response_format)
            .size(config.size)
            .build()?;

        // Make API call
        let response = self
            .client
            .images()
            .generate(request)
            .await
            .context("Failed to generate image")?;

        // Extract image data from the response
        let image_data = response
            .data
            .first()
            .ok_or_else(|| anyhow::anyhow!("No image data in response"))?;

        // The async-openai Image type is an enum with Url and B64Json variants
        let image_bytes = match image_data.as_ref() {
            Image::B64Json { b64_json, .. } => base64::engine::general_purpose::STANDARD
                .decode(b64_json.as_ref())
                .context("Failed to decode base64 image data")?,
            Image::Url { url, .. } => {
                // If we got a URL instead, we need to fetch it
                anyhow::bail!("Expected base64 data but got URL: {url}");
            }
        };

        // Track usage
        let (width, height) = ImageConfig::get_dimensions(&config.size);
        let quality_str = match config.quality {
            ImageQuality::HD => "hd",
            _ => "standard",
        };
        let model_name = format!("dall-e-3-{}x{}-{}", width, height, quality_str);

        self.token_counter
            .lock()
            .await
            .record_image_generation(&model_name, width, height, 1)
            .await?;

        // Cache result
        let cache_params: HashMap<String, serde_json::Value> = params
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::String(v)))
            .collect();

        self.image_cache
            .put_image(cache_key, image_bytes.clone(), cache_params)
            .await?;

        Ok(image_bytes)
    }

    /// Validate generated image
    async fn validate_image(
        &self,
        data: &[u8],
        criteria: &ValidationCriteria,
    ) -> Result<ValidationResult> {
        let img = image::load_from_memory(data)?;
        let mut result = ValidationResult {
            passed: true,
            score: 1.0,
            issues: Vec::new(),
            suggestions: Vec::new(),
        };

        // Check dimensions
        let (width, height) = img.dimensions();
        if let ValidationCriteria::Sprite(_sprite_type) = criteria {
            let style = self.style_manager.lock().await.get_style().await;
            let expected_size = style.sprite_specs.character_size;

            // For sprites, we expect them to fit within reasonable bounds
            if width > expected_size.0 * 4 || height > expected_size.1 * 4 {
                result.issues.push(format!(
                    "Sprite too large: {}x{} (expected around {}x{})",
                    width, height, expected_size.0, expected_size.1
                ));
                result.score *= 0.8;
            }
        }

        // Check color count
        let color_count = self.count_unique_colors(&img);
        let style = self.style_manager.lock().await.get_style().await;

        if color_count > style.palette.max_colors as usize * 2 {
            result.issues.push(format!(
                "Too many colors: {} (max {})",
                color_count, style.palette.max_colors
            ));
            result.score *= 0.7;
            result
                .suggestions
                .push("Consider palette quantization".to_string());
        }

        // Check for anti-aliasing
        if self.has_anti_aliasing(&img) {
            result
                .issues
                .push("Detected anti-aliasing or soft edges".to_string());
            result.score *= 0.6;
            result.passed = false;
        }

        result.passed = result.score >= 0.7;
        Ok(result)
    }

    /// Count unique colors in image
    fn count_unique_colors(&self, img: &DynamicImage) -> usize {
        let rgba = img.to_rgba8();
        let mut colors = std::collections::HashSet::new();

        for pixel in rgba.pixels() {
            if pixel[3] > 0 {
                // Ignore fully transparent
                colors.insert((pixel[0], pixel[1], pixel[2]));
            }
        }

        colors.len()
    }

    /// Check for anti-aliasing
    fn has_anti_aliasing(&self, img: &DynamicImage) -> bool {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        // Sample edges between different colored pixels
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = rgba.get_pixel(x, y);

                // Skip transparent pixels
                if center[3] < 255 {
                    continue;
                }

                // Check neighbors
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = (x as i32 + dx) as u32;
                        let ny = (y as i32 + dy) as u32;

                        if nx < width && ny < height {
                            let neighbor = rgba.get_pixel(nx, ny);

                            // Check for intermediate colors (anti-aliasing)
                            if neighbor[3] > 0 && neighbor[3] < 255 {
                                return true; // Semi-transparent edge
                            }

                            // Check for gradient-like transitions
                            if neighbor != center && neighbor[3] == 255 {
                                let diff_r = (center[0] as i32 - neighbor[0] as i32).abs();
                                let diff_g = (center[1] as i32 - neighbor[1] as i32).abs();
                                let diff_b = (center[2] as i32 - neighbor[2] as i32).abs();

                                // If color difference is too subtle, likely anti-aliasing
                                if diff_r < 30
                                    && diff_g < 30
                                    && diff_b < 30
                                    && (diff_r > 0 || diff_g > 0 || diff_b > 0)
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Enforce palette consistency
    async fn enforce_palette_consistency(&self, image_data: &[u8]) -> Result<Vec<u8>> {
        let img = image::load_from_memory(image_data)?;
        let processed = self
            .style_manager
            .lock()
            .await
            .enforce_consistency(&img)
            .await?;

        // Convert back to bytes
        let mut buffer = Vec::new();
        processed.write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )?;

        Ok(buffer)
    }

    /// Extract style information from generated style guide
    async fn extract_style_information(&self, _style_guide_data: &[u8]) -> Result<()> {
        // This would analyze the style guide image and extract:
        // - Color palette
        // - Shading patterns
        // - Outline styles
        // - Common motifs

        // For now, we'll just note that we have a style guide
        tracing::info!("Style guide generated and stored for consistency enforcement");

        Ok(())
    }

    /// Format helpers (these might be better in a shared module)
    fn format_shading(&self, technique: &super::consistency::ShadingTechnique) -> &'static str {
        use super::consistency::ShadingTechnique;
        match technique {
            ShadingTechnique::Flat => "flat color",
            ShadingTechnique::TwoTone => "two-tone",
            ShadingTechnique::ThreeTone => "three-tone",
            ShadingTechnique::Dithered => "dithered",
            ShadingTechnique::Pillow => "pillow shading",
        }
    }

    fn format_outline(&self, style: &super::consistency::OutlineStyle) -> &'static str {
        use super::consistency::OutlineStyle;
        match style {
            OutlineStyle::None => "no outline",
            OutlineStyle::SinglePixel(_) => "single pixel outline",
            OutlineStyle::DoublePixel(_) => "double pixel outline",
            OutlineStyle::Selective(_) => "selective outline",
            OutlineStyle::ColoredPerObject => "colored outline",
        }
    }

    fn format_perspective(&self, perspective: &super::consistency::Perspective) -> &'static str {
        use super::consistency::Perspective;
        match perspective {
            Perspective::TopDown => "top-down",
            Perspective::ThreeQuarterView => "3/4 isometric",
            Perspective::Isometric => "isometric",
            Perspective::SideScroller => "side-scrolling",
        }
    }
}

#[async_trait::async_trait]
impl AiGenerator for ImageGenerator {
    async fn estimate_tokens(&self, _request: &str) -> Result<usize> {
        // Images don't use text tokens in the same way
        // Return estimated "token equivalent" based on image complexity
        Ok(self
            .token_counter
            .lock()
            .await
            .estimate_image_tokens(1024, 1024))
    }

    async fn estimate_cost(&self, _request: &str) -> Result<f64> {
        // DALL-E 3 standard quality 1024x1024
        Ok(0.04)
    }

    async fn is_cached(&self, key: &str) -> bool {
        self.cache.lock().await.get(key).await.is_some()
    }

    async fn clear_cache(&self, key: &str) -> Result<()> {
        self.cache.lock().await.clear(key).await
    }
}

/// Sprite generation request
#[derive(Debug, Clone)]
pub struct SpriteRequest {
    pub name: String,
    pub sprite_type: String,
    pub description: String,
}

/// Validation criteria for generated images
#[derive(Debug, Clone)]
pub enum ValidationCriteria {
    StyleGuide,
    Sprite(String),
    Tileset(String),
    UIElement(String),
    Background,
}

/// Validation result
#[derive(Debug)]
pub struct ValidationResult {
    pub passed: bool,
    pub score: f32,
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
}

/// Game concept for style guide generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConcept {
    pub title: String,
    pub genre: String,
    pub mood: String,
    pub visual_inspirations: Vec<String>,
    pub color_themes: Vec<String>,
}

/// Sprite sheet generation utilities
pub mod sprite_sheets {
    use super::*;

    /// Generate a complete sprite sheet for a character
    pub async fn generate_character_sheet(
        generator: &ImageGenerator,
        character_name: &str,
        character_class: &str,
        animations: Vec<String>,
    ) -> Result<DynamicImage> {
        let mut sprites = Vec::new();

        // Generate each animation frame
        for animation in &animations {
            let description = format!(
                "{character_name} {character_class} character performing {animation} animation, 16-bit pixel art sprite"
            );

            let sprite_data = generator
                .generate_sprite(&format!("character_{animation}"), &description, None)
                .await?;

            let sprite = image::load_from_memory(&sprite_data)?;
            sprites.push(sprite);
        }

        // Pack into sprite sheet
        pack_sprites(sprites, 2)
    }

    /// Generate tileset for environments
    pub async fn generate_tileset(
        generator: &ImageGenerator,
        theme: &str,
        tile_types: Vec<String>,
    ) -> Result<DynamicImage> {
        let mut tiles = Vec::new();

        for tile_type in &tile_types {
            let description =
                format!("{theme} environment tile: {tile_type}, 16-bit pixel art, seamless tiling");

            let tile_data = generator
                .generate_sprite(&format!("tile_{tile_type}"), &description, None)
                .await?;

            let tile = image::load_from_memory(&tile_data)?;
            tiles.push(tile);
        }

        pack_sprites(tiles, 0)
    }
}

/// Pack multiple sprites into a sprite sheet
pub fn pack_sprites(sprites: Vec<DynamicImage>, padding: u32) -> Result<DynamicImage> {
    super::consistency::sprite_sheets::pack_sprites(sprites, padding)
}

/// Recoloring utilities for cost optimization
pub mod recoloring {
    use super::*;

    /// Recolor a sprite with a new palette
    pub fn recolor_sprite(
        sprite: &DynamicImage,
        source_palette: &ColorPalette,
        target_palette: &ColorPalette,
    ) -> Result<DynamicImage> {
        let rgba = sprite.to_rgba8();
        let (width, height) = rgba.dimensions();
        let mut recolored = RgbaImage::new(width, height);

        // Build color mapping
        let color_map = build_color_map(source_palette, target_palette)?;

        // Apply recoloring
        for (x, y, pixel) in rgba.enumerate_pixels() {
            let source_color = Color::new(pixel[0], pixel[1], pixel[2]);

            let new_color = if pixel[3] < 128 {
                // Preserve transparency
                *pixel
            } else if let Some(target_color) = color_map.get(&source_color) {
                Rgba([target_color.r, target_color.g, target_color.b, pixel[3]])
            } else {
                // Find nearest color in source palette and map
                let nearest = find_nearest_in_palette(&source_color, source_palette);
                if let Some(target) = color_map.get(&nearest) {
                    Rgba([target.r, target.g, target.b, pixel[3]])
                } else {
                    *pixel // Keep original if no mapping found
                }
            };

            recolored.put_pixel(x, y, new_color);
        }

        Ok(DynamicImage::ImageRgba8(recolored))
    }

    fn build_color_map(
        source: &ColorPalette,
        target: &ColorPalette,
    ) -> Result<HashMap<Color, Color>> {
        let mut map = HashMap::new();

        // Map primary colors
        for (i, source_color) in source.primary_colors.iter().enumerate() {
            if let Some(target_color) = target.primary_colors.get(i) {
                map.insert(*source_color, *target_color);
            }
        }

        // Map secondary colors
        for (i, source_color) in source.secondary_colors.iter().enumerate() {
            if let Some(target_color) = target.secondary_colors.get(i) {
                map.insert(*source_color, *target_color);
            }
        }

        // Map accent colors
        for (i, source_color) in source.accent_colors.iter().enumerate() {
            if let Some(target_color) = target.accent_colors.get(i) {
                map.insert(*source_color, *target_color);
            }
        }

        Ok(map)
    }

    fn find_nearest_in_palette(color: &Color, palette: &ColorPalette) -> Color {
        let mut all_colors: Vec<&Color> = Vec::new();
        all_colors.extend(&palette.primary_colors);
        all_colors.extend(&palette.secondary_colors);
        all_colors.extend(&palette.accent_colors);

        all_colors
            .into_iter()
            .min_by_key(|&p| {
                let dr = color.r as i32 - p.r as i32;
                let dg = color.g as i32 - p.g as i32;
                let db = color.b as i32 - p.b as i32;
                dr * dr + dg * dg + db * db
            })
            .cloned()
            .unwrap_or(*color)
    }
}
