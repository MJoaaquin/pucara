use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::components::Player;
use crate::player::PLAYER_SIZE;
use crate::resources::Points;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// TODO: this system logic should be change when we implement a physycs crate
pub fn world_limit(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > x_max {
            translation.y = y_max
        }

        transform.translation = translation;
    }
}

pub fn exit_game(mut event_writter: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        println!("Saliste del Juego 😭");
        event_writter.send(AppExit)
    }
}

pub fn show_points(points: Res<Points>) {
    if points.is_changed() {
        println!("{} enemigos muertos! 👾", points.value)
    }
}
