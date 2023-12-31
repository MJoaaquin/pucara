use super::components::Player;
use super::{PLAYER_POSITION, PLAYER_SPEED};
use bevy::math::vec2;
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::bullet::components::Bullet;
use crate::game::enemy::components::Enemy;
use crate::game::resources::Health;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, PLAYER_POSITION, 0.0),
            texture: asset_server.load("sprites/ship_0010.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn move_primary_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    if let Ok(mut transform) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            direction = Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction = Vec3::new(1.0, 0.0, 0.0);
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// TODO: this system logic should be change when we implement a physycs crate
pub fn damage_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    mut health: ResMut<Health>,
) {
    if let Ok((player, player_transform)) = player_query.get_single_mut() {
        for transform in enemies_query.into_iter() {
            // check if some enemy is crashing with the player
            let collision = collide(
                player_transform.translation,
                vec2(16.0, 16.0),
                transform.translation,
                vec2(16.0, 16.0),
            );

            if let Some(_) = collision {
                println!("Chocaste! 🛩️💥");

                // reduce life from the player
                health.value -= 10;

                // if then player's life is equal to 0 kill it
                if matches!(health.value, 0) {
                    println!("Perdiste! 💀");
                    commands.entity(player).despawn();
                }
            }
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        if let Ok(transform) = player_query.get_single() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/tile_0000.png"),
                    ..default()
                },
                Bullet {
                    direction: Vec2::new(0.0, 1.0),
                },
            ));

            let shoot_sound = asset_server.load("audio/shoot.ogg");
            audio.play(shoot_sound);
        }
    }
}
