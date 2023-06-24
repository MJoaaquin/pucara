pub mod bullet;
pub mod enemy;
mod player;
mod systems;

use bevy::prelude::*;

use systems::*;

use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_system(world_limit)
        .add_system(exit_game)
        .run()
}
