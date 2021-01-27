use bevy::ecs::Entity;

use crate::engine::TilePosition;

#[derive(Debug, Clone)]
pub struct EntityTile {
    pub entity: Entity,
    pub position: TilePosition,
}

impl From<(Entity, TilePosition)> for EntityTile {
    fn from(x: (Entity, TilePosition)) -> Self {
        EntityTile {
            entity: x.0,
            position: x.1,
        }
    }
}
