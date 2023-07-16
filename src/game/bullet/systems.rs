use bevy::prelude::*;

use super::components::*;
use super::BULLET_SPEED;

pub fn bullet_movement(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        let direction = Vec3::new(bullet.direction.x, bullet.direction.y, 0.0);
        transform.translation += direction * BULLET_SPEED * time.delta_seconds();
    }
}
