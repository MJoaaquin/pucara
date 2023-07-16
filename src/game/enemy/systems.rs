use rand::prelude::*;
use std::time::Duration;

use super::components::*;
use super::resources::*;
use super::{ENEMY_QUANTITY, ENEMY_SPEED};
use crate::game::bullet::components::Bullet;
use crate::game::resources::Points;
use bevy::math::vec2;
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::PrimaryWindow};

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

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

// TODO: this system logic should be change when we implement a physycs crate
pub fn despawn_enemy(
    mut commands: Commands,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in enemies_query.into_iter() {
        // if enemy is out of window kill it
        if transform.translation.y < -1.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn enemy_damage(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut points: ResMut<Points>,
) {
    for (enemy, enemy_transform) in enemies.into_iter() {
        for (bullet, bullet_transform) in bullets.into_iter() {
            let collition = collide(
                enemy_transform.translation,
                vec2(16.0, 16.0),
                bullet_transform.translation,
                vec2(1.0, 1.0),
            );

            if let Some(_) = collition {
                println!("Enemigo muerto! 🛩️💥");

                commands.entity(enemy).despawn();
                commands.entity(bullet).despawn();

                points.value += 1;
            }
        }
    }
}
