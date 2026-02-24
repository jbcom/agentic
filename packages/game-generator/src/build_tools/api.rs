//! GiantBomb API client

use crate::types::*;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

pub struct GiantBombClient {
    client: reqwest::blocking::Client,
    api_key: String,
}

impl GiantBombClient {
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self { client, api_key })
    }

    /// Fetch platform information
    pub fn fetch_platforms(&self) -> Result<Vec<PlatformInfo>> {
        println!("Fetching platform information...");

        let url = format!(
            "{}/platforms/?api_key={}&format=json&field_list=id,name,abbreviation,deck,install_base,original_price,release_date,online_support",
            GIANTBOMB_API_BASE, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .context("Failed to fetch platforms")?;

        if !response.status().is_success() {
            anyhow::bail!("Platform API returned status: {}", response.status());
        }

        let platform_response: GiantBombResponse<PlatformInfo> = response
            .json()
            .context("Failed to parse platform response")?;

        if platform_response.status_code != 1 {
            anyhow::bail!("Platform API error: {}", platform_response.error);
        }

        // Filter to just vintage platforms
        let vintage_platforms: Vec<PlatformInfo> = platform_response
            .results
            .into_iter()
            .filter(|p| {
                VINTAGE_PLATFORMS
                    .iter()
                    .any(|vp| p.name.contains(vp) || vp.contains(p.name.as_str()))
            })
            .collect();

        println!("  Found {} vintage platforms", vintage_platforms.len());
        Ok(vintage_platforms)
    }

    /// Fetch games for a timeline period
    pub fn fetch_timeline_games(
        &self,
        start_year: i32,
        end_year: i32,
    ) -> Result<HashMap<i32, HashMap<String, Game>>> {
        let mut timeline: HashMap<i32, HashMap<String, Game>> = HashMap::new();
        let mut processed_ids = HashSet::new();

        for year in start_year..=end_year {
            println!("  Fetching games from {year}...");

            let year_games = self.fetch_year_games(year, &mut processed_ids)?;
            if !year_games.is_empty() {
                timeline.insert(year, year_games);
            }

            // Rate limit
            thread::sleep(Duration::from_millis(500));
        }

        Ok(timeline)
    }

    /// Fetch games for a specific year
    fn fetch_year_games(
        &self,
        year: i32,
        processed_ids: &mut HashSet<u32>,
    ) -> Result<HashMap<String, Game>> {
        let url = format!(
            "{}/games/?api_key={}&format=json&limit=100\
            &filter=original_release_date:{}-01-01|{}-12-31\
            &field_list=id,guid,name,deck,image,original_release_date,platforms,genres,developers,site_detail_url\
            &sort=number_of_user_reviews:desc",
            GIANTBOMB_API_BASE, self.api_key, year, year
        );

        let response = match self.client.get(&url).send() {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("    Warning: Failed to fetch {year} games: {e}");
                return Ok(HashMap::new());
            }
        };

        if !response.status().is_success() {
            eprintln!(
                "    Warning: API returned status {} for year {}",
                response.status(),
                year
            );
            return Ok(HashMap::new());
        }

        let gb_response: GiantBombResponse<Game> = match response.json() {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("    Warning: Failed to parse response for {year}: {e}");
                return Ok(HashMap::new());
            }
        };

        if gb_response.status_code != 1 {
            eprintln!("    Warning: API error for {}: {}", year, gb_response.error);
            return Ok(HashMap::new());
        }

        // Group games by genre and pick the best reviewed one
        let mut year_games: HashMap<String, Game> = HashMap::new();
        let total_games = gb_response.results.len();
        let mut games_without_platforms = 0;
        let mut games_without_vintage_platforms = 0;

        for game in gb_response.results {
            // Skip if we've already processed this game
            if !processed_ids.insert(game.id) {
                continue;
            }

            // Check if game has platforms
            let platforms = match &game.platforms {
                Some(p) if !p.is_empty() => p,
                _ => {
                    games_without_platforms += 1;
                    continue;
                }
            };

            // Filter by vintage platforms
            let has_vintage_platform = platforms.iter().any(|p| {
                VINTAGE_PLATFORMS
                    .iter()
                    .any(|vp| p.name.contains(vp) || vp.contains(p.name.as_str()))
            });

            if !has_vintage_platform {
                games_without_vintage_platforms += 1;
                continue;
            }

            // Extract primary genre (or use "Action" as default)
            let primary_genre = game
                .genres
                .as_ref()
                .and_then(|genres| genres.first())
                .map(|g| g.name.clone())
                .unwrap_or_else(|| "Action".to_string());

            // Only keep if it's the first game we've seen for this genre this year
            use std::collections::hash_map::Entry;
            if let Entry::Vacant(e) = year_games.entry(primary_genre.clone()) {
                eprintln!(
                    "    Added: {} ({}) - {}",
                    game.name,
                    &primary_genre,
                    platforms
                        .iter()
                        .map(|p| p.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                e.insert(game);
            }

            // Stop after we have enough genres for this year
            if year_games.len() >= TOP_GENRES_PER_YEAR {
                break;
            }
        }

        if year_games.is_empty() && total_games > 0 {
            eprintln!(
                "    No games selected from {total_games} total (no platforms: {games_without_platforms}, no vintage platforms: {games_without_vintage_platforms})"
            );
        }

        Ok(year_games)
    }

    /// Enhance games with detailed image information
    pub fn enhance_games_with_images(
        &self,
        timeline: HashMap<i32, HashMap<String, Game>>,
    ) -> Result<Vec<(i32, String, Game)>> {
        println!("Fetching detailed images for games...");
        let mut games_with_images = Vec::new();

        for year in timeline.keys().cloned().collect::<Vec<_>>() {
            if let Some(year_games) = timeline.get(&year) {
                for (genre, game) in year_games {
                    let enhanced_game = self.enhance_game_images(game)?;
                    games_with_images.push((year, genre.clone(), enhanced_game));

                    // Rate limit
                    thread::sleep(Duration::from_millis(200));
                }
            }
        }

        Ok(games_with_images)
    }

    /// Enhance a single game with detailed images
    fn enhance_game_images(&self, game: &Game) -> Result<Game> {
        let mut enhanced_game = game.clone();

        // If we have image tags, fetch detailed images
        if let Some(image_info) = &game.image
            && let Some(tags) = &image_info.image_tags
            && !tags.trim().is_empty()
        {
            // Fetch images using the game's GUID
            let images_url = format!(
                "{}/images/{}/?api_key={}&format=json&limit=10",
                GIANTBOMB_API_BASE, game.guid, self.api_key
            );

            if let Ok(response) = self.client.get(&images_url).send()
                && response.status().is_success()
                && let Ok(images_response) = response.json::<GiantBombResponse<GameImage>>()
                && images_response.status_code == 1
                && !images_response.results.is_empty()
            {
                // Use the first image result to enhance our image info
                if let Some(first_image) = images_response.results.first() {
                    enhanced_game.image = Some(ImageInfo {
                        icon_url: Some(first_image.icon_url.clone()),
                        medium_url: Some(first_image.medium_url.clone()),
                        screen_url: Some(first_image.screen_url.clone()),
                        screen_large_url: Some(first_image.screen_large_url.clone()),
                        small_url: Some(first_image.small_url.clone()),
                        super_url: Some(first_image.super_url.clone()),
                        thumb_url: Some(first_image.thumb_url.clone()),
                        tiny_url: Some(first_image.tiny_url.clone()),
                        original_url: first_image.original_url.clone(),
                        image_tags: image_info.image_tags.clone(),
                    });
                }
            }
        }

        Ok(enhanced_game)
    }
}
