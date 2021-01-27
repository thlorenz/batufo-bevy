use crate::engine::TilePosition;

use super::EntityTile;

#[derive(Default)]
pub struct TileState {
    pub hovered_tile: Option<EntityTile>,
    pub hero_tile: Option<TilePosition>,
    pub path_hovered_to_hero: Option<Vec<(u32, u32)>>,
}
