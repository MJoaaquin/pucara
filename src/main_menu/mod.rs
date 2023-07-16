use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    }
}

fn main_menu() {
    println!("Estás en el menu principal")
}
