use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component tracking experience and levels
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Progression {
    pub level: u32,
    pub experience: u32,
    pub next_level_xp: u32,
}

impl Default for Progression {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
            next_level_xp: 100,
        }
    }
}

impl Progression {
    /// Add XP and return number of levels gained
    pub fn add_xp(&mut self, amount: u32) -> u32 {
        self.experience += amount;
        let mut levels_gained = 0;

        while self.experience >= self.next_level_xp {
            self.experience -= self.next_level_xp;
            self.level += 1;
            levels_gained += 1;
            // Progressive XP curve (example: each level needs 20% more XP)
            self.next_level_xp = (self.next_level_xp as f32 * 1.2) as u32;
        }

        levels_gained
    }
}

/// Event fired when an entity levels up
#[derive(Event, Debug, Clone, Reflect)]
pub struct LevelUpEvent {
    pub entity: Entity,
    pub new_level: u32,
}

/// System that handles XP gain from combat
pub fn handle_xp_gain(
    _commands: Commands,
    mut level_up_events: EventWriter<LevelUpEvent>,
    mut query: Query<(Entity, &mut Progression)>,
) {
    for (entity, mut progression) in query.iter_mut() {
        // In a real game, this would be based on actual combat results
        // For the template, we just show how levels are processed
        if progression.experience >= progression.next_level_xp {
            let old_level = progression.level;
            let levels_gained = progression.add_xp(0);
            if levels_gained > 0 {
                info!(
                    "Entity {:?} leveled up: {} -> {}",
                    entity, old_level, progression.level
                );
                level_up_events.write(LevelUpEvent {
                    entity,
                    new_level: progression.level,
                });
            }
        }
    }
}
