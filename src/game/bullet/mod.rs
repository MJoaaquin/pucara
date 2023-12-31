use bevy::prelude::*;

pub mod components;
mod systems;

const BULLET_SPEED: f32 = 800.0;

use crate::AppState;
use systems::bullet_movement;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_movement.run_if(in_state(AppState::InGame)));
    }
}
