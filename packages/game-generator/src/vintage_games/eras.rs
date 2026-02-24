//! Era definitions and functions

use super::games::{TIMELINE_GAMES, TimelineGame};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Era {
    ArcadeGoldenAge, // 1980-1983
    EarlyConsole,    // 1984-1987
    Late8BitEarly16, // 1988-1991
    Peak16Bit,       // 1992-1995
}

impl Era {
    /// Get the year range for this era
    pub fn year_range(self) -> (i32, i32) {
        match self {
            Era::ArcadeGoldenAge => (1980, 1983),
            Era::EarlyConsole => (1984, 1987),
            Era::Late8BitEarly16 => (1988, 1991),
            Era::Peak16Bit => (1992, 1995),
        }
    }

    /// Get a human-readable name for this era
    pub fn name(self) -> &'static str {
        match self {
            Era::ArcadeGoldenAge => "Arcade Golden Age",
            Era::EarlyConsole => "Early Console Era",
            Era::Late8BitEarly16 => "Late 8-bit / Early 16-bit",
            Era::Peak16Bit => "Peak 16-bit Era",
        }
    }
}

/// Get era for a given year
pub fn era_for_year(year: i32) -> Option<Era> {
    match year {
        1980..=1983 => Some(Era::ArcadeGoldenAge),
        1984..=1987 => Some(Era::EarlyConsole),
        1988..=1991 => Some(Era::Late8BitEarly16),
        1992..=1995 => Some(Era::Peak16Bit),
        _ => None,
    }
}

/// Get a description of what makes each era special
pub fn era_description(era: Era) -> &'static str {
    match era {
        Era::ArcadeGoldenAge => {
            "Simple, focused gameplay with emerging genres. Technical constraints led to creative solutions."
        }
        Era::EarlyConsole => {
            "Home consoles arrive, genres solidify. RPGs emerge with Dragon Quest and Zelda."
        }
        Era::Late8BitEarly16 => {
            "Refined 8-bit masterpieces meet early 16-bit innovation. Peak creativity within constraints."
        }
        Era::Peak16Bit => {
            "Genre perfection with 2D art at its finest. The golden age of sprite-based games."
        }
    }
}

/// Get games from a specific era
pub fn games_by_era(era: Era) -> Vec<&'static TimelineGame> {
    let (start, end) = era.year_range();

    TIMELINE_GAMES
        .iter()
        .filter(|game| game.year >= start && game.year <= end)
        .collect()
}

/// Get all eras in chronological order
pub fn all_eras() -> Vec<Era> {
    vec![
        Era::ArcadeGoldenAge,
        Era::EarlyConsole,
        Era::Late8BitEarly16,
        Era::Peak16Bit,
    ]
}

/// Get the dominant genres for each era
pub fn era_genres(era: Era) -> Vec<String> {
    let games = games_by_era(era);
    let mut genre_counts = std::collections::HashMap::new();

    for game in games {
        *genre_counts.entry(game.genre.to_string()).or_insert(0) += 1;
    }

    let mut genres: Vec<(String, usize)> = genre_counts.into_iter().collect();
    genres.sort_by(|a, b| b.1.cmp(&a.1));

    genres.into_iter().map(|(genre, _)| genre).take(5).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_era_boundaries() {
        assert_eq!(era_for_year(1980), Some(Era::ArcadeGoldenAge));
        assert_eq!(era_for_year(1983), Some(Era::ArcadeGoldenAge));
        assert_eq!(era_for_year(1984), Some(Era::EarlyConsole));
        assert_eq!(era_for_year(1995), Some(Era::Peak16Bit));
        assert_eq!(era_for_year(1979), None);
        assert_eq!(era_for_year(1996), None);
    }
}
