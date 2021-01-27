use bevy::prelude::Transform;
use crisscross::TileRaycaster;

use crate::{
    ai::{find_shot, Shot},
    arena::Tilepath,
};

use super::PositionConverter;

pub struct Sniper {
    tile_caster: TileRaycaster,
    converter: PositionConverter,
}

impl Sniper {
    pub fn new(tile_caster: TileRaycaster, converter: PositionConverter) -> Self {
        Self {
            tile_caster,
            converter,
        }
    }

    pub fn find_shot(
        &self,
        tile_path: &Tilepath,
        origin: &Transform,
        target: &Transform,
        range: Option<f32>,
    ) -> Option<Shot> {
        let origin_tile = self.converter.tile_from_translation(&origin.translation)?;
        let target_tile = self.converter.tile_from_translation(&target.translation)?;
        let shot = find_shot(&self.tile_caster, &tile_path, &origin_tile, &target_tile)?;
        match range {
            None => Some(shot),
            Some(range) if shot.distance <= range => Some(shot),
            Some(_) => None,
        }
    }
}
