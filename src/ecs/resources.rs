use bevy::{math::Vec3, prelude::Entity};

use crate::engine::{TilePosition, WorldPosition};

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

pub struct PositionConverter {
    tile_size: u32,
}

impl PositionConverter {
    pub fn new(tile_size: u32) -> Self {
        Self { tile_size }
    }

    pub fn tile_from_translation(&self, pos: &Vec3) -> Option<TilePosition> {
        let wp: WorldPosition = pos.into();
        wp.to_tile_position(self.tile_size)
    }

    pub fn translation_from_tile(&self, tp: &TilePosition) -> Vec3 {
        let wp = tp.to_world_position(self.tile_size);
        wp.into()
    }

    pub fn translation_from_col_row(&self, (col, row): (u32, u32)) -> Vec3 {
        self.translation_from_tile(&TilePosition::centered(col, row, self.tile_size))
    }
}
