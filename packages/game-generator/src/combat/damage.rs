use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Type of damage dealt in combat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum DamageType {
    Physical,
    Magical,
    Eldritch,
    Corrupted,
    True,
}

/// Stats relevant for damage calculation
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct CombatStats {
    pub attack: f32,
    pub defense: f32,
    pub magic_attack: f32,
    pub magic_defense: f32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            attack: 10.0,
            defense: 5.0,
            magic_attack: 10.0,
            magic_defense: 5.0,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
        }
    }
}

/// Event fired when damage is dealt
#[derive(Event, Debug, Clone, Reflect)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage_type: DamageType,
    pub raw_amount: f32,
    pub is_critical: bool,
}

/// Configuration for damage calculation
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct DamageConfig {
    pub variance: f32,
    pub min_damage: f32,
}

impl Default for DamageConfig {
    fn default() -> Self {
        Self {
            variance: 0.1,
            min_damage: 1.0,
        }
    }
}

/// System for calculating and applying damage
pub fn calculate_damage(
    attacker_stats: &CombatStats,
    target_stats: &CombatStats,
    damage_type: DamageType,
    config: &DamageConfig,
) -> (f32, bool) {
    // Check for critical hit
    let is_critical = rand::random::<f32>() < attacker_stats.crit_chance;

    let base_damage = match damage_type {
        DamageType::Physical => (attacker_stats.attack * 2.0 - target_stats.defense).max(0.0),
        DamageType::Magical => {
            (attacker_stats.magic_attack * 2.0 - target_stats.magic_defense).max(0.0)
        }
        DamageType::Eldritch => {
            // Eldritch damage scales with both, but targets lower defense
            let power = (attacker_stats.attack + attacker_stats.magic_attack) * 0.75;
            let target_def = target_stats.defense.min(target_stats.magic_defense);
            (power * 2.0 - target_def).max(0.0)
        }
        DamageType::Corrupted => {
            // Corrupted damage partially ignores defense
            (attacker_stats.attack * 1.5 - target_stats.defense * 0.5).max(0.0)
        }
        DamageType::True => attacker_stats.attack,
    };

    let mut final_damage = base_damage;

    // Apply critical multiplier
    if is_critical {
        final_damage *= attacker_stats.crit_multiplier;
    }

    // Apply variance
    let variance_factor = 1.0 + (rand::random::<f32>() * 2.0 - 1.0) * config.variance;
    final_damage *= variance_factor;

    (final_damage.max(config.min_damage), is_critical)
}
