//! Vintage game timeline module (1980-1995)
//!
//! This module contains a curated timeline of exemplar games from the golden and retro eras.
//! Each year features the highest-rated game from up to 3 different genres.
//! Games are selected to serve as creative inspiration for the AI RPG generator.

pub mod eras;
pub mod games;
pub mod graph;
pub mod platforms;

// Re-export commonly used items
pub use eras::{Era, era_description, era_for_year, games_by_era};
pub use games::{
    TIMELINE_GAMES, TimelineGame, all_genres, games_by_genre, games_by_year, search_games,
};
pub use graph::{GameNode, build_game_graph};
pub use platforms::{PLATFORM_INFO, PlatformInfo, get_platform_info};

/// Timeline span
pub const TIMELINE_START: i32 = 1980;
pub const TIMELINE_END: i32 = 1995;
