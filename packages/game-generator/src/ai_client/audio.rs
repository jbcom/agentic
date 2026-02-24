//! Audio generation module for game music and sound effects
//!
//! Features:
//! - 16-bit style chiptune music generation
//! - Sound effect generation with retro constraints
//! - MIDI pattern generation for game loops
//! - Audio style consistency across tracks

use anyhow::{Context, Result};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    AiGenerator,
    cache::{AiCache, CachedData},
    tokens::TokenCounter,
};

/// Audio generator for music and sound effects
#[derive(Clone)]
pub struct AudioGenerator {
    client: Arc<Client<OpenAIConfig>>,
    cache: Arc<Mutex<AiCache>>,
    token_counter: Arc<Mutex<TokenCounter>>,
    template_env: Arc<Mutex<Environment<'static>>>,
}

/// Configuration for audio generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Style preset (e.g., "snes_rpg", "genesis_action")
    pub style: String,
    /// Duration in seconds
    pub duration: f32,
    /// Tempo in BPM
    pub tempo: u16,
    /// Key signature
    pub key: String,
    /// Time signature
    pub time_signature: String,
    /// Instruments to use
    pub instruments: Vec<String>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            style: "snes_rpg".to_string(),
            duration: 60.0,
            tempo: 120,
            key: "C major".to_string(),
            time_signature: "4/4".to_string(),
            instruments: vec![
                "square_wave".to_string(),
                "triangle_wave".to_string(),
                "noise".to_string(),
            ],
        }
    }
}

impl AudioConfig {
    /// Configuration for battle music
    pub fn for_battle() -> Self {
        Self {
            style: "intense_battle".to_string(),
            duration: 90.0,
            tempo: 140,
            key: "D minor".to_string(),
            time_signature: "4/4".to_string(),
            instruments: vec![
                "square_wave".to_string(),
                "sawtooth_wave".to_string(),
                "noise_drums".to_string(),
                "bass_pulse".to_string(),
            ],
        }
    }

    /// Configuration for exploration music
    pub fn for_exploration() -> Self {
        Self {
            style: "peaceful_exploration".to_string(),
            duration: 120.0,
            tempo: 90,
            key: "G major".to_string(),
            time_signature: "3/4".to_string(),
            instruments: vec![
                "triangle_wave".to_string(),
                "sine_wave".to_string(),
                "soft_square".to_string(),
            ],
        }
    }

    /// Configuration for menu/UI sounds
    pub fn for_ui() -> Self {
        Self {
            style: "ui_sounds".to_string(),
            duration: 0.5,
            tempo: 120,
            key: "C major".to_string(),
            time_signature: "4/4".to_string(),
            instruments: vec!["square_wave".to_string()],
        }
    }
}

impl AudioGenerator {
    /// Create a new audio generator
    pub fn new(
        client: Arc<Client<OpenAIConfig>>,
        cache: Arc<Mutex<AiCache>>,
        token_counter: Arc<Mutex<TokenCounter>>,
    ) -> Self {
        let mut env = Environment::new();

        // Load all audio prompt templates
        let templates = [
            (
                "theme_song",
                include_str!("../prompts/audio/theme_song.jinja"),
            ),
            (
                "battle_music",
                include_str!("../prompts/audio/battle_music.jinja"),
            ),
            (
                "victory_fanfare",
                include_str!("../prompts/audio/victory_fanfare.jinja"),
            ),
            (
                "sound_effect",
                include_str!("../prompts/audio/sound_effect.jinja"),
            ),
        ];

        for (name, template) in templates {
            env.add_template(name, template).ok();
        }

        Self {
            client,
            cache,
            token_counter,
            template_env: Arc::new(Mutex::new(env)),
        }
    }

    /// Generate music track description (to be converted to MIDI/audio externally)
    pub async fn generate_music_description(
        &self,
        music_type: &str,
        config: AudioConfig,
    ) -> Result<MusicDescription> {
        // Determine which template to use
        let template_name = match music_type {
            "battle" => "battle_music",
            "victory" => "victory_fanfare",
            "theme" | "main" => "theme_song",
            _ => "theme_song", // default
        };

        // Prepare context for template
        let context = json!({
            "game_name": "Vintage RPG",
            "style": config.style,
            "mood": match music_type {
                "battle" => "intense and dramatic",
                "victory" => "triumphant and celebratory",
                "exploration" => "mysterious and adventurous",
                _ => "epic and nostalgic"
            },
            "tempo": config.tempo,
            "key": config.key,
            "time_signature": config.time_signature,
            "duration": config.duration,
            "instrumentation": config.instruments.join(", "),
            "genre": "16-bit RPG",
            "constraints": "Use only retro synthesizer sounds, chiptune elements",
        });

        // Generate cache key
        let mut params = HashMap::new();
        params.insert("type".to_string(), music_type.to_string());
        params.insert("style".to_string(), config.style.clone());
        params.insert("tempo".to_string(), config.tempo.to_string());

        let cache_key = self
            .cache
            .lock()
            .await
            .generate_key("audio_music", template_name, &params);

        // Check cache
        if let Some(cached) = self.cache.lock().await.get(&cache_key).await
            && let CachedData::Text(data) = &cached.data
            && let Ok(description) = serde_json::from_str::<MusicDescription>(data)
        {
            return Ok(description);
        }

        // Render template
        let env = self.template_env.lock().await;
        let template = env
            .get_template(template_name)
            .context("Failed to get audio template")?;
        let prompt = template
            .render(&context)
            .context("Failed to render audio template")?;

        // Create message
        let messages = vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a professional video game music composer specializing in 16-bit era soundtracks. You create detailed technical specifications for music that captures the nostalgia and technical constraints of classic game consoles.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ];

        // Make API call
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages(messages)
            .temperature(0.8)
            .max_tokens(2000u32)
            .build()?;

        let response = self.client.chat().create(request).await?;
        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| anyhow::anyhow!("No response content"))?;

        // Parse response into structured format
        let description = self.parse_music_description(content, music_type, &config)?;

        // Cache result
        let cache_data = serde_json::to_string(&description)?;
        let mut cache_params = HashMap::new();
        for (k, v) in params {
            cache_params.insert(k, serde_json::Value::String(v));
        }
        self.cache
            .lock()
            .await
            .put(cache_key, CachedData::Text(cache_data), cache_params)
            .await?;

        // Track usage
        if let Some(usage) = response.usage {
            self.token_counter
                .lock()
                .await
                .record_usage(
                    "gpt-4o-mini",
                    usage.prompt_tokens as usize,
                    usage.completion_tokens as usize,
                )
                .await?;
        }

        Ok(description)
    }

    /// Parse AI response into structured music description
    fn parse_music_description(
        &self,
        content: &str,
        music_type: &str,
        config: &AudioConfig,
    ) -> Result<MusicDescription> {
        // Try to extract sections from the response
        let mut sections = Vec::new();

        // Common section patterns to look for
        let section_patterns = [
            "intro", "verse", "chorus", "bridge", "outro", "main", "loop",
        ];

        for pattern in &section_patterns {
            if content.to_lowercase().contains(pattern) {
                sections.push(MusicSection {
                    name: pattern.to_string(),
                    duration: config.duration / 5.0, // Simple division
                    description: format!("{pattern} section with characteristic elements"),
                });
            }
        }

        // Ensure we have at least basic structure
        if sections.is_empty() {
            sections = vec![
                MusicSection {
                    name: "intro".to_string(),
                    duration: 8.0,
                    description: "Opening section establishing the mood".to_string(),
                },
                MusicSection {
                    name: "main_loop".to_string(),
                    duration: config.duration - 16.0,
                    description: "Primary melodic content".to_string(),
                },
                MusicSection {
                    name: "outro".to_string(),
                    duration: 8.0,
                    description: "Closing section with fade".to_string(),
                },
            ];
        }

        Ok(MusicDescription {
            title: format!("{} - {}", capitalize_first(music_type), config.style),
            style: config.style.clone(),
            tempo: config.tempo,
            key: config.key.clone(),
            time_signature: config.time_signature.clone(),
            structure: sections,
            instruments: config.instruments.clone(),
            notes: content.to_string(),
        })
    }

    /// Generate sound effect description
    pub async fn generate_sound_effect(
        &self,
        effect_type: &str,
        duration: f32,
    ) -> Result<SoundEffectDescription> {
        // Prepare context for template
        let context = json!({
            "effect_type": effect_type,
            "duration": duration,
            "constraints": "16-bit era limitations",
            "waveforms": ["square", "triangle", "sawtooth", "noise"],
            "console": "SNES/Genesis era",
        });

        // Generate cache key
        let mut params = HashMap::new();
        params.insert("type".to_string(), effect_type.to_string());
        params.insert("duration".to_string(), duration.to_string());

        let cache_key = self
            .cache
            .lock()
            .await
            .generate_key("audio_sfx", effect_type, &params);

        // Check cache
        if let Some(cached) = self.cache.lock().await.get(&cache_key).await
            && let CachedData::Text(data) = &cached.data
            && let Ok(sfx) = serde_json::from_str::<SoundEffectDescription>(data)
        {
            return Ok(sfx);
        }

        // Render template
        let env = self.template_env.lock().await;
        let template = env
            .get_template("sound_effect")
            .context("Failed to get sound effect template")?;
        let prompt = template
            .render(&context)
            .context("Failed to render sound effect template")?;

        // Create message
        let messages = vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a sound designer specializing in retro video game audio. You create detailed synthesis parameters for authentic 16-bit era sound effects.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ];

        // Make API call
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages(messages)
            .temperature(0.7)
            .max_tokens(1000u32)
            .build()?;

        let response = self.client.chat().create(request).await?;
        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| anyhow::anyhow!("No response content"))?;

        // Parse response
        let sfx = self.parse_sound_effect(content, effect_type, duration)?;

        // Cache result
        let cache_data = serde_json::to_string(&sfx)?;
        let mut cache_params = HashMap::new();
        for (k, v) in params {
            cache_params.insert(k, serde_json::Value::String(v));
        }
        self.cache
            .lock()
            .await
            .put(cache_key, CachedData::Text(cache_data), cache_params)
            .await?;

        // Track usage
        if let Some(usage) = response.usage {
            self.token_counter
                .lock()
                .await
                .record_usage(
                    "gpt-4o-mini",
                    usage.prompt_tokens as usize,
                    usage.completion_tokens as usize,
                )
                .await?;
        }

        Ok(sfx)
    }

    /// Parse AI response into structured sound effect
    fn parse_sound_effect(
        &self,
        content: &str,
        effect_type: &str,
        duration: f32,
    ) -> Result<SoundEffectDescription> {
        // Extract waveform type
        let waveform = if content.contains("square") {
            "square"
        } else if content.contains("triangle") {
            "triangle"
        } else if content.contains("sawtooth") {
            "sawtooth"
        } else if content.contains("noise") {
            "noise"
        } else {
            "square" // default
        }
        .to_string();

        // Extract frequency values (look for Hz mentions)
        let freq_start = self.extract_frequency(content, "start", 440.0);
        let freq_end = self.extract_frequency(content, "end", freq_start / 2.0);

        // Extract ADSR values
        let envelope = self.extract_envelope(content, duration);

        // Extract effects
        let mut effects = Vec::new();
        if content.contains("echo") || content.contains("delay") {
            effects.push("echo".to_string());
        }
        if content.contains("reverb") {
            effects.push("reverb".to_string());
        }
        if content.contains("pitch") && content.contains("bend") {
            effects.push("pitch_bend".to_string());
        }

        Ok(SoundEffectDescription {
            name: effect_type.to_string(),
            duration,
            waveform,
            frequency_start: freq_start,
            frequency_end: freq_end,
            amplitude_envelope: envelope,
            effects,
        })
    }

    /// Extract frequency value from text
    fn extract_frequency(&self, text: &str, context: &str, default: f32) -> f32 {
        // Simple regex to find frequency values
        let pattern = format!(r"{context}[^\d]*(\d+)\s*[Hh]z");
        if let Ok(re) = regex::Regex::new(&pattern)
            && let Some(cap) = re.captures(text)
            && let Some(freq_str) = cap.get(1)
            && let Ok(freq) = freq_str.as_str().parse::<f32>()
        {
            return freq;
        }

        // Look for general frequency mentions
        if text.contains(&format!("{context} frequency"))
            && let Ok(re) = regex::Regex::new(r"(\d+)\s*[Hh]z")
        {
            for cap in re.captures_iter(text) {
                if let Some(freq_str) = cap.get(1)
                    && let Ok(freq) = freq_str.as_str().parse::<f32>()
                {
                    return freq;
                }
            }
        }

        default
    }

    /// Extract ADSR envelope from text
    fn extract_envelope(&self, text: &str, total_duration: f32) -> AmplitudeEnvelope {
        // Default ADSR values as percentages of total duration
        let mut attack = 0.01;
        let mut decay = 0.1;
        let mut sustain = 0.5;
        let mut release = 0.2;

        // Look for specific ADSR mentions
        if text.contains("fast attack") || text.contains("immediate") {
            attack = 0.001;
        } else if text.contains("slow attack") {
            attack = 0.1;
        }

        if text.contains("long decay") {
            decay = 0.3;
        } else if text.contains("short decay") || text.contains("quick decay") {
            decay = 0.05;
        }

        if text.contains("sustain") {
            if text.contains("high sustain") {
                sustain = 0.8;
            } else if text.contains("low sustain") {
                sustain = 0.3;
            }
        }

        if text.contains("long release") || text.contains("fade") {
            release = 0.5;
        } else if text.contains("short release") || text.contains("cut") {
            release = 0.05;
        }

        // Scale by actual duration
        AmplitudeEnvelope {
            attack: attack * total_duration.min(1.0),
            decay: decay * total_duration.min(1.0),
            sustain,
            release: release * total_duration.min(1.0),
        }
    }

    /// Generate a set of related sound effects
    pub async fn generate_sound_effect_set(
        &self,
        theme: &str,
        effects: Vec<String>,
    ) -> Result<HashMap<String, SoundEffectDescription>> {
        let mut results = HashMap::new();

        for effect in effects {
            let themed_effect = format!("{theme} {effect}");
            let description = self.generate_sound_effect(&themed_effect, 0.5).await?;
            results.insert(effect, description);
        }

        Ok(results)
    }
}

#[async_trait::async_trait]
impl AiGenerator for AudioGenerator {
    async fn estimate_tokens(&self, request: &str) -> Result<usize> {
        // Audio generation typically uses less tokens for descriptions
        Ok(request.len() / 4)
    }

    async fn estimate_cost(&self, _request: &str) -> Result<f64> {
        // Placeholder cost estimation
        Ok(0.01)
    }

    async fn is_cached(&self, key: &str) -> bool {
        self.cache.lock().await.get(key).await.is_some()
    }

    async fn clear_cache(&self, key: &str) -> Result<()> {
        self.cache.lock().await.clear(key).await
    }
}

/// Music track description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicDescription {
    pub title: String,
    pub style: String,
    pub tempo: u16,
    pub key: String,
    pub time_signature: String,
    pub structure: Vec<MusicSection>,
    pub instruments: Vec<String>,
    pub notes: String,
}

/// Music section description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSection {
    pub name: String,
    pub duration: f32,
    pub description: String,
}

/// Sound effect description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectDescription {
    pub name: String,
    pub duration: f32,
    pub waveform: String,
    pub frequency_start: f32,
    pub frequency_end: f32,
    pub amplitude_envelope: AmplitudeEnvelope,
    pub effects: Vec<String>,
}

/// ADSR envelope for amplitude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplitudeEnvelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

/// Retro audio constraints
pub mod constraints {
    use super::*;

    /// SNES audio constraints
    pub fn snes_constraints() -> AudioConstraints {
        AudioConstraints {
            channels: 8,
            sample_rate: 32000,
            bit_depth: 16,
            max_samples: 64,
            effects: vec![
                "echo".to_string(),
                "reverb".to_string(),
                "pitch_bend".to_string(),
            ],
        }
    }

    /// Genesis/Mega Drive constraints
    pub fn genesis_constraints() -> AudioConstraints {
        AudioConstraints {
            channels: 6, // FM synthesis
            sample_rate: 22050,
            bit_depth: 16,
            max_samples: 0, // No samples, pure FM
            effects: vec!["vibrato".to_string(), "tremolo".to_string()],
        }
    }

    /// NES constraints
    pub fn nes_constraints() -> AudioConstraints {
        AudioConstraints {
            channels: 5, // 2 square, 1 triangle, 1 noise, 1 sample
            sample_rate: 11025,
            bit_depth: 8,
            max_samples: 1,
            effects: vec!["sweep".to_string()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioConstraints {
    pub channels: usize,
    pub sample_rate: u32,
    pub bit_depth: u8,
    pub max_samples: usize,
    pub effects: Vec<String>,
}

/// Helper function to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
