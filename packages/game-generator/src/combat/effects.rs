use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Types of status effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum EffectType {
    Poison,
    Bleed,
    Stun,
    Haste,
    Slow,
    Madness,
    VoidCorruption,
}

/// A single instance of a status effect
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct StatusEffect {
    pub effect_type: EffectType,
    pub power: f32,
    #[serde(skip)]
    pub duration: Timer,
    pub source: Option<Entity>,
}

/// Component that tracks all active status effects on an entity
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct EffectRegistry {
    pub effects: Vec<StatusEffect>,
}

impl EffectRegistry {
    pub fn add_effect(&mut self, effect: StatusEffect) {
        // Simple stacking logic: if effect of same type exists, refresh duration if new one is stronger or longer
        // More complex stacking could be implemented here
        if let Some(existing) = self
            .effects
            .iter_mut()
            .find(|e| e.effect_type == effect.effect_type)
        {
            if effect.power >= existing.power {
                existing.power = effect.power;
                existing.duration = effect.duration;
            }
        } else {
            self.effects.push(effect);
        }
    }

    pub fn remove_effect(&mut self, effect_type: EffectType) {
        self.effects.retain(|e| e.effect_type != effect_type);
    }

    pub fn has_effect(&self, effect_type: EffectType) -> bool {
        self.effects.iter().any(|e| e.effect_type == effect_type)
    }
}

/// System that updates status effect timers and removes expired ones
pub fn update_effects(time: Res<Time>, mut query: Query<&mut EffectRegistry>) {
    for mut registry in query.iter_mut() {
        registry.effects.retain_mut(|effect| {
            effect.duration.tick(time.delta());
            !effect.duration.finished()
        });
    }
}

/// Example system for handling Madness effect
pub fn handle_madness(mut query: Query<(&EffectRegistry, &mut Transform)>, time: Res<Time>) {
    for (registry, mut transform) in query.iter_mut() {
        if let Some(madness) = registry
            .effects
            .iter()
            .find(|e| e.effect_type == EffectType::Madness)
        {
            // Madness causes erratic movement
            let jitter = (time.elapsed_secs() * 10.0).sin() * madness.power * 0.1;
            transform.translation.x += jitter;
        }
    }
}
