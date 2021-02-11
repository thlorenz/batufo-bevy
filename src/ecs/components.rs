use bevy::math::Vec3;

use crate::engine::TilePosition;

mod orthogonal_mover;
pub use orthogonal_mover::*;

#[derive(Default)]
pub struct Hero;

#[derive(Default)]
pub struct HeroHull;

#[derive(Default)]
pub struct Velocity(pub Vec3);

#[derive(Default)]
pub struct HeadLights(pub bool);

pub struct FloorTile(pub TilePosition);

pub struct Projectile {
    pub origin: Vec3,
    /// Squared range of projectile for world position, taking tile_size into account.
    pub range: f32,

    /// Damage to deal out to life cycle components on collision
    pub health_damage: u16,
}

pub struct ProjectileSpawner {
    /// Squared range of spawned projectiles for normalized Tilemap (tile_size: 1)
    pub range: f32,
    pub ticks_to_reload: u16,
    pub ticks_until_reloaded: u16,
    pub health_damage: u16,
}

impl Default for ProjectileSpawner {
    fn default() -> Self {
        Self {
            range: 10.0,
            ticks_to_reload: 60 * 5,
            ticks_until_reloaded: 0,
            health_damage: 1,
        }
    }
}

impl ProjectileSpawner {
    pub fn is_ready(&self) -> bool {
        self.ticks_until_reloaded == 0
    }
}

pub struct HeroFollower;
pub struct HeroShooter;

pub struct LifeCycle {
    health: u16,
}
impl Default for LifeCycle {
    fn default() -> Self {
        Self { health: 100 }
    }
}
impl LifeCycle {
    pub fn health(&self) -> u16 {
        self.health
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
    pub fn deduct(&mut self, amount: u16) {
        self.health -= amount;
    }
    pub fn terminate(&mut self) {
        self.health = 0;
    }
}
