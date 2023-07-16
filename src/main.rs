mod game;
mod main_menu;
mod systems;

use crate::systems::{change_state, spawn_camera};
use bevy::prelude::*;

#[derive(States, Debug, Clone, Hash, Copy, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_startup_system(spawn_camera)
        .add_system(change_state)
        .run()
}
