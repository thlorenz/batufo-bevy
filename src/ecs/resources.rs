use bevy::prelude::Entity;

use crate::engine::position::TilePosition;

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

#[derive(Default)]
pub struct TileState {
    pub hovered_tile: Option<EntityTile>,
    pub hero_tile: Option<TilePosition>,
    pub path_hovered_to_hero: Option<Vec<(u32, u32)>>,
}
