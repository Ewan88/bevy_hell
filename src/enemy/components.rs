use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub last_damage: f64,
}

impl Enemy {
    pub fn receive_damage(&mut self, damage: f32) {
        self.health -= damage;
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub countdown: Timer,
}

impl SpawnTimer {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();
        Self {
            countdown: Timer::from_seconds(rng.gen_range(0.5..2.), TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct AttackTimer {
    pub countdown: Timer,
}

impl AttackTimer {
    pub fn new() -> Self {
        Self {
            countdown: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}
