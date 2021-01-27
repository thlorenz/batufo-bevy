use bevy::math::Vec3;

use crate::engine::{TilePosition, WorldPosition};

#[derive(Clone)]
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
