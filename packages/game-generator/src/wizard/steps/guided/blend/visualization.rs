use super::engine::create_blend;
use crate::wizard::steps::guided::types::GuidedModeState;
use bevy_egui::egui;

/// Render the blend visualization UI
pub fn render_blend_visualization(ui: &mut egui::Ui, state: &mut GuidedModeState) {
    let mut clear_blend = false;

    if let Some(blend) = &state.blend_result {
        ui.group(|ui| {
            ui.heading(format!("🧪 {}", blend.name));
            ui.label(
                egui::RichText::new(&blend.description)
                    .italics()
                    .color(egui::Color32::from_gray(180)),
            );

            ui.separator();

            // Genre distribution
            ui.collapsing("Genre Distribution", |ui| {
                render_genre_chart(ui, &blend.genres);
            });

            // Mechanics
            ui.collapsing("Combined Mechanics", |ui| {
                render_mechanics_cloud(ui, &blend.mechanics);
            });

            // Complexity and balance
            ui.collapsing("Game Balance", |ui| {
                render_balance_metrics(ui, blend.complexity_score, blend.action_strategy_balance);
            });

            // Synergies
            if !blend.synergies.is_empty() {
                ui.collapsing("✨ Synergies", |ui| {
                    for synergy in &blend.synergies {
                        ui.group(|ui| {
                            ui.label(format!("{} + {}", synergy.game1, synergy.game2));
                            ui.label(
                                egui::RichText::new(&synergy.description)
                                    .small()
                                    .color(egui::Color32::from_rgb(100, 200, 100)),
                            );
                        });
                    }
                });
            }

            // Conflicts
            if !blend.conflicts.is_empty() {
                ui.collapsing("⚠️ Conflicts to Resolve", |ui| {
                    for conflict in &blend.conflicts {
                        ui.group(|ui| {
                            ui.label(format!("{} vs {}", conflict.game1, conflict.game2));
                            ui.label(
                                egui::RichText::new(&conflict.conflict_type)
                                    .small()
                                    .color(egui::Color32::from_rgb(200, 200, 100)),
                            );
                            ui.label(
                                egui::RichText::new(format!("💡 {}", conflict.resolution))
                                    .small()
                                    .italics(),
                            );
                        });
                    }
                });
            }

            // Recommendations
            ui.collapsing("💡 Recommended Features", |ui| {
                for feature in &blend.recommended_features {
                    ui.label(format!("• {feature}"));
                }
            });

            // Art styles
            ui.collapsing("🎨 Visual Style", |ui| {
                for style in &blend.art_styles {
                    ui.label(format!("• {style}"));
                }
            });

            // Export button
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("📥 Export Configuration").clicked() {
                    // Export will be handled by the export module
                }

                if ui.button("🔄 Modify Selection").clicked() {
                    clear_blend = true;
                }
            });
        });
    } else if state.selected_games.len() >= 2 {
        // Show blend button
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            if ui
                .button(egui::RichText::new("🧪 Create Blend").size(20.0))
                .clicked()
            {
                create_blend(state);
            }
            ui.add_space(10.0);
            ui.label("Click to analyze game compatibility and generate blend");
        });
    }

    // Apply deferred state changes
    if clear_blend {
        state.blend_result = None;
    }
}

/// Render genre distribution as a simple bar chart
fn render_genre_chart(ui: &mut egui::Ui, genres: &std::collections::HashMap<String, f32>) {
    ui.group(|ui| {
        let mut sorted_genres: Vec<_> = genres.iter().collect();
        sorted_genres.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        for (genre, weight) in sorted_genres {
            ui.horizontal(|ui| {
                ui.label(format!("{genre:>15}"));

                // Draw bar
                let bar_width = 200.0 * weight;
                let bar_height = 16.0;
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(bar_width, bar_height),
                    egui::Sense::hover(),
                );

                ui.painter().rect_filled(rect, 2.0, genre_color(genre));

                ui.label(format!("{:.0}%", weight * 100.0));
            });
        }
    });
}

/// Get color for a genre
fn genre_color(genre: &str) -> egui::Color32 {
    match genre {
        "Action" => egui::Color32::from_rgb(200, 50, 50),
        "Role-Playing" => egui::Color32::from_rgb(50, 50, 200),
        "Strategy" => egui::Color32::from_rgb(200, 150, 50),
        "Puzzle" => egui::Color32::from_rgb(150, 50, 200),
        "Adventure" => egui::Color32::from_rgb(50, 200, 150),
        "Platformer" => egui::Color32::from_rgb(50, 200, 50),
        "Shooter" => egui::Color32::from_rgb(200, 100, 50),
        "Sports" => egui::Color32::from_rgb(100, 200, 50),
        "Racing" => egui::Color32::from_rgb(200, 200, 50),
        _ => egui::Color32::from_gray(128),
    }
}

/// Render mechanics as a tag cloud
fn render_mechanics_cloud(ui: &mut egui::Ui, mechanics: &std::collections::HashSet<String>) {
    ui.horizontal_wrapped(|ui| {
        for mechanic in mechanics {
            let formatted = mechanic.replace('_', " ");
            ui.label(
                egui::RichText::new(&formatted)
                    .background_color(egui::Color32::from_gray(60))
                    .color(egui::Color32::from_gray(220)),
            );
        }
    });
}

/// Render balance metrics with visual bars
fn render_balance_metrics(ui: &mut egui::Ui, complexity: f32, action_balance: f32) {
    ui.group(|ui| {
        // Complexity meter
        ui.horizontal(|ui| {
            ui.label("Complexity:");
            render_meter(ui, complexity, "Simple", "Complex");
        });

        // Action/Strategy balance
        ui.horizontal(|ui| {
            ui.label("Gameplay:");
            render_meter(ui, action_balance, "Strategic", "Action");
        });
    });
}

/// Render a meter from 0.0 to 1.0
fn render_meter(ui: &mut egui::Ui, value: f32, low_label: &str, high_label: &str) {
    let meter_width = 150.0;
    let meter_height = 20.0;

    ui.vertical(|ui| {
        // Draw meter background
        let (rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(meter_width, meter_height),
            egui::Sense::hover(),
        );

        // Background gradient
        ui.painter()
            .rect_filled(rect, 4.0, egui::Color32::from_gray(40));

        // Value indicator
        let indicator_x = rect.min.x + (meter_width * value);
        let indicator_rect = egui::Rect::from_center_size(
            egui::Pos2::new(indicator_x, rect.center().y),
            egui::Vec2::new(4.0, meter_height - 4.0),
        );

        ui.painter()
            .rect_filled(indicator_rect, 2.0, egui::Color32::from_rgb(100, 149, 237));

        // Labels
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(low_label)
                    .small()
                    .color(egui::Color32::from_gray(150)),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(high_label)
                        .small()
                        .color(egui::Color32::from_gray(150)),
                );
            });
        });
    });
}
