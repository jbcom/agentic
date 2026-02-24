use super::types::{GameCardStyle, GuidedModeState};
use crate::vintage_games::TimelineGame;
use bevy_egui::egui;

/// Render a detailed game card for the selection panel
pub fn render_game_card(
    ui: &mut egui::Ui,
    game: &'static TimelineGame,
    state: &mut GuidedModeState,
    can_remove: bool,
) -> bool {
    let mut removed = false;
    let style = GameCardStyle::default();

    ui.group(|ui| {
        // Apply style settings
        ui.spacing_mut().item_spacing = egui::Vec2::new(style.padding, style.padding);
        // Note: rounding is now set per widget, not globally on the style

        // Check if this game is in the selected games
        let is_selected = state.selected_games.values().any(|g| g.id == game.id);
        if is_selected {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_gray(40);
        }

        // Header with game name and remove button
        ui.horizontal(|ui| {
            ui.heading(game.name);

            if can_remove {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("❌").clicked() {
                        removed = true;
                    }
                });
            }
        });

        // Year and genre
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{}", game.year))
                    .strong()
                    .color(egui::Color32::from_rgb(100, 149, 237)),
            );
            ui.separator();
            ui.label(game.genre);

            if let Some(developer) = game.developer {
                ui.separator();
                ui.label(egui::RichText::new(developer).italics().small());
            }
        });

        // Description
        if let Some(deck) = game.deck {
            ui.separator();
            ui.label(
                egui::RichText::new(deck)
                    .small()
                    .color(egui::Color32::from_gray(180)),
            );
        }

        // Platforms
        if !game.platforms.is_empty() {
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("Platforms:");
                for platform in game.platforms {
                    ui.label(
                        egui::RichText::new(&**platform)
                            .background_color(egui::Color32::from_gray(50))
                            .small(),
                    );
                }
            });
        }
    });

    removed
}

/// Render the selected games panel
pub fn render_selected_games(ui: &mut egui::Ui, state: &mut GuidedModeState) {
    ui.group(|ui| {
        ui.heading("📚 Selected Games");

        if state.selected_games.is_empty() {
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("No games selected yet")
                        .size(16.0)
                        .color(egui::Color32::from_gray(150)),
                );
                ui.add_space(10.0);
                ui.label("Click on games from the timeline to add them");
                ui.label("Select at least 2 games to create a blend");
            });
        } else {
            ui.horizontal(|ui| {
                ui.label(format!("{} games selected", state.selected_games.len()));

                if state.selected_games.len() >= 2 {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if state.blend_result.is_some() && ui.button("🔄 Re-blend").clicked() {
                            state.blend_result = None;
                        }
                    });
                }
            });

            ui.separator();

            // Show selected games in a scrollable area
            egui::ScrollArea::vertical()
                .id_salt("selected_games_scroll")
                .max_height(300.0)
                .show(ui, |ui| {
                    let mut games_to_remove = Vec::new();

                    // Clone the games to avoid borrow checker issues
                    let selected_games: Vec<(u32, &'static TimelineGame)> = state
                        .selected_games
                        .iter()
                        .map(|(id, game)| (*id, *game))
                        .collect();

                    for (id, game) in selected_games {
                        // Create a temporary UI state to avoid mutable borrow conflicts
                        let removed = ui
                            .group(|ui| {
                                // Inline the game card rendering to avoid passing mutable state
                                let mut removed = false;

                                // Header with game name and remove button
                                ui.horizontal(|ui| {
                                    ui.heading(game.name);

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.small_button("❌").clicked() {
                                                removed = true;
                                            }
                                        },
                                    );
                                });

                                // Year and genre
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{}", game.year))
                                            .strong()
                                            .color(egui::Color32::from_rgb(100, 149, 237)),
                                    );
                                    ui.separator();
                                    ui.label(game.genre);

                                    if let Some(developer) = game.developer {
                                        ui.separator();
                                        ui.label(egui::RichText::new(developer).italics().small());
                                    }
                                });

                                // Description
                                if let Some(deck) = game.deck {
                                    ui.separator();
                                    ui.label(
                                        egui::RichText::new(deck)
                                            .small()
                                            .color(egui::Color32::from_gray(180)),
                                    );
                                }

                                // Platforms
                                if !game.platforms.is_empty() {
                                    ui.separator();
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label("Platforms:");
                                        for platform in game.platforms {
                                            ui.label(
                                                egui::RichText::new(&**platform)
                                                    .background_color(egui::Color32::from_gray(50))
                                                    .small(),
                                            );
                                        }
                                    });
                                }

                                removed
                            })
                            .inner;

                        if removed {
                            games_to_remove.push(id);
                        }
                        ui.add_space(5.0);
                    }

                    // Remove games after iteration
                    for id in games_to_remove {
                        state.selected_games.remove(&id);
                        // Clear blend result if we removed a game
                        state.blend_result = None;
                    }
                });
        }
    });
}

/// Render game attributes for blending visualization
pub fn render_game_attributes(ui: &mut egui::Ui, game: &TimelineGame) {
    ui.group(|ui| {
        ui.label(egui::RichText::new(game.name).strong());

        // Genre badge
        ui.horizontal(|ui| {
            ui.label("Genre:");
            ui.label(
                egui::RichText::new(game.genre)
                    .background_color(egui::Color32::from_rgb(70, 130, 180))
                    .color(egui::Color32::WHITE),
            );
        });

        // Year era
        let era = match game.year {
            1980..=1983 => "Arcade Golden Age",
            1984..=1987 => "Early Console",
            1988..=1991 => "8-bit/16-bit Transition",
            1992..=1995 => "16-bit Peak",
            _ => "Unknown Era",
        };

        ui.horizontal(|ui| {
            ui.label("Era:");
            ui.label(egui::RichText::new(era).italics().small());
        });

        // Complexity estimate based on year and genre
        let complexity = estimate_game_complexity(game);
        ui.horizontal(|ui| {
            ui.label("Complexity:");
            render_complexity_bar(ui, complexity);
        });
    });
}

/// Estimate game complexity based on genre and year
fn estimate_game_complexity(game: &TimelineGame) -> f32 {
    let base_complexity = match game.genre {
        "Role-Playing" => 0.8,
        "Strategy" => 0.7,
        "Adventure" => 0.6,
        "Action" => 0.4,
        "Puzzle" => 0.5,
        "Sports" => 0.3,
        "Racing" => 0.3,
        _ => 0.5,
    };

    // Complexity increases over time
    let year_factor = (game.year - 1980) as f32 / 15.0 * 0.3;

    (base_complexity + year_factor).min(1.0)
}

/// Render a visual complexity bar
fn render_complexity_bar(ui: &mut egui::Ui, complexity: f32) {
    let bar_width = 100.0;
    let bar_height = 10.0;

    let (rect, _) =
        ui.allocate_exact_size(egui::Vec2::new(bar_width, bar_height), egui::Sense::hover());

    // Background
    ui.painter()
        .rect_filled(rect, 2.0, egui::Color32::from_gray(50));

    // Filled portion
    let filled_rect = egui::Rect::from_min_size(
        rect.min,
        egui::Vec2::new(bar_width * complexity, bar_height),
    );

    let color = if complexity < 0.3 {
        egui::Color32::from_rgb(100, 200, 100)
    } else if complexity < 0.7 {
        egui::Color32::from_rgb(200, 200, 100)
    } else {
        egui::Color32::from_rgb(200, 100, 100)
    };

    ui.painter().rect_filled(filled_rect, 2.0, color);

    // Complexity label
    ui.label(egui::RichText::new(format!("{:.0}%", complexity * 100.0)).small());
}
