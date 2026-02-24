use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    // Core Identity
    pub name: String,
    pub tagline: String,
    pub genre: String,
    pub setting: String,
    pub era: String,

    // Visual Style
    pub art_style: ArtStyle,
    pub color_palette: ColorPalette,
    pub reference_games: Vec<String>,

    // World Design
    pub world: WorldConfig,
    pub towns: Vec<TownConfig>,
    pub dungeons: Vec<DungeonConfig>,

    // RPG Systems
    pub party_system: PartySystem,
    pub combat_system: CombatSystem,
    pub dialog_system: DialogSystem,
    pub inventory_system: InventorySystem,
    pub shop_system: ShopSystem,
    pub quest_system: QuestSystem,

    // Story
    pub main_quest: QuestLine,
    pub side_quests: Vec<QuestLine>,
    pub characters: Vec<Character>,

    // Audio
    pub music_style: String,
    pub sound_effects_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtStyle {
    pub sprite_size: u32,
    pub tile_size: u32,
    pub animation_frames: HashMap<String, u32>,
    pub perspective: String, // "3/4 top-down"
    pub shading: String,
    pub outline: OutlineStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: Vec<String>, // Hex colors
    pub secondary: Vec<String>,
    pub ui: Vec<String>,
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineStyle {
    pub enabled: bool,
    pub color: String,
    pub thickness: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub name: String,
    pub size: String, // "medium", "large", etc.
    pub regions: Vec<Region>,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub biome: String,
    pub description: String,
    pub key_locations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub connection_type: String, // "road", "bridge", "cave", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownConfig {
    pub name: String,
    pub size: String,
    pub description: String,
    pub shops: Vec<String>,
    pub key_npcs: Vec<String>,
    pub inn: bool,
    pub save_point: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonConfig {
    pub name: String,
    pub theme: String,
    pub floors: u32,
    pub boss: String,
    pub treasures: Vec<String>,
    pub gimmick: String, // Unique mechanic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartySystem {
    pub max_party_size: u32,
    pub switchable: bool,
    pub formation_system: bool,
    pub character_classes: Vec<CharacterClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClass {
    pub name: String,
    pub description: String,
    pub stat_growth: HashMap<String, String>,
    pub abilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSystem {
    pub style: String, // "turn-based", "atb", "tactical"
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogSystem {
    pub style: String, // "branching", "linear"
    pub portrait_style: String,
    pub text_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySystem {
    pub grid_based: bool,
    pub capacity: String,
    pub categories: Vec<String>,
    pub equipment_slots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopSystem {
    pub currency: String,
    pub haggling: bool,
    pub shop_types: Vec<String>,
    pub special_shops: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSystem {
    pub journal: bool,
    pub markers: bool,
    pub reward_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestLine {
    pub name: String,
    pub description: String,
    pub steps: Vec<QuestStep>,
    pub rewards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestStep {
    pub description: String,
    pub objective_type: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub role: String,
    pub personality: String,
    pub backstory: String,
    pub portrait_description: String,
}

// Internal types for generation
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldData {
    pub regions: Vec<RegionData>,
    pub connections: Vec<ConnectionData>,
    pub towns: Vec<TownData>,
    pub dungeons: Vec<DungeonData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionData {
    pub name: String,
    pub map_data: String,
    pub encounters: Vec<String>,
    pub music: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionData {
    pub from: String,
    pub to: String,
    pub connection_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownData {
    pub name: String,
    pub map_data: String,
    pub npcs: Vec<NpcData>,
    pub shops: Vec<ShopData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonData {
    pub name: String,
    pub floors: Vec<FloorData>,
    pub boss: BossData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcData {
    pub name: String,
    pub sprite: String,
    pub dialog_tree: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopData {
    pub name: String,
    pub inventory: Vec<String>,
    pub prices: HashMap<String, u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorData {
    pub layout: String,
    pub encounters: Vec<String>,
    pub treasures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BossData {
    pub name: String,
    pub sprite: String,
    pub attacks: Vec<String>,
    pub dialog: String,
}
