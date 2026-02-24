pub mod damage;
pub mod effects;
pub mod progression;
pub mod state;

use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for reflection
            .register_type::<damage::CombatStats>()
            .register_type::<damage::DamageConfig>()
            .register_type::<effects::EffectRegistry>()
            .register_type::<progression::Progression>()
            .register_type::<state::CombatState>()
            .register_type::<state::CombatManager>()
            // Add states
            .init_state::<state::CombatState>()
            // Add resources
            .init_resource::<damage::DamageConfig>()
            .init_resource::<state::CombatManager>()
            // Add events
            .add_event::<damage::DamageEvent>()
            .add_event::<progression::LevelUpEvent>()
            // Add systems
            .add_systems(
                Update,
                (
                    effects::update_effects,
                    effects::handle_madness,
                    state::manage_combat_state,
                ),
            );
    }
}

/// Prelude for easy access to combat types
pub mod prelude {
    pub use crate::damage::{CombatStats, DamageConfig, DamageEvent, DamageType};
    pub use crate::effects::{EffectRegistry, EffectType, StatusEffect};
    pub use crate::progression::{LevelUpEvent, Progression};
    pub use crate::state::{CombatManager, CombatState};
    pub use crate::CombatPlugin;
}
