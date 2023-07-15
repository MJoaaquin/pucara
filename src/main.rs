pub mod bullet;
pub mod enemy;
mod player;
mod resources;
mod systems;

use bevy::prelude::*;

use systems::*;

use crate::resources::{Health, Points};
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .init_resource::<Health>()
        .init_resource::<Points>()
        .add_plugins(DefaultPlugins)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_system(world_limit)
        .add_system(exit_game)
        .add_system(show_points)
        .run()
}
