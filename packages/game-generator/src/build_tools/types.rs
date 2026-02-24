//! Common types used throughout the build tools

use serde::{Deserialize, Serialize};

pub const GIANTBOMB_API_BASE: &str = "https://www.giantbomb.com/api";
pub const USER_AGENT: &str = "VintageGameGenerator/1.0";
pub const RESULTS_PER_PAGE: u32 = 100;
pub const TOP_GENRES_PER_YEAR: usize = 3;
pub const VINTAGE_PLATFORMS: &[&str] = &[
    "Arcade",
    "NES",
    "Game Boy",
    "SNES",
    "Genesis",
    "PC",
    "Amiga",
    "Commodore 64",
    "Apple II",
    "MSX",
];

#[derive(Debug, Serialize, Deserialize)]
pub struct GiantBombResponse<T> {
    pub error: String,
    pub limit: u32,
    pub offset: u32,
    pub number_of_page_results: u32,
    pub number_of_total_results: u32,
    pub status_code: u32,
    pub results: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: u32,
    pub guid: String,
    pub name: String,
    #[serde(default)]
    pub aliases: Option<String>,
    #[serde(default)]
    pub deck: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image: Option<ImageInfo>,
    #[serde(default)]
    pub original_release_date: Option<String>,
    #[serde(default)]
    pub platforms: Option<Vec<Platform>>,
    #[serde(default)]
    pub genres: Option<Vec<Genre>>,
    #[serde(default)]
    pub themes: Option<Vec<Theme>>,
    #[serde(default)]
    pub developers: Option<Vec<Developer>>,
    #[serde(default)]
    pub site_detail_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageInfo {
    pub icon_url: Option<String>,
    pub medium_url: Option<String>,
    pub screen_url: Option<String>,
    pub screen_large_url: Option<String>,
    pub small_url: Option<String>,
    pub super_url: Option<String>,
    pub thumb_url: Option<String>,
    pub tiny_url: Option<String>,
    pub original_url: String,
    #[serde(default)]
    pub image_tags: Option<String>, // Changed from Vec<ImageTag> to String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageTag {
    pub api_detail_url: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameImage {
    pub icon_url: String,
    pub medium_url: String,
    pub screen_url: String,
    pub screen_large_url: String,
    pub small_url: String,
    pub super_url: String,
    pub thumb_url: String,
    pub tiny_url: String,
    pub original_url: String,
    pub image_tags: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Platform {
    pub id: u32,
    pub name: String,
    pub abbreviation: Option<String>,
    #[serde(default)]
    pub install_base: Option<u64>,
    #[serde(default)]
    pub original_price: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub id: u32,
    pub name: String,
    pub abbreviation: Option<String>,
    pub deck: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub install_base: Option<u64>,
    pub original_price: Option<String>,
    pub release_date: Option<String>,
    pub online_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Developer {
    pub id: u32,
    pub name: String,
}

/// Deserialize a string or number to u64
pub fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct StringOrU64Visitor;

    impl<'de> Visitor<'de> for StringOrU64Visitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or u64")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(InnerVisitor)
        }
    }

    struct InnerVisitor;

    impl<'de> Visitor<'de> for InnerVisitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or u64")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value
                .parse::<u64>()
                .map(Some)
                .map_err(|_| de::Error::custom(format!("cannot parse '{value}' as u64")))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }
    }

    deserializer.deserialize_option(StringOrU64Visitor)
}
