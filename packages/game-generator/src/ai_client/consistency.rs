//! Style consistency management for visual assets
//!
//! Ensures all generated images maintain coherent style, colors, and aesthetics
//! Optimized for 16-bit nostalgic game art with sprite sheet support

use anyhow::{Context, Result};
use image::{DynamicImage, Rgba, RgbaImage};
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Style consistency manager for maintaining visual coherence
#[derive(Clone)]
pub struct StyleManager {
    /// Current style configuration
    style_config: Arc<Mutex<StyleConfig>>,
    /// Template engine for prompts
    template_engine: Arc<Environment<'static>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    /// Base style name (e.g., "snes_rpg", "genesis_action")
    pub style_name: String,
    /// Active color palette
    pub palette: ColorPalette,
    /// Visual style rules
    pub rules: StyleRules,
    /// Sprite specifications
    pub sprite_specs: SpriteSpecs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Palette name
    pub name: String,
    /// Primary colors (main character, UI elements)
    pub primary_colors: Vec<Color>,
    /// Secondary colors (backgrounds, effects)
    pub secondary_colors: Vec<Color>,
    /// Accent colors (highlights, special effects)
    pub accent_colors: Vec<Color>,
    /// Transparency color for sprites
    pub transparency_color: Color,
    /// Maximum colors allowed (16-bit limitation)
    pub max_colors: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn transparent() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
            a: 0,
        }
    }

    /// Convert to 16-bit color (5-6-5 RGB)
    pub fn to_16bit(&self) -> u16 {
        let r = (self.r >> 3) as u16;
        let g = (self.g >> 2) as u16;
        let b = (self.b >> 3) as u16;
        (r << 11) | (g << 5) | b
    }

    /// Create from 16-bit color
    pub fn from_16bit(color: u16) -> Self {
        let r = ((color >> 11) & 0x1F) << 3;
        let g = ((color >> 5) & 0x3F) << 2;
        let b = (color & 0x1F) << 3;
        Self::new(r as u8, g as u8, b as u8)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRules {
    /// Pixel size (1 for true pixel art, 2+ for scaled)
    pub pixel_size: u32,
    /// Outline style
    pub outline_style: OutlineStyle,
    /// Shading technique
    pub shading_technique: ShadingTechnique,
    /// Perspective
    pub perspective: Perspective,
    /// Dithering pattern
    pub dithering: DitheringPattern,
    /// Light source direction
    pub light_direction: LightDirection,
    /// Additional constraints
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutlineStyle {
    None,
    SinglePixel(Color),
    DoublePixel(Color),
    Selective(Color),
    ColoredPerObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShadingTechnique {
    Flat,
    TwoTone,
    ThreeTone,
    Dithered,
    Pillow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Perspective {
    TopDown,
    ThreeQuarterView,
    Isometric,
    SideScroller,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DitheringPattern {
    None,
    Checkerboard,
    Bayer2x2,
    Bayer4x4,
    Floyd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightDirection {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSpecs {
    /// Standard sprite dimensions
    pub character_size: (u32, u32),
    /// Tile size for environments
    pub tile_size: (u32, u32),
    /// UI element specifications
    pub ui_specs: UiSpecs,
    /// Animation frame counts
    pub animation_frames: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSpecs {
    pub button_size: (u32, u32),
    pub icon_size: (u32, u32),
    pub font_size: u32,
    pub border_width: u32,
}

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleManager {
    /// Create a new style manager
    pub fn new() -> Self {
        let default_config = StyleConfig::default_16bit_rpg();

        // Initialize template engine
        let mut env = Environment::new();

        // Load style consistency template
        let style_template = include_str!("../prompts/image/style_consistency.jinja");
        env.add_template("style_consistency", style_template)
            .expect("Failed to load style consistency template");

        Self {
            style_config: Arc::new(Mutex::new(default_config)),
            template_engine: Arc::new(env),
        }
    }

    /// Load a predefined style
    pub async fn load_style(&self, style_name: &str) -> Result<()> {
        let config = match style_name {
            "snes_rpg" => StyleConfig::snes_rpg_style(),
            "genesis_action" => StyleConfig::genesis_action_style(),
            "gb_retro" => StyleConfig::gameboy_style(),
            "nes_platformer" => StyleConfig::nes_platformer_style(),
            _ => return Err(anyhow::anyhow!("Unknown style: {style_name}")),
        };

        *self.style_config.lock().await = config;
        Ok(())
    }

    /// Get current style configuration
    pub async fn get_style(&self) -> StyleConfig {
        self.style_config.lock().await.clone()
    }

    /// Create consistent prompt additions for image generation
    pub async fn create_style_prompt(&self, base_prompt: &str) -> Result<String> {
        let config = self.style_config.lock().await;

        // Prepare context for template
        let ctx = context! {
            base_prompt => base_prompt,
            style_name => &config.style_name,
            palette_name => &config.palette.name,
            max_colors => config.palette.max_colors,
            color_list => self.format_color_list(&config.palette),
            pixel_size => config.rules.pixel_size,
            outline_style => self.format_outline_style(&config.rules.outline_style),
            shading_technique => self.format_shading(&config.rules.shading_technique),
            light_direction => self.format_light_direction(&config.rules.light_direction),
            perspective => self.format_perspective(&config.rules.perspective),
            character_width => config.sprite_specs.character_size.0,
            character_height => config.sprite_specs.character_size.1,
            constraints => config.rules.constraints.join(", ")
        };

        // Render template
        let template = self
            .template_engine
            .get_template("style_consistency")
            .context("Failed to get style consistency template")?;

        let style_prompt = template
            .render(ctx)
            .context("Failed to render style consistency template")?;

        Ok(style_prompt)
    }

    /// Process an image to enforce style consistency
    pub async fn enforce_consistency(&self, img: &DynamicImage) -> Result<DynamicImage> {
        let config = self.style_config.lock().await;

        // Step 1: Quantize to palette
        let quantized = self.quantize_to_palette(img, &config.palette)?;

        // Step 2: Apply pixel scaling if needed
        let scaled = if config.rules.pixel_size > 1 {
            self.apply_pixel_scaling(&quantized, config.rules.pixel_size)?
        } else {
            quantized
        };

        // Step 3: Apply outline if needed
        let outlined = match &config.rules.outline_style {
            OutlineStyle::None => scaled,
            _ => self.apply_outline(&scaled, &config.rules.outline_style)?,
        };

        // Step 4: Apply dithering if needed
        let dithered = match config.rules.dithering {
            DitheringPattern::None => outlined,
            _ => self.apply_dithering(&outlined, &config.rules.dithering)?,
        };

        Ok(dithered)
    }

    /// Quantize image colors to the current palette
    fn quantize_to_palette(
        &self,
        img: &DynamicImage,
        palette: &ColorPalette,
    ) -> Result<DynamicImage> {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        // Collect all palette colors
        let mut all_colors = Vec::new();
        all_colors.extend(&palette.primary_colors);
        all_colors.extend(&palette.secondary_colors);
        all_colors.extend(&palette.accent_colors);

        // Create quantized image
        let mut quantized = RgbaImage::new(width, height);

        for (x, y, pixel) in rgba.enumerate_pixels() {
            let color = if pixel[3] < 128 {
                // Transparent pixel
                palette.transparency_color
            } else {
                // Find nearest color
                self.find_nearest_color(Color::new(pixel[0], pixel[1], pixel[2]), &all_colors)
            };

            quantized.put_pixel(x, y, Rgba([color.r, color.g, color.b, color.a]));
        }

        Ok(DynamicImage::ImageRgba8(quantized))
    }

    /// Apply pixel scaling for retro effect
    fn apply_pixel_scaling(&self, img: &DynamicImage, scale: u32) -> Result<DynamicImage> {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let new_width = width / scale;
        let new_height = height / scale;

        // Downscale
        let small = image::imageops::resize(
            &rgba,
            new_width,
            new_height,
            image::imageops::FilterType::Nearest,
        );

        // Upscale back
        let scaled =
            image::imageops::resize(&small, width, height, image::imageops::FilterType::Nearest);

        Ok(DynamicImage::ImageRgba8(scaled))
    }

    /// Apply outline to sprites
    fn apply_outline(&self, img: &DynamicImage, style: &OutlineStyle) -> Result<DynamicImage> {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let mut outlined = rgba.clone();

        let outline_color = match style {
            OutlineStyle::SinglePixel(c)
            | OutlineStyle::DoublePixel(c)
            | OutlineStyle::Selective(c) => *c,
            _ => Color::new(0, 0, 0),
        };

        // Simple outline algorithm
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let pixel = rgba.get_pixel(x, y);

                // Skip if not transparent
                if pixel[3] > 128 {
                    continue;
                }

                // Check neighbors
                let mut has_opaque_neighbor = false;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = (x as i32 + dx) as u32;
                        let ny = (y as i32 + dy) as u32;

                        if nx < width && ny < height {
                            let neighbor = rgba.get_pixel(nx, ny);
                            if neighbor[3] > 128 {
                                has_opaque_neighbor = true;
                                break;
                            }
                        }
                    }
                    if has_opaque_neighbor {
                        break;
                    }
                }

                if has_opaque_neighbor {
                    outlined.put_pixel(
                        x,
                        y,
                        Rgba([outline_color.r, outline_color.g, outline_color.b, 255]),
                    );
                }
            }
        }

        Ok(DynamicImage::ImageRgba8(outlined))
    }

    /// Apply dithering pattern
    fn apply_dithering(
        &self,
        img: &DynamicImage,
        pattern: &DitheringPattern,
    ) -> Result<DynamicImage> {
        match pattern {
            DitheringPattern::None => Ok(img.clone()),
            DitheringPattern::Checkerboard => {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                let mut dithered = rgba.clone();

                // Apply checkerboard pattern
                for y in 0..height {
                    for x in 0..width {
                        if (x + y) % 2 == 0 {
                            let pixel = dithered.get_pixel_mut(x, y);
                            // Darken alternating pixels slightly
                            pixel[0] = (pixel[0] as f32 * 0.9) as u8;
                            pixel[1] = (pixel[1] as f32 * 0.9) as u8;
                            pixel[2] = (pixel[2] as f32 * 0.9) as u8;
                        }
                    }
                }

                Ok(DynamicImage::ImageRgba8(dithered))
            }
            DitheringPattern::Bayer2x2 => {
                // Simple 2x2 Bayer matrix dithering
                let bayer_matrix = [[0.0, 0.5], [0.75, 0.25]];
                self.apply_bayer_dithering(img, &bayer_matrix, 2)
            }
            DitheringPattern::Bayer4x4 => {
                // 4x4 Bayer matrix
                let bayer_matrix = [
                    [0.0, 0.5, 0.125, 0.625],
                    [0.75, 0.25, 0.875, 0.375],
                    [0.1875, 0.6875, 0.0625, 0.5625],
                    [0.9375, 0.4375, 0.8125, 0.3125],
                ];
                self.apply_bayer_dithering(img, &bayer_matrix, 4)
            }
            DitheringPattern::Floyd => {
                // Floyd-Steinberg dithering - for now, simplified version
                Ok(img.clone())
            }
        }
    }

    /// Apply Bayer matrix dithering
    fn apply_bayer_dithering<const N: usize>(
        &self,
        img: &DynamicImage,
        matrix: &[[f32; N]; N],
        _size: usize,
    ) -> Result<DynamicImage> {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let mut dithered = rgba.clone();

        for y in 0..height {
            for x in 0..width {
                let pixel = dithered.get_pixel_mut(x, y);
                let threshold = matrix[(y as usize) % N][(x as usize) % N];

                // Apply threshold-based adjustment
                for i in 0..3 {
                    let value = pixel[i] as f32 / 255.0;
                    let adjusted = if value > threshold { 255 } else { 0 };
                    pixel[i] = ((pixel[i] as f32 * 0.7) + (adjusted as f32 * 0.3)) as u8;
                }
            }
        }

        Ok(DynamicImage::ImageRgba8(dithered))
    }

    /// Find nearest color in palette
    fn find_nearest_color(&self, color: Color, palette: &[Color]) -> Color {
        palette
            .iter()
            .min_by_key(|&&p| {
                let dr = color.r as i32 - p.r as i32;
                let dg = color.g as i32 - p.g as i32;
                let db = color.b as i32 - p.b as i32;
                dr * dr + dg * dg + db * db
            })
            .copied()
            .unwrap_or(color)
    }

    /// Format color list for prompt
    fn format_color_list(&self, palette: &ColorPalette) -> String {
        let mut colors = Vec::new();

        for (i, color) in palette.primary_colors.iter().enumerate() {
            colors.push(format!(
                "Primary{}: #{:02X}{:02X}{:02X}",
                i + 1,
                color.r,
                color.g,
                color.b
            ));
        }

        colors.join(", ")
    }

    fn format_outline_style(&self, style: &OutlineStyle) -> &'static str {
        match style {
            OutlineStyle::None => "no outline",
            OutlineStyle::SinglePixel(_) => "single pixel black outline",
            OutlineStyle::DoublePixel(_) => "double pixel black outline",
            OutlineStyle::Selective(_) => "selective outline on key elements",
            OutlineStyle::ColoredPerObject => "colored outline matching object",
        }
    }

    fn format_shading(&self, technique: &ShadingTechnique) -> &'static str {
        match technique {
            ShadingTechnique::Flat => "flat colors",
            ShadingTechnique::TwoTone => "two-tone shading",
            ShadingTechnique::ThreeTone => "three-tone shading",
            ShadingTechnique::Dithered => "dithered shading",
            ShadingTechnique::Pillow => "pillow shading",
        }
    }

    fn format_light_direction(&self, direction: &LightDirection) -> &'static str {
        match direction {
            LightDirection::TopLeft => "top-left",
            LightDirection::Top => "top",
            LightDirection::TopRight => "top-right",
            LightDirection::Left => "left",
            LightDirection::Center => "center",
            LightDirection::Right => "right",
            LightDirection::BottomLeft => "bottom-left",
            LightDirection::Bottom => "bottom",
            LightDirection::BottomRight => "bottom-right",
        }
    }

    fn format_perspective(&self, perspective: &Perspective) -> &'static str {
        match perspective {
            Perspective::TopDown => "top-down view",
            Perspective::ThreeQuarterView => "3/4 view (45-degree angle)",
            Perspective::Isometric => "isometric view",
            Perspective::SideScroller => "side-scrolling view",
        }
    }
}

impl StyleConfig {
    /// Default 16-bit RPG style
    pub fn default_16bit_rpg() -> Self {
        Self::snes_rpg_style()
    }

    /// SNES RPG style configuration
    pub fn snes_rpg_style() -> Self {
        Self {
            style_name: "snes_rpg".to_string(),
            palette: ColorPalette {
                name: "SNES Fantasy".to_string(),
                primary_colors: vec![
                    Color::new(34, 32, 52),
                    Color::new(69, 40, 60),
                    Color::new(102, 57, 49),
                    Color::new(143, 86, 59),
                ],
                secondary_colors: vec![
                    Color::new(223, 113, 38),
                    Color::new(217, 160, 102),
                    Color::new(238, 195, 154),
                    Color::new(251, 242, 54),
                ],
                accent_colors: vec![
                    Color::new(153, 229, 80),
                    Color::new(106, 190, 48),
                    Color::new(55, 148, 110),
                    Color::new(75, 105, 47),
                ],
                transparency_color: Color::transparent(),
                max_colors: 16,
            },
            rules: StyleRules {
                pixel_size: 1,
                outline_style: OutlineStyle::SinglePixel(Color::new(0, 0, 0)),
                shading_technique: ShadingTechnique::ThreeTone,
                perspective: Perspective::ThreeQuarterView,
                dithering: DitheringPattern::None,
                light_direction: LightDirection::TopLeft,
                constraints: vec![
                    "No anti-aliasing".to_string(),
                    "Hard pixel edges".to_string(),
                    "Classic JRPG aesthetic".to_string(),
                ],
            },
            sprite_specs: SpriteSpecs {
                character_size: (16, 24),
                tile_size: (16, 16),
                ui_specs: UiSpecs {
                    button_size: (64, 24),
                    icon_size: (16, 16),
                    font_size: 8,
                    border_width: 2,
                },
                animation_frames: Self::default_animation_frames(),
            },
        }
    }

    /// Genesis action game style
    pub fn genesis_action_style() -> Self {
        Self {
            style_name: "genesis_action".to_string(),
            palette: ColorPalette {
                name: "Genesis Action".to_string(),
                primary_colors: vec![
                    Color::new(0, 0, 0),
                    Color::new(29, 43, 83),
                    Color::new(126, 37, 83),
                    Color::new(0, 135, 81),
                ],
                secondary_colors: vec![
                    Color::new(171, 82, 54),
                    Color::new(95, 87, 79),
                    Color::new(194, 195, 199),
                    Color::new(255, 241, 232),
                ],
                accent_colors: vec![
                    Color::new(255, 0, 77),
                    Color::new(255, 163, 0),
                    Color::new(255, 236, 39),
                    Color::new(0, 228, 54),
                ],
                transparency_color: Color::transparent(),
                max_colors: 16,
            },
            rules: StyleRules {
                pixel_size: 1,
                outline_style: OutlineStyle::None,
                shading_technique: ShadingTechnique::TwoTone,
                perspective: Perspective::SideScroller,
                dithering: DitheringPattern::Checkerboard,
                light_direction: LightDirection::Top,
                constraints: vec![
                    "High contrast".to_string(),
                    "Bold colors".to_string(),
                    "Action-oriented".to_string(),
                ],
            },
            sprite_specs: SpriteSpecs {
                character_size: (32, 32),
                tile_size: (16, 16),
                ui_specs: UiSpecs {
                    button_size: (48, 16),
                    icon_size: (16, 16),
                    font_size: 8,
                    border_width: 1,
                },
                animation_frames: Self::default_animation_frames(),
            },
        }
    }

    /// Game Boy style
    pub fn gameboy_style() -> Self {
        Self {
            style_name: "gb_retro".to_string(),
            palette: ColorPalette {
                name: "Game Boy Green".to_string(),
                primary_colors: vec![
                    Color::new(15, 56, 15),    // Darkest green
                    Color::new(48, 98, 48),    // Dark green
                    Color::new(139, 172, 15),  // Light green
                    Color::new(188, 190, 150), // Lightest green
                ],
                secondary_colors: vec![],
                accent_colors: vec![],
                transparency_color: Color::transparent(),
                max_colors: 4,
            },
            rules: StyleRules {
                pixel_size: 1,
                outline_style: OutlineStyle::None,
                shading_technique: ShadingTechnique::TwoTone,
                perspective: Perspective::TopDown,
                dithering: DitheringPattern::Bayer2x2,
                light_direction: LightDirection::Top,
                constraints: vec![
                    "4 colors only".to_string(),
                    "Monochrome green".to_string(),
                    "160x144 resolution aware".to_string(),
                ],
            },
            sprite_specs: SpriteSpecs {
                character_size: (16, 16),
                tile_size: (8, 8),
                ui_specs: UiSpecs {
                    button_size: (32, 16),
                    icon_size: (8, 8),
                    font_size: 8,
                    border_width: 1,
                },
                animation_frames: Self::default_animation_frames(),
            },
        }
    }

    /// NES platformer style
    pub fn nes_platformer_style() -> Self {
        Self {
            style_name: "nes_platformer".to_string(),
            palette: ColorPalette {
                name: "NES Classic".to_string(),
                primary_colors: vec![
                    Color::new(0, 0, 0),    // Black
                    Color::new(31, 0, 116), // Dark blue
                    Color::new(127, 11, 0), // Dark red
                    Color::new(0, 74, 0),   // Dark green
                ],
                secondary_colors: vec![
                    Color::new(187, 187, 187), // Light gray
                    Color::new(255, 255, 255), // White
                    Color::new(171, 161, 255), // Light blue
                    Color::new(255, 119, 119), // Light red
                ],
                accent_colors: vec![
                    Color::new(255, 238, 0), // Yellow
                    Color::new(255, 102, 0), // Orange
                    Color::new(127, 255, 0), // Light green
                    Color::new(255, 0, 255), // Magenta
                ],
                transparency_color: Color::transparent(),
                max_colors: 13,
            },
            rules: StyleRules {
                pixel_size: 1,
                outline_style: OutlineStyle::None,
                shading_technique: ShadingTechnique::Flat,
                perspective: Perspective::SideScroller,
                dithering: DitheringPattern::None,
                light_direction: LightDirection::Top,
                constraints: vec![
                    "3 colors per sprite".to_string(),
                    "Sprite flicker aware".to_string(),
                    "8x8 or 8x16 sprites".to_string(),
                ],
            },
            sprite_specs: SpriteSpecs {
                character_size: (16, 16),
                tile_size: (8, 8),
                ui_specs: UiSpecs {
                    button_size: (32, 16),
                    icon_size: (8, 8),
                    font_size: 8,
                    border_width: 1,
                },
                animation_frames: Self::default_animation_frames(),
            },
        }
    }

    fn default_animation_frames() -> HashMap<String, u32> {
        let mut frames = HashMap::new();
        frames.insert("idle".to_string(), 2);
        frames.insert("walk".to_string(), 4);
        frames.insert("run".to_string(), 6);
        frames.insert("jump".to_string(), 3);
        frames.insert("attack".to_string(), 4);
        frames.insert("hurt".to_string(), 2);
        frames.insert("death".to_string(), 4);
        frames
    }
}

/// Sprite sheet optimization utilities
pub mod sprite_sheets {
    use super::*;
    use image::{GenericImage, GenericImageView, RgbaImage};

    /// Pack multiple sprites into a sprite sheet
    pub fn pack_sprites(sprites: Vec<DynamicImage>, padding: u32) -> Result<DynamicImage> {
        if sprites.is_empty() {
            return Err(anyhow::anyhow!("No sprites to pack"));
        }

        // Calculate optimal sheet dimensions
        let sprite_count = sprites.len();
        let cols = (sprite_count as f32).sqrt().ceil() as u32;
        let rows = ((sprite_count as f32) / cols as f32).ceil() as u32;

        // Get max sprite dimensions
        let (max_width, max_height) = sprites
            .iter()
            .map(|s| s.dimensions())
            .fold((0, 0), |(mw, mh), (w, h)| (mw.max(w), mh.max(h)));

        // Create sprite sheet
        let sheet_width = cols * (max_width + padding) + padding;
        let sheet_height = rows * (max_height + padding) + padding;
        let mut sheet = RgbaImage::new(sheet_width, sheet_height);

        // Fill with transparency
        for pixel in sheet.pixels_mut() {
            *pixel = Rgba([255, 0, 255, 0]); // Magenta transparency
        }

        // Pack sprites
        for (idx, sprite) in sprites.iter().enumerate() {
            let col = idx as u32 % cols;
            let row = idx as u32 / cols;
            let x = padding + col * (max_width + padding);
            let y = padding + row * (max_height + padding);

            sheet.copy_from(&sprite.to_rgba8(), x, y)?;
        }

        Ok(DynamicImage::ImageRgba8(sheet))
    }

    /// Extract sprites from a sprite sheet
    pub fn extract_sprites(
        sheet: &DynamicImage,
        sprite_width: u32,
        sprite_height: u32,
        padding: u32,
    ) -> Result<Vec<DynamicImage>> {
        let (sheet_width, sheet_height) = sheet.dimensions();
        let cols = (sheet_width - padding) / (sprite_width + padding);
        let rows = (sheet_height - padding) / (sprite_height + padding);

        let mut sprites = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                let x = padding + col * (sprite_width + padding);
                let y = padding + row * (sprite_height + padding);

                if x + sprite_width <= sheet_width && y + sprite_height <= sheet_height {
                    let sprite = sheet.crop_imm(x, y, sprite_width, sprite_height);
                    sprites.push(sprite);
                }
            }
        }

        Ok(sprites)
    }

    /// Generate sprite sheet metadata
    pub fn generate_metadata(
        sprites: &[DynamicImage],
        names: Vec<String>,
        sheet_width: u32,
        padding: u32,
    ) -> SpriteSheetMetadata {
        let mut frames = HashMap::new();
        let (sprite_width, sprite_height) = if !sprites.is_empty() {
            sprites[0].dimensions()
        } else {
            (0, 0)
        };

        let cols = (sheet_width - padding) / (sprite_width + padding);

        for (idx, name) in names.into_iter().enumerate() {
            let col = idx as u32 % cols;
            let row = idx as u32 / cols;
            let x = padding + col * (sprite_width + padding);
            let y = padding + row * (sprite_height + padding);

            frames.insert(
                name,
                SpriteFrame {
                    x,
                    y,
                    width: sprite_width,
                    height: sprite_height,
                },
            );
        }

        SpriteSheetMetadata {
            frames,
            padding,
            format: "rgba8".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheetMetadata {
    pub frames: HashMap<String, SpriteFrame>,
    pub padding: u32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteFrame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
