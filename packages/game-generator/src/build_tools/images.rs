//! Image downloading functionality

use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub struct ImageDownloader {
    client: reqwest::blocking::Client,
    images_dir: String,
}

impl ImageDownloader {
    pub fn new(images_dir: &str) -> Result<Self> {
        // Create the images directory if it doesn't exist
        fs::create_dir_all(images_dir)?;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            images_dir: images_dir.to_string(),
        })
    }

    /// Download game cover images from timeline games in parallel
    pub fn download_game_covers(&self, timeline_games: &[serde_json::Value]) -> Result<()> {
        println!("Downloading game cover images...");

        // Prepare download tasks
        let download_tasks: Vec<(u32, String, PathBuf)> = timeline_games
            .iter()
            .filter_map(|game| {
                let game_id = game.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

                // Try to get small image URL first, fallback to thumb
                let image_url = game
                    .get("image_small_url")
                    .and_then(|v| v.as_str())
                    .or_else(|| game.get("image_thumb_url").and_then(|v| v.as_str()))?;

                let image_path = Path::new(&self.images_dir).join(format!("{game_id}.jpg"));

                // Skip if already downloaded
                if image_path.exists() {
                    None
                } else {
                    Some((game_id, image_url.to_string(), image_path))
                }
            })
            .collect();

        let total_to_download = download_tasks.len();
        let already_existed = timeline_games.len() - total_to_download;

        // Atomic counters for thread-safe progress tracking
        let downloaded = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicUsize::new(0));

        // Create a shared client for all threads
        let client = Arc::new(self.client.clone());

        // Process downloads in parallel with limited concurrency
        // Using chunks to respect rate limits (5 concurrent downloads)
        download_tasks.par_chunks(5).for_each(|chunk| {
            for (game_id, url, image_path) in chunk {
                match Self::download_single_image(&client, url, image_path) {
                    Ok(true) => {
                        downloaded.fetch_add(1, Ordering::Relaxed);
                        let current = downloaded.load(Ordering::Relaxed);
                        if current.is_multiple_of(10) {
                            println!("  Progress: {current}/{total_to_download} images downloaded");
                        }
                    }
                    Ok(false) => {
                        // File already exists (shouldn't happen as we pre-filter)
                        // Count as already downloaded, not failed
                    }
                    Err(e) => {
                        // This includes HTTP errors (4xx, 5xx) and IO errors
                        eprintln!("  Warning: Failed to download image for game {game_id}: {e}");
                        failed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }

            // Small delay between chunks to respect rate limits
            std::thread::sleep(Duration::from_millis(100));
        });

        let final_downloaded = downloaded.load(Ordering::Relaxed);
        let final_failed = failed.load(Ordering::Relaxed);

        println!(
            "  Downloaded {final_downloaded} new game covers ({already_existed} already existed, {final_failed} failed)"
        );

        Ok(())
    }

    /// Download a single image (static method for parallel processing)
    /// Returns Ok(true) if downloaded successfully, Err if HTTP or IO error
    fn download_single_image(
        client: &reqwest::blocking::Client,
        url: &str,
        image_path: &Path,
    ) -> Result<bool> {
        // Download the image
        let response = client.get(url).send()?;

        if response.status().is_success() {
            let bytes = response.bytes()?;
            fs::write(image_path, &bytes)?;
            Ok(true)
        } else {
            // Return an error for HTTP failures so they get properly counted
            anyhow::bail!("HTTP {} for URL: {}", response.status(), url)
        }
    }

    /// Download platform logos (if available)
    pub fn download_platform_logos(&self, platforms: &[crate::types::PlatformInfo]) -> Result<()> {
        println!("Downloading platform logos...");
        let logos_dir = Path::new(&self.images_dir).join("platforms");
        fs::create_dir_all(&logos_dir)?;

        let downloaded = 0;

        for platform in platforms {
            // GiantBomb doesn't provide platform logos in their API,
            // but we could potentially fetch them from other sources
            // or use pre-defined logos. For now, we'll skip this.
            // This is a placeholder for future enhancement.
            let _ = platform;
        }

        if downloaded > 0 {
            println!("  Downloaded {downloaded} platform logos");
        }

        Ok(())
    }
}
