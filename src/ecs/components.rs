use bevy::math::Vec3;

use crate::engine::TilePosition;

mod orthogonal_mover;
pub use orthogonal_mover::*;

#[derive(Default)]
pub struct Hero;

#[derive(Default)]
pub struct Velocity(pub Vec3);

#[derive(Default)]
pub struct HeadLights(pub bool);

pub struct FloorTile(pub TilePosition);

pub struct ProjectileSpawner {
    pub range: f32,
}

pub struct HeroFollower;
