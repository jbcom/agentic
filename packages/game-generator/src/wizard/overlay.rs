//! Overlay module for displaying images, text, and managing clickable areas
//!
//! This module provides a comprehensive overlay system for:
//! - Image overlays (like "under construction" badges)
//! - Text overlays with styling
//! - Clickable hotspots with hover effects
//! - Layered rendering with proper interaction handling

use crate::wizard::image_loader;
use bevy::prelude::*;
use bevy_egui::egui;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Types of overlay content
#[derive(Clone)]
pub enum OverlayContent {
    /// Image overlay with texture
    Image {
        texture_id: egui::TextureId,
        tint: egui::Color32,
    },
    /// Text overlay with styling
    Text {
        text: String,
        font_id: egui::FontId,
        color: egui::Color32,
        background: Option<egui::Color32>,
        outline: Option<(f32, egui::Color32)>, // width, color
    },
    /// Clickable hotspot
    Hotspot {
        id: String,
        hover_color: egui::Color32,
        hover_stroke_width: f32,
        tooltip: Option<String>,
    },
}

/// Configuration for any overlay
#[derive(Clone)]
pub struct OverlayConfig {
    pub content: OverlayContent,
    pub rect: egui::Rect,
    pub block_interaction: bool,
    pub opacity: f32,
    pub z_order: i32, // Higher values render on top
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            content: OverlayContent::Image {
                texture_id: egui::TextureId::default(),
                tint: egui::Color32::WHITE,
            },
            rect: egui::Rect::NOTHING,
            block_interaction: true,
            opacity: 1.0,
            z_order: 0,
        }
    }
}

/// Renders an overlay at the specified position using native egui features
pub fn render_overlay(ui: &mut egui::Ui, config: &OverlayConfig) -> Option<String> {
    // Use egui's Area for proper layering
    let area_id = ui.make_persistent_id(format!("overlay_{:?}_{}", config.rect, config.z_order));

    egui::Area::new(area_id)
        .order(egui::Order::Foreground)
        .fixed_pos(config.rect.min)
        .show(ui.ctx(), |ui| {
            ui.set_clip_rect(config.rect);

            let sense = if config.block_interaction {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            };

            let response =
                ui.allocate_rect(config.rect.translate(-config.rect.min.to_vec2()), sense);
            let mut clicked_id = None;

            match &config.content {
                OverlayContent::Image { texture_id, tint } => {
                    // Use egui's Image widget for better integration
                    let image = egui::Image::from_texture((*texture_id, config.rect.size())).tint(
                        if config.opacity < 1.0 {
                            let alpha = (tint.a() as f32 * config.opacity) as u8;
                            egui::Color32::from_rgba_unmultiplied(
                                tint.r(),
                                tint.g(),
                                tint.b(),
                                alpha,
                            )
                        } else {
                            *tint
                        },
                    );

                    ui.add(image);

                    if config.block_interaction && response.clicked() {
                        clicked_id = Some("_blocked_".to_string());
                    }
                }

                OverlayContent::Text {
                    text,
                    font_id,
                    color,
                    background,
                    outline,
                } => {
                    // Use egui's native text rendering with RichText
                    if let Some(bg_color) = background {
                        ui.painter().rect_filled(response.rect, 0.0, *bg_color);
                    }

                    // Create RichText for better text handling
                    let rich_text = egui::RichText::new(text)
                        .font(font_id.clone())
                        .color(*color);

                    if let Some((width, outline_color)) = outline {
                        // Use egui's stroke for text outline effect
                        let galley = ui
                            .fonts(|f| f.layout_no_wrap(text.to_string(), font_id.clone(), *color));
                        let text_pos = response.rect.center() - galley.size() / 2.0;

                        // Draw outline by rendering text multiple times with offset
                        // The width determines how far out we draw
                        let offsets = [
                            (-*width, -*width),
                            (0.0, -*width),
                            (*width, -*width),
                            (-*width, 0.0),
                            (*width, 0.0),
                            (-*width, *width),
                            (0.0, *width),
                            (*width, *width),
                        ];

                        for (dx, dy) in offsets {
                            ui.painter().add(egui::Shape::Text(egui::epaint::TextShape {
                                pos: text_pos + egui::vec2(dx, dy),
                                galley: galley.clone(),
                                underline: egui::Stroke::NONE,
                                fallback_color: *outline_color,
                                override_text_color: Some(*outline_color),
                                opacity_factor: 1.0,
                                angle: 0.0,
                            }));
                        }
                    }

                    // Draw main text centered
                    ui.put(response.rect, egui::Label::new(rich_text));
                }

                OverlayContent::Hotspot {
                    id,
                    hover_color,
                    hover_stroke_width,
                    tooltip,
                } => {
                    // Use native hover handling
                    if response.hovered() {
                        ui.painter().rect_stroke(
                            response.rect,
                            4.0,
                            egui::Stroke::new(*hover_stroke_width, *hover_color),
                            egui::epaint::StrokeKind::Outside,
                        );

                        if response.clicked() {
                            clicked_id = Some(id.clone());
                        }

                        if let Some(tooltip_text) = tooltip {
                            response.on_hover_text(tooltip_text);
                        }
                    }
                }
            }

            clicked_id
        })
        .inner
}

/// Helper to create an "under construction" overlay for a specific rect
pub fn under_construction_overlay(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    texture_id: egui::TextureId,
) -> bool {
    let config = OverlayConfig {
        content: OverlayContent::Image {
            texture_id,
            tint: egui::Color32::WHITE,
        },
        rect,
        block_interaction: true,
        opacity: 0.9,
        z_order: 100, // High z-order to appear on top
    };

    render_overlay(ui, &config).is_some()
}

/// Apply under construction overlay to a clickable area using egui texture
pub fn apply_under_construction(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    under_construction_texture: Option<egui::TextureId>,
) -> bool {
    if let Some(texture_id) = under_construction_texture {
        under_construction_overlay(ui, rect, texture_id)
    } else {
        // If texture isn't loaded yet, just block interaction
        let response = ui.allocate_rect(rect, egui::Sense::click());
        response.clicked()
    }
}

/// Render multiple overlays in z-order using egui's layer system
pub fn render_overlay_stack(ui: &mut egui::Ui, overlays: &mut [OverlayConfig]) -> Option<String> {
    // Sort by z_order
    overlays.sort_by_key(|o| o.z_order);

    let mut clicked_id = None;

    // Use egui's layer system for proper ordering
    for (idx, overlay) in overlays.iter().enumerate() {
        let layer_id = egui::Id::new(format!("overlay_layer_{idx}"));

        // Higher z_order means rendered later (on top)
        let order = if overlay.z_order > 50 {
            egui::Order::Tooltip // Very high priority
        } else if overlay.z_order > 0 {
            egui::Order::Foreground
        } else {
            egui::Order::Middle
        };

        egui::Area::new(layer_id)
            .order(order)
            .fixed_pos(overlay.rect.min)
            .interactable(overlay.block_interaction)
            .show(ui.ctx(), |ui| {
                if let Some(id) = render_overlay(ui, overlay)
                    && clicked_id.is_none()
                {
                    clicked_id = Some(id);
                }
            });
    }

    clicked_id
}

/// Builder for creating text overlays
pub struct TextOverlayBuilder {
    text: String,
    font_size: f32,
    font_family: egui::FontFamily,
    color: egui::Color32,
    background: Option<egui::Color32>,
    outline: Option<(f32, egui::Color32)>,
}

impl TextOverlayBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 14.0,
            font_family: egui::FontFamily::Proportional,
            color: egui::Color32::WHITE,
            background: None,
            outline: None,
        }
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = color;
        self
    }

    pub fn background(mut self, color: egui::Color32) -> Self {
        self.background = Some(color);
        self
    }

    pub fn outline(mut self, width: f32, color: egui::Color32) -> Self {
        self.outline = Some((width, color));
        self
    }

    pub fn build(self, rect: egui::Rect) -> OverlayConfig {
        OverlayConfig {
            content: OverlayContent::Text {
                text: self.text,
                font_id: egui::FontId::new(self.font_size, self.font_family),
                color: self.color,
                background: self.background,
                outline: self.outline,
            },
            rect,
            block_interaction: false,
            opacity: 1.0,
            z_order: 1,
        }
    }
}

/// Trait to add overlay functionality to clickable areas
pub trait OverlayExt {
    fn with_overlay(self, ui: &mut egui::Ui, overlay: Option<&OverlayConfig>) -> egui::Response;
}

impl OverlayExt for egui::Response {
    fn with_overlay(
        mut self,
        ui: &mut egui::Ui,
        overlay: Option<&OverlayConfig>,
    ) -> egui::Response {
        if let Some(overlay_config) = overlay {
            // Check if overlay covers this response's rect
            if overlay_config.rect.intersects(self.rect) {
                // Render the overlay
                let overlay_clicked = render_overlay(ui, overlay_config);

                // If overlay blocks interaction and was clicked, consume the click
                if overlay_config.block_interaction && overlay_clicked.is_some() {
                    // Make the response think it wasn't clicked
                    self = ui.allocate_rect(self.rect, egui::Sense::hover());
                }
            }
        }
        self
    }
}

/// Helper function to load under construction texture using egui
pub fn load_under_construction_texture(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    image_loader::load_wizard_asset(
        ctx,
        "under_construction_overlay_transparent.png",
        "under_construction",
    )
}

/// Helper function to check if a point is within a rectangle
pub fn point_in_rect(point: egui::Pos2, rect: egui::Rect) -> bool {
    rect.contains(point)
}

/// Calculate overlay rect for a specific area within an image
pub fn calculate_overlay_rect(image_rect: egui::Rect, area: egui::Rect, scale: f32) -> egui::Rect {
    egui::Rect {
        min: egui::pos2(
            image_rect.min.x + area.min.x * scale,
            image_rect.min.y + area.min.y * scale,
        ),
        max: egui::pos2(
            image_rect.min.x + area.max.x * scale,
            image_rect.min.y + area.max.y * scale,
        ),
    }
}

// Re-export types for RON configuration
pub use clickable_area_types::*;

mod clickable_area_types {
    use super::*;

    /// Overall image dimensions.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ImageSize {
        pub width: u32,
        pub height: u32,
    }

    /// Axis-aligned bounding box with inclusive pixel edges.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct Bounds {
        pub x_min: u32,
        pub y_min: u32,
        pub x_max: u32,
        pub y_max: u32,
    }

    impl Bounds {
        pub fn to_rect(self, img_size: ImageSize, drawn_rect: egui::Rect) -> egui::Rect {
            let sx = drawn_rect.width() / img_size.width as f32;
            let sy = drawn_rect.height() / img_size.height as f32;

            let x0 = drawn_rect.left() + self.x_min as f32 * sx;
            let y0 = drawn_rect.top() + self.y_min as f32 * sy;
            let x1 = drawn_rect.left() + (self.x_max + 1) as f32 * sx;
            let y1 = drawn_rect.top() + (self.y_max + 1) as f32 * sy;

            egui::Rect::from_min_max(egui::Pos2::new(x0, y0), egui::Pos2::new(x1, y1))
        }
    }

    /// A named hotspot region within an image
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Hotspot {
        pub id: String,
        pub bounds: Bounds,
        pub label: String,
    }

    /// A text area overlay on the image
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextArea {
        pub id: String,
        pub bounds: Bounds,
        pub description: String,
    }

    /// Configuration for clickable areas with overlays
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ClickableAreaConfig {
        pub image_size: ImageSize,
        pub hotspots: Vec<Hotspot>,
        #[serde(default)]
        pub text_areas: Vec<TextArea>,
    }

    impl ClickableAreaConfig {
        /// Load configuration from a RON file
        pub fn from_ron_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
            let contents = std::fs::read_to_string(path)?;
            let config = ron::from_str(&contents)?;
            Ok(config)
        }
    }
}

/// Show an image with overlays using native egui layering
#[allow(clippy::too_many_arguments)]
pub fn show_image_with_overlays(
    ui: &mut egui::Ui,
    base_texture: &egui::TextureHandle,
    target_size: egui::Vec2,
    config: &ClickableAreaConfig,
    hover_color: egui::Color32,
    text_overlays: &[(&str, &str, egui::FontId)],
    disabled_hotspots: &[&str],
    under_construction_texture: Option<egui::TextureId>,
) -> Option<String> {
    let mut clicked_id = None;

    // Allocate space for the entire widget
    let (response, painter) = ui.allocate_painter(target_size, egui::Sense::hover());
    let drawn_rect = response.rect;

    // Draw base image using painter (lowest layer)
    painter.image(
        base_texture.id(),
        drawn_rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    // Process hotspots using ui.interact for proper event handling
    for hotspot in &config.hotspots {
        let rect = hotspot.bounds.to_rect(config.image_size, drawn_rect);
        let is_disabled = disabled_hotspots.contains(&hotspot.id.as_str());

        if is_disabled {
            // Draw under construction overlay if disabled
            if let Some(texture_id) = under_construction_texture {
                painter.image(
                    texture_id,
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::from_white_alpha(230), // 90% opacity
                );
            }

            // Still allocate the rect to block interaction
            ui.allocate_rect(rect, egui::Sense::click());
        } else {
            // Interactive hotspot using native interaction
            let hotspot_response = ui.interact(
                rect,
                ui.make_persistent_id(&hotspot.id),
                egui::Sense::click(),
            );

            if hotspot_response.hovered() {
                painter.rect_stroke(
                    rect,
                    4.0,
                    egui::Stroke::new(2.0, hover_color),
                    egui::epaint::StrokeKind::Outside,
                );
            }

            if hotspot_response.clicked() {
                clicked_id = Some(hotspot.id.clone());
            }

            // Add hover text after checking click to avoid move issues
            hotspot_response.on_hover_text(&hotspot.label);
        }
    }

    // Draw text overlays on top
    for text_area in &config.text_areas {
        if let Some((_, text, font_id)) =
            text_overlays.iter().find(|(id, _, _)| *id == text_area.id)
        {
            let rect = text_area.bounds.to_rect(config.image_size, drawn_rect);

            // Use egui's built-in text with stroke effect
            let galley =
                painter.layout_no_wrap(text.to_string(), font_id.clone(), egui::Color32::WHITE);

            let text_pos = rect.center() - galley.size() / 2.0;

            // Draw black outline using multiple offset positions
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx != 0 || dy != 0 {
                        painter.galley(
                            text_pos + egui::vec2(dx as f32, dy as f32),
                            galley.clone(),
                            egui::Color32::BLACK,
                        );
                    }
                }
            }

            // Draw white text on top
            painter.galley(text_pos, galley, egui::Color32::WHITE);
        }
    }

    clicked_id
}
