use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

const ENEMY_QUANTITY: u8 = 10;
const ENEMY_SPEED: f32 = 250.0;

use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_enemy_spawning)
            .add_system(spawn_enemies)
            .add_system(enemy_movement)
            .add_system(despawn_enemy);
    }
}
