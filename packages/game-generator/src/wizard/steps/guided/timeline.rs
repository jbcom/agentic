use super::types::{Decade, GuidedModeState};
use crate::vintage_games::{self, TimelineGame};
use bevy_egui::egui;

/// Render the timeline browser UI
pub fn render_timeline(ui: &mut egui::Ui, state: &mut GuidedModeState) {
    ui.group(|ui| {
        ui.heading("🎮 Gaming Timeline");
        ui.separator();

        // Decade selector buttons
        ui.horizontal(|ui| {
            // 1980s button
            let eighties_selected = state.selected_decade == Some(Decade::Eighties);
            let eighties_response = ui.add_sized(
                [120.0, 60.0],
                egui::Button::new(
                    egui::RichText::new(format!("{} 1980s", Decade::Eighties.icon())).size(18.0),
                )
                .selected(eighties_selected),
            );

            if eighties_response.clicked() {
                state.selected_decade = Some(Decade::Eighties);
            }

            eighties_response.on_hover_text(Decade::Eighties.description());

            // 1990s button
            let nineties_selected = state.selected_decade == Some(Decade::Nineties);
            let nineties_response = ui.add_sized(
                [120.0, 60.0],
                egui::Button::new(
                    egui::RichText::new(format!("{} 1990s", Decade::Nineties.icon())).size(18.0),
                )
                .selected(nineties_selected),
            );

            if nineties_response.clicked() {
                state.selected_decade = Some(Decade::Nineties);
            }

            nineties_response.on_hover_text(Decade::Nineties.description());
        });

        ui.separator();

        // Show selected decade info
        if let Some(decade) = state.selected_decade {
            ui.label(
                egui::RichText::new(decade.description())
                    .italics()
                    .color(egui::Color32::from_gray(180)),
            );

            ui.separator();

            // Year-by-year timeline
            let (start_year, end_year) = decade.year_range();

            egui::ScrollArea::vertical()
                .id_salt("timeline_scroll")
                .max_height(ui.available_height() - 20.0)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    for year in start_year..=end_year {
                        render_year_section(ui, state, year);
                    }
                });
        } else {
            // Welcome message
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(
                    egui::RichText::new("Select a decade to explore gaming history")
                        .size(20.0)
                        .color(egui::Color32::from_gray(150)),
                );
                ui.add_space(20.0);
                ui.label("Choose from the golden age of arcade games (1980s)");
                ui.label("or the 16-bit renaissance (1990s)");
            });
        }
    });
}

/// Render games for a specific year
fn render_year_section(ui: &mut egui::Ui, state: &mut GuidedModeState, year: i32) {
    let games = vintage_games::games_by_year(year);

    if games.is_empty() {
        return;
    }

    // Year header
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.heading(format!("{year}"));
            ui.label(format!("{} games", games.len()));
        });

        ui.separator();

        // Games grid
        ui.horizontal_wrapped(|ui| {
            for game in games {
                render_timeline_game_card(ui, state, game);
            }
        });
    });

    ui.add_space(10.0);
}

/// Render a compact game card in the timeline
fn render_timeline_game_card(
    ui: &mut egui::Ui,
    state: &mut GuidedModeState,
    game: &'static TimelineGame,
) {
    let is_selected = state.selected_games.contains_key(&game.id);
    let is_hovered = state.ui_state.hovered_game == Some(game.id);

    // Card frame
    let card_size = egui::Vec2::new(100.0, 120.0);
    let (rect, response) = ui.allocate_exact_size(card_size, egui::Sense::click());

    // Background color based on state
    let bg_color = if is_selected {
        egui::Color32::from_rgb(70, 130, 180).gamma_multiply(0.5)
    } else if is_hovered {
        egui::Color32::from_gray(70)
    } else {
        egui::Color32::from_gray(45)
    };

    ui.painter().rect_filled(rect, 4.0, bg_color);

    // Hover detection
    if response.hovered() {
        state.ui_state.hovered_game = Some(game.id);
    }

    // Click handling
    if response.clicked() {
        if is_selected {
            state.selected_games.remove(&game.id);
        } else {
            state.selected_games.insert(game.id, game);
        }
    }

    // Card content
    let mut child_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Center)),
    );
    child_ui.set_clip_rect(rect);

    // Genre icon
    let genre_icon = match game.genre {
        "Action" => "⚔️",
        "Role-Playing" => "🗡️",
        "Adventure" => "🏛️",
        "Puzzle" => "🧩",
        "Strategy" => "♟️",
        "Shooter" => "🎯",
        "Platformer" => "🏃",
        "Sports" => "⚽",
        "Racing" => "🏎️",
        _ => "🎮",
    };

    child_ui.add_space(5.0);
    child_ui.label(egui::RichText::new(genre_icon).size(24.0));

    // Game name (truncated)
    child_ui.add_space(5.0);
    let name_text = if game.name.len() > 12 {
        format!("{}...", &game.name[..12])
    } else {
        game.name.to_string()
    };

    child_ui.label(egui::RichText::new(name_text).size(11.0).strong());

    // Genre
    child_ui.label(
        egui::RichText::new(game.genre)
            .size(9.0)
            .color(egui::Color32::from_gray(160)),
    );

    // Platform count
    if !game.platforms.is_empty() {
        child_ui.label(
            egui::RichText::new(format!("{} platforms", game.platforms.len()))
                .size(8.0)
                .color(egui::Color32::from_gray(140)),
        );
    }

    // Selection indicator
    if is_selected {
        let check_pos = rect.max - egui::Vec2::new(15.0, 15.0);
        ui.painter()
            .circle_filled(check_pos, 8.0, egui::Color32::from_rgb(70, 130, 180));
        ui.painter().text(
            check_pos,
            egui::Align2::CENTER_CENTER,
            "✓",
            egui::FontId::proportional(10.0),
            egui::Color32::WHITE,
        );
    }

    // Tooltip on hover
    response.on_hover_ui(|ui| {
        render_game_tooltip(ui, game);
    });
}

/// Render detailed tooltip for a game
fn render_game_tooltip(ui: &mut egui::Ui, game: &TimelineGame) {
    ui.heading(game.name);

    ui.horizontal(|ui| {
        ui.label("Year:");
        ui.strong(game.year.to_string());
        ui.separator();
        ui.label("Genre:");
        ui.strong(game.genre);
    });

    if let Some(developer) = game.developer {
        ui.horizontal(|ui| {
            ui.label("Developer:");
            ui.label(developer);
        });
    }

    if let Some(deck) = game.deck {
        ui.separator();
        ui.label(deck);
    }

    if !game.platforms.is_empty() {
        ui.separator();
        ui.label("Platforms:");
        ui.horizontal_wrapped(|ui| {
            for platform in game.platforms {
                ui.label(
                    egui::RichText::new(platform.to_string())
                        .background_color(egui::Color32::from_gray(40))
                        .small(),
                );
            }
        });
    }
}

/// Get all available genres from the timeline
pub fn get_all_genres() -> Vec<String> {
    vintage_games::all_genres()
}

/// Get games filtered by search query and genre
pub fn get_filtered_games(search: &str, genre_filter: Option<&str>) -> Vec<&'static TimelineGame> {
    let mut games = if search.is_empty() {
        vintage_games::TIMELINE_GAMES.iter().collect()
    } else {
        vintage_games::search_games(search)
    };

    if let Some(genre) = genre_filter {
        games.retain(|game| game.genre == genre);
    }

    games
}
