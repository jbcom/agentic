use crate::wizard::config::ConfigManager;
use crate::wizard::overlay::{
    ClickableAreaConfig as ClickableImageConfig, show_image_with_overlays,
};
use bevy_egui::egui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LanguageChoice {
    Rust,
    Python,
    Ruby,
}

impl LanguageChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageChoice::Rust => "rust",
            LanguageChoice::Python => "python",
            LanguageChoice::Ruby => "ruby",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "rust" => Some(LanguageChoice::Rust),
            "python" => Some(LanguageChoice::Python),
            "ruby" => Some(LanguageChoice::Ruby),
            _ => None,
        }
    }
}

pub fn draw_language_step(
    ui: &mut egui::Ui,
    config_manager: &mut Option<ConfigManager>,
) -> Option<LanguageChoice> {
    let mut selected = None;

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        // Title
        ui.heading(egui::RichText::new("Select Target Language").size(28.0).strong());
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Choose the programming language for your game").size(16.0));
        ui.add_space(40.0);

        // Load the language icons configuration
        let config_result = ClickableImageConfig::from_ron_file(
            "crates/vintage_game_generator/assets/wizard/programming_languages_icons.ron"
        );

        // Load the programming languages image and under construction overlay
        let languages_image = std::fs::read("crates/vintage_game_generator/assets/wizard/programming_languages_icons.png").ok();
        let under_construction_image = std::fs::read("crates/vintage_game_generator/assets/wizard/under_construction_overlay_transparent.png").ok();

        if let (Ok(config), Some(image_bytes)) = (config_result, languages_image) {
            if let Ok(image) = image::load_from_memory(&image_bytes) {
                let texture = ui.ctx().load_texture(
                    "programming_languages",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [image.width() as _, image.height() as _],
                        &image.to_rgba8().into_raw()
                    ),
                    Default::default()
                );

                // Display the clickable image
                let target_size = egui::Vec2::new(600.0, 343.0); // Maintain aspect ratio
                if let Some(clicked_id) = show_image_with_overlays(
                    ui,
                    &texture,
                    target_size,
                    &config,
                    egui::Color32::from_rgb(100, 149, 237), // Highlight color
                    &[],  // No text overlays
                    &[],  // No disabled hotspots
                    None, // No under construction texture
                ) {
                    selected = LanguageChoice::parse(&clicked_id);
                }

                ui.add_space(20.0);

                // Language descriptions below the image
                ui.columns(3, |columns| {
                    // Python column
                    columns[0].vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Python").size(20.0).strong().color(egui::Color32::from_rgb(53, 114, 165)));
                        ui.add_space(10.0);
                        ui.label("🎯 Best for:");
                        ui.label("• Beginners");
                        ui.label("• Quick prototypes");
                        ui.label("• Educational games");
                        ui.add_space(10.0);
                        ui.label("💪 Strengths:");
                        ui.label("• Easy to learn");
                        ui.label("• Rapid development");
                        ui.label("• PyGame library");
                    });

                    // Rust column
                    columns[1].vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Rust").size(20.0).strong().color(egui::Color32::from_rgb(255, 106, 0)));
                        ui.add_space(10.0);
                        ui.label("🎯 Best for:");
                        ui.label("• Performance games");
                        ui.label("• Browser games");
                        ui.label("• Complex systems");
                        ui.add_space(10.0);
                        ui.label("💪 Strengths:");
                        ui.label("• Blazing fast");
                        ui.label("• Memory safe");
                        ui.label("• WASM support");
                    });

                    // Ruby column
                    columns[2].vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Ruby").size(20.0).strong().color(egui::Color32::from_rgb(204, 52, 45)));
                        ui.add_space(10.0);

                        // Show under construction for Ruby
                        if let Some(overlay_bytes) = &under_construction_image {
                            if let Ok(overlay) = image::load_from_memory(overlay_bytes) {
                                let overlay_texture = ui.ctx().load_texture(
                                    "under_construction",
                                    egui::ColorImage::from_rgba_unmultiplied(
                                        [overlay.width() as _, overlay.height() as _],
                                        &overlay.to_rgba8().into_raw()
                                    ),
                                    Default::default()
                                );
                                ui.image(&overlay_texture);
                            }
                        } else {
                            ui.label(egui::RichText::new("🚧 Coming Soon! 🚧").size(16.0));
                            ui.add_space(10.0);
                            ui.label("Ruby support is");
                            ui.label("under development");
                        }
                    });
                });

            } else {
                // Fallback if image loading fails
                ui.label("Failed to load language selection image");
            }
        } else {
            // Fallback UI without images
            ui.horizontal(|ui| {
                if ui.button(egui::RichText::new("Python 🐍").size(20.0)).clicked() {
                    selected = Some(LanguageChoice::Python);
                }
                ui.add_space(20.0);
                if ui.button(egui::RichText::new("Rust 🦀").size(20.0)).clicked() {
                    selected = Some(LanguageChoice::Rust);
                }
                ui.add_space(20.0);
                ui.add_enabled_ui(false, |ui| {
                    let _ = ui.button(egui::RichText::new("Ruby 💎 (Coming Soon)").size(20.0));
                });
            });
        }

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // Additional info
        ui.label(egui::RichText::new("All languages will generate complete, playable retro-style games").size(14.0).weak());
        ui.label(egui::RichText::new("with appropriate libraries and frameworks for each language").size(14.0).weak());
    });

    // Update config manager if a selection was made
    if let (Some(choice), Some(config)) = (selected, config_manager.as_mut())
        && let Err(e) = config.set_language(choice.as_str())
    {
        eprintln!("Failed to save language selection: {e}");
    }

    selected
}
