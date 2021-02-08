use crate::engine::TilePosition;

#[derive(Debug)]
pub struct ProjectileRequestedEvent {
    pub origin: TilePosition,
    pub direction: f32,
    pub velocity: f32,
    pub range: f32,
    pub health_damage: u16,
}

impl ProjectileRequestedEvent {
    pub fn new(
        origin: TilePosition,
        direction: f32,
        velocity: f32,
        range: f32,
        health_damage: u16,
    ) -> Self {
        Self {
            origin,
            direction,
            velocity,
            range,
            health_damage,
        }
    }
}
