mod components;
mod styles;
mod systems;

use crate::main_menu::systems::layout::spawn_main_menu;
use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    }
}
