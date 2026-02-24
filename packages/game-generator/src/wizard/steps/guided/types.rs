use bevy::prelude::*;
use bevy_egui::egui;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use vintage_blending_core::graph::BlendPath;

/// State for the guided mode workflow
#[derive(Debug, Default, Resource)]
pub struct GuidedModeState {
    pub selected_decade: Option<Decade>,
    pub selected_games: HashMap<u32, &'static crate::vintage_games::TimelineGame>,
    pub blend_result: Option<BlendResult>,
    pub ui_state: GuiState,
    pub search_query: String,
    pub genre_filter: Option<String>,
    pub current_step: u32,
}

impl GuidedModeState {
    pub fn toggle_game_selection(&mut self, game_id: u32) {
        if self.selected_games.contains_key(&game_id) {
            self.selected_games.remove(&game_id);
        } else {
            // This would need to be called with the actual game reference
            // For now, we'll handle this in the UI code
        }
    }
}

#[derive(Debug, Default)]
pub struct GuiState {
    pub hovered_game: Option<u32>,
    pub show_blend_details: bool,
    pub scroll_position: f32,
    pub timeline_scroll: f32,
}

/// Decades for timeline browsing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Decade {
    Eighties, // 1980-1989
    Nineties, // 1990-1995
}

impl Decade {
    pub fn from_year(year: i32) -> Option<Self> {
        match year {
            1980..=1989 => Some(Decade::Eighties),
            1990..=1995 => Some(Decade::Nineties),
            _ => None,
        }
    }

    pub fn year_range(&self) -> (i32, i32) {
        match self {
            Decade::Eighties => (1980, 1989),
            Decade::Nineties => (1990, 1995),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Decade::Eighties => "1980s",
            Decade::Nineties => "1990s",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Decade::Eighties => "The birth of gaming: From arcade classics to the NES revolution",
            Decade::Nineties => "The 16-bit golden age: RPGs flourish and genres mature",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Decade::Eighties => "🕹️",
            Decade::Nineties => "🎮",
        }
    }
}

/// Represents the blended result of selected games
#[derive(Debug, Clone)]
pub struct BlendResult {
    pub name: String,
    pub description: String,
    pub blend_path: BlendPath,
    pub genres: HashMap<String, f32>, // Genre to weight
    pub mechanics: HashSet<String>,
    pub art_styles: Vec<String>,
    pub complexity_score: f32,
    pub action_strategy_balance: f32,
    pub synergies: Vec<Synergy>,
    pub conflicts: Vec<Conflict>,
    pub recommended_features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Synergy {
    pub game1: String,
    pub game2: String,
    pub description: String,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub game1: String,
    pub game2: String,
    pub conflict_type: String,
    pub resolution: String,
}

/// Visual style for game cards
pub struct GameCardStyle {
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub border_radius: f32,
    pub selected_color: egui::Color32,
    pub hover_color: egui::Color32,
    pub normal_color: egui::Color32,
}

impl Default for GameCardStyle {
    fn default() -> Self {
        Self {
            width: 150.0,
            height: 200.0,
            padding: 10.0,
            border_radius: 8.0,
            selected_color: egui::Color32::from_rgb(100, 149, 237),
            hover_color: egui::Color32::from_rgb(80, 80, 80),
            normal_color: egui::Color32::from_rgb(50, 50, 50),
        }
    }
}

/// Configuration export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidedModeExport {
    pub blend_name: String,
    pub description: String,
    pub source_games: Vec<SourceGame>,
    pub genre_weights: HashMap<String, f32>,
    pub mechanics: Vec<String>,
    pub art_styles: Vec<String>,
    pub complexity: f32,
    pub action_strategy_balance: f32,
    pub recommended_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceGame {
    pub name: String,
    pub year: i32,
    pub genre: String,
    pub developer: Option<String>,
}
