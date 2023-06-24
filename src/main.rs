use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const PLAYER_SPEED: f32 = 450.0;
const PLAYER_SIZE: f32 = 16.0;
const PLAYER_POSITION: f32 = 20.0;
const ENEMY_QUANTITY: u8 = 10;
const ENEMY_SPEED: f32 = 250.0;
const BULLET_SPEED: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_enemy_spawning)
        .add_system(spawn_enemies)
        .add_system(move_primary_player)
        .add_system(world_limit)
        .add_system(enemy_movement)
        .add_system(despawn_enemy)
        .add_system(damage_player)
        .add_system(shoot)
        .add_system(bullet_movement)
        .run()
}

#[derive(Component)]
pub struct Player {
    pub health: u8,
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnConfig {
    timer: Timer,
}

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
        Player { health: 50 },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn setup_enemy_spawning(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig {
        timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
    })
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemies_query: Query<&Enemy>,
    time: Res<Time>,
    mut config: ResMut<EnemySpawnConfig>,
) {
    let window = window_query.get_single().unwrap();
    config.timer.tick(time.delta());

    if config.timer.finished() {
        if enemies_query.iter().len() < ENEMY_QUANTITY.into() {
            let mut rng = rand::thread_rng();
            let rand_x: f32 = rng.gen_range(0.0..=window.width());
            let rand_y = window.height() - 10.0;

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                    texture: asset_server.load("sprites/ship_0022.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(0.0, -1.0).normalize(),
                },
            ));
        }
    }
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

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

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

fn despawn_enemy(mut commands: Commands, enemies_query: Query<(Entity, &Transform), With<Enemy>>) {
    for (entity, transform) in enemies_query.into_iter() {
        // if enemy is out of window kill it
        if transform.translation.y < -1.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn damage_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player, player_transform, mut player_information)) = player_query.get_single_mut() {
        for transform in enemies_query.into_iter() {
            // check if some enemy is crashing with the player
            if transform.translation.distance(player_transform.translation) < 16.0 {
                println!("Chocaste! 🛩️💥");

                // reduce life from the player
                player_information.health -= 10;

                // if then player's life is equal to 0 kill it
                if matches!(player_information.health, 0) {
                    println!("Perdiste! 💀");
                    commands.entity(player).despawn();
                }
            }
        }
    }
}

fn shoot(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
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
        }
    }
}

fn bullet_movement(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        let direction = Vec3::new(bullet.direction.x, bullet.direction.y, 0.0);
        transform.translation += direction * BULLET_SPEED * time.delta_seconds();
    }
}
