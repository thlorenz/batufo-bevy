use bevy::math::Vec3;

use crate::engine::position::TilePosition;

#[derive(Default)]
pub struct Hero;

#[derive(Default)]
pub struct Speed;

#[derive(Default)]
pub struct Velocity(pub Vec3);

#[derive(Default)]
pub struct HeadLights(pub bool);

pub struct FloorTile(pub TilePosition);
