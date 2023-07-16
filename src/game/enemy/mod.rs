use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

const ENEMY_QUANTITY: u8 = 10;
const ENEMY_SPEED: f32 = 250.0;

use crate::AppState;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_enemy_spawning).add_systems(
            (spawn_enemies, enemy_movement, despawn_enemy, enemy_damage)
                .in_set(OnUpdate(AppState::InGame)),
        );
    }
}
