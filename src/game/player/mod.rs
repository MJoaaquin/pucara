use bevy::prelude::*;

pub mod components;
mod systems;

const PLAYER_SPEED: f32 = 450.0;
pub const PLAYER_SIZE: f32 = 16.0;
const PLAYER_POSITION: f32 = 20.0;

use crate::AppState;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(move_primary_player.run_if(in_state(AppState::InGame)))
            .add_system(damage_player.run_if(in_state(AppState::InGame)))
            .add_system(shoot.run_if(in_state(AppState::InGame)));
    }
}
