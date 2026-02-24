use crate::wizard::config::ConfigManager;
use crate::wizard::image_loader;
use crate::wizard::overlay::{
    ClickableAreaConfig as ClickableImageConfig, show_image_with_overlays,
};
use bevy_egui::egui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WelcomeAction {
    GuidedMode,
    FreeformMode,
}

pub fn draw_welcome_step(
    ui: &mut egui::Ui,
    config_manager: &mut Option<ConfigManager>,
) -> Option<WelcomeAction> {
    let mut action = None;

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        // Title
        ui.heading(egui::RichText::new("🎮 Vintage Game Generator").size(36.0).strong());
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Choose your creation mode").size(20.0));
        ui.add_space(40.0);

        // Try to load and display the mode selection image using pure egui
        let mut image_loaded = false;

        // Load the welcome image and its configuration
        if let Some(texture) = image_loader::load_wizard_asset(
            ui.ctx(),
            "welcome_mode_selection.png",
            "welcome_mode_selection"
        ) {
            // Try to load the corresponding RON configuration
            let ron_paths = vec![
                "assets/wizard/welcome_mode_selection.ron",
                "crates/vintage_game_generator/assets/wizard/welcome_mode_selection.ron",
            ];

            for ron_path in &ron_paths {
                if let Ok(config) = ClickableImageConfig::from_ron_file(ron_path) {

                        // Calculate display size maintaining aspect ratio
                        let available_width = ui.available_width() * 0.9;
                        let texture_size = texture.size_vec2();
                        let aspect_ratio = texture_size.x / texture_size.y;
                        let target_size = egui::Vec2::new(
                            available_width.min(800.0),
                            (available_width.min(800.0) / aspect_ratio).min(600.0)
                        );

                        // Define text overlays
                        let font_heading = egui::FontId::proportional(32.0);
                        let font_subheading = egui::FontId::proportional(18.0);
                        let font_cta = egui::FontId::proportional(20.0);

                        let text_overlays = vec![
                            ("left_top", "GUIDED MODE", font_heading.clone()),
                            ("left_bottom", "Browse & Blend Classic Games", font_cta.clone()),
                            ("right_top", "FREEFORM MODE", font_heading.clone()),
                            ("right_bottom", "AI-Powered Game Creation", font_cta.clone()),
                            ("center_divider", "OR", font_subheading),
                        ];

                        // Show the clickable image with text
                        if let Some(clicked_id) = show_image_with_overlays(
                            ui,
                            &texture,
                            target_size,
                            &config,
                            egui::Color32::from_rgb(100, 149, 237),
                            &text_overlays,
                            &[],  // No disabled hotspots
                            None, // No under construction texture
                        ) {
                            match clicked_id.as_str() {
                                "guided" => {
                                    action = Some(WelcomeAction::GuidedMode);
                                    if let Some(config) = config_manager.as_mut()
                                        && let Err(e) = config.set_wizard_mode("guided")
                                    {
                                        eprintln!("Failed to save mode selection: {e}");
                                    }
                                },
                                "freeform" => {
                                    action = Some(WelcomeAction::FreeformMode);
                                    if let Some(config) = config_manager.as_mut()
                                        && let Err(e) = config.set_wizard_mode("freeform")
                                    {
                                        eprintln!("Failed to save mode selection: {e}");
                                    }
                                },
                                _ => {}
                            }
                        }

                        image_loaded = true;
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Click on either side to choose your path").size(14.0).weak());
                        break;
                    }
                }
            }

        // If image couldn't be loaded, show a simple selection UI
        if !image_loaded {
            ui.label(egui::RichText::new("Choose your creation mode:").size(16.0));
            ui.add_space(20.0);
        }

        // Two-panel selection with proper spacing
        let available_width = ui.available_width();
        let panel_width = (available_width - 40.0) / 2.0; // Account for spacing

        ui.horizontal(|ui| {
            // Guided Mode Panel
            ui.allocate_ui_with_layout(
                egui::Vec2::new(panel_width, 450.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    let response = ui.group(|ui| {
                        ui.set_min_height(440.0);
                        ui.set_min_width(panel_width - 10.0);

                        ui.add_space(20.0);

                        // Mode title
                        ui.label(egui::RichText::new("GUIDED MODE").size(24.0).strong().color(egui::Color32::from_rgb(100, 149, 237)));
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Browse Gaming History").size(16.0));
                        ui.add_space(20.0);

                        // Icon with text overlay - try multiple paths
                        let icon_paths = vec![
                            "assets/wizard/guided_mode_icon_transparent.png",
                            "crates/vintage_game_generator/assets/wizard/guided_mode_icon_transparent.png",
                        ];

                        let mut icon_shown = false;
                        for path in &icon_paths {
                            if let Ok(image_bytes) = std::fs::read(path)
                                && let Ok(image) = image::load_from_memory(&image_bytes)
                            {
                                let size = egui::Vec2::new(200.0, 200.0);
                                let texture_id = ui.ctx().load_texture(
                                    "guided_mode_icon_transparent",
                                    egui::ColorImage::from_rgba_unmultiplied(
                                        [image.width() as _, image.height() as _],
                                        &image.to_rgba8().into_raw()
                                    ),
                                    Default::default()
                                );
                                ui.image((texture_id.id(), size));
                                icon_shown = true;
                                break;
                            }
                        }

                        if !icon_shown {
                            // Fallback visual
                            ui.label(egui::RichText::new("📅").size(80.0));
                        }

                        ui.add_space(20.0);

                        // Description
                        ui.label("Select from classic games");
                        ui.label("spanning three decades:");
                        ui.add_space(10.0);
                        ui.label("• 1970s - Dawn of Gaming");
                        ui.label("• 1980s - Arcade Golden Age");
                        ui.label("• 1990s - Console Revolution");

                        ui.add_space(30.0);

                        let button_response = ui.button(
                            egui::RichText::new("Choose Guided").size(18.0)
                        );

                        button_response.clicked()
                    });

                    if response.inner {
                        action = Some(WelcomeAction::GuidedMode);
                        if let Some(config) = config_manager.as_mut()
                            && let Err(e) = config.set_wizard_mode("guided")
                        {
                            eprintln!("Failed to save mode selection: {e}");
                        }
                    }
                });

            ui.add_space(20.0);

            // Freeform Mode Panel
            ui.allocate_ui_with_layout(
                egui::Vec2::new(panel_width, 450.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    let response = ui.group(|ui| {
                        ui.set_min_height(440.0);
                        ui.set_min_width(panel_width - 10.0);

                        ui.add_space(20.0);

                        // Mode title
                        ui.label(egui::RichText::new("FREEFORM MODE").size(24.0).strong().color(egui::Color32::from_rgb(255, 140, 90)));
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Create Your Vision").size(16.0));
                        ui.add_space(20.0);

                        // Icon - try multiple paths
                        let icon_paths = vec![
                            "assets/wizard/freeform_mode_icon.png",
                            "crates/vintage_game_generator/assets/wizard/freeform_mode_icon.png",
                        ];

                        let mut icon_shown = false;
                        for path in &icon_paths {
                            if let Ok(image_bytes) = std::fs::read(path)
                                && let Ok(image) = image::load_from_memory(&image_bytes)
                            {
                                let size = egui::Vec2::new(200.0, 200.0);
                                let texture_id = ui.ctx().load_texture(
                                    "freeform_mode_icon",
                                    egui::ColorImage::from_rgba_unmultiplied(
                                        [image.width() as _, image.height() as _],
                                        &image.to_rgba8().into_raw()
                                    ),
                                    Default::default()
                                );
                                ui.image((texture_id.id(), size));
                                icon_shown = true;
                                break;
                            }
                        }

                        if !icon_shown {
                            // Fallback visual
                            ui.label(egui::RichText::new("✨").size(80.0));
                        }

                        ui.add_space(20.0);

                        // Description
                        ui.label("Design from scratch with");
                        ui.label("AI-powered assistance:");
                        ui.add_space(10.0);
                        ui.label("• Describe your game idea");
                        ui.label("• Get intelligent suggestions");
                        ui.label("• Full creative control");

                        ui.add_space(30.0);

                        let button_response = ui.button(
                            egui::RichText::new("Choose Freeform").size(18.0)
                        );

                        button_response.clicked()
                    });

                    if response.inner {
                        action = Some(WelcomeAction::FreeformMode);
                        if let Some(config) = config_manager.as_mut()
                            && let Err(e) = config.set_wizard_mode("freeform")
                        {
                            eprintln!("Failed to save mode selection: {e}");
                        }
                    }
                });
        });

        ui.add_space(20.0);

        // Info text at bottom
        ui.separator();
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Both modes lead to the same powerful game generation engine").size(14.0).weak());
    });

    action
}
