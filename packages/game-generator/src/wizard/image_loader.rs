//! Pure egui image loading utilities for wizard UI
//!
//! This module provides image loading functionality that works directly with egui,
//! avoiding the complexity of Bevy's asset server for UI elements.

use anyhow::Result;
use bevy_egui::egui::{ColorImage, Context, TextureHandle};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, Mutex};

/// Global texture cache for UI images
static TEXTURE_CACHE: LazyLock<Mutex<HashMap<String, TextureHandle>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Load an image from disk and convert it to an egui texture
pub fn load_texture_from_path(
    ctx: &Context,
    path: impl AsRef<Path>,
    name: &str,
) -> Result<TextureHandle> {
    // Check cache first
    if let Ok(cache) = TEXTURE_CACHE.lock()
        && let Some(texture) = cache.get(name)
    {
        return Ok(texture.clone());
    }

    // Load image from disk
    let image_bytes = std::fs::read(path.as_ref())?;
    let image = image::load_from_memory(&image_bytes)?;

    // Convert to egui ColorImage
    let size = [image.width() as _, image.height() as _];
    let rgba = image.to_rgba8();
    let pixels = rgba.as_flat_samples();
    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    // Create texture
    let texture = ctx.load_texture(
        name,
        color_image,
        bevy_egui::egui::TextureOptions::default(),
    );

    // Cache it
    if let Ok(mut cache) = TEXTURE_CACHE.lock() {
        cache.insert(name.to_string(), texture.clone());
    }

    Ok(texture)
}

/// Load an image from memory and convert it to an egui texture
pub fn load_texture_from_memory(
    ctx: &Context,
    image_data: &[u8],
    name: &str,
) -> Result<TextureHandle> {
    // Check cache first
    if let Ok(cache) = TEXTURE_CACHE.lock()
        && let Some(texture) = cache.get(name)
    {
        return Ok(texture.clone());
    }

    // Load image
    let image = image::load_from_memory(image_data)?;

    // Convert to egui ColorImage
    let size = [image.width() as _, image.height() as _];
    let rgba = image.to_rgba8();
    let pixels = rgba.as_flat_samples();
    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    // Create texture
    let texture = ctx.load_texture(
        name,
        color_image,
        bevy_egui::egui::TextureOptions::default(),
    );

    // Cache it
    if let Ok(mut cache) = TEXTURE_CACHE.lock() {
        cache.insert(name.to_string(), texture.clone());
    }

    Ok(texture)
}

/// Clear the texture cache (useful when switching projects or modes)
pub fn clear_texture_cache() {
    if let Ok(mut cache) = TEXTURE_CACHE.lock() {
        cache.clear();
    }
}

/// Helper struct for managing wizard UI images
pub struct WizardImages {
    pub under_construction: Option<TextureHandle>,
    pub logo: Option<TextureHandle>,
    pub guided_mode_icon: Option<TextureHandle>,
    pub freeform_mode_icon: Option<TextureHandle>,
}

impl Default for WizardImages {
    fn default() -> Self {
        Self::new()
    }
}

impl WizardImages {
    pub fn new() -> Self {
        Self {
            under_construction: None,
            logo: None,
            guided_mode_icon: None,
            freeform_mode_icon: None,
        }
    }

    /// Load all wizard images
    pub fn load_all(&mut self, ctx: &Context) {
        // Try to load each image, but don't fail if one is missing
        let base_path = "crates/vintage_game_generator/assets/wizard/";

        if let Ok(texture) = load_texture_from_path(
            ctx,
            format!("{base_path}under_construction_overlay_transparent.png"),
            "under_construction",
        ) {
            self.under_construction = Some(texture);
        }

        if let Ok(texture) =
            load_texture_from_path(ctx, format!("{base_path}logo_main.png"), "logo_main")
        {
            self.logo = Some(texture);
        }

        if let Ok(texture) = load_texture_from_path(
            ctx,
            format!("{base_path}guided_mode_icon_transparent.png"),
            "guided_mode_icon",
        ) {
            self.guided_mode_icon = Some(texture);
        }

        if let Ok(texture) = load_texture_from_path(
            ctx,
            format!("{base_path}freeform_mode_icon.png"),
            "freeform_mode_icon",
        ) {
            self.freeform_mode_icon = Some(texture);
        }
    }
}

/// Load a specific wizard asset with fallback paths
pub fn load_wizard_asset(ctx: &Context, filename: &str, name: &str) -> Option<TextureHandle> {
    let possible_paths = vec![
        format!("assets/wizard/{}", filename),
        format!("crates/vintage_game_generator/assets/wizard/{}", filename),
        format!("../assets/wizard/{}", filename),
    ];

    for path in possible_paths {
        if let Ok(texture) = load_texture_from_path(ctx, &path, name) {
            return Some(texture);
        }
    }

    None
}
