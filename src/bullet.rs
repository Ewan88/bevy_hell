use bevy::prelude::*;

use crate::enemy::Enemy;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub lifetime: Timer,
}

#[allow(dead_code)]
impl Bullet {
    pub fn new(direction: Vec2) -> Self {
        Self {
            direction,
            lifetime: Timer::from_seconds(3., TimerMode::Once),
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bullet_movement, bullet_lifetime, bullet_collision));
    }
}

fn bullet_movement(
    time: Res<Time>,
    mut bullet_query: Query<(&mut Transform, &Bullet), With<Bullet>>,
) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        transform.translation.x += bullet.direction.x * 400. * time.delta_seconds();
        transform.translation.y += bullet.direction.y * 400. * time.delta_seconds();
    }
}

fn bullet_lifetime(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in bullet_query.iter_mut() {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_collision(
    mut bullet_query: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Enemy, &Transform), (With<Enemy>, Without<Bullet>)>,
) {
    for bullet_transform in bullet_query.iter_mut() {
        for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
            if bullet_transform
                .translation
                .distance(enemy_transform.translation)
                < 32.
            {
                enemy.receive_damage(10.);
            }
        }
    }
}
