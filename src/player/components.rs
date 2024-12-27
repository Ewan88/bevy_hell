use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub recent_damage: bool,
    pub last_damage: f64,
    pub xp: u32,
    pub level: u32,
    pub next_level: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.,
            max_health: 100.,
            recent_damage: false,
            last_damage: 0.,
            xp: 0,
            level: 1,
            next_level: 1000,
        }
    }

    pub fn receive_damage(&mut self) {
        self.health -= 1.2;
        self.recent_damage = true;
    }

    pub fn xp_to_next_level(&self) -> u32 {
        2 * self.next_level
    }

    pub fn gain_xp(&mut self, xp: u32) {
        self.xp += xp;
    }
}
