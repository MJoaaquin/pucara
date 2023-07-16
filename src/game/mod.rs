use crate::game::bullet::BulletPlugin;
use crate::game::enemy::EnemyPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::resources::{Health, Points};
use crate::game::systems::{exit_game, show_health, show_points, world_limit};
use crate::AppState;
use bevy::prelude::*;

pub mod bullet;
pub mod enemy;
pub mod player;
pub mod resources;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Health>()
            .init_resource::<Points>()
            .add_plugins(DefaultPlugins)
            .add_plugin(BulletPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_systems(
                (show_points, show_health, world_limit, exit_game)
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}
