use crate::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn change_state(
    mut commands: Commands,
    state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        match state.0 {
            AppState::MainMenu => commands.insert_resource(NextState(Some(AppState::InGame))),
            AppState::InGame => commands.insert_resource(NextState(Some(AppState::MainMenu))),
            AppState::GameOver => commands.insert_resource(NextState(Some(AppState::MainMenu))),
        }
    }
}
