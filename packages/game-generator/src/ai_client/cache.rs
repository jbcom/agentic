//! Intelligent caching system for AI operations
//!
//! Reduces API calls and costs by caching:
//! - Text generation results
//! - Image generation results  
//! - Audio generation results
//! - Embeddings and blend calculations
//! - Style consistency data

use anyhow::{Context, Result};
use image::ImageEncoder;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main cache manager for all AI operations
#[derive(Clone)]
pub struct AiCache {
    /// In-memory cache for fast access
    memory_cache: Arc<RwLock<HashMap<String, CachedItem>>>,
    /// Disk cache directory
    cache_dir: PathBuf,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum memory cache size in MB
    pub max_memory_mb: usize,
    /// Maximum disk cache size in MB
    pub max_disk_mb: usize,
    /// Default TTL in seconds
    pub default_ttl: u64,
    /// Enable compression for large items
    pub enable_compression: bool,
    /// Cache directory path
    pub cache_dir: PathBuf,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 100,
            max_disk_mb: 1000,
            default_ttl: 3600 * 24 * 7, // 1 week
            enable_compression: true,
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from(".cache"))
                .join("ai_rpg_generator"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedItem {
    /// Cache key
    pub key: String,
    /// Cached data
    pub data: CachedData,
    /// Metadata about the cached item
    pub metadata: CacheMetadata,
    /// Size in bytes
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CachedData {
    /// Text generation result
    Text(String),
    /// Image data (PNG/JPEG bytes)
    Image(Vec<u8>),
    /// Audio data (WAV/MP3 bytes)
    Audio(Vec<u8>),
    /// JSON data
    Json(serde_json::Value),
    /// Binary data
    Binary(Vec<u8>),
    /// Embedding vector
    Embedding(Vec<f32>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Expiration timestamp
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Original prompt/request hash
    pub request_hash: String,
    /// Model used for generation
    pub model: Option<String>,
    /// Generation parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Number of times accessed
    pub access_count: u32,
    /// Last access time
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Number of items in cache
    pub items_count: u64,
    /// Total bytes saved
    pub bytes_saved: u64,
    /// Estimated cost saved (in USD)
    pub cost_saved: f64,
    /// Current memory usage
    pub memory_usage_bytes: usize,
    /// Current disk usage
    pub disk_usage_bytes: u64,
}

impl AiCache {
    /// Create a new cache instance
    pub fn new() -> Result<Self> {
        let config = CacheConfig::default();
        Self::with_config(config)
    }

    /// Create cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Result<Self> {
        // Ensure cache directory exists
        std::fs::create_dir_all(&config.cache_dir).context("Failed to create cache directory")?;

        Ok(Self {
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir: config.cache_dir.clone(),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        })
    }

    /// Generate cache key from request parameters
    pub fn generate_key(
        &self,
        prefix: &str,
        content: &str,
        params: &HashMap<String, String>,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prefix);
        hasher.update(content);

        // Sort parameters for consistent hashing
        let mut sorted_params: Vec<_> = params.iter().collect();
        sorted_params.sort_by_key(|(k, _)| *k);

        for (key, value) in sorted_params {
            hasher.update(key);
            hasher.update(value);
        }

        format!("{:x}", hasher.finalize())
    }

    /// Get item from cache
    pub async fn get(&self, key: &str) -> Option<CachedItem> {
        // Check memory cache first
        {
            let mut cache = self.memory_cache.write().await;
            if let Some(item) = cache.get_mut(key) {
                // Check expiration
                let now = chrono::Utc::now();
                if item.metadata.expires_at > now {
                    // Update access stats
                    item.metadata.access_count += 1;
                    item.metadata.last_accessed = now;

                    // Update global stats
                    let mut stats = self.stats.write().await;
                    stats.hits += 1;

                    return Some(item.clone());
                } else {
                    // Remove expired item
                    cache.remove(key);
                }
            }
        }

        // Check disk cache
        if let Ok(item) = self.load_from_disk(key).await {
            // Add to memory cache if space available
            let mut cache = self.memory_cache.write().await;
            if self.can_fit_in_memory(&item).await {
                cache.insert(key.to_string(), item.clone());
            }

            // Update stats
            let mut stats = self.stats.write().await;
            stats.hits += 1;

            return Some(item);
        }

        // Cache miss
        let mut stats = self.stats.write().await;
        stats.misses += 1;

        None
    }

    /// Put item in cache
    pub async fn put(
        &self,
        key: String,
        data: CachedData,
        params: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::seconds(self.config.default_ttl as i64);

        let size = match &data {
            CachedData::Text(s) => s.len(),
            CachedData::Image(v) | CachedData::Audio(v) | CachedData::Binary(v) => v.len(),
            CachedData::Json(j) => serde_json::to_vec(j)?.len(),
            CachedData::Embedding(v) => v.len() * std::mem::size_of::<f32>(),
        };

        let metadata = CacheMetadata {
            created_at: now,
            expires_at,
            request_hash: key.clone(),
            model: params
                .get("model")
                .and_then(|v| v.as_str())
                .map(String::from),
            parameters: params,
            access_count: 0,
            last_accessed: now,
        };

        let item = CachedItem {
            key: key.clone(),
            data,
            metadata,
            size,
        };

        // Save to disk first
        self.save_to_disk(&item).await?;

        // Add to memory cache if space available
        if self.can_fit_in_memory(&item).await {
            let mut cache = self.memory_cache.write().await;
            cache.insert(key, item);
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.memory_usage_bytes = self.calculate_memory_usage().await;

        Ok(())
    }

    /// Clear specific cache entry
    pub async fn clear(&self, key: &str) -> Result<()> {
        // Remove from memory
        self.memory_cache.write().await.remove(key);

        // Remove from disk
        let path = self.cache_path(key);
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }

        Ok(())
    }

    /// Clear all expired entries
    pub async fn clear_expired(&self) -> Result<usize> {
        let now = chrono::Utc::now();
        let mut cleared = 0;

        // Clear from memory
        {
            let mut cache = self.memory_cache.write().await;
            cache.retain(|_, item| {
                if item.metadata.expires_at <= now {
                    cleared += 1;
                    false
                } else {
                    true
                }
            });
        }

        // Clear from disk
        let entries = tokio::fs::read_dir(&self.cache_dir).await?;
        let mut entries = tokio_stream::wrappers::ReadDirStream::new(entries);

        use futures::StreamExt;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("cache")
                && let Ok(content) = tokio::fs::read(&path).await
                && let Ok(item) = bincode::deserialize::<CachedItem>(&content)
                && item.metadata.expires_at <= now
            {
                tokio::fs::remove_file(&path).await?;
                cleared += 1;
            }
        }

        Ok(cleared)
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        let stats_guard = self.stats.read().await;
        CacheStats {
            hits: stats_guard.hits,
            misses: stats_guard.misses,
            total_size_bytes: stats_guard.total_size_bytes,
            items_count: stats_guard.items_count,
            bytes_saved: stats_guard.bytes_saved,
            cost_saved: stats_guard.cost_saved,
            memory_usage_bytes: stats_guard.memory_usage_bytes,
            disk_usage_bytes: stats_guard.disk_usage_bytes,
        }
    }

    /// Check if item fits in memory cache
    async fn can_fit_in_memory(&self, item: &CachedItem) -> bool {
        let current_usage = self.calculate_memory_usage().await;
        let max_bytes = self.config.max_memory_mb * 1024 * 1024;
        current_usage + item.size <= max_bytes
    }

    /// Calculate current memory usage
    async fn calculate_memory_usage(&self) -> usize {
        let cache = self.memory_cache.read().await;
        cache.values().map(|item| item.size).sum()
    }

    /// Get disk cache path for key
    fn cache_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{key}.cache"))
    }

    /// Load item from disk
    async fn load_from_disk(&self, key: &str) -> Result<CachedItem> {
        let path = self.cache_path(key);
        let content = tokio::fs::read(&path).await?;

        let item: CachedItem = if self.config.enable_compression {
            let decompressed = zstd::decode_all(&content[..])?;
            bincode::deserialize(&decompressed)?
        } else {
            bincode::deserialize(&content)?
        };

        Ok(item)
    }

    /// Save item to disk
    async fn save_to_disk(&self, item: &CachedItem) -> Result<()> {
        let path = self.cache_path(&item.key);

        let content = if self.config.enable_compression {
            let serialized = bincode::serialize(item)?;
            zstd::encode_all(&serialized[..], 3)?
        } else {
            bincode::serialize(item)?
        };

        tokio::fs::write(&path, content).await?;

        Ok(())
    }
}

/// Cache-aware wrapper for any async function
pub async fn cached<F, T>(cache: &AiCache, key: &str, f: F) -> Result<T>
where
    F: FnOnce() -> futures::future::BoxFuture<'static, Result<T>>,
    T: Serialize + for<'de> Deserialize<'de>,
{
    // Check cache first
    if let Some(item) = cache.get(key).await
        && let CachedData::Json(json) = item.data
        && let Ok(value) = serde_json::from_value(json)
    {
        return Ok(value);
    }

    // Execute function
    let result = f().await?;

    // Cache result
    let json = serde_json::to_value(&result)?;
    cache
        .put(key.to_string(), CachedData::Json(json), HashMap::new())
        .await?;

    Ok(result)
}

/// Special cache for image data with automatic format optimization
#[derive(Clone)]
pub struct ImageCache {
    base_cache: Arc<AiCache>,
}

impl ImageCache {
    pub fn new(base_cache: Arc<AiCache>) -> Self {
        Self { base_cache }
    }

    /// Get image with automatic format conversion
    pub async fn get_image(&self, key: &str, preferred_format: ImageFormat) -> Option<Vec<u8>> {
        if let Some(item) = self.base_cache.get(key).await
            && let CachedData::Image(data) = item.data
        {
            // Convert format if needed
            if preferred_format != ImageFormat::Original {
                return self.convert_format(&data, preferred_format).ok();
            }
            return Some(data);
        }
        None
    }

    /// Put image with automatic optimization
    pub async fn put_image(
        &self,
        key: String,
        data: Vec<u8>,
        params: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        // Optimize image before caching
        let optimized = self.optimize_image(&data)?;

        self.base_cache
            .put(key, CachedData::Image(optimized), params)
            .await
    }

    fn optimize_image(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Load image
        let img = image::load_from_memory(data)?;

        // Convert to optimized format (PNG for sprites with transparency)
        let mut buffer = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )?;

        Ok(buffer)
    }

    fn convert_format(&self, data: &[u8], format: ImageFormat) -> Result<Vec<u8>> {
        let img = image::load_from_memory(data)?;
        let mut buffer = Vec::new();

        match format {
            ImageFormat::Png => img.write_to(
                &mut std::io::Cursor::new(&mut buffer),
                image::ImageFormat::Png,
            )?,
            ImageFormat::Jpeg(quality) => {
                use image::codecs::jpeg::JpegEncoder;
                let encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
                encoder.write_image(
                    img.as_bytes(),
                    img.width(),
                    img.height(),
                    img.color().into(),
                )?;
            }
            ImageFormat::WebP => {
                // Would need webp crate for this
                img.write_to(
                    &mut std::io::Cursor::new(&mut buffer),
                    image::ImageFormat::Png,
                )?
            }
            ImageFormat::Original => return Ok(data.to_vec()),
        }

        Ok(buffer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpeg(u8),
    WebP,
    Original,
}
